use std::ffi::{
  CString,
  c_char,
};

mod file;
mod text;
mod note;
mod page;

#[no_mangle]
unsafe extern fn cyfile_string_drop(string: *mut c_char) {
  drop(CString::from_raw(string))
}