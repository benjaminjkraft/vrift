use wasm_bindgen::convert::TryFromJsValue;
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Copy)]
#[wasm_bindgen]
pub struct SetupStats {
    pub power: f64,
    pub luck: f64,
    pub cf: bool,
}

#[wasm_bindgen]
impl SetupStats {
    #[wasm_bindgen]
    pub fn new(power: u32, luck: u32, cf: bool) -> SetupStats {
        return SetupStats {
            power: power as f64,
            luck: luck as f64,
            cf,
        };
    }

    #[wasm_bindgen]
    pub fn uc(cf: bool) -> SetupStats {
        return SetupStats::new(0, 10000, cf);
    }
}

// use internals to work around
// https://github.com/rustwasm/wasm-bindgen/issues/2231#issuecomment-1828855232
impl TryFrom<JsValue> for SetupStats {
    type Error = JsValue;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        Self::try_from_js_value(value)
    }
}
