pub mod ffi;
pub use ffi::*;

extern crate libc;

#[test]
fn it_works() {
    unsafe {
        let c = cmsCreateContext(std::ptr::null_mut(), std::ptr::null_mut());
        cmsDeleteContext(c);
    }
}
