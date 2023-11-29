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
    let ccdt_ssdb_rulpc_4au_cf = SetupStats::new(28600, 101, true);
    let uc_cf = SetupStats::uc(true);

    let plan = Plan::new(10, 7, 5, true, true, true, |f| {
        if f % 8 == 0 {
            uc_cf
        } else {
            ccdt_ssdb_rulpc_4au_cf
        }
    });
    println!("10/7/5  SSi/UU/SSt  CCDT SSDB RULPC 4aura CF");
    println!("{}", plan.dist_to_string(&plan.sim(), 1));
    println!();
}
