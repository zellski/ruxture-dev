use crate::pixel::PixelFormat;

use std::collections::HashSet;

mod compressed;
mod uncompressed;

lazy_static! {
    pub static ref PIXEL_FORMATS: Vec<PixelFormat> = { get_all_formats() };
}

fn get_all_formats() -> Vec<PixelFormat> {
    let mut formats = Vec::new();
    formats.extend(compressed::get_formats());
    formats.extend(uncompressed::get_formats());

    check_for_collissions(&formats);

    formats
}

fn check_for_collissions(formats: &Vec<PixelFormat>) {
    let mut gl_formats = HashSet::new();
    let mut vk_formats = HashSet::new();
    for pixel_format in formats {
        if let Some(gl_format) = pixel_format.gl_format {
            // (internal_format, base_format, colour_space) should be unique
            let unique_tuple = (
                gl_format,
                pixel_format.comp_layout,
                pixel_format.comp_content,
            );
            if gl_formats.contains(&unique_tuple) {
                println!(
                    "WARNING: Mapping collission for GL tuple {:?}: {}",
                    unique_tuple, pixel_format
                );
            } else {
                gl_formats.insert(unique_tuple);
            }
        }
        if let Some(vk_format) = pixel_format.vk_format {
            if vk_formats.contains(&vk_format) {
                println!(
                    "WARNING: Mapping collision for VK format {:?}: {}.",
                    vk_format, pixel_format
                );
            } else {
                vk_formats.insert(vk_format);
            }
        }
    }
}
