use crate::errors::AppError;
use crate::specs::ConcretizedTileSpecs;
use crate::OutputTileData;
use image::RgbImage;

pub fn process_tile(
    tile_spec: &ConcretizedTileSpecs,
    image: &RgbImage,
) -> Result<OutputTileData, AppError> {
    let mut pixel_palette_indexes = vec![];

    // scan image and get the indexes into the palette
    for y in 0..tile_spec.height_pixels {
        for x in 0..tile_spec.width_pixels {
            let x = (tile_spec.x_pixels + x) as u32;
            let y = (tile_spec.y_pixels + y) as u32;
            let pix = image.get_pixel(x, y);
            let combined = (pix[0] as u32) << 16 | (pix[1] as u32) << 8 | (pix[2] as u32) << 0;
            let color_index = tile_spec
                .colors
                .iter()
                .position(|element| *element == combined);

            match color_index {
                None => {
                    println!("Color info for not found color: {:#08x}", combined);
                    return Err(AppError::CannotFindColor {
                        x,
                        y,
                        color: combined,
                    });
                }
                Some(index) => {
                    pixel_palette_indexes.push(index);
                }
            }
        }
    }

    // encode the paletted image into packed bytes.
    let mut resulting_bytes = vec![];
    for big_step_idx in (0..pixel_palette_indexes.len()).step_by(8/tile_spec.bpp) {
        let mut byte: u8 = 0;
        for intrabyte_idx in 0..(8/tile_spec.bpp) {
            byte = byte << tile_spec.bpp;
            let pixel = pixel_palette_indexes
                .get(intrabyte_idx + big_step_idx)
                .cloned();
            byte |= pixel.unwrap_or(0) as u8;
            //println!("{:08b}: {:?}",byte, pixel )
        }
        //println!("{:08b}", byte);
        resulting_bytes.push(byte);
    }

    // print stats
    println!("- {}: {} bytes", tile_spec.name, resulting_bytes.len());

    // result
    Ok(OutputTileData::new(resulting_bytes, tile_spec))
}
