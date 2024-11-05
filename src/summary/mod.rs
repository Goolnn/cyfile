use crate::Date;

pub struct Summary {
    pub cover: Option<Vec<u8>>,

    pub category: String,
    pub title: String,

    pub number: (u32, u32),

    pub comment: String,

    pub created_date: Date,
    pub updated_date: Date,
}

impl Summary {}
