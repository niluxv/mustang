//! Basic utilities to convert between C an Rust types.

pub(crate) unsafe fn mem2slice<'a>(mem: *const libc::c_char) -> &'a [u8] {
    let mem: *const u8 = mem.cast();
    let mut len = 0;
    let mut ptr = mem;
    while ptr.read() != 0 {
        len += 1;
        ptr = ptr.add(1);
    }
    core::slice::from_raw_parts(mem, len)
}
