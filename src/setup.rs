#[derive(Debug)]
pub struct SetupStats {
    pub power: f64,
    pub luck: f64,
    pub cf: bool,
}

impl SetupStats {
    pub fn new(power: u32, luck: u32, cf: bool) -> SetupStats {
        return SetupStats {
            power: power as f64,
            luck: luck as f64,
            cf,
        };
    }

    pub fn uc(cf: bool) -> SetupStats {
        return SetupStats::new(0, 10000, cf);
    }
}
