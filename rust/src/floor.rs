use crate::mouse;
use crate::mouse::Mouse;
use crate::setup::SetupStats;
#[cfg(test)]
use crate::util;

pub type FloorNum = usize;
pub type StepNum = usize;
pub const MAX_FLOOR: FloorNum = 200;
pub const MAX_STEP: StepNum = 24525;

#[derive(Debug)]
pub struct Floor {
    pub num: FloorNum,
    #[allow(unused)]
    name: String,
    mice: Vec<(&'static Mouse, f64)>,
}

#[derive(PartialEq, Debug)]
pub struct Outcome {
    pub push: f64,
    pub push_ta: f64,
    pub fail: f64,
    pub fail_bulwark: f64,
}

impl Floor {
    pub fn number(uu: bool, num: FloorNum) -> Self {
        if num == 0 {
            // Just for completeness, not actually used.
            return Self {
                num,
                name: String::from("Outside"),
                mice: vec![
                    (&mouse::ELIXIR_MAKER, 0.6897),
                    (&mouse::TIMID_EXPLORER, 0.3103),
                ],
            };
        }
        if num > MAX_FLOOR {
            panic!("good luck!");
        }

        let round = (num - 1) / 8 + 1;
        let name = format!(
            "{} {}",
            match num % 8 {
                0 => "Boss",
                1 => "Puppetry",
                2 => "Thievery",
                3 => "Melee",
                4 => "Bard",
                5 => "Magic",
                6 => "Noble",
                7 => "Dusty",
                _ => unreachable!(),
            },
            round
        );
        if num % 8 == 0 {
            return Self {
                num,
                name,
                mice: vec![(
                    if uu {
                        &mouse::THE_TOTAL_ECLIPSE
                    } else {
                        &mouse::SHADE_OF_THE_ECLIPSE
                    },
                    1.0,
                )],
            };
        } else {
            let mice = vec![
                (
                    match num % 8 {
                        0 => unreachable!(),
                        1 => &mouse::PUPPETTO,
                        2 => &mouse::CUTPURSE,
                        3 => &mouse::MARTIAL,
                        4 => &mouse::ONE_MOUSE_BAND,
                        5 => &mouse::MOUSE_OF_ELEMENTS,
                        6 => &mouse::CURSED_CRUSADER,
                        7 => &mouse::WITHERED_REMAINS,
                        _ => unreachable!(),
                    },
                    match (uu, round) {
                        (false, 1) => 0.5772,
                        (false, 2) => 0.3932,
                        (false, 3) => 0.3407,
                        (false, _) => 0.2961,
                        (true, 1) => 0.4620,
                        (true, 2) => 0.2890,
                        (true, 3) => 0.2700,
                        (true, _) => 0.2170,
                    },
                ),
                (
                    match num % 8 {
                        0 => unreachable!(),
                        1 => &mouse::PUPPET_CHAMPION,
                        2 => &mouse::CHAMPION_THIEF,
                        3 => &mouse::PRAETORIAN_CHAMPION,
                        4 => &mouse::CHAMPION_DANSEUSE,
                        5 => &mouse::MAGIC_CHAMPION,
                        6 => &mouse::FALLEN_CHAMPION_FOOTMAN,
                        7 => &mouse::ARCH_CHAMPION_NECROMANCER,
                        _ => unreachable!(),
                    },
                    match (uu, round) {
                        (_, 1) => 0.0,
                        (false, 2) => 0.1161,
                        (false, 3) => 0.1985,
                        (false, _) => 0.2884,
                        (true, 2) => 0.0730,
                        (true, 3) => 0.1320,
                        (true, _) => 0.2520,
                    },
                ),
                (
                    &mouse::UNWAVERING_ADVENTURER,
                    match (uu, round) {
                        (false, 1) => 0.1707,
                        (false, 2) => 0.0633,
                        (false, 3) => 0.0619,
                        (false, _) => 0.0857,
                        (true, 1) => 0.1150,
                        (true, 2) => 0.0720,
                        (true, 3) => 0.0650,
                        (true, _) => 0.0560,
                    },
                ),
                (
                    &mouse::BERZERKER,
                    match (uu, round) {
                        (false, 1) => 0.0407,
                        (false, 2) => 0.0158,
                        (false, 3) => 0.0237,
                        (false, _) => 0.0198,
                        (true, 1) => 0.0380,
                        (true, 2) => 0.0230,
                        (true, 3) => 0.0200,
                        (true, _) => 0.0170,
                    },
                ),
                (
                    &mouse::LUMI_LANCER,
                    match (uu, round) {
                        (false, 1) => 0.0325,
                        (false, 2) => 0.0158,
                        (false, 3) => 0.0273,
                        (false, _) => 0.0121,
                        (true, 1) => 0.0230,
                        (true, 2) => 0.0140,
                        (true, 3) => 0.0130,
                        (true, _) => 0.0090,
                    },
                ),
                (
                    &mouse::POSSESSED_ARMAMENTS,
                    match (uu, round) {
                        (_, 1) => 0.0,
                        (false, 2) => 0.2929,
                        (false, 3) => 0.1184,
                        (false, _) => 0.0363,
                        (true, 2) => 0.2310,
                        (true, 3) => 0.1080,
                        (true, _) => 0.0330,
                    },
                ),
                (
                    &mouse::PRESTIGIOUS_ADVENTURER,
                    match (uu, round) {
                        (_, 1) => 0.0,
                        (_, 2) => 0.0,
                        (false, 3) => 0.1275,
                        (false, _) => 0.0747,
                        (true, 3) => 0.0980,
                        (true, _) => 0.0550,
                    },
                ),
                (
                    &mouse::SOLDIER_OF_THE_SHADE,
                    match (uu, round) {
                        (_, 1) => 0.0,
                        (_, 2) => 0.0,
                        (_, 3) => 0.0,
                        (false, _) => 0.0973,
                        (true, _) => 0.0810,
                    },
                ),
                (
                    &mouse::TERRIFIED_ADVENTURER,
                    match (uu, round) {
                        (false, 1) => 0.1789,
                        (false, 2) => 0.1029,
                        (false, 3) => 0.1020,
                        (false, _) => 0.0896,
                        (true, 1) => 0.1830,
                        (true, 2) => 0.1120,
                        (true, 3) => 0.1020,
                        (true, _) => 0.0800,
                    },
                ),
                (
                    &mouse::BULWARK_OF_ASCENT,
                    match (uu, round) {
                        (false, _) => 0.0,
                        (true, 1) => 0.1790,
                        (true, 2) => 0.1860,
                        (true, 3) => 0.1920,
                        (true, _) => 0.2000,
                    },
                ),
            ];

            return Self {
                num,
                name,
                mice: mice.into_iter().filter(|&(_, prob)| prob > 0.0).collect(),
            };
        }
    }

