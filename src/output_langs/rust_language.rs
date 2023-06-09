use super::{OutputDevice, OutputLanguage};
use crate::{errors::AppError, output::OutputResult};

/// Implementation for the output language writer for Rust.
pub struct RustOutputLanguage {}
impl RustOutputLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl OutputLanguage for RustOutputLanguage {
    fn write_to(
        &self,
        sprites: &OutputResult,
        writer: &mut dyn OutputDevice,
    ) -> Result<(), AppError> {
        writer.write_str("// Autogenerated by wasm4_sprite_converter\n\n")?;
        for sprite in sprites.iter() {
            writer.write_str(&format!("// -- {}\n", sprite.name()))?;
            // write data declaration
            writer.write_str(&format!(
                "pub const SPRITEDATA_{}: [u8; {}]= [",
                sprite.name(),
                sprite.bytes().len()
            ))?;
            // write bytes in the array
            for byte in sprite.bytes() {
                writer.write_str(&format!("{},", byte))?;
            }
            // end byte array
            writer.write_str("];\n")?;
            writer.write_str(&format!(
                "pub const SPRITE_{}_WIDTH: u32 = {};\n",
                sprite.name(),
                sprite.width_pixels()
            ))?;
            writer.write_str(&format!(
                "pub const SPRITE_{}_HEIGHT: u32 = {};\n",
                sprite.name(),
                sprite.height_pixels()
            ))?;
            writer.write_str(&format!(
                "pub const SPRITE_{}_BPP: u32 = {};\n",
                sprite.name(),
                sprite.bpp()
            ))?;
            writer.write_str("\n\n")?;
        }
        Ok(())
    }
}
