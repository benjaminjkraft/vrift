use crate::floor;
use crate::setup::SetupStats;
#[cfg(test)]
use insta::assert_snapshot;
use js_sys;
use wasm_bindgen::prelude::*;

const INTERNAL_EPSILON: f64 = 1e-50;
const PRINT_EPSILON: f64 = 1e-4;
type HuntNum = i16;
type Dist = Vec<f64>; // length: floor::MAX_STEP

#[derive(Debug)]
struct State {
    dist: Dist,
    end_dist: Dist,
    start_step: floor::StepNum,
    end_step: floor::StepNum,
    hunt: HuntNum,
}

impl State {
    fn done(&self) -> bool {
        return self.start_step >= self.end_step;
    }

    fn start() -> State {
        let mut dist = vec![0.0; floor::MAX_STEP];
        dist[0] = 1.0;
        return State {
            dist,
            end_dist: vec![0.0; floor::MAX_STEP],
            start_step: 0,
            end_step: 1,
            hunt: 0,
        };
    }
}

#[wasm_bindgen]
pub struct Plan {
    speed: floor::StepNum,
    ta_mult: floor::StepNum,
    sync: HuntNum,
    siphon: HuntNum,

    uu: bool,

    step_lookup_table: floor::StepLookupTable,
    setup_lookup_table: Vec<SetupStats>,
    outcome_lookup_table: Vec<floor::Outcome>,
}

// TODO: should go in lib.rs since it's just for js?
#[wasm_bindgen]
impl Plan {
    #[wasm_bindgen(constructor)]
    pub fn new_for_js(
        // TODO: maybe just make this a struct?
        speed_level: u8,
        sync_level: u8,
        siphon_level: u8,
        super_siphon: bool,
        uu: bool,
        string_stepping: bool,
        // TODO: make this typed in TS as `(floor: number) => SetupStats`
        setup: &js_sys::Function,
    ) -> Plan {
        return Plan::new(
            speed_level,
            sync_level,
            siphon_level,
            super_siphon,
            uu,
            string_stepping,
            |f| {
                let js_ret = setup.call1(&JsValue::null(), &JsValue::from(f)).unwrap();
                SetupStats::try_from(js_ret).unwrap()
            },
        );
    }
}

impl Plan {
    pub fn new(
        speed_level: u8,
        // TODO: allow partial run (2 constructors?)
        sync_level: u8,
        siphon_level: u8,
        super_siphon: bool,
        uu: bool,
        string_stepping: bool,
        // TODO: handle change-within-floor by making this a fn of steps
        setup: impl Fn(floor::FloorNum) -> SetupStats,
    ) -> Plan {
        let step_lookup_table = floor::make_step_lookup_table();
        let floors = floor::Floor::all(uu);
        let setup_lookup_table: Vec<SetupStats> =
            floors.iter().map(|floor| setup(floor.num)).collect();
        let outcome_lookup_table = floors
            .iter()
            .map(|floor| floor.hunt(&setup_lookup_table[floor.num]))
            .collect();

        return Plan {
            speed: speed_level as floor::StepNum,
            ta_mult: if string_stepping { 4 } else { 2 },
            sync: 30 + 10 * (sync_level as HuntNum),
            siphon: 5 * (siphon_level as HuntNum) * if super_siphon { 2 } else { 1 },
            uu,
            step_lookup_table,
            setup_lookup_table,
            outcome_lookup_table,
        };
    }

    fn min_step_for_hunt(&self, hunt: HuntNum) -> floor::StepNum {
        // end if
        //  sync + siphon*(round-1) > hunt+1,
        // or equivalently if
        //  r > (hunt+1-sync)/siphon + 1
        // but handle the mathematical horror that is round-towards-zero. I think.
        // TODO: test
        let min_round = ((hunt + self.siphon + 1 - self.sync) / self.siphon + 1).max(1);
        return floor::floor_to_first_step((min_round * 8 - 7) as floor::FloorNum);
    }