    pub fn all(uu: bool) -> Vec<Floor> {
        return (0..=MAX_FLOOR).map(|num| Floor::number(uu, num)).collect();
    }

    pub fn hunt(&self, setup: &SetupStats) -> Outcome {
        let mut outcome = Outcome {
            push: 0.0,
            push_ta: 0.0,
            fail: 0.0,
            fail_bulwark: 0.0,
        };
        self.mice.iter().for_each(|&(mouse, prob)| {
            let cr = mouse.cr(setup);
            if mouse == &mouse::TERRIFIED_ADVENTURER {
                outcome.push_ta += cr * prob;
            } else {
                outcome.push += cr * prob;
            }
            if mouse == &mouse::BULWARK_OF_ASCENT {
                outcome.fail_bulwark += (1.0 - cr) * prob
            } else {
                outcome.fail += (1.0 - cr) * prob
            }
        });
        return outcome;
    }
}

pub fn floor_to_first_step(f: FloorNum) -> StepNum {
    let r = (f + 7) / 8 - 1;
    let m = (f + 7) % 8;
    return
        // to start of round
        70 * (r * (r+3) / 2) + r
        // to start of floor
        + (m * (r+2) * 10);
}

#[derive(Debug, Clone, Copy)]
pub struct StepInfo {
    pub floor: FloorNum,
    pub floor_start: StepNum,
    pub next_eclipse: StepNum,
}
pub type StepLookupTable = Vec<StepInfo>; // length: MAX_STEP

pub fn make_step_lookup_table() -> StepLookupTable {
    let mut table = Vec::new();
    let mut last = floor_to_first_step(1);
    for f in 1..=MAX_FLOOR {
        let next = floor_to_first_step(f + 1);
        let next_eclipse = floor_to_first_step(f - f % 8 + 8);
        for _ in last..next {
            table.push(StepInfo {
                floor: f,
                floor_start: last,
                next_eclipse,
            });
        }
        last = next
    }
    return table;
}

#[test]
fn test_floor_to_first_step() {
    assert_eq!(floor_to_first_step(1), 0);
    assert_eq!(floor_to_first_step(2), 20);
    assert_eq!(floor_to_first_step(3), 40);
    assert_eq!(floor_to_first_step(4), 60);
    assert_eq!(floor_to_first_step(5), 80);
    assert_eq!(floor_to_first_step(6), 100);
    assert_eq!(floor_to_first_step(7), 120);
    assert_eq!(floor_to_first_step(8), 140);
    assert_eq!(floor_to_first_step(9), 141);
    assert_eq!(floor_to_first_step(10), 171);
    assert_eq!(floor_to_first_step(11), 201);
    assert_eq!(floor_to_first_step(36), 1164);
    assert_eq!(floor_to_first_step(37), 1224);
    assert_eq!(MAX_STEP, floor_to_first_step(MAX_FLOOR + 1));

    let table = make_step_lookup_table();
    for s in 0..table.len() {
        assert!(
            floor_to_first_step(table[s].floor) <= s && s < floor_to_first_step(table[s].floor + 1)
        );
    }
}

#[test]
fn test_floors_add_to_100() {
    for uu in [true, false] {
        for (num, floor) in Floor::all(uu).into_iter().enumerate() {
            let total: f64 = floor.mice.into_iter().map(|(_, prob)| prob).sum();
            util::assert_approx_eq!(total, 1.0, "{} ({})", num, uu);
        }
    }
}

#[test]
fn test_run_floor() {
    let ccdt_rusc = SetupStats::new(14251, 84, false);
    let uc = SetupStats::uc(true);
    let f1 = Floor::number(false, 1);
    let f8 = Floor::number(false, 8);
    let f15 = Floor::number(false, 15);
    let f1uu = Floor::number(true, 1);
    let f8uu = Floor::number(true, 8);
    let f15uu = Floor::number(true, 15);

    let f1_ccdt_rusc = f1.hunt(&ccdt_rusc);
    util::assert_approx_eq!(f1_ccdt_rusc.push, 0.8211);
    util::assert_approx_eq!(f1_ccdt_rusc.push_ta, 0.1789);
    util::assert_approx_eq!(f1_ccdt_rusc.fail, 0.0);
    util::assert_approx_eq!(f1_ccdt_rusc.fail_bulwark, 0.0);

    let f8_ccdt_rusc = f8.hunt(&ccdt_rusc);
    util::assert_approx_eq!(f8_ccdt_rusc.push, 0.6719);
    util::assert_approx_eq!(f8_ccdt_rusc.push_ta, 0.0);
    util::assert_approx_eq!(f8_ccdt_rusc.fail, 0.3281);
    util::assert_approx_eq!(f8_ccdt_rusc.fail_bulwark, 0.0);

    let f15_ccdt_rusc = f15.hunt(&ccdt_rusc);
    util::assert_approx_eq!(f15_ccdt_rusc.push, 0.8490);
    util::assert_approx_eq!(f15_ccdt_rusc.push_ta, 0.1029);
    util::assert_approx_eq!(f15_ccdt_rusc.fail, 0.0481);
    util::assert_approx_eq!(f15_ccdt_rusc.fail_bulwark, 0.0);

    let f1uu_ccdt_rusc = f1uu.hunt(&ccdt_rusc);
    util::assert_approx_eq!(f1uu_ccdt_rusc.push, 0.7420);
    util::assert_approx_eq!(f1uu_ccdt_rusc.push_ta, 0.1830);
    util::assert_approx_eq!(f1uu_ccdt_rusc.fail, 0.0);
    util::assert_approx_eq!(f1uu_ccdt_rusc.fail_bulwark, 0.0750);

    let f8uu_ccdt_rusc = f8uu.hunt(&ccdt_rusc);
    util::assert_approx_eq!(f8uu_ccdt_rusc.push, 0.5145);
    util::assert_approx_eq!(f8uu_ccdt_rusc.push_ta, 0.0);
    util::assert_approx_eq!(f8uu_ccdt_rusc.fail, 0.4855);
    util::assert_approx_eq!(f8uu_ccdt_rusc.fail_bulwark, 0.0);

    let f15uu_ccdt_rusc = f15uu.hunt(&ccdt_rusc);
    util::assert_approx_eq!(f15uu_ccdt_rusc.push, 0.7765);
    util::assert_approx_eq!(f15uu_ccdt_rusc.push_ta, 0.1120);
    util::assert_approx_eq!(f15uu_ccdt_rusc.fail, 0.0334);
    util::assert_approx_eq!(f15uu_ccdt_rusc.fail_bulwark, 0.0780);

    let f1_uc = f1.hunt(&uc);
    util::assert_approx_eq!(f1_uc.push, 0.8211);
    util::assert_approx_eq!(f1_uc.push_ta, 0.1789);
    util::assert_approx_eq!(f1_uc.fail, 0.0);
    util::assert_approx_eq!(f1_uc.fail_bulwark, 0.0);

    let f8_uc = f8.hunt(&uc);
    util::assert_approx_eq!(f8_uc.push, 1.0);
    util::assert_approx_eq!(f8_uc.push_ta, 0.0);
    util::assert_approx_eq!(f8_uc.fail, 0.0);
    util::assert_approx_eq!(f8_uc.fail_bulwark, 0.0);

    let f15_uc = f15.hunt(&uc);
    util::assert_approx_eq!(f15_uc.push, 0.8971);
    util::assert_approx_eq!(f15_uc.push_ta, 0.1029);
    util::assert_approx_eq!(f15_uc.fail, 0.0);
    util::assert_approx_eq!(f15_uc.fail_bulwark, 0.0);

    let f1uu_uc = f1uu.hunt(&uc);
    util::assert_approx_eq!(f1uu_uc.push, 0.8170);
    util::assert_approx_eq!(f1uu_uc.push_ta, 0.1830);
    util::assert_approx_eq!(f1uu_uc.fail, 0.0);
    util::assert_approx_eq!(f1uu_uc.fail_bulwark, 0.0);

    let f8uu_uc = f8uu.hunt(&uc);
    util::assert_approx_eq!(f8uu_uc.push, 1.0);
    util::assert_approx_eq!(f8uu_uc.push_ta, 0.0);
    util::assert_approx_eq!(f8uu_uc.fail, 0.0);
    util::assert_approx_eq!(f8uu_uc.fail_bulwark, 0.0);

    let f15uu_uc = f15uu.hunt(&uc);
    util::assert_approx_eq!(f15uu_uc.push, 0.8880);
    util::assert_approx_eq!(f15uu_uc.push_ta, 0.1120);
    util::assert_approx_eq!(f15uu_uc.fail, 0.0);
    util::assert_approx_eq!(f15uu_uc.fail_bulwark, 0.0);
}
