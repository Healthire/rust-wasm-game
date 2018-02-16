use std::ffi::CString;
use std::os::raw::{c_char, c_void};
use std::mem;

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    return ptr as *mut c_void;
}

#[no_mangle]
pub extern "C" fn dealloc(data_ptr: *mut c_void, size: usize) {
    unsafe {
        let _buf = Vec::from_raw_parts(data_ptr, 0, size);
    }
}

#[no_mangle]
pub extern "C" fn alloc_str(size: usize) -> *mut c_char {
    let buf = CString::new(vec![0; size]).unwrap();
    return buf.into_raw();
}

#[no_mangle]
pub extern "C" fn dealloc_str(ptr: *mut c_char) {
    unsafe {
        let _buf = CString::from_raw(ptr);
    }
}
