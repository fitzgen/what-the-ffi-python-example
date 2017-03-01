extern crate libc;
extern crate strsim;

use libc::{c_char, c_int, size_t};
use std::panic;
use std::slice;
use std::str;

#[no_mangle]
pub extern fn levenshtein_distance(left: *const c_char,
                                   left_length: size_t,
                                   right: *const c_char,
                                   right_length: size_t,
                                   result: *mut size_t) -> c_int {
    let result = panic::catch_unwind(|| {
        if left.is_null() || right.is_null() || result.is_null() {
            return 1;
        }

        let (left, right) = unsafe {
            (slice::from_raw_parts(left as *const u8, left_length),
             slice::from_raw_parts(right as *const u8, right_length))
        };

        let left = match str::from_utf8(left) {
            Ok(s) => s,
            Err(_) => return 1,
        };

        let right = match str::from_utf8(right) {
            Ok(s) => s,
            Err(_) => return 1,
        };

        let distance = strsim::levenshtein(left, right);

        let mut result = unsafe {
            result.as_mut()
                .expect("null checked at beginning")

        };

        *result = distance;
        0
    });
    match result {
        Ok(r) => r,
        Err(_) => 1,
    }
}
