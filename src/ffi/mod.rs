use std::ffi::c_void;

mod file;
mod text;
mod note;

#[no_mangle]
unsafe extern fn cyfile_drop(ptr: *mut c_void) {
  std::ptr::drop_in_place(ptr);
}