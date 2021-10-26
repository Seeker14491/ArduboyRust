use crate::c::types::*;

extern "C" {
    #[link_name = "arduino_random_between"]
    fn arduino_random_between_raw(min: c_long, max: c_long) -> c_long;

    #[link_name = "arduino_random_less_than"]
    fn arduino_random_less_than_raw(max: c_long) -> c_long;
}

pub unsafe fn random_between(min: i32, max: i32) -> i32 {
    arduino_random_between_raw(min, max)
}

pub unsafe fn random_less_than(max: i32) -> i32 {
    arduino_random_less_than_raw(max)
}
