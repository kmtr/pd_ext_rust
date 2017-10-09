extern crate pd_sys;

use std::ffi::CString;

#[no_mangle]
pub unsafe extern "C" fn hello_rust() {
    let c_str = CString::new(b"Hello, from Rust world" as &[u8]).unwrap();
    pd_sys::post(c_str.as_ptr());
}
