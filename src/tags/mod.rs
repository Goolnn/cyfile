use crate::codec::{Codec, Decode, Encode};
use crate::error::FileResult;
use std::collections::HashSet;

pub type Tag = String;

pub struct Tags(HashSet<Tag>);

impl Tags {
    pub fn new() -> Self {
        Self(HashSet::new())
    }

    pub fn insert(&mut self, tag: Tag) {
        self.0.insert(tag.to_string());
    }

    pub fn remove(&mut self, tag: &Tag) {
        self.0.remove(tag);
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn inner_mut(&mut self) -> &mut HashSet<Tag> {
        &mut self.0
    }

    pub fn inner(&self) -> &HashSet<Tag> {
        &self.0
    }
}

impl Default for Tags {
    fn default() -> Self {
        Self::new()
    }
}

impl Encode for Tags {
    fn encode(&self, codec: &mut Codec) -> FileResult<()> {
        // 标签数量
        codec.write_primitive(self.len() as u32)?;

        // 标签数据
        for tag in self.inner() {
            codec.write_string::<u32>(tag)?;
        }

        Ok(())
    }
}

impl Decode for Tags {
    fn decode(codec: &mut Codec) -> FileResult<Self> {
        // 标签数量
        let note_count = codec.read_primitive::<u32>()?;

        let mut tags = HashSet::with_capacity(note_count as usize);

        // 标签数据
        for _ in 0..note_count {
            let tag = codec.read_string::<u32>()?;

            tags.insert(tag);
        }

        Ok(Self(tags))
    }
}
