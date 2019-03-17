use crate::pixel::gl::GlFormat;
use crate::pixel::vulkan::VkFormat;

use crate::pixel::BaseFormat::*;
use crate::pixel::ColourSpace::*;
use crate::pixel::{BaseFormat, ColourSpace, PixelFormat};

pub fn get_formats() -> Vec<PixelFormat> {
    vec![
        vulkan_a2(),
        vulkan_scaled(),
        vulkan_only_formats(),
        gl_supported_formats(),
    ]
    .iter()
    .cloned()
    .flatten()
    .collect()
}

fn vulkan_a2() -> Vec<PixelFormat> {
    vec![
        uncompressed("A2R10G10B10_UNORM", A2R10G10B10, Linear)
            .with_vulkan(VkFormat::VK_FORMAT_A2R10G10B10_UNORM_PACK32),
        uncompressed("A2R10G10B10_SNORM", A2R10G10B10, Linear)
            .with_vulkan(VkFormat::VK_FORMAT_A2R10G10B10_SNORM_PACK32),
        uncompressed("A2R10G10B10_USCALED", A2R10G10B10, Linear)
            .with_vulkan(VkFormat::VK_FORMAT_A2R10G10B10_USCALED_PACK32),
        uncompressed("A2R10G10B10_SSCALED", A2R10G10B10, Linear)
            .with_vulkan(VkFormat::VK_FORMAT_A2R10G10B10_SSCALED_PACK32),
        uncompressed("A2R10G10B10_UINT", A2R10G10B10, Linear)
            .with_vulkan(VkFormat::VK_FORMAT_A2R10G10B10_UINT_PACK32),
        uncompressed("A2R10G10B10_SINT", A2R10G10B10, Linear)
            .with_vulkan(VkFormat::VK_FORMAT_A2R10G10B10_SINT_PACK32),
        uncompressed("A2B10G10R10_UNORM", A2R10G10B10, Linear)
            .with_vulkan(VkFormat::VK_FORMAT_A2B10G10R10_UNORM_PACK32),
        uncompressed("A2B10G10R10_SNORM", A2R10G10B10, Linear)
            .with_vulkan(VkFormat::VK_FORMAT_A2B10G10R10_SNORM_PACK32),
        uncompressed("A2B10G10R10_USCALED", A2R10G10B10, Linear)
            .with_vulkan(VkFormat::VK_FORMAT_A2B10G10R10_USCALED_PACK32),
        uncompressed("A2B10G10R10_SSCALED", A2R10G10B10, Linear)
            .with_vulkan(VkFormat::VK_FORMAT_A2B10G10R10_SSCALED_PACK32),
        uncompressed("A2B10G10R10_UINT", A2R10G10B10, Linear)
            .with_vulkan(VkFormat::VK_FORMAT_A2B10G10R10_UINT_PACK32),
        uncompressed("A2B10G10R10_SINT", A2R10G10B10, Linear)
            .with_vulkan(VkFormat::VK_FORMAT_A2B10G10R10_SINT_PACK32),
    ]
}

