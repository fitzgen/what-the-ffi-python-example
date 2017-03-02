extern crate libc;
extern crate strsim;

use libc::{c_char, c_int, size_t};
use std::collections::HashMap;
use std::mem;
use std::panic;
use std::ptr;
use std::slice;
use std::str;

// Ok not to be `#[repr(C)]` because we only expose it behind
// pointers, which are `#[repr(C)]`.
pub struct LevDistCache {
    cache: HashMap<(String, String), usize>,
}

impl LevDistCache {
    fn new() -> Box<LevDistCache> {
        Box::new(LevDistCache {
            cache: HashMap::new()
        })
    }

    fn distance(&mut self, left: &str, right: &str) -> usize {
        *self.cache
            .entry((left.into(), right.into()))
            .or_insert_with(|| strsim::levenshtein(left, right))
    }
}

#[no_mangle]
pub extern fn ldc_new() -> *mut LevDistCache {
    let result = panic::catch_unwind(|| {
        let mut cache = LevDistCache::new();
        let result = {
            &mut *cache as *mut _
        };
        mem::forget(cache);
        result
    });
    match result {
        Ok(r) => r,
        Err(_) => ptr::null_mut(),
    }
}

#[no_mangle]
pub extern fn ldc_distance(cache: *mut LevDistCache, left: *const c_char,
                           left_length: size_t,
                           right: *const c_char,
                           right_length: size_t,
                           result: *mut size_t) -> c_int {
    let result = panic::catch_unwind(|| {
        if cache.is_null() || left.is_null() ||
           right.is_null() || result.is_null() {
            return 1;
        }

        let (cache, left, right) = unsafe {
            (mem::transmute::<_, &mut LevDistCache>(cache),
             slice::from_raw_parts(left as *const u8, left_length),
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

        let distance = cache.distance(left, right);

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

#[no_mangle]
pub extern fn ldc_delete(cache: *mut LevDistCache) {
    let _ = panic::catch_unwind(|| {
        unsafe {
            let _: Box<LevDistCache> = mem::transmute(cache);
        }
    });
}