    fn step(&self, prev: State) -> State {
        let mut next_dist: Dist = vec![0.0; floor::MAX_STEP];
        for step_before in prev.start_step..prev.end_step {
            let floor::StepInfo {
                floor,
                floor_start,
                next_eclipse,
            } = self.step_lookup_table[step_before];
            let setup = &self.setup_lookup_table[floor];
            let outcome = &self.outcome_lookup_table[floor];

            let speed = self.speed + if setup.cf { 1 } else { 0 };
            let prev_prob = prev.dist[step_before];
            next_dist[next_eclipse.min(step_before + speed)] += prev_prob * outcome.push;
            next_dist[next_eclipse.min(step_before + self.ta_mult * speed)] +=
                prev_prob * outcome.push_ta;
            if self.uu {
                next_dist[floor_start.max(step_before.max(5) - 5)] += prev_prob * outcome.fail;
                next_dist[floor_start.max(step_before.max(10) - 10)] +=
                    prev_prob * outcome.fail_bulwark;
            } else {
                next_dist[step_before] += prev_prob * outcome.fail;
            }
        }

        let mut next_start = prev.start_step.max(10) - 10;
        let mut next_end = prev.end_step + (self.speed + 1) * self.ta_mult;
        for step in next_start..next_end {
            if next_dist[step] > INTERNAL_EPSILON {
                next_start = step;
                break;
            }
        }
        for step in (next_start..next_end).rev() {
            if next_dist[step] > INTERNAL_EPSILON {
                next_end = step + 1;
                break;
            }
        }

        let min_step = self.min_step_for_hunt(prev.hunt);
        let mut end_dist = prev.end_dist;
        if min_step > next_start {
            for step in next_start..min_step {
                end_dist[step] = next_dist[step];
                next_dist[step] = 0.0;
            }
            next_start = min_step;
        }

        return State {
            dist: next_dist,
            end_dist,
            start_step: next_start,
            end_step: next_end,
            hunt: prev.hunt + 1,
        };
    }

    pub fn sim(&self) -> Dist {
        let mut state = State::start();
        while !state.done() {
            state = self.step(state);
        }
        return state.end_dist;
    }

    pub fn dist_to_string(&self, dist: &Dist, prec: usize) -> String {
        let mut floor_dist = [0.0; floor::MAX_FLOOR + 1];
        for step in 0..floor::MAX_STEP {
            floor_dist[self.step_lookup_table[step].floor] += dist[step]
        }

        let mut cumulative = 1.0;
        let rows: Vec<(floor::FloorNum, f64, f64)> = floor_dist
            .into_iter()
            .enumerate()
            .skip_while(|&(_, prob)| prob < PRINT_EPSILON)
            .map(|(floor, prob)| {
                let row = (floor, prob, cumulative);
                cumulative -= prob;
                return row;
            })
            .collect();
        let last_index = rows
            .iter()
            .rposition(|&(_, prob, _)| prob > PRINT_EPSILON)
            .unwrap();
        let last_floor = rows[last_index].0;
        let rows_str = rows
            .into_iter()
            .take_while(|&(floor, _, _)| floor <= last_floor)
            .map(|(floor, prob, cumulative)| {
                return format!(
                    "{:>2} | {:>3} | {:>pwidth$.prec$}% | {:>cwidth$.prec$}%",
                    // note: doesn't count getting to the floor (i.e. core looting, not prestige)
                    (floor - 1) / 8,
                    floor,
                    100.0 * prob,
                    100.0 * cumulative,
                    pwidth = prec + 3,
                    cwidth = prec + 4,
                    prec = prec,
                );
            })
            .collect::<Vec<String>>()
            .join("\n");
        return format!("te |  fl | exact | at least\n{}", rows_str);
    }

    pub fn dist_to_string_short(&self, dist: &Dist, prec: usize) -> String {
        let mut te_dist = [0.0; floor::MAX_FLOOR / 8 + 1];
        for step in 0..floor::MAX_STEP {
            te_dist[self.step_lookup_table[step].floor / 8] += dist[step]
        }

        let mut cumulative = 1.0;
        let rows: Vec<(floor::FloorNum, f64, f64)> = te_dist
            .into_iter()
            .enumerate()
            .skip_while(|&(_, prob)| prob < PRINT_EPSILON)
            .map(|(te, prob)| {
                let row = (te, prob, cumulative);
                cumulative -= prob;
                return row;
            })
            .collect();
        let last_index = rows
            .iter()
            .rposition(|&(_, prob, _)| prob > PRINT_EPSILON)
            .unwrap();
        let last_te = rows[last_index].0;
        let rows_str = rows
            .into_iter()
            .take_while(|&(te, _, _)| te <= last_te)
            .map(|(te, prob, cumulative)| {
                return format!(
                    "{:>2} | {:>pwidth$.prec$}% | {:>cwidth$.prec$}%",
                    te,
                    100.0 * prob,
                    100.0 * cumulative,
                    pwidth = prec + 3,
                    cwidth = prec + 4,
                    prec = prec,
                );
            })
            .collect::<Vec<String>>()
            .join("\n");
        return format!("te | exact | at least\n{}", rows_str);
    }
}

#[test]
fn test_snapshots() {
    let tdt_aerb_r2023_2au = SetupStats::new(11416, 45, false);
    let ccdt_pb9_rusc_2au = SetupStats::new(14251, 84, false);
    let ccdt_pb9_rupc_2au = SetupStats::new(18650, 64, false);
    let ccdt_pb9_rupc_2au_cf = SetupStats::new(18650, 64, true);
    let ccdt_ssdb_rulpc_4au_cf = SetupStats::new(28600, 101, true);
    let uc_cf = SetupStats::uc(true);

    let plan = Plan::new(10, 7, 5, true, true, true, |_| uc_cf);
    let out = plan.dist_to_string(&plan.sim(), 3);
    assert_snapshot!("10:7:5 SSi:UU:SSt UC CF", out);

    let plan = Plan::new(2, 1, 1, true, true, true, |_| uc_cf);
    let out = plan.dist_to_string(&plan.sim(), 3);
    assert_snapshot!("2:1:1 SSi:UU:SSt UC CF", out);

    let plan = Plan::new(10, 7, 5, true, true, true, |f| {
        if f % 8 == 0 {
            uc_cf
        } else {
            ccdt_ssdb_rulpc_4au_cf
        }
    });
    let out = plan.dist_to_string(&plan.sim(), 3);
    assert_snapshot!("10:7:5 SSi:UU:SSt CCDT:SSDB:RULPC 4au CF UC-EC", out);

    let plan = Plan::new(10, 7, 5, true, true, false, |f| {
        if f % 8 == 0 {
            ccdt_pb9_rupc_2au_cf
        } else if f < 16 {
            ccdt_pb9_rusc_2au
        } else {
            ccdt_pb9_rupc_2au
        }
    });
    let out = plan.dist_to_string(&plan.sim(), 3);
    assert_snapshot!("10:7:5 SSi:UU CCDT:PB9:RUSC+RUPC 2au CF-EC", out);

    let plan = Plan::new(10, 7, 5, false, false, false, |_| tdt_aerb_r2023_2au);
    let out = plan.dist_to_string(&plan.sim(), 3);
    assert_snapshot!("10:7:5 TDT:AERB:R2023 2au", out);

    let plan = Plan::new(4, 3, 2, false, false, false, |_| tdt_aerb_r2023_2au);
    let out = plan.dist_to_string(&plan.sim(), 3);
    assert_snapshot!("4:3:2 TDT:AERB:R2023 2au", out);

    let plan = Plan::new(1, 1, 1, false, false, false, |_| tdt_aerb_r2023_2au);
    let out = plan.dist_to_string(&plan.sim(), 3);
    assert_snapshot!("2:1:1 TDT:AERB:R2023 2au", out);
}
