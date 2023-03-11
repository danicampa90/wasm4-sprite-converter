mod output_device;
mod rust_encoder;

pub use output_device::{ConsoleOutput, FileOutput};
pub use rust_encoder::RustEncoder;

use crate::{errors::AppError, output::OutputResult};

pub trait OutputDevice {
    fn write_str(&mut self, str: &str) -> Result<(), AppError>;
}

pub trait Encoder {
    fn write_to(&self, tiles: &OutputResult, writer: &mut dyn OutputDevice)
        -> Result<(), AppError>;
}
