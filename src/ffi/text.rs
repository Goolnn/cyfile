use crate::ffi::{
  Wrapped,
  Raw,
};

use std::ffi::{
  CString,
  CStr,
  c_char,
};

pub struct Text {
  raw: Raw<crate::Text>,

  content: CString,
  comment: CString,
}

impl Text {
  pub unsafe fn set_content(&mut self, content: *const c_char) {
    let content = CStr::from_ptr(content);

    self.raw_mut().set_content(content.to_str().unwrap());

    self.content = CString::from(content);
  }

  pub unsafe fn set_comment(&mut self, comment: *const c_char) {
    let comment = CStr::from_ptr(comment);

    self.raw_mut().set_comment(comment.to_str().unwrap());

    self.comment = CString::from(comment);
  }

  pub fn content(&self) -> *const c_char {
    self.content.as_ptr()
  }

  pub fn comment(&self) -> *const c_char {
    self.comment.as_ptr()
  }
}

impl Wrapped<crate::Text> for Text {
  fn owner(raw: crate::Text) -> *mut Self {
    let content = CString::new(raw.content()).unwrap();
    let comment = CString::new(raw.comment()).unwrap();

    Box::into_raw(Box::new(Self {
      raw: Raw::Owner(raw),

      content,
      comment,
    }))
  }

  fn refer(raw: *mut crate::Text) -> *mut Self {
    let content = CString::new(unsafe { &*raw }.content()).unwrap();
    let comment = CString::new(unsafe { &*raw }.comment()).unwrap();

    Box::into_raw(Box::new(Self {
      raw: Raw::Refer(raw),

      content,
      comment,
    }))
  }

  fn raw(&self) -> &crate::Text {
    unsafe {
      match &self.raw {
        Raw::Owner(owner) => owner,
        Raw::Refer(refer) => &**refer,
      }
    }
  }

  fn raw_mut(&mut self) -> &mut crate::Text {
    unsafe {
      match &mut self.raw {
        Raw::Owner(owner) => owner,
        Raw::Refer(refer) => &mut **refer,
      }
    }
  }
}

#[no_mangle]
pub unsafe extern fn cyfile_text_new() -> *mut Text {
  Text::owner(crate::Text::new())
}

#[no_mangle]
pub unsafe extern fn cyfile_text_with_content_and_comment(content: *const c_char, comment: *const c_char) -> *mut Text {
  Text::owner(crate::Text::with_content_and_comment(CStr::from_ptr(content).to_str().unwrap(), CStr::from_ptr(comment).to_str().unwrap()))
}

#[no_mangle]
pub unsafe extern fn cyfile_text_with_content(content: *const c_char) -> *mut Text {
  Text::owner(crate::Text::with_content(CStr::from_ptr(content).to_str().unwrap()))
}

#[no_mangle]
pub unsafe extern fn cyfile_text_with_comment(comment: *const c_char) -> *mut Text {
  Text::owner(crate::Text::with_comment(CStr::from_ptr(comment).to_str().unwrap()))
}

#[no_mangle]
pub unsafe extern fn cyfile_text_set_content(text: *mut Text, content: *const c_char) {
  Text::deref_mut(text).set_content(content);
}

#[no_mangle]
pub unsafe extern fn cyfile_text_set_comment(text: *mut Text, comment: *const c_char) {
  Text::deref_mut(text).set_comment(comment);
}

#[no_mangle]
pub unsafe extern fn cyfile_text_content(text: *mut Text) -> *const c_char {
  Text::deref(text).content()
}

#[no_mangle]
pub unsafe extern fn cyfile_text_comment(text: *mut Text) -> *const c_char {
  Text::deref(text).comment()
}

#[no_mangle]
pub unsafe extern fn cyfile_text_debug(text: *mut Text) {
  print!("{:?}", Text::deref(text).raw());
}

#[no_mangle]
pub unsafe extern fn cyfile_text_drop(text: *mut Text) {
  std::ptr::drop_in_place(text);
}