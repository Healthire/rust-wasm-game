pub mod webgl;
pub mod websocket;
pub mod exports;

use std::ffi::CString;
use std::os::raw::c_void;
use std::ptr::null_mut;
use std::cell::RefCell;
use std::slice;

type MainLoopCallback = unsafe extern "C" fn(*const c_void, usize);

#[derive(Debug)]
pub enum InputEvent {
    MouseMove(i32, i32),
    MouseDown { button: i8, position: (i32, i32) },
    MouseUp { button: i8, position: (i32, i32) },
    KeyDown(i32),
    KeyUp(i32),
}

extern "C" {
    fn js_console_log(ptr: *const u8);
    fn js_set_main_loop(fn_ptr: MainLoopCallback);
}

thread_local!(static MAIN_LOOP_CALLBACK: RefCell<*mut c_void> = RefCell::new(null_mut()));

#[allow(dead_code)]
pub fn console_log(s: &str) {
    let c_str = CString::new(s).unwrap();
    unsafe {
        js_console_log(c_str.as_ptr() as *const u8);
    }
}

// Will exit the application and pass callback into javascript, where it will be called on a timer
// Should only be called once at the exit point of your application
pub fn set_main_loop_callback<T>(callback: T)
where
    T: FnMut(&[InputEvent]) + 'static,
{
    MAIN_LOOP_CALLBACK
        .with(|cb| *cb.borrow_mut() = Box::into_raw(Box::new(callback)) as *mut c_void);

    pub unsafe extern "C" fn wrapper<T>(events_ptr: *const c_void, size: usize)
    where
        T: FnMut(&[InputEvent]),
    {
        let input_events = slice::from_raw_parts(events_ptr as *const InputEvent, size);
        MAIN_LOOP_CALLBACK.with(|cb| {
            let callback = *cb.borrow_mut() as *mut T;
            (*callback)(input_events)
        })
    }

    unsafe {
        js_set_main_loop(wrapper::<T>);
    }
}