fn vulkan_scaled() -> Vec<PixelFormat> {
    vec![
        uncompressed("R8_USCALED", R8, Linear).with_vulkan(VkFormat::VK_FORMAT_R8_USCALED),
        uncompressed("R8_SSCALED", R8, Linear).with_vulkan(VkFormat::VK_FORMAT_R8_SSCALED),
        uncompressed("R8G8_USCALED", R8G8, Linear).with_vulkan(VkFormat::VK_FORMAT_R8G8_USCALED),
        uncompressed("R8G8_SSCALED", R8G8, Linear).with_vulkan(VkFormat::VK_FORMAT_R8G8_SSCALED),
        uncompressed("R8G8B8_USCALED", R8G8B8, Linear)
            .with_vulkan(VkFormat::VK_FORMAT_R8G8B8_USCALED),
        uncompressed("R8G8B8_SSCALED", R8G8B8, Linear)
            .with_vulkan(VkFormat::VK_FORMAT_R8G8B8_SSCALED),
        uncompressed("B8G8R8_USCALED", B8G8R8, Linear)
            .with_vulkan(VkFormat::VK_FORMAT_B8G8R8_USCALED),
        uncompressed("B8G8R8_SSCALED", B8G8R8, Linear)
            .with_vulkan(VkFormat::VK_FORMAT_B8G8R8_SSCALED),
        uncompressed("R8G8B8A8_USCALED", R8G8B8A8, Linear)
            .with_vulkan(VkFormat::VK_FORMAT_R8G8B8A8_USCALED),
        uncompressed("R8G8B8A8_SSCALED", R8G8B8A8, Linear)
            .with_vulkan(VkFormat::VK_FORMAT_R8G8B8A8_SSCALED),
        uncompressed("B8G8R8A8_USCALED", B8G8R8A8, Linear)
            .with_vulkan(VkFormat::VK_FORMAT_B8G8R8A8_USCALED),
        uncompressed("B8G8R8A8_SSCALED", B8G8R8A8, Linear)
            .with_vulkan(VkFormat::VK_FORMAT_B8G8R8A8_SSCALED),
        uncompressed("A8B8G8R8_USCALED", A8B8G8R8, Linear)
            .with_vulkan(VkFormat::VK_FORMAT_A8B8G8R8_USCALED_PACK32),
        uncompressed("A8B8G8R8_SSCALED", A8B8G8R8, Linear)
            .with_vulkan(VkFormat::VK_FORMAT_A8B8G8R8_SSCALED_PACK32),
        uncompressed("R16_USCALED", R16, Linear).with_vulkan(VkFormat::VK_FORMAT_R16_USCALED),
        uncompressed("R16_SSCALED", R16, Linear).with_vulkan(VkFormat::VK_FORMAT_R16_SSCALED),
        uncompressed("R16G16_USCALED", R16G16, Linear)
            .with_vulkan(VkFormat::VK_FORMAT_R16G16_USCALED),
        uncompressed("R16G16_SSCALED", R16G16, Linear)
            .with_vulkan(VkFormat::VK_FORMAT_R16G16_SSCALED),
        uncompressed("R16G16B16_USCALED", R16G16B16, Linear)
            .with_vulkan(VkFormat::VK_FORMAT_R16G16B16_USCALED),
        uncompressed("R16G16B16_SSCALED", R16G16B16, Linear)
            .with_vulkan(VkFormat::VK_FORMAT_R16G16B16_SSCALED),
        uncompressed("R16G16B16A16_USCALED", R16G16B16A16, Linear)
            .with_vulkan(VkFormat::VK_FORMAT_R16G16B16A16_USCALED),
        uncompressed("R16G16B16A16_SSCALED", R16G16B16A16, Linear)
            .with_vulkan(VkFormat::VK_FORMAT_R16G16B16A16_SSCALED),
    ]
}

