use crate::util;
use libc::c_char;

#[no_mangle]
unsafe extern "C" fn strstr(haystack: *const c_char, needle: *const c_char) -> *const c_char {
    libc!(libc::strstr(haystack, needle));

    let out = {
        let needle = util::mem2slice(needle);
        let haystack = util::mem2slice(haystack);
        memchr::memmem::find(haystack, needle)
    };

    match out {
        None => core::ptr::null(),
        Some(idx) => haystack.add(idx),
    }
}
