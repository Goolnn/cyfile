// use crate::codec::Reader;
// use crate::Date;
// use crate::Page;
// use crate::Project;
// use std::fs::File;
// use std::io::Result;
// use std::path::Path;

// #[derive(Default)]
// pub struct Create {
//     title: String,

//     created_date: Date,
//     saved_date: Date,

//     pages: Vec<Page>,
// }

// pub struct Open {
//     file: File,
// }

// pub struct ProjectBuilder<State> {
//     state: State,
// }

// impl ProjectBuilder<()> {
//     pub fn create() -> ProjectBuilder<Create> {
//         ProjectBuilder {
//             state: Create::default(),
//         }
//     }

//     pub fn open(filepath: impl AsRef<Path>) -> Result<ProjectBuilder<Open>> {
//         Ok(ProjectBuilder {
//             state: Open {
//                 file: File::open(filepath)?,
//             },
//         })
//     }
// }

// impl ProjectBuilder<Create> {
//     pub fn with_title(mut self, title: impl ToString) -> Self {
//         self.state.title = title.to_string();

//         self
//     }

//     pub fn with_page(mut self, page: Page) -> Self {
//         self.state.pages.push(page);

//         self
//     }

//     pub fn build(self) -> Project {
//         Project {
//             title: self.state.title,

//             created_date: self.state.created_date,
//             saved_date: self.state.saved_date,

//             pages: self.state.pages,
//         }
//     }
// }

// impl ProjectBuilder<Open> {
//     pub fn build(self) -> Project {
//         let mut reader = Reader::new(self.state.file);

//         reader.read_bytes(15).unwrap();
//         reader.read_bytes(2).unwrap();

//         reader.read_object::<Project>().unwrap()
//     }
// }
