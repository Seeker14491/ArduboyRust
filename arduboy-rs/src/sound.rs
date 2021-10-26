use crate::c::types::*;

extern "C" {
    #[link_name = "sound_tone"]
    pub fn tone(frequency: c_uint, duration: c_ulong);
}
