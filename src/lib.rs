//! # cyFile: Tool to handle archive files for Cangyan
//!
//! cyFile is an open-source project spun off from another project called "Cangyan", written in the
//! Rust programming language. It is designed to efficiently handle "Cangyan" archive files (with
//! the .cy extension). "Cangyan" is developed to support translation groups in their works of
//! translating, retouching, and typesetting image artworks. It aims to enhance work efficiency by
//! integrating commonly used features necessary for translation works, thus reducing communication
//! barriers between different departments within the team. While enhancing project functionalities,
//! efforts were made to ensure that the software is user-friendly, enabling users to easily adapt
//! to and incorporate the software into their workflow without encountering significant learning
//! curves.

pub mod error;

mod codec;
mod credit;
mod date;
mod file;
mod note;
mod page;
mod project;
mod text;

pub use credit::Credit;
pub use date::Date;
pub use file::ExportArguments;
pub use file::File;
pub use file::Version;
pub use note::Note;
pub use page::Page;
pub use project::Project;
pub use text::Text;
