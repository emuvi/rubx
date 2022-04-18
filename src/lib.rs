pub mod debug;
pub mod fires;
pub mod paths;
pub mod texts;
pub mod times;
pub mod winds;

mod tests;

use std::error::Error;
pub type RubxError = Box<dyn Error + Send + Sync>;