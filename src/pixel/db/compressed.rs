use crate::pixel::dxt10::Dxt10Format;
use crate::pixel::gl::GlFormat;
use crate::pixel::vulkan::VkFormat;

use crate::pixel::BaseFormat::*;
use crate::pixel::ColourSpace::*;
use crate::pixel::{BaseFormat, ColourSpace, Dimensions, PixelFormat};

pub fn get_formats() -> Vec<PixelFormat> {
    vec![bc_formats(), eac_formats()]
        .iter()
        .cloned()
        .flatten()
        .collect()
}

fn bc_formats() -> Vec<PixelFormat> {
    vec![
        // BC1
        block_2d("BC1_RGB_UNORM", 4, 4, R8G8B8, Linear)
            .with_gl(GlFormat::COMPRESSED_RGB_S3TC_DXT1_EXT)
            // no dxt10
            .with_vulkan(VkFormat::VK_FORMAT_BC1_RGB_UNORM_BLOCK),
        block_2d("BC1_RGBA_UNORM", 4, 4, R8G8B8A8, Linear)
            .with_four_cc(b"DXT1")
            .with_gl(GlFormat::COMPRESSED_RGBA_S3TC_DXT1_EXT)
            .with_dxt10(Dxt10Format::DXGI_FORMAT_BC1_UNORM)
            .with_vulkan(VkFormat::VK_FORMAT_BC1_RGBA_UNORM_BLOCK),
        block_2d("BC1_RGB_SRGB", 4, 4, R8G8B8A8, sRGB)
            .with_gl(GlFormat::COMPRESSED_SRGB_ALPHA_S3TC_DXT1_EXT)
            // no dxt10
            .with_vulkan(VkFormat::VK_FORMAT_BC1_RGB_SRGB_BLOCK),
        block_2d("BC1_RGBA_SRGB", 4, 4, R8G8B8A8, sRGB)
            .with_gl(GlFormat::COMPRESSED_SRGB_S3TC_DXT1_EXT)
            .with_dxt10(Dxt10Format::DXGI_FORMAT_BC1_UNORM_SRGB)
            .with_vulkan(VkFormat::VK_FORMAT_BC1_RGBA_SRGB_BLOCK),
        // BC2
        block_2d("BC2_RGBA_UNORM", 4, 4, R8G8B8A8, Linear)
            .with_four_cc(b"DXT3")
            .with_gl(GlFormat::COMPRESSED_RGBA_S3TC_DXT3_EXT)
            .with_vulkan(VkFormat::VK_FORMAT_BC2_UNORM_BLOCK),
        block_2d("BC2_RGBA_SRGB", 4, 4, R8G8B8A8, sRGB)
            .with_gl(GlFormat::COMPRESSED_SRGB_ALPHA_S3TC_DXT3_EXT)
            .with_vulkan(VkFormat::VK_FORMAT_BC2_SRGB_BLOCK),
    ]
}

fn eac_formats() -> Vec<PixelFormat> {
    vec![
        block_2d("EAC_R11_UNORM", 4, 4, R11, Linear)
            .with_four_cc(b"EAC1")
            .with_gl(GlFormat::COMPRESSED_R11_EAC)
            .with_vulkan(VkFormat::VK_FORMAT_EAC_R11_UNORM_BLOCK),
        block_2d("EAC_R11_SNORM", 4, 4, R11, Linear)
            .with_gl(GlFormat::COMPRESSED_SIGNED_R11_EAC)
            .with_vulkan(VkFormat::VK_FORMAT_EAC_R11_SNORM_BLOCK),
        block_2d("EAC_R11G11_UNORM", 4, 4, R11G11, Linear)
            .with_four_cc(b"EAC2")
            .with_gl(GlFormat::COMPRESSED_RG11_EAC)
            .with_vulkan(VkFormat::VK_FORMAT_EAC_R11G11_UNORM_BLOCK),
        block_2d("EAC_R11G11_SNORM", 4, 4, R11G11, Linear)
            .with_gl(GlFormat::COMPRESSED_SIGNED_RG11_EAC)
            .with_vulkan(VkFormat::VK_FORMAT_EAC_R11G11_SNORM_BLOCK),
    ]
}

fn block_2d(
    name: &'static str,
    block_x: u32,
    block_y: u32,
    base_format: BaseFormat,
    colour_space: ColourSpace,
) -> PixelFormat {
    let block_dim = Some(Dimensions(block_x, block_y, 0));
    let format = PixelFormat {
        name,
        base_format,
        colour_space,
        block_dim,
        vk_format: None,
        gl_format: None,
        four_cc: None,
        dxt10_format: None,
    };
    format
}
