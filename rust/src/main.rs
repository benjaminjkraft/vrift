mod floor;
mod mouse;
mod run;
mod setup;
mod util;

// TODO:
// - UI:
//   - wasm-bindgen setup
//   - comparison chart a la the run comparison charts
//   - setup constructor
// - setup based on traps rather than power/luck
// - per-step rather than per-floor setup
// - mid-run start
// - adjust ARs to MHCT
// - README
// eventually:
// - mid-run upgrades

use crate::run::Plan;
use crate::setup::SetupStats;

fn main() {
    struct NamedStats<'a> {
        name: &'a str,
        setup: SetupStats,
    }
    println!("(all setups assume CCDT, SSDB, CF, 4 aura)");
    let setups: Vec<NamedStats> = vec![
        NamedStats {
            name: "RULPC",
            setup: SetupStats::new(28600, 101, true),
        },
        NamedStats {
            name: "winter RULPC",
            setup: SetupStats::new(28600, 106, true),
        },
        NamedStats {
            name: "RRSC",
            setup: SetupStats::new(30475, 106, true),
        },
        NamedStats {
            name: "winter RRSC",
            setup: SetupStats::new(30475, 111, true),
        },
        NamedStats {
            name: "ember",
            setup: SetupStats::new(59500, 81, true),
        },
        NamedStats {
            name: "winter ember",
            setup: SetupStats::new(59500, 86, true),
        },
    ];
    let uc_cf = SetupStats::uc(true);

    setups.into_iter().for_each(|s| {
        let plan = Plan::new(10, 7, 5, true, true, true, |f| {
            if f % 8 == 0 || f > 15 * 8 {
                uc_cf
            } else {
                s.setup
            }
        });
        println!("{}", s.name);
        println!("{}", plan.dist_to_string_short(&plan.sim(), 1));
        println!();
    });
}
