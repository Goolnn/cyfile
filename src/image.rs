use crate::codec;
use crate::codec::Codec;
use crate::codec::Reader;
use crate::codec::Writer;
use image::ImageReader;
use std::io::Cursor;
use std::io::Read;
use std::io::Seek;
use std::io::Write;

pub mod error;

#[derive(Debug, Clone)]
pub struct Image {
    data: Vec<u8>,

    width: u32,
    height: u32,
}

impl Image {
    pub fn new<T>(data: T) -> error::Result<Self>
    where
        T: ToOwned<Owned = Vec<u8>>,
    {
        let data = data.to_owned();

        let (width, height) = ImageReader::new(Cursor::new(&data)).into_dimensions()?;

        Ok(Self {
            data,

            width,
            height,
        })
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn map(&self, x: f64, y: f64) -> (u32, u32) {
        let mapped_x = self.width as f64 * (x + 1.0) / 2.0;
        let mapped_y = self.height as f64 * (1.0 - (y + 1.0) / 2.0);

        (mapped_x as u32, mapped_y as u32)
    }

    pub fn unmap(&self, x: u32, y: u32) -> (f64, f64) {
        let unmapped_x = (x as f64 / self.width as f64) * 2.0 - 1.0;
        let unmapped_y = 1.0 - (y as f64 / self.height as f64) * 2.0;

        (unmapped_x, unmapped_y)
    }
}

impl AsRef<[u8]> for Image {
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}

impl Codec for Image {
    fn decode<S>(reader: &mut Reader<S>) -> codec::Result<Self>
    where
        S: Read + Seek,
    {
        let data = reader.read_bytes_with_len::<u32>()?;

        Ok(Self::new(data)?)
    }

    fn encode<S>(&self, writer: &mut Writer<S>) -> codec::Result<()>
    where
        S: Write + Seek,
    {
        writer.write_bytes_with_len::<u32>(&self.data)?;

        Ok(())
    }
}
