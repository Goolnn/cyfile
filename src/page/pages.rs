use crate::codec::{Codec, Decode, Encode};
use crate::error::{FileError, FileResult};
use crate::Page;
use std::collections::VecDeque;

pub struct Pages(VecDeque<Page>);

impl Pages {
    pub fn new() -> Self {
        Self(VecDeque::new())
    }

    pub fn push_front(&mut self, text: Page) {
        self.0.push_front(text);
    }

    pub fn push_back(&mut self, text: Page) {
        self.0.push_back(text);
    }

    pub fn insert(&mut self, index: usize, text: Page) {
        self.0.insert(index, text);
    }

    pub fn remove(&mut self, index: usize) {
        self.0.remove(index);
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Page> {
        self.0.get_mut(index)
    }

    pub fn get(&self, index: usize) -> Option<&Page> {
        self.0.get(index)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn inner_mut(&mut self) -> &mut VecDeque<Page> {
        &mut self.0
    }

    pub fn inner(&self) -> &VecDeque<Page> {
        &self.0
    }
}

impl<const N: usize> From<[Page; N]> for Pages {
    fn from(value: [Page; N]) -> Self {
        Self(VecDeque::from(value))
    }
}

impl From<VecDeque<Page>> for Pages {
    fn from(value: VecDeque<Page>) -> Self {
        Self(value)
    }
}

impl Default for Pages {
    fn default() -> Self {
        Self::new()
    }
}

impl Encode for Pages {
    fn encode(&self, codec: &mut Codec) -> FileResult<()> {
        match codec.version() {
            (0, 0) => {
                codec.write_primitive(self.len() as u8)?;
            }

            (0, 2) => {
                codec.write_primitive::<u32>(self.len() as u32)?;
            }

            _ => return Err(FileError::InvalidVersion),
        }

        for page in &self.0 {
            codec.write_object(page)?;
        }

        Ok(())
    }
}

impl Decode for Pages {
    fn decode(codec: &mut Codec) -> FileResult<Self> {
        Ok(Self(codec.read_collection::<u32, Page>()?))
    }
}
