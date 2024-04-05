use crate::Text;

use std::ffi::{
  CString,
  CStr,
  c_char,
};

#[no_mangle]
unsafe extern fn cyfile_text_new() -> *mut Text {
  Box::into_raw(Box::new(Text::new()))
}

#[no_mangle]
unsafe extern fn cyfile_text_with_content_and_comment(
  content: *const c_char,
  comment: *const c_char,
) -> *mut Text {
  Box::into_raw(Box::new(Text::with_content_and_comment(
    CStr::from_ptr(content).to_string_lossy().as_ref(),
    CStr::from_ptr(comment).to_string_lossy().as_ref(),
  )))
}

#[no_mangle]
unsafe extern fn cyfile_text_with_content(content: *const c_char) -> *mut Text {
  Box::into_raw(Box::new(Text::with_content(
    CStr::from_ptr(content).to_string_lossy().as_ref()
  )))
}

#[no_mangle]
unsafe extern fn cyfile_text_with_comment(comment: *const c_char) -> *mut Text {
  Box::into_raw(Box::new(Text::with_comment(
    CStr::from_ptr(comment).to_string_lossy().as_ref()
  )))
}

#[no_mangle]
unsafe extern fn cyfile_text_clear_content(text: *mut Text) {
  (*text).clear_content();
}

#[no_mangle]
unsafe extern fn cyfile_text_clear_comment(text: *mut Text) {
  (*text).clear_comment();
}

#[no_mangle]
unsafe extern fn cyfile_text_set_content(text: *mut Text, content: *const c_char) {
  (*text).set_content(
    CStr::from_ptr(content).to_string_lossy().as_ref()
  );
}

#[no_mangle]
unsafe extern fn cyfile_text_set_comment(text: *mut Text, comment: *const c_char) {
  (*text).set_comment(
    CStr::from_ptr(comment).to_string_lossy().as_ref()
  );
}

#[no_mangle]
unsafe extern fn cyfile_text_content(text: *mut Text) -> *mut c_char {
  CString::new((*text).content()).unwrap().into_raw()
}

#[no_mangle]
unsafe extern fn cyfile_text_comment(text: *mut Text) -> *mut c_char {
  CString::new((*text).comment()).unwrap().into_raw()
}

#[no_mangle]
unsafe extern fn cyfile_text_debug(text: *mut Text) {
  println!("{:?}", *text);
}

#[no_mangle]
unsafe extern fn cyfile_text_drop(text: *mut Text) {
  drop(Box::from_raw(text));
}