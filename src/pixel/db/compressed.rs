use crate::pixel::dxt10::Dxt10Format;
use crate::pixel::gl::GlFormat;
use crate::pixel::vulkan::VkFormat;

use crate::pixel::CompContent::*;
use crate::pixel::CompLayout::*;
use crate::pixel::{CompContent, CompLayout, Dimensions, PixelFormat};

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
        block_2d("BC1", 4, 4, R8G8B8, UNORM)
            .with_gl(GlFormat::COMPRESSED_RGB_S3TC_DXT1_EXT)
            // no dxt10
            .with_vulkan(VkFormat::VK_FORMAT_BC1_RGB_UNORM_BLOCK),
        block_2d("BC1", 4, 4, R8G8B8A8, UNORM)
            .with_four_cc(b"DXT1")
            .with_gl(GlFormat::COMPRESSED_RGBA_S3TC_DXT1_EXT)
            .with_dxt10(Dxt10Format::DXGI_FORMAT_BC1_UNORM)
            .with_vulkan(VkFormat::VK_FORMAT_BC1_RGBA_UNORM_BLOCK),
        block_2d("BC1", 4, 4, R8G8B8A8, SRGB)
            .with_gl(GlFormat::COMPRESSED_SRGB_ALPHA_S3TC_DXT1_EXT)
            // no dxt10
            .with_vulkan(VkFormat::VK_FORMAT_BC1_RGB_SRGB_BLOCK),
        block_2d("BC1", 4, 4, R8G8B8A8, SRGB)
            .with_gl(GlFormat::COMPRESSED_SRGB_S3TC_DXT1_EXT)
            .with_dxt10(Dxt10Format::DXGI_FORMAT_BC1_UNORM_SRGB)
            .with_vulkan(VkFormat::VK_FORMAT_BC1_RGBA_SRGB_BLOCK),
        // BC2
        block_2d("BC2", 4, 4, R8G8B8A8, UNORM)
            .with_four_cc(b"DXT3")
            .with_gl(GlFormat::COMPRESSED_RGBA_S3TC_DXT3_EXT)
            .with_vulkan(VkFormat::VK_FORMAT_BC2_UNORM_BLOCK),
        block_2d("BC2", 4, 4, R8G8B8A8, SRGB)
            .with_gl(GlFormat::COMPRESSED_SRGB_ALPHA_S3TC_DXT3_EXT)
            .with_vulkan(VkFormat::VK_FORMAT_BC2_SRGB_BLOCK),
    ]
}

fn eac_formats() -> Vec<PixelFormat> {
    vec![
        block_2d("EAC", 4, 4, R11, UNORM)
            .with_four_cc(b"EAC1")
            .with_gl(GlFormat::COMPRESSED_R11_EAC)
            .with_vulkan(VkFormat::VK_FORMAT_EAC_R11_UNORM_BLOCK),
        block_2d("EAC", 4, 4, R11, SNORM)
            .with_gl(GlFormat::COMPRESSED_SIGNED_R11_EAC)
            .with_vulkan(VkFormat::VK_FORMAT_EAC_R11_SNORM_BLOCK),
        block_2d("EAC", 4, 4, R11G11, UNORM)
            .with_four_cc(b"EAC2")
            .with_gl(GlFormat::COMPRESSED_RG11_EAC)
            .with_vulkan(VkFormat::VK_FORMAT_EAC_R11G11_UNORM_BLOCK),
        block_2d("EAC", 4, 4, R11G11, SNORM)
            .with_gl(GlFormat::COMPRESSED_SIGNED_RG11_EAC)
            .with_vulkan(VkFormat::VK_FORMAT_EAC_R11G11_SNORM_BLOCK),
    ]
}

fn block_2d(
    tag: &'static str,
    block_x: u32,
    block_y: u32,
    comp_layout: CompLayout,
    comp_content: CompContent,
) -> PixelFormat {
    let block_dim = Some(Dimensions(block_x, block_y, 0));
    let format = PixelFormat {
        tag,
        comp_layout,
        comp_content,
        block_dim,
        vk_format: None,
        gl_format: None,
        four_cc: None,
        dxt10_format: None,
    };
    format
}
