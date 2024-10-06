use crate::Page;
use crate::Project;

#[derive(Default)]
pub struct ProjectBuilder {
    project: Project,
}

impl ProjectBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_title(mut self, title: impl ToString) -> Self {
        self.project.title = title.to_string();

        self
    }

    pub fn with_page(mut self, page: Page) -> Self {
        self.project.pages.push(page);

        self
    }

    pub fn build(self) -> Project {
        self.project
    }
}
