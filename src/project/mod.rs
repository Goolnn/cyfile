mod builder;

pub use builder::ProjectBuilder;

use crate::Date;
use crate::Page;

#[derive(Default)]
pub struct Project {
    title: String,

    created_date: Date,
    saved_date: Date,

    pages: Vec<Page>,
}

impl Project {
    pub fn set_title(&mut self, title: impl ToString) {
        self.title = title.to_string();
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn created_date(&self) -> Date {
        self.created_date
    }

    pub fn saved_date(&self) -> Date {
        self.saved_date
    }

    pub fn pages_mut(&mut self) -> &mut Vec<Page> {
        &mut self.pages
    }

    pub fn pages(&self) -> &[Page] {
        &self.pages
    }
}
