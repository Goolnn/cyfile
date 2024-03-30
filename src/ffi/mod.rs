mod file;
mod text;
mod note;

trait Wrapped<T> {
  fn new(raw: T) -> *mut Self;

  fn raw(&self) -> &T;
  fn raw_mut(&mut self) -> &mut T;

  unsafe fn deref(ptr: *mut Self) -> &'static mut Self {
    &mut *ptr
  }
}