//! # cyFile: Tool to handle archive files for Cangyan
//!
//! cyFile is an open-source project spun off from another open-source project
//! called “[Cangyan](https://github.com/Goolnn/cangyan),” written in the Rust
//! programming language. This tool is designed to efficiently handle archive
//! files (with the `.cy` extension) generated by the Cangyan application. Cangyan
//! is a tool developed to simplify the work of translation teams in translation,
//! retouching, and image typesetting tasks. By integrating commonly used features
//! for translation workflows, it improves efficiency and reduces communication
//! costs between different groups within the team.
//!
//! While enhancing the functionality of the project, cyFile also emphasizes
//! user-friendliness, ensuring that users can easily adapt to and incorporate it
//! into their workflows without encountering significant learning curves.

pub mod codec;
pub mod file;

mod date;
mod note;
mod page;
mod project;
mod text;

pub use date::Date;
pub use file::ExportArguments;
pub use file::File;
pub use file::Version;
pub use note::Note;
pub use page::Page;
pub use project::Project;
pub use text::Text;
