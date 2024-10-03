use crate::codec::{Codec, Decode, Encode};
use crate::error::FileResult;
use crate::Note;
use std::collections::VecDeque;

pub struct Notes(VecDeque<Note>);

impl Notes {
    pub fn new() -> Self {
        Self(VecDeque::new())
    }

    pub fn push_front(&mut self, text: Note) {
        self.0.push_front(text);
    }

    pub fn push_back(&mut self, text: Note) {
        self.0.push_back(text);
    }

    pub fn insert(&mut self, index: usize, text: Note) {
        self.0.insert(index, text);
    }

    pub fn remove(&mut self, index: usize) {
        self.0.remove(index);
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Note> {
        self.0.get_mut(index)
    }

    pub fn get(&self, index: usize) -> Option<&Note> {
        self.0.get(index)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn inner_mut(&mut self) -> &mut VecDeque<Note> {
        &mut self.0
    }

    pub fn inner(&self) -> &VecDeque<Note> {
        &self.0
    }
}

impl<const N: usize> From<[Note; N]> for Notes {
    fn from(value: [Note; N]) -> Self {
        Self(VecDeque::from(value))
    }
}

impl From<VecDeque<Note>> for Notes {
    fn from(value: VecDeque<Note>) -> Self {
        Self(value)
    }
}

impl Default for Notes {
    fn default() -> Self {
        Self::new()
    }
}

impl Encode for Notes {
    fn encode(&self, codec: &mut Codec) -> FileResult<()> {
        codec.write_primitive::<u32>(self.len() as u32)?;

        for note in &self.0 {
            codec.write_object(note)?;
        }

        Ok(())
    }
}

impl Decode for Notes {
    fn decode(codec: &mut Codec) -> FileResult<Self> {
        Ok(Self(codec.read_collection::<u32, Note>()?))
    }
}
