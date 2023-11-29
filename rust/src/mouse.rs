use crate::setup::SetupStats;
#[cfg(test)]
use crate::util;
#[cfg(test)]
use std::collections::HashMap;

#[derive(Debug)]
pub struct Mouse {
    id: u32,
    pub name: &'static str,
    power: f64,
    eff: f64,
}

impl PartialEq for Mouse {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id;
    }
}

#[test]
fn test_id_uniqueness() {
    let mut ids: HashMap<u32, Mouse> = HashMap::new();
    for mouse in ALL_MICE {
        match ids.get(&mouse.id) {
            Some(other) => {
                assert!(
                    false,
                    "Two mice with ID {}: {}, {}",
                    mouse.id, mouse.name, other.name
                );
            }
            None => {
                ids.insert(mouse.id, mouse);
            }
        }
    }
}

impl Eq for Mouse {}

impl Mouse {
    pub fn cr(&self, setup: &SetupStats) -> f64 {
        let power_term = self.eff * setup.power;
        let luck_term = 2.0 * (self.eff.min(1.4) * setup.luck).floor().powi(2);
        let cr = (power_term + luck_term) / (power_term + self.power);
        return cr.min(1.0);
    }
}

#[test]
fn test_cr() {
    let ccdt_rusc = SetupStats::new(14251, 84, false);
    let uc = SetupStats::uc(true);
    util::assert_approx_eq!(PUPPETTO.cr(&ccdt_rusc), 1.0);
    util::assert_approx_eq!(WITHERED_REMAINS.cr(&ccdt_rusc), 0.9874);
    util::assert_approx_eq!(BULWARK_OF_ASCENT.cr(&ccdt_rusc), 0.5809);
    util::assert_approx_eq!(SHADE_OF_THE_ECLIPSE.cr(&ccdt_rusc), 0.6719);
    util::assert_approx_eq!(THE_TOTAL_ECLIPSE.cr(&ccdt_rusc), 0.5145);
    util::assert_approx_eq!(PUPPETTO.cr(&uc), 1.0);
    util::assert_approx_eq!(WITHERED_REMAINS.cr(&uc), 1.0);
    util::assert_approx_eq!(BULWARK_OF_ASCENT.cr(&uc), 1.0);
    util::assert_approx_eq!(SHADE_OF_THE_ECLIPSE.cr(&uc), 1.0);
    util::assert_approx_eq!(THE_TOTAL_ECLIPSE.cr(&uc), 1.0);
}

