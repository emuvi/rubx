pub mod rux_debug;
pub mod rux_fires;
pub mod rux_paths;
pub mod rux_texts;
pub mod rux_times;
pub mod rux_winds;

mod tests;

use std::error::Error;
pub type RubxError = Box<dyn Error + Send + Sync>;
pub type RubxResult<T> = Result<T, RubxError>;
