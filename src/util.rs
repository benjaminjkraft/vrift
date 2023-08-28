#[cfg(test)]
macro_rules! assert_approx_eq {
    ( $x:expr, $y:expr ) => {
        if !($x - $y < 1e-4 && $y - $x < 1e-4) {
            panic!("expected {} ~= {}", $x, $y);
        }
    };
    ( $x:expr, $y:expr, $( $fmt:expr ),* ) => {
        if !($x - $y < 1e-4 && $y - $x < 1e-4) {
            panic!("{}: expected {} ~= {}", format!($($fmt,)*), $x, $y);
        }
    };
}

#[cfg(test)]
pub(crate) use assert_approx_eq;
