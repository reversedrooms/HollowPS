use std::env;

use lazy_static::lazy_static;

lazy_static! {
    static ref SKIP_TUTORIAL: i32 = env::var("SKIP_TUTORIAL").map_or(0, |v| v.parse().unwrap());
}

pub fn should_skip_tutorial() -> bool {
    *SKIP_TUTORIAL != 0
}
