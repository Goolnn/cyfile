use crate::Text;
use crate::Note;

use std::ffi::{
  c_double,
  c_uint,
};

#[no_mangle]
pub unsafe extern fn cyfile_note_new() -> *mut Note {
  Box::into_raw(Box::new(Note::new()))
}

#[no_mangle]
pub unsafe extern fn cyfile_note_with_coordinate_and_choice(x: c_double, y: c_double, choice: c_uint) -> *mut Note {
  Box::into_raw(Box::new(Note::with_coordinate_and_choice(x, y, choice)))
}

#[no_mangle]
pub unsafe extern fn cyfile_note_with_coordinate(x: c_double, y: c_double) -> *mut Note {
  Box::into_raw(Box::new(Note::with_coordinate(x, y)))
}

#[no_mangle]
pub unsafe extern fn cyfile_note_set_x(note: *mut Note, x: c_double) {
  (*note).set_x(x);
}

#[no_mangle]
pub unsafe extern fn cyfile_note_set_y(note: *mut Note, y: c_double) {
  (*note).set_y(y);
}

#[no_mangle]
pub unsafe extern fn cyfile_note_set_choice(note: *mut Note, choice: c_uint) {
  (*note).set_choice(choice);
}

#[no_mangle]
pub unsafe extern fn cyfile_note_x(note: *mut Note) -> c_double {
  (*note).x()
}

#[no_mangle]
pub unsafe extern fn cyfile_note_y(note: *mut Note) -> c_double {
  (*note).y()
}

#[no_mangle]
pub unsafe extern fn cyfile_note_choice(note: *mut Note) -> c_uint {
  (*note).choice()
}

#[no_mangle]
pub unsafe extern fn cyfile_note_texts_len(note: *mut Note) -> c_uint {
  (*note).texts().len() as c_uint
}

#[no_mangle]
pub unsafe extern fn cyfile_note_texts_mut(note: *mut Note) -> *mut *mut Text {
  let mut list = Vec::new();

  for text in (*note).texts_mut() {
    list.push(text as *mut Text);
  }

  (*Box::into_raw(list.into_boxed_slice())).as_mut_ptr()
}

#[no_mangle]
pub unsafe extern fn cyfile_note_texts(note: *mut Note) -> *mut *const Text {
  let mut list = Vec::new();

  for text in (*note).texts() {
    list.push(text as *const Text);
  }

  (*Box::into_raw(list.into_boxed_slice())).as_mut_ptr()
}

#[no_mangle]
pub unsafe extern fn cyfile_note_remove_text(note: *mut Note, index: c_uint) {
  (*note).remove_text(index);
}

#[no_mangle]
pub unsafe extern fn cyfile_note_add_text(note: *mut Note, text: *mut Text) {
  (*note).add_text(*Box::from_raw(text));
}

#[no_mangle]
pub unsafe extern fn cyfile_note_debug(note: *mut Note) {
  println!("{:?}", *note);
}