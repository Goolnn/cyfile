use crate::Date;
use crate::Page;
use crate::Project;
use std::fs::File;
use std::io::Result;
use std::io::Seek;
use std::io::SeekFrom;
use std::path::Path;

pub trait Build<T> {
    fn build(self) -> T;
}

#[derive(Default)]
pub struct Create {
    title: String,

    created_date: Date,
    saved_date: Date,

    pages: Vec<Page>,
}

pub struct Open {
    file: File,
}

pub struct ProjectBuilder<State> {
    state: State,
}

impl ProjectBuilder<()> {
    pub fn create() -> ProjectBuilder<Create> {
        ProjectBuilder {
            state: Create::default(),
        }
    }

    pub fn open(filepath: impl AsRef<Path>) -> Result<ProjectBuilder<Open>> {
        Ok(ProjectBuilder {
            state: Open {
                file: File::open(filepath)?,
            },
        })
    }
}

impl<State: Build<Project>> ProjectBuilder<State> {
    pub fn build(self) -> Project {
        self.state.build()
    }
}

impl ProjectBuilder<Create> {
    pub fn with_title(mut self, title: impl ToString) -> Self {
        self.state.title = title.to_string();

        self
    }

    pub fn with_page(mut self, page: Page) -> Self {
        self.state.pages.push(page);

        self
    }
}

impl Build<Project> for Create {
    fn build(self) -> Project {
        Project {
            title: self.title,

            created_date: self.created_date,
            saved_date: self.saved_date,

            pages: self.pages,
        }
    }
}

impl ProjectBuilder<Open> {
    // TODO
}

impl Build<Project> for Open {
    fn build(self) -> Project {
        todo!()
    }
}
