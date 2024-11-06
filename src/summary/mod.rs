use crate::Date;
use crate::Project;

pub struct Summary {
    pub cover: Vec<u8>,

    pub category: String,
    pub title: String,

    pub number: (u32, u32),

    pub comment: String,

    pub created_date: Date,
    pub updated_date: Date,
}

impl From<Summary> for Project {
    fn from(value: Summary) -> Self {
        Self::new()
            .with_cover(value.cover)
            .with_category(value.category)
            .with_title(value.title)
            .with_number(value.number)
            .with_comment(value.comment)
            .with_created_date(value.created_date)
            .with_updated_date(value.updated_date)
    }
}
