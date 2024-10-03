use crate::codec::{Codec, Decode, Encode};
use crate::error::FileResult;
use crate::Text;
use std::collections::VecDeque;

pub struct Texts(VecDeque<Text>);

impl Texts {
    pub fn new() -> Self {
        Self(VecDeque::new())
    }

    pub fn push_front(&mut self, text: Text) {
        self.0.push_front(text);
    }

    pub fn push_back(&mut self, text: Text) {
        self.0.push_back(text);
    }

    pub fn insert(&mut self, index: usize, text: Text) {
        self.0.insert(index, text);
    }

    pub fn remove(&mut self, index: usize) {
        self.0.remove(index);
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Text> {
        self.0.get_mut(index)
    }

    pub fn get(&self, index: usize) -> Option<&Text> {
        self.0.get(index)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn inner_mut(&mut self) -> &mut VecDeque<Text> {
        &mut self.0
    }

    pub fn inner(&self) -> &VecDeque<Text> {
        &self.0
    }
}

impl<const N: usize> From<[Text; N]> for Texts {
    fn from(value: [Text; N]) -> Self {
        Self(VecDeque::from(value))
    }
}

impl From<VecDeque<Text>> for Texts {
    fn from(value: VecDeque<Text>) -> Self {
        Self(value)
    }
}

impl Default for Texts {
    fn default() -> Self {
        Self::new()
    }
}

impl Encode for Texts {
    fn encode(&self, codec: &mut Codec) -> FileResult<()> {
        for text in &self.0 {
            codec.write_object(text)?;
        }

        Ok(())
    }
}

impl Decode for Texts {
    fn decode(codec: &mut Codec) -> FileResult<Self> {
        let len = codec.read_primitive::<u32>()?;

        let mut texts = VecDeque::with_capacity(len as usize);

        for _ in 0..len {
            texts.push_back(codec.read_object::<Text>()?);
        }

        Ok(Self::from(texts))
    }
}
