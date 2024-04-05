use std::ffi::{
  CString,
  c_void,
  c_char,
};

use std::ptr::drop_in_place;

mod file;
mod text;
mod note;
mod page;

#[no_mangle]
unsafe extern fn cyfile_string_drop(string: *mut c_char) {
  drop(CString::from_raw(string));
}

#[no_mangle]
unsafe extern fn cyfile_array_mut_drop(array: *mut *mut c_void) {
  drop_in_place(array);
}

#[no_mangle]
unsafe extern fn cyfile_array_drop(array: *mut *const c_void) {
  drop_in_place(array);
}