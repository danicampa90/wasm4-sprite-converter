use std::{fs::File, io::Write, path::PathBuf};

use crate::errors::AppError;

use super::OutputDevice;

pub struct ConsoleOutput {}
impl ConsoleOutput {
    pub fn new() -> ConsoleOutput {
        Self {}
    }
}
impl OutputDevice for ConsoleOutput {
    fn write_str(&mut self, str: &str) -> Result<(), AppError> {
        print!("{}", str);
        Ok(())
    }
}

pub struct FileOutput {
    file: File,
}
impl FileOutput {
    pub fn new(file_path: &PathBuf) -> Result<Self, AppError> {
        Ok(Self {
            file: std::fs::File::create(file_path)?,
        })
    }
}
impl OutputDevice for FileOutput {
    fn write_str(&mut self, str: &str) -> Result<(), AppError> {
        write!(self.file, "{}", str)?;
        Ok(())
    }
}
