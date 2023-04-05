use image::ImageError;

#[derive(Debug)]
pub enum AppError {
    SerdeError(serde_yaml::Error),
    IoError(std::io::Error),
    ImageLoadError(ImageError),
    CannotConvertToRGB,
    CannotFindColor { x: u32, y: u32, color: u32 },
}

impl From<serde_yaml::Error> for AppError {
    fn from(value: serde_yaml::Error) -> Self {
        AppError::SerdeError(value)
    }
}

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        AppError::IoError(value)
    }
}

impl From<ImageError> for AppError {
    fn from(value: ImageError) -> Self {
        AppError::ImageLoadError(value)
    }
}
