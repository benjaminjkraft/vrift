mod floor;
mod mouse;
mod run;
mod setup;
mod util;

use crate::run::Plan;
use crate::setup::SetupStats;

fn main() {
    let plan = Plan::new(10, 7, 5, true, true, false, |f| {
        if f % 8 == 0 {
            SetupStats::new(18650, 64, true)
        } else if f < 16 {
            SetupStats::new(14251, 84, false)
        } else {
            SetupStats::new(18650, 64, false)
        }
    });
    println!("10/7/5  SSi/UU  CCDT PB9 RUPC");
    println!("{}", plan.dist_to_string(&plan.sim()));
    println!();

    // let plan = Plan::new(10, 7, 5, true, true, true, |_| SetupStats::uc(true));
    // println!("10/7/5  SSi/UU/SSt  UC CF");
    // println!("{}", plan.dist_to_string(&plan.sim()));
    // println!();
}
