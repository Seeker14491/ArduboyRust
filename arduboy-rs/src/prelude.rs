pub use crate::c::types::*;
pub use crate::print::*;
pub use crate::random::*;

use core::cmp;

pub const WIDTH: u8 = 128;
pub const HEIGHT: u8 = 64;

pub const UP: ButtonSet = ButtonSet {
    flag_set: 0b10000000,
};
pub const RIGHT: ButtonSet = ButtonSet {
    flag_set: 0b01000000,
};
pub const LEFT: ButtonSet = ButtonSet {
    flag_set: 0b00100000,
};
pub const DOWN: ButtonSet = ButtonSet {
    flag_set: 0b00010000,
};
pub const A: ButtonSet = ButtonSet {
    flag_set: 0b00001000,
};
pub const B: ButtonSet = ButtonSet {
    flag_set: 0b00000100,
};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct ButtonSet {
    flag_set: u8,
}

impl ButtonSet {
    pub unsafe fn pressed(&self) -> bool {
        crate::pressed(self.flag_set)
    }

    pub unsafe fn just_pressed(&self) -> bool {
        crate::just_pressed(self.flag_set)
    }

    pub unsafe fn just_released(&self) -> bool {
        crate::just_released(self.flag_set)
    }
}

impl core::ops::BitOr for ButtonSet {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        Self {
            flag_set: self.flag_set | other.flag_set,
        }
    }
}

pub fn constrain<T: Ord>(x: T, a: T, b: T) -> T {
    cmp::max(cmp::min(x, b), a)
}

//pub fn random() -> i32 {
//    (random::next() >> 1) as i32
//}
//
//pub fn random_max(max: i32) -> i32 {
//    if max == 0 {
//        0
//    } else {
//        mod_i32(random(), max)
//    }
//}
//
//pub fn random_min_max(min: i32, max: i32) -> i32 {
//    if min >= max {
//        return min;
//    }
//
//    return random_max(max - min) + min;
//}
//
//pub fn random_seed(seed: u64) {
//    if seed == 0 {
//        return;
//    }
//
//    unsafe {
//        random::STATE[0] = seed as u32;
//        random::STATE[1] = (seed >> 32) as u32
//    }
//}
