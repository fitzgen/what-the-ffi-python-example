extern crate libc;
use libc::c_int;

#[no_mangle]
pub extern fn life_universe_everything() -> c_int {
    42
}
