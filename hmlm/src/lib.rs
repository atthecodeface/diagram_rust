extern crate xml;

mod types;
mod utils;

pub mod error;
pub mod writer;
pub mod reader;
pub use error::{HmlmError, HmlmResult};
pub use types::{MarkupEvent, MarkupName, MarkupAttributes, ParseError};
pub use types::{FilePosition};
pub use reader::reader::EventReader;
pub use types::{HmlFilePosition};
