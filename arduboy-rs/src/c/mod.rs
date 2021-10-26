use self::types::*;

pub mod types;

extern "C" {
    pub fn strlen(cstr: *const c_char) -> c_size_t;
}
