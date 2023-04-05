use std::path::PathBuf;


mod errors;
mod output;
mod output_langs;
mod encoder;
mod specs;
mod cli_options;

use clap::Parser;
use errors::AppError;
use image::RgbImage;
use output::{OutputResult, EncodedSprite, OutputLanguage};
use output_langs::{ConsoleOutput, Encoder, FileOutput, OutputDevice, RustEncoder};
use cli_options::Cli;

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

    for tile in specifications.sprites.iter() {
        let concrete_specs = MergedSpriteSpecs::new(tile, &specifications);
        let output_tile = encoder::encode_sprite(&concrete_specs, &image)?;
        output.add(output_tile);
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

    let encoder: Box<dyn Encoder> = match options.language.unwrap() { 
        OutputLanguage::Rust => Box::new(RustEncoder::new())
    };
    encoder.write_to(output, &mut *writer)?;
    Ok(())
}

fn load_image(path: &PathBuf) -> Result<RgbImage, AppError> {
    let img = image::open(path)?;
    img.as_rgb8().cloned().ok_or(AppError::CannotConvertToRGB)
}
