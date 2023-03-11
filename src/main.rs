use std::path::PathBuf;

use clap::Parser;

mod errors;
mod output;
mod output_langs;
mod processor;
mod specs;

use errors::AppError;
use image::RgbImage;
use output::{OutputResult, OutputTileData};
use output_langs::{ConsoleOutput, Encoder, FileOutput, OutputDevice, RustEncoder};

use crate::specs::ConcretizedTileSpecs;

#[derive(Parser)]
#[command(
    about,
    long_about = "Converts a tile map into a rust module containing the packed info."
)]
struct Cli {
    /// Specifications of the tile map
    specfile: PathBuf,

    /// Output code to file
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Input image
    input: PathBuf,
}

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
    let options = Cli::parse();
    println!("Processing {}", options.input.display());
    let specifications = specs::load_specs(&options.specfile)?;
    let image = load_image(&options.input)?;

    let mut output = OutputResult::new();

    for tile in specifications.tiles.iter() {
        let concrete_specs = ConcretizedTileSpecs::new(tile, &specifications);
        let output_tile = processor::process_tile(&concrete_specs, &image)?;
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

    let encoder: Box<dyn Encoder> = Box::new(RustEncoder::new());
    encoder.write_to(output, &mut *writer)?;
    Ok(())
}

fn load_image(path: &PathBuf) -> Result<RgbImage, AppError> {
    let img = image::open(path)?;
    img.as_rgb8().cloned().ok_or(AppError::CannotConvertToRGB)
}
