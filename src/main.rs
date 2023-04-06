use std::path::PathBuf;

mod cli_options;
mod encoder;
mod errors;
mod output;
mod output_langs;
mod specs;

use clap::Parser;
use cli_options::{Cli, OutputLanguageChoice};
use errors::AppError;
use image::RgbImage;
use output::{EncodedSprite, OutputResult};
use output_langs::{ConsoleOutput, FileOutput, OutputDevice, OutputLanguage, RustOutputLanguage};

use crate::specs::MergedSpriteSpecs;

fn main() {
    match wrapped_main() {
        Ok(()) => (),
        Err(e) => {
            println!("Error!");
            println!("{:?}", e);
            std::process::exit(1);
        }
    }
}

fn wrapped_main() -> Result<(), AppError> {
    let options = Cli::parse().apply_commandline_default();
    println!("Processing {}", options.input.display());
    let specifications = specs::load_specs(&options.specifications)?;
    let image = load_image(&options.input)?;

    let mut output = OutputResult::new();

    for sprite in specifications.sprites.iter() {
        let concrete_specs = MergedSpriteSpecs::new(sprite, &specifications);
        let encoded_sprite = encoder::encode_sprite(&concrete_specs, &image)?;
        output.add(encoded_sprite);
    }
    write_results(&output, &options)?;

    println!("Successfully completed");
    Ok(())
}

fn write_results(output: &OutputResult, options: &Cli) -> Result<(), AppError> {
    let mut writer: Box<dyn OutputDevice> = match &options.output {
        None => Box::new(ConsoleOutput::new()),
        Some(filename) => Box::new(FileOutput::new(filename)?),
    };

    let encoder: Box<dyn OutputLanguage> = match options.language.unwrap() {
        OutputLanguageChoice::Rust => Box::new(RustOutputLanguage::new()),
    };
    encoder.write_to(output, &mut *writer)?;
    Ok(())
}

fn load_image(path: &PathBuf) -> Result<RgbImage, AppError> {
    let img = image::open(path)?;
    img.as_rgb8().cloned().ok_or(AppError::CannotConvertToRGB)
}