fn vulkan_only_formats() -> Vec<PixelFormat> {
    vec![
        uncompressed("R4G4_UNORM_PACK8", R64, Linear)
            .with_vulkan(VkFormat::VK_FORMAT_R4G4_UNORM_PACK8),
        uncompressed("B4G4R4A4_UNORM", B4G4R4A4, Linear)
            .with_vulkan(VkFormat::VK_FORMAT_B4G4R4A4_UNORM_PACK16),
        uncompressed("B5G6R5_UNORM", B5G6R5, Linear)
            .with_vulkan(VkFormat::VK_FORMAT_B5G6R5_UNORM_PACK16),
        uncompressed("B5G5R5A1_UNORM", B5G5R5A1, Linear)
            .with_vulkan(VkFormat::VK_FORMAT_B5G5R5A1_UNORM_PACK16),
        uncompressed("A1R5G5B5_UNORM", A1R5G5B5, Linear)
            .with_vulkan(VkFormat::VK_FORMAT_A1R5G5B5_UNORM_PACK16),
        uncompressed("R8_SRGB", R8, sRGB).with_vulkan(VkFormat::VK_FORMAT_R8_SRGB),
        uncompressed("R64_UINT", R64, Linear).with_vulkan(VkFormat::VK_FORMAT_R64_UINT),
        uncompressed("R64_SINT", R64, Linear).with_vulkan(VkFormat::VK_FORMAT_R64_SINT),
        uncompressed("R64_SFLOAT", R64, Linear).with_vulkan(VkFormat::VK_FORMAT_R64_SFLOAT),
        uncompressed("R64G64_UINT", R64G64, Linear).with_vulkan(VkFormat::VK_FORMAT_R64G64_UINT),
        uncompressed("R64G64_SINT", R64G64, Linear).with_vulkan(VkFormat::VK_FORMAT_R64G64_SINT),
        uncompressed("R64G64_SFLOAT", R64G64, Linear)
            .with_vulkan(VkFormat::VK_FORMAT_R64G64_SFLOAT),
        uncompressed("R64G64B64_UINT", R64G64B64, Linear)
            .with_vulkan(VkFormat::VK_FORMAT_R64G64B64_UINT),
        uncompressed("R64G64B64_SINT", R64G64B64, Linear)
            .with_vulkan(VkFormat::VK_FORMAT_R64G64B64_SINT),
        uncompressed("R64G64B64_SFLOAT", R64G64B64, Linear)
            .with_vulkan(VkFormat::VK_FORMAT_R64G64B64_SFLOAT),
        uncompressed("R64G64B64A64_UINT", R64G64B64A64, Linear)
            .with_vulkan(VkFormat::VK_FORMAT_R64G64B64A64_UINT),
        uncompressed("R64G64B64A64_SINT", R64G64B64A64, Linear)
            .with_vulkan(VkFormat::VK_FORMAT_R64G64B64A64_SINT),
        uncompressed("R64G64B64A64_SFLOAT", R64G64B64A64, Linear)
            .with_vulkan(VkFormat::VK_FORMAT_R64G64B64A64_SFLOAT),
        uncompressed("D16_UNORM", D16, Linear).with_vulkan(VkFormat::VK_FORMAT_D16_UNORM),
        uncompressed("X8_D24_UNORM", X8D24, Linear)
            .with_vulkan(VkFormat::VK_FORMAT_X8_D24_UNORM_PACK32),
        uncompressed("D32_SFLOAT", D32, Linear).with_vulkan(VkFormat::VK_FORMAT_D32_SFLOAT),
        uncompressed("S8_UINT", S8, Linear).with_vulkan(VkFormat::VK_FORMAT_S8_UINT),
        uncompressed("D16_UNORM_S8_UINT", D16S8, Linear)
            .with_vulkan(VkFormat::VK_FORMAT_D16_UNORM_S8_UINT),
        uncompressed("D24_UNORM_S8_UINT", D24S8, Linear)
            .with_vulkan(VkFormat::VK_FORMAT_D24_UNORM_S8_UINT),
        uncompressed("D32_SFLOAT_S8_UINT", D32S8, Linear)
            .with_vulkan(VkFormat::VK_FORMAT_D32_SFLOAT_S8_UINT),
    ]
}

