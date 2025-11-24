use crate::codec;
use crate::codec::Codec;
use crate::codec::Reader;
use crate::codec::Writer;
use std::io::Read;
use std::io::Seek;
use std::io::Write;

#[derive(Debug, Default, Clone, Copy)]
pub struct Area {
    pub start_x: f64,
    pub start_y: f64,

    pub end_x: f64,
    pub end_y: f64,
}

impl Area {
    pub fn min_x(&self) -> f64 {
        self.start_x.min(self.end_x)
    }

    pub fn min_y(&self) -> f64 {
        self.start_y.min(self.end_y)
    }

    pub fn max_x(&self) -> f64 {
        self.start_x.max(self.end_x)
    }

    pub fn max_y(&self) -> f64 {
        self.start_y.max(self.end_y)
    }

    pub fn center_x(&self) -> f64 {
        (self.min_x() + self.max_x()) / 2.0
    }

    pub fn center_y(&self) -> f64 {
        (self.min_y() + self.max_y()) / 2.0
    }

    pub fn width(&self) -> f64 {
        self.max_x() - self.min_x()
    }

    pub fn height(&self) -> f64 {
        self.max_y() - self.min_y()
    }
}

impl From<((f64, f64), (f64, f64))> for Area {
    fn from(((start_x, start_y), (end_x, end_y)): ((f64, f64), (f64, f64))) -> Self {
        Self {
            start_x,
            start_y,

            end_x,
            end_y,
        }
    }
}

impl From<(f64, f64)> for Area {
    fn from((x, y): (f64, f64)) -> Self {
        Self {
            start_x: x,
            start_y: y,

            end_x: x,
            end_y: y,
        }
    }
}

impl Codec for Area {
    fn decode<S>(reader: &mut Reader<S>) -> codec::Result<Self>
    where
        S: Read + Seek,
    {
        let start_x = reader.read_primitive()?;
        let start_y = reader.read_primitive()?;

        let end_x = reader.read_primitive()?;
        let end_y = reader.read_primitive()?;

        Ok(Area {
            start_x,
            start_y,

            end_x,
            end_y,
        })
    }

    fn encode<S>(&self, writer: &mut Writer<S>) -> codec::Result<()>
    where
        S: Write + Seek,
    {
        writer.write_primitive(self.start_x)?;
        writer.write_primitive(self.start_y)?;

        writer.write_primitive(self.end_x)?;
        writer.write_primitive(self.end_y)?;

        Ok(())
    }
}
