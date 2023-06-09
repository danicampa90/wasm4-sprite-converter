use std::path::PathBuf;

use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(
    about,
    long_about = "Converts a sprite map into a rust module containing the packed info."
)]
pub struct Cli {
    /// Input specifications of the sprites to generate
    pub specifications: PathBuf,

    /// Input image
    pub input: PathBuf,

    /// If specified, output the code to file instad of stdout
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Output language. Rust by default.
    #[arg(short, long, value_enum)]
    pub language: Option<OutputLanguageChoice>,
}

/// Output language switch
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum OutputLanguageChoice {
    // Rust output
    Rust,
}

impl Cli {
    pub fn apply_commandline_default(self) -> Cli {
        Cli {
            specifications: self.specifications,
            output: self.output,
            input: self.input,
            language: Some(self.language.unwrap_or(OutputLanguageChoice::Rust)),
        }
    }
}
