mod decode;
mod encode;

pub(crate) use decode::Decode;
pub(crate) use encode::Encode;

pub use decode::Reader;
pub use encode::Writer;

pub trait Primitive {}

pub trait Length: Primitive + TryInto<usize> + TryFrom<usize> {}

impl Primitive for u8 {}
impl Primitive for u16 {}
impl Primitive for u32 {}
impl Primitive for u64 {}
impl Primitive for u128 {}
impl Primitive for i8 {}
impl Primitive for i16 {}
impl Primitive for i32 {}
impl Primitive for i64 {}
impl Primitive for i128 {}
impl Primitive for f32 {}
impl Primitive for f64 {}
impl Primitive for bool {}

impl Length for u8 {}
impl Length for u16 {}
impl Length for u32 {}
impl Length for u64 {}
impl Length for u128 {}

pub type Version = (u8, u8);
