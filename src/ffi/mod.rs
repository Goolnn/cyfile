mod file;
mod text;
mod note;

trait Wrapped<T> {
  fn owner(raw: T) -> *mut Self;
  fn refer(raw: *mut T) -> *mut Self;

  fn raw(&self) -> &T;
  fn raw_mut(&mut self) -> &mut T;

  unsafe fn deref(ptr: *mut Self) -> &'static Self {
    &*ptr
  }

  unsafe fn deref_mut(ptr: *mut Self) -> &'static mut Self {
    &mut *ptr
  }
}

enum Raw<T> {
  Owner(T),
  Refer(*mut T),
}