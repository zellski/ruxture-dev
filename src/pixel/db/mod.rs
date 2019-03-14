use crate::pixel::PixelFormat;

mod compressed;
mod uncompressed;

lazy_static! {
    pub static ref PIXEL_FORMATS: Vec<PixelFormat> = { get_all_formats() };
}

fn get_all_formats() -> Vec<PixelFormat> {
    let mut result = Vec::new();
    result.extend(compressed::get_formats());
    result.extend(uncompressed::get_formats());
    result
}