pub const TIMID_EXPLORER: Mouse = Mouse {
    id: 990,
    name: "Timid Explorer Mouse",
    power: 3300.0,
    eff: 1.0,
};
pub const ELIXIR_MAKER: Mouse = Mouse {
    id: 991,
    name: "Elixir Maker Mouse",
    power: 5050.0,
    eff: 1.0,
};
pub const PUPPETTO: Mouse = Mouse {
    id: 1006,
    name: "Puppetto Mouse",
    power: 2900.0,
    eff: 1.0,
};
pub const CUTPURSE: Mouse = Mouse {
    id: 1009,
    name: "Cutpurse Mouse",
    power: 6650.0,
    eff: 2.0,
};
pub const MARTIAL: Mouse = Mouse {
    id: 998,
    name: "Martial Mouse",
    power: 8800.0,
    eff: 3.0,
};
pub const ONE_MOUSE_BAND: Mouse = Mouse {
    id: 985,
    name: "One-Mouse Band",
    power: 11750.0,
    eff: 4.0,
};
pub const MOUSE_OF_ELEMENTS: Mouse = Mouse {
    id: 996,
    name: "Mouse of Elements",
    power: 16000.0,
    eff: 5.0,
};
pub const CURSED_CRUSADER: Mouse = Mouse {
    id: 1001,
    name: "Cursed Crusader Mouse",
    power: 21500.0,
    eff: 6.0,
};
pub const WITHERED_REMAINS: Mouse = Mouse {
    id: 987,
    name: "Withered Remains Mouse",
    power: 29000.0,
    eff: 7.0,
};
pub const SHADE_OF_THE_ECLIPSE: Mouse = Mouse {
    id: 989,
    name: "Shade of the Eclipse Mouse",
    power: 7000000.0,
    eff: 1000.0,
};
pub const PUPPET_CHAMPION: Mouse = Mouse {
    id: 1007,
    name: "Puppet Champion Mouse",
    power: 72000.0,
    eff: 9.0,
};
pub const CHAMPION_THIEF: Mouse = Mouse {
    id: 1010,
    name: "Champion Thief Mouse",
    power: 72000.0,
    eff: 9.0,
};
pub const PRAETORIAN_CHAMPION: Mouse = Mouse {
    id: 999,
    name: "Praetorian Champion Mouse",
    power: 72000.0,
    eff: 9.0,
};
pub const CHAMPION_DANSEUSE: Mouse = Mouse {
    id: 986,
    name: "Champion Danseuse Mouse",
    power: 72000.0,
    eff: 9.0,
};
pub const MAGIC_CHAMPION: Mouse = Mouse {
    id: 997,
    name: "Magic Champion Mouse",
    power: 72000.0,
    eff: 9.0,
};
pub const FALLEN_CHAMPION_FOOTMAN: Mouse = Mouse {
    id: 1002,
    name: "Fallen Champion Footman Mouse",
    power: 72000.0,
    eff: 9.0,
};
pub const ARCH_CHAMPION_NECROMANCER: Mouse = Mouse {
    id: 988,
    name: "Arch Champion Necromancer Mouse",
    power: 72000.0,
    eff: 9.0,
};
pub const THE_TOTAL_ECLIPSE: Mouse = Mouse {
    id: 992,
    name: "The Total Eclipse",
    power: 13500000.0,
    eff: 1000.0,
};
pub const UNWAVERING_ADVENTURER: Mouse = Mouse {
    id: 993,
    name: "Unwavering Adventurer Mouse",
    power: 4800.0,
    eff: 1.75,
};
pub const BERZERKER: Mouse = Mouse {
    id: 995,
    name: "Berzerker Mouse",
    power: 8250.0,
    eff: 1.75,
};
pub const LUMI_LANCER: Mouse = Mouse {
    id: 994,
    name: "Lumi-lancer Mouse",
    power: 23000.0,
    eff: 1.75,
};
pub const POSSESSED_ARMAMENTS: Mouse = Mouse {
    id: 1004,
    name: "Possessed Armaments Mouse",
    power: 38000.0,
    eff: 10.0,
};
pub const PRESTIGIOUS_ADVENTURER: Mouse = Mouse {
    id: 1005,
    name: "Prestigious Adventurer Mouse",
    power: 150000.0,
    eff: 25.0,
};
pub const SOLDIER_OF_THE_SHADE: Mouse = Mouse {
    id: 1003,
    name: "Soldier of the Shade Mouse",
    power: 350000.0,
    eff: 50.0,
};
pub const TERRIFIED_ADVENTURER: Mouse = Mouse {
    id: 1008,
    name: "Terrified Adventurer Mouse",
    power: 100.0,
    eff: 2.0,
};
pub const BULWARK_OF_ASCENT: Mouse = Mouse {
    id: 1000,
    name: "Bulwark of Ascent Mouse",
    power: 818250.0,
    eff: 75.0,
};

#[allow(unused)]
pub const ALL_MICE: [Mouse; 26] = [
    TIMID_EXPLORER,
    ELIXIR_MAKER,
    PUPPETTO,
    CUTPURSE,
    MARTIAL,
    ONE_MOUSE_BAND,
    MOUSE_OF_ELEMENTS,
    CURSED_CRUSADER,
    WITHERED_REMAINS,
    SHADE_OF_THE_ECLIPSE,
    PUPPET_CHAMPION,
    CHAMPION_THIEF,
    PRAETORIAN_CHAMPION,
    CHAMPION_DANSEUSE,
    MAGIC_CHAMPION,
    FALLEN_CHAMPION_FOOTMAN,
    ARCH_CHAMPION_NECROMANCER,
    THE_TOTAL_ECLIPSE,
    UNWAVERING_ADVENTURER,
    BERZERKER,
    LUMI_LANCER,
    POSSESSED_ARMAMENTS,
    PRESTIGIOUS_ADVENTURER,
    SOLDIER_OF_THE_SHADE,
    TERRIFIED_ADVENTURER,
    BULWARK_OF_ASCENT,
];
