mod output_device;
mod rust_language;

pub use output_device::{ConsoleOutput, FileOutput};
pub use rust_language::RustOutputLanguage;

use crate::{errors::AppError, output::OutputResult};

/// Abstracts an output device, which can be the standard output or a file.
pub trait OutputDevice {
    fn write_str(&mut self, str: &str) -> Result<(), AppError>;
}

/// Abstracts a target language.
pub trait OutputLanguage {
    fn write_to(
        &self,
        spritess: &OutputResult,
        writer: &mut dyn OutputDevice,
    ) -> Result<(), AppError>;
}
