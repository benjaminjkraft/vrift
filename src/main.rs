mod floor;
mod mouse;
mod run;
mod setup;
mod util;

use crate::run::Plan;
use crate::setup::SetupStats;

fn main() {
    // let ccdt_pb9_rusc_2au = SetupStats::new(14251, 84, false);
    // let ccdt_pb9_rupc_2au = SetupStats::new(18650, 64, false);
    // let ccdt_pb9_rupc_2au_cf = SetupStats::new(18650, 64, true);
    // let ccdt_ssdb_rusc_4au_cf = SetupStats::new(23064, 101, true);
    // let ccdt_ssdb_rupc_4au_cf = SetupStats::new(28600, 81, true);
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
    println!("{}", plan.dist_to_string(&plan.sim()));
    println!();

    // let plan = Plan::new(10, 7, 5, true, true, false, |f| {
    //     if f % 8 == 0 {
    //         ccdt_pb9_rusc_2au
    //     } else if f < 16 {
    //         ccdt_pb9_rupc_2au
    //     } else {
    //         ccdt_pb9_rupc_2au_cf
    //     }
    // });
    // println!("10/7/5  SSi/UU  CCDT PB9 RUPC_2au");
    // println!("{}", plan.dist_to_string(&plan.sim()));
    // println!();

    // let plan = Plan::new(10, 7, 5, true, true, true, |_| uc_cf);
    // println!("10/7/5  SSi/UU/SSt  UC CF");
    // println!("{}", plan.dist_to_string(&plan.sim()));
    // println!();
}
