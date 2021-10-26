#![cfg(target_arch = "avr")]
#![no_std]
#![allow(clippy::missing_safety_doc)]

extern crate panic_halt;

use crate::print::Printable;
use c::types::*;
use core::mem;
use core::ops::Not;

pub mod c;
pub mod prelude;
pub mod print;
pub mod sound;

mod random;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
pub enum Color {
    Black,
    White,
}

impl Not for Color {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}

extern "C" {
    #[link_name = "arduboy_begin"]
    pub fn begin();

    #[link_name = "arduboy_clear"]
    pub fn clear();

    #[link_name = "arduboy_display"]
    pub fn display();

    #[link_name = "arduboy_display_and_clear_buffer"]
    pub fn display_and_clear_buffer();

    #[link_name = "arduboy_draw_fast_hline"]
    fn draw_fast_hline_raw(x: i16, y: i16, w: u8, color: u8);

    #[link_name = "arduboy_draw_fast_vline"]
    fn draw_fast_vline_raw(x: i16, y: i16, h: u8, color: u8);

    #[link_name = "arduboy_draw_pixel"]
    fn draw_pixel_raw(x: i16, y: i16, color: u8);

    #[link_name = "arduboy_fill_rect"]
    fn fill_rect_raw(x: i16, y: i16, w: u8, h: u8, color: u8);

    // #[link_name = "arduboy_generate_random_seed"]
    // fn generate_random_seed() -> c_ulong;

    #[link_name = "arduboy_get_pixel"]
    fn get_pixel_raw(x: u8, y: u8) -> u8;

    #[link_name = "arduboy_init_random_seed"]
    pub fn init_random_seed();

    #[link_name = "arduboy_just_pressed"]
    pub fn just_pressed(button: u8) -> bool;

    #[link_name = "arduboy_just_released"]
    pub fn just_released(button: u8) -> bool;

    #[link_name = "arduboy_next_frame"]
    pub fn next_frame() -> bool;

    #[link_name = "arduboy_poll_buttons"]
    pub fn poll_buttons();

    #[link_name = "arduboy_pressed"]
    pub fn pressed(buttons: u8) -> bool;

    #[link_name = "arduboy_print_chars"]
    fn print_chars(cstr: *const c_char);

    // #[link_name = "arduboy_print_char"]
    // fn print_char(c: c_char) -> c_size_t;

    #[link_name = "arduboy_print_int"]
    fn print_int(n: c_int, base: c_int) -> c_size_t;

    #[link_name = "arduboy_print_long"]
    fn print_long(n: c_long, base: c_int) -> c_size_t;

    // #[link_name = "arduboy_print_unsigned_char"]
    // fn print_unsigned_char(n: c_uchar, base: c_int) -> c_size_t;

    #[link_name = "arduboy_print_unsigned_int"]
    fn print_unsigned_int(n: c_uint, base: c_int) -> c_size_t;

    #[link_name = "arduboy_print_unsigned_long"]
    fn print_unsigned_long(n: c_ulong, base: c_int) -> c_size_t;

    #[link_name = "arduboy_set_cursor"]
    pub fn set_cursor(x: i16, y: i16);

    #[link_name = "arduboy_set_frame_rate"]
    pub fn set_frame_rate(rate: u8);
}

pub unsafe fn print(x: impl Printable) {
    x.print();
}

pub unsafe fn draw_fast_hline(x: i16, y: i16, w: u8, color: Color) {
    draw_fast_hline_raw(x, y, w, color as u8);
}

pub unsafe fn draw_fast_vline(x: i16, y: i16, h: u8, color: Color) {
    draw_fast_vline_raw(x, y, h, color as u8);
}

pub unsafe fn draw_pixel(x: i16, y: i16, color: Color) {
    draw_pixel_raw(x, y, color as u8);
}

pub unsafe fn fill_rect(x: i16, y: i16, w: u8, h: u8, color: Color) {
    fill_rect_raw(x, y, w, h, color as u8);
}

pub unsafe fn get_pixel(x: u8, y: u8) -> Color {
    mem::transmute::<u8, Color>(get_pixel_raw(x, y))
}

//pub unsafe fn init_random_seed() {
//    random::STATE[0] = generate_random_seed();
//    random::STATE[1] = generate_random_seed();
//}