fn gl_supported_formats() -> Vec<PixelFormat> {
    vec![
        uncompressed("R4G4B4A4_UNORM", R4G4B4A4, Linear)
            .with_gl(GlFormat::RGBA4)
            .with_vulkan(VkFormat::VK_FORMAT_R4G4B4A4_UNORM_PACK16),
        uncompressed("R5G6B5_UNORM", R5G6B5, Linear)
            .with_gl(GlFormat::RGB565)
            .with_vulkan(VkFormat::VK_FORMAT_R5G6B5_UNORM_PACK16),
        uncompressed("R5G5B5A1_UNORM", R5G5B5A1, Linear)
            .with_gl(GlFormat::RGB5_A1)
            .with_vulkan(VkFormat::VK_FORMAT_R5G5B5A1_UNORM_PACK16),
        uncompressed("R8_UNORM", R8, Linear)
            .with_gl(GlFormat::R8)
            .with_vulkan(VkFormat::VK_FORMAT_R8_UNORM),
        uncompressed("R8_SNORM", R8, Linear)
            .with_gl(GlFormat::R8_SNORM)
            .with_vulkan(VkFormat::VK_FORMAT_R8_SNORM),
        uncompressed("R8_UINT", R8, Linear)
            .with_gl(GlFormat::R8UI)
            .with_vulkan(VkFormat::VK_FORMAT_R8_UINT),
        uncompressed("R8_SINT", R8, Linear)
            .with_gl(GlFormat::R8I)
            .with_vulkan(VkFormat::VK_FORMAT_R8_SINT),
        uncompressed("R8G8_UNORM", R8G8, Linear)
            .with_gl(GlFormat::RG8)
            .with_vulkan(VkFormat::VK_FORMAT_R8G8_UNORM),
        uncompressed("R8G8_SNORM", R8G8, Linear)
            .with_gl(GlFormat::RG8_SNORM)
            .with_vulkan(VkFormat::VK_FORMAT_R8G8_SNORM),
        uncompressed("R8G8_UINT", R8G8, Linear)
            .with_gl(GlFormat::RG8UI)
            .with_vulkan(VkFormat::VK_FORMAT_R8G8_UINT),
        uncompressed("R8G8_SINT", R8G8, Linear)
            .with_gl(GlFormat::RG8I)
            .with_vulkan(VkFormat::VK_FORMAT_R8G8_SINT),
        uncompressed("R8G8_SRGB", R8G8, sRGB)
            .with_gl(GlFormat::RG8)
            .with_vulkan(VkFormat::VK_FORMAT_R8G8_SRGB),
        uncompressed("R8G8B8_UNORM", R8G8B8, Linear)
            .with_gl(GlFormat::RGB8)
            .with_vulkan(VkFormat::VK_FORMAT_R8G8B8_UNORM),
        uncompressed("R8G8B8_SNORM", R8G8B8, Linear)
            .with_gl(GlFormat::RGB8_SNORM)
            .with_vulkan(VkFormat::VK_FORMAT_R8G8B8_SNORM),
        uncompressed("R8G8B8_UINT", R8G8B8, Linear)
            .with_gl(GlFormat::RGB8UI)
            .with_vulkan(VkFormat::VK_FORMAT_R8G8B8_UINT),
        uncompressed("R8G8B8_SINT", R8G8B8, Linear)
            .with_gl(GlFormat::RGB8I)
            .with_vulkan(VkFormat::VK_FORMAT_R8G8B8_SINT),
        uncompressed("R8G8B8_SRGB", R8G8B8, sRGB)
            .with_gl(GlFormat::RGB8)
            .with_vulkan(VkFormat::VK_FORMAT_R8G8B8_SRGB),
        uncompressed("B8G8R8_UNORM", B8G8R8, Linear)
            .with_gl(GlFormat::RGB8)
            .with_vulkan(VkFormat::VK_FORMAT_B8G8R8_UNORM),
        uncompressed("B8G8R8_SNORM", B8G8R8, Linear)
            .with_gl(GlFormat::RGB8_SNORM)
            .with_vulkan(VkFormat::VK_FORMAT_B8G8R8_SNORM),
        uncompressed("B8G8R8_UINT", B8G8R8, Linear)
            .with_gl(GlFormat::RGB8UI)
            .with_vulkan(VkFormat::VK_FORMAT_B8G8R8_UINT),
        uncompressed("B8G8R8_SINT", B8G8R8, Linear)
            .with_gl(GlFormat::RGB8I)
            .with_vulkan(VkFormat::VK_FORMAT_B8G8R8_SINT),
        uncompressed("B8G8R8_SRGB", B8G8R8, sRGB)
            .with_gl(GlFormat::RGB8)
            .with_vulkan(VkFormat::VK_FORMAT_B8G8R8_SRGB),
        uncompressed("R8G8B8A8_UNORM", R8G8B8A8, Linear)
            .with_gl(GlFormat::RGBA8)
            .with_vulkan(VkFormat::VK_FORMAT_R8G8B8A8_UNORM),
        uncompressed("R8G8B8A8_SNORM", R8G8B8A8, Linear)
            .with_gl(GlFormat::RGBA8_SNORM)
            .with_vulkan(VkFormat::VK_FORMAT_R8G8B8A8_SNORM),
        uncompressed("R8G8B8A8_UINT", R8G8B8A8, Linear)
            .with_gl(GlFormat::RGBA8UI)
            .with_vulkan(VkFormat::VK_FORMAT_R8G8B8A8_UINT),
        uncompressed("R8G8B8A8_SINT", R8G8B8A8, Linear)
            .with_gl(GlFormat::RGBA8I)
            .with_vulkan(VkFormat::VK_FORMAT_R8G8B8A8_SINT),
        uncompressed("R8G8B8A8_SRGB", R8G8B8A8, sRGB)
            .with_gl(GlFormat::RGBA8)
            .with_vulkan(VkFormat::VK_FORMAT_R8G8B8A8_SRGB),
        uncompressed("B8G8R8A8_UNORM", B8G8R8A8, Linear)
            .with_gl(GlFormat::RGBA8)
            .with_vulkan(VkFormat::VK_FORMAT_B8G8R8A8_UNORM),
        uncompressed("B8G8R8A8_SNORM", B8G8R8A8, Linear)
            .with_gl(GlFormat::RGBA8_SNORM)
            .with_vulkan(VkFormat::VK_FORMAT_B8G8R8A8_SNORM),
        uncompressed("B8G8R8A8_UINT", B8G8R8A8, Linear)
            .with_gl(GlFormat::RGBA8UI)
            .with_vulkan(VkFormat::VK_FORMAT_B8G8R8A8_UINT),
        uncompressed("B8G8R8A8_SINT", B8G8R8A8, Linear)
            .with_gl(GlFormat::RGBA8I)
            .with_vulkan(VkFormat::VK_FORMAT_B8G8R8A8_SINT),
        uncompressed("B8G8R8A8_SRGB", B8G8R8A8, sRGB)
            .with_gl(GlFormat::RGBA8)
            .with_vulkan(VkFormat::VK_FORMAT_B8G8R8A8_SRGB),
        uncompressed("A8B8G8R8_UNORM", A8B8G8R8, Linear)
            .with_gl(GlFormat::RGBA8)
            .with_vulkan(VkFormat::VK_FORMAT_A8B8G8R8_UNORM_PACK32),
        uncompressed("A8B8G8R8_SNORM", A8B8G8R8, Linear)
            .with_gl(GlFormat::RGBA8_SNORM)
            .with_vulkan(VkFormat::VK_FORMAT_A8B8G8R8_SNORM_PACK32),
        uncompressed("A8B8G8R8_UINT", A8B8G8R8, Linear)
            .with_gl(GlFormat::RGBA8UI)
            .with_vulkan(VkFormat::VK_FORMAT_A8B8G8R8_UINT_PACK32),
        uncompressed("A8B8G8R8_SINT", A8B8G8R8, Linear)
            .with_gl(GlFormat::RGBA8I)
            .with_vulkan(VkFormat::VK_FORMAT_A8B8G8R8_SINT_PACK32),
        uncompressed("A8B8G8R8_SRGB", A8B8G8R8, sRGB)
            .with_gl(GlFormat::RGBA8)
            .with_vulkan(VkFormat::VK_FORMAT_A8B8G8R8_SRGB_PACK32),
        uncompressed("R16_UNORM", R16, Linear)
            .with_gl(GlFormat::R16)
            .with_vulkan(VkFormat::VK_FORMAT_R16_UNORM),
        uncompressed("R16_SNORM", R16, Linear)
            .with_gl(GlFormat::R16_SNORM)
            .with_vulkan(VkFormat::VK_FORMAT_R16_SNORM),
        uncompressed("R16_UINT", R16, Linear)
            .with_gl(GlFormat::R16UI)
            .with_vulkan(VkFormat::VK_FORMAT_R16_UINT),
        uncompressed("R16_SINT", R16, Linear)
            .with_gl(GlFormat::R16I)
            .with_vulkan(VkFormat::VK_FORMAT_R16_SINT),
        uncompressed("R16_SFLOAT", R16, Linear)
            .with_gl(GlFormat::R16F)
            .with_vulkan(VkFormat::VK_FORMAT_R16_SFLOAT),
        uncompressed("R16G16_UNORM", R16G16, Linear)
            .with_gl(GlFormat::RG16)
            .with_vulkan(VkFormat::VK_FORMAT_R16G16_UNORM),
        uncompressed("R16G16_SNORM", R16G16, Linear)
            .with_gl(GlFormat::RG16_SNORM)
            .with_vulkan(VkFormat::VK_FORMAT_R16G16_SNORM),
        uncompressed("R16G16_UINT", R16G16, Linear)
            .with_gl(GlFormat::RG16UI)
            .with_vulkan(VkFormat::VK_FORMAT_R16G16_UINT),
        uncompressed("R16G16_SINT", R16G16, Linear)
            .with_gl(GlFormat::RG16I)
            .with_vulkan(VkFormat::VK_FORMAT_R16G16_SINT),
        uncompressed("R16G16_SFLOAT", R16G16, Linear)
            .with_gl(GlFormat::RG16F)
            .with_vulkan(VkFormat::VK_FORMAT_R16G16_SFLOAT),
        uncompressed("R16G16B16_UNORM", R16G16B16, Linear)
            .with_gl(GlFormat::RGB16)
            .with_vulkan(VkFormat::VK_FORMAT_R16G16B16_UNORM),
        uncompressed("R16G16B16_SNORM", R16G16B16, Linear)
            .with_gl(GlFormat::RGB16_SNORM)
            .with_vulkan(VkFormat::VK_FORMAT_R16G16B16_SNORM),
        uncompressed("R16G16B16_UINT", R16G16B16, Linear)
            .with_gl(GlFormat::RGB16UI)
            .with_vulkan(VkFormat::VK_FORMAT_R16G16B16_UINT),
        uncompressed("R16G16B16_SINT", R16G16B16, Linear)
            .with_gl(GlFormat::RGB16I)
            .with_vulkan(VkFormat::VK_FORMAT_R16G16B16_SINT),
        uncompressed("R16G16B16_SFLOAT", R16G16B16, Linear)
            .with_gl(GlFormat::RGB16F)
            .with_vulkan(VkFormat::VK_FORMAT_R16G16B16_SFLOAT),
        uncompressed("R16G16B16A16_UNORM", R16G16B16A16, Linear)
            .with_gl(GlFormat::RGBA16)
            .with_vulkan(VkFormat::VK_FORMAT_R16G16B16A16_UNORM),
        uncompressed("R16G16B16A16_SNORM", R16G16B16A16, Linear)
            .with_gl(GlFormat::RGBA16_SNORM)
            .with_vulkan(VkFormat::VK_FORMAT_R16G16B16A16_SNORM),
        uncompressed("R16G16B16A16_UINT", R16G16B16A16, Linear)
            .with_gl(GlFormat::RGBA16UI)
            .with_vulkan(VkFormat::VK_FORMAT_R16G16B16A16_UINT),
        uncompressed("R16G16B16A16_SINT", R16G16B16A16, Linear)
            .with_gl(GlFormat::RGBA16I)
            .with_vulkan(VkFormat::VK_FORMAT_R16G16B16A16_SINT),
        uncompressed("R16G16B16A16_SFLOAT", R16G16B16A16, Linear)
            .with_gl(GlFormat::RGBA16F)
            .with_vulkan(VkFormat::VK_FORMAT_R16G16B16A16_SFLOAT),
        uncompressed("R32_UINT", R32, Linear)
            .with_gl(GlFormat::R32UI)
            .with_vulkan(VkFormat::VK_FORMAT_R32_UINT),
        uncompressed("R32_SINT", R32, Linear)
            .with_gl(GlFormat::R32I)
            .with_vulkan(VkFormat::VK_FORMAT_R32_SINT),
        uncompressed("R32_SFLOAT", R32, Linear)
            .with_gl(GlFormat::R32F)
            .with_vulkan(VkFormat::VK_FORMAT_R32_SFLOAT),
        uncompressed("R32G32_UINT", R32G32, Linear)
            .with_gl(GlFormat::RG32UI)
            .with_vulkan(VkFormat::VK_FORMAT_R32G32_UINT),
        uncompressed("R32G32_SINT", R32G32, Linear)
            .with_gl(GlFormat::RG32I)
            .with_vulkan(VkFormat::VK_FORMAT_R32G32_SINT),
        uncompressed("R32G32_SFLOAT", R32G32, Linear)
            .with_gl(GlFormat::RG32F)
            .with_vulkan(VkFormat::VK_FORMAT_R32G32_SFLOAT),
        uncompressed("R32G32B32_UINT", R32G32B32, Linear)
            .with_gl(GlFormat::RGB32UI)
            .with_vulkan(VkFormat::VK_FORMAT_R32G32B32_UINT),
        uncompressed("R32G32B32_SINT", R32G32B32, Linear)
            .with_gl(GlFormat::RGB32I)
            .with_vulkan(VkFormat::VK_FORMAT_R32G32B32_SINT),
        uncompressed("R32G32B32_SFLOAT", R32G32B32, Linear)
            .with_gl(GlFormat::RGB32F)
            .with_vulkan(VkFormat::VK_FORMAT_R32G32B32_SFLOAT),
        uncompressed("R32G32B32A32_UINT", R32G32B32A32, Linear)
            .with_gl(GlFormat::RGBA32UI)
            .with_vulkan(VkFormat::VK_FORMAT_R32G32B32A32_UINT),
        uncompressed("R32G32B32A32_SINT", R32G32B32A32, Linear)
            .with_gl(GlFormat::RGBA32I)
            .with_vulkan(VkFormat::VK_FORMAT_R32G32B32A32_SINT),
        uncompressed("R32G32B32A32_SFLOAT", R32G32B32A32, Linear)
            .with_gl(GlFormat::RGBA32F)
            .with_vulkan(VkFormat::VK_FORMAT_R32G32B32A32_SFLOAT),
        uncompressed("B10G11R11_UFLOAT", B10G11R11, Linear)
            .with_gl(GlFormat::R11F_G11F_B10F)
            .with_vulkan(VkFormat::VK_FORMAT_B10G11R11_UFLOAT_PACK32),
        uncompressed("E5B9G9R9_UFLOAT", E5B9G9R9, Linear)
            .with_gl(GlFormat::RGB9_E5)
            .with_vulkan(VkFormat::VK_FORMAT_E5B9G9R9_UFLOAT_PACK32),
    ]
}
fn uncompressed(
    name: &'static str,
    base_format: BaseFormat,
    colour_space: ColourSpace,
) -> PixelFormat {
    let format = PixelFormat {
        name,
        base_format,
        colour_space,
        block_dim: None,
        vk_format: None,
        gl_format: None,
        four_cc: None,
        dxt10_format: None,
    };
    format
}
