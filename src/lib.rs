use libc;
use std::ffi::CString;
use std::os::raw::c_char;

mod robocraft_bindings;
mod robocraft_bot_bindings;

pub(crate) unsafe fn allocate_cstring(input: &str) -> *mut c_char {
    let input_c = CString::new(input).expect("Rust &str -> CString conversion failed");
    let space = libc::malloc(libc::strlen(input_c.as_ptr()) + 1) as *mut c_char;
    libc::strcpy(space, input_c.as_ptr());
    return space;
}

#[cfg(test)]
mod tests {
    /*#[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }*/
}
