use crate::pixel::dxt10::Dxt10Format;
use crate::pixel::gl::GlFormat;
use crate::pixel::vulkan::VkFormat;

use crate::pixel::CompContent::*;
use crate::pixel::CompLayout::*;
use crate::pixel::{CompContent, CompLayout, PixelFormat};

pub fn get_formats() -> Vec<PixelFormat> {
    vec![vulkan_only_formats(), gl_supported_formats()]
        .iter()
        .cloned()
        .flatten()
        .collect()
}

fn uncompressed(comp_layout: CompLayout, comp_content: CompContent) -> PixelFormat {
    PixelFormat {
        tag: "Uncompressed",
        comp_layout,
        comp_content,
        block_dim: None,
        vk_format: None,
        gl_format: None,
        four_cc: None,
        dxt10_format: None,
    }
}

fn vulkan_only_formats() -> Vec<PixelFormat> {
    vec![
        uncompressed(A2R10G10B10, SNORM).with_vulkan(VkFormat::VK_FORMAT_A2R10G10B10_SNORM_PACK32),
        uncompressed(A2R10G10B10, SINT).with_vulkan(VkFormat::VK_FORMAT_A2R10G10B10_SINT_PACK32),
        uncompressed(A2B10G10R10, SNORM).with_vulkan(VkFormat::VK_FORMAT_A2B10G10R10_SNORM_PACK32),
        uncompressed(A2B10G10R10, SINT).with_vulkan(VkFormat::VK_FORMAT_A2B10G10R10_SINT_PACK32),
        uncompressed(R64, UNORM).with_vulkan(VkFormat::VK_FORMAT_R4G4_UNORM_PACK8),
        uncompressed(B4G4R4A4, UNORM).with_vulkan(VkFormat::VK_FORMAT_B4G4R4A4_UNORM_PACK16),
        uncompressed(B5G6R5, UNORM).with_vulkan(VkFormat::VK_FORMAT_B5G6R5_UNORM_PACK16),
        uncompressed(B5G5R5A1, UNORM).with_vulkan(VkFormat::VK_FORMAT_B5G5R5A1_UNORM_PACK16),
        uncompressed(A1R5G5B5, UNORM).with_vulkan(VkFormat::VK_FORMAT_A1R5G5B5_UNORM_PACK16),
        uncompressed(R64, UINT).with_vulkan(VkFormat::VK_FORMAT_R64_UINT),
        uncompressed(R64, SINT).with_vulkan(VkFormat::VK_FORMAT_R64_SINT),
        uncompressed(R64, SFLOAT).with_vulkan(VkFormat::VK_FORMAT_R64_SFLOAT),
        uncompressed(R64G64, UINT).with_vulkan(VkFormat::VK_FORMAT_R64G64_UINT),
        uncompressed(R64G64, SINT).with_vulkan(VkFormat::VK_FORMAT_R64G64_SINT),
        uncompressed(R64G64, SFLOAT).with_vulkan(VkFormat::VK_FORMAT_R64G64_SFLOAT),
        uncompressed(R64G64B64, UINT).with_vulkan(VkFormat::VK_FORMAT_R64G64B64_UINT),
        uncompressed(R64G64B64, SINT).with_vulkan(VkFormat::VK_FORMAT_R64G64B64_SINT),
        uncompressed(R64G64B64, SFLOAT).with_vulkan(VkFormat::VK_FORMAT_R64G64B64_SFLOAT),
        uncompressed(R64G64B64A64, UINT).with_vulkan(VkFormat::VK_FORMAT_R64G64B64A64_UINT),
        uncompressed(R64G64B64A64, SINT).with_vulkan(VkFormat::VK_FORMAT_R64G64B64A64_SINT),
        uncompressed(R64G64B64A64, SFLOAT).with_vulkan(VkFormat::VK_FORMAT_R64G64B64A64_SFLOAT),
        uncompressed(D16S8, SPECIAL).with_vulkan(VkFormat::VK_FORMAT_D16_UNORM_S8_UINT),
    ]
}

fn gl_supported_formats() -> Vec<PixelFormat> {
    vec![
        // OpenGL ES 2
        uncompressed(A8, UNORM).with_dxt10(Dxt10Format::DXGI_FORMAT_A8_UNORM),
        // OpenGL ES 3
        uncompressed(R8, UNORM).
            with_gl(GlFormat::SR8)
            .with_vulkan(VkFormat::VK_FORMAT_R8_SRGB),
        uncompressed(R8G8, UNORM)
            .with_gl(GlFormat::SRG8)
            .with_vulkan(VkFormat::VK_FORMAT_R8G8_SRGB),
        // OpenGL 4
        uncompressed(R4G4B4A4, UNORM)
            .with_gl(GlFormat::RGBA4)
            .with_vulkan(VkFormat::VK_FORMAT_R4G4B4A4_UNORM_PACK16),
        uncompressed(R5G6B5, UNORM)
            .with_gl(GlFormat::RGB565)
            .with_vulkan(VkFormat::VK_FORMAT_R5G6B5_UNORM_PACK16),
        uncompressed(R5G5B5A1, UNORM)
            .with_gl(GlFormat::RGB5_A1)
            .with_vulkan(VkFormat::VK_FORMAT_R5G5B5A1_UNORM_PACK16),
        uncompressed(R8, UNORM)
            .with_gl(GlFormat::R8)
            .with_vulkan(VkFormat::VK_FORMAT_R8_UNORM),
        uncompressed(R8, SNORM)
            .with_gl(GlFormat::R8_SNORM)
            .with_vulkan(VkFormat::VK_FORMAT_R8_SNORM),
        uncompressed(R8, UINT)
            .with_gl(GlFormat::R8UI)
            .with_vulkan(VkFormat::VK_FORMAT_R8_UINT),
        uncompressed(R8, SINT)
            .with_gl(GlFormat::R8I)
            .with_vulkan(VkFormat::VK_FORMAT_R8_SINT),
        uncompressed(R8G8, UNORM)
            .with_gl(GlFormat::RG8)
            .with_vulkan(VkFormat::VK_FORMAT_R8G8_UNORM),
        uncompressed(R8G8, SNORM)
            .with_gl(GlFormat::RG8_SNORM)
            .with_vulkan(VkFormat::VK_FORMAT_R8G8_SNORM),
        uncompressed(R8G8, UINT)
            .with_gl(GlFormat::RG8UI)
            .with_vulkan(VkFormat::VK_FORMAT_R8G8_UINT),
        uncompressed(R8G8, SINT)
            .with_gl(GlFormat::RG8I)
            .with_vulkan(VkFormat::VK_FORMAT_R8G8_SINT),
        uncompressed(R8G8B8, UNORM)
            .with_gl(GlFormat::RGB8)
            .with_vulkan(VkFormat::VK_FORMAT_R8G8B8_UNORM),
        uncompressed(R8G8B8, SNORM)
            .with_gl(GlFormat::RGB8_SNORM)
            .with_vulkan(VkFormat::VK_FORMAT_R8G8B8_SNORM),
        uncompressed(R8G8B8, UINT)
            .with_gl(GlFormat::RGB8UI)
            .with_vulkan(VkFormat::VK_FORMAT_R8G8B8_UINT),
        uncompressed(R8G8B8, SINT)
            .with_gl(GlFormat::RGB8I)
            .with_vulkan(VkFormat::VK_FORMAT_R8G8B8_SINT),
        uncompressed(R8G8B8, SRGB)
            .with_gl(GlFormat::RGB8)
            .with_vulkan(VkFormat::VK_FORMAT_R8G8B8_SRGB),
        uncompressed(B8G8R8, UNORM)
            .with_gl(GlFormat::RGB8)
            .with_vulkan(VkFormat::VK_FORMAT_B8G8R8_UNORM),
        uncompressed(B8G8R8, SNORM)
            .with_gl(GlFormat::RGB8_SNORM)
            .with_vulkan(VkFormat::VK_FORMAT_B8G8R8_SNORM),
        uncompressed(B8G8R8, UINT)
            .with_gl(GlFormat::RGB8UI)
            .with_vulkan(VkFormat::VK_FORMAT_B8G8R8_UINT),
        uncompressed(B8G8R8, SINT)
            .with_gl(GlFormat::RGB8I)
            .with_vulkan(VkFormat::VK_FORMAT_B8G8R8_SINT),
        uncompressed(B8G8R8, SRGB)
            .with_gl(GlFormat::RGB8)
            .with_vulkan(VkFormat::VK_FORMAT_B8G8R8_SRGB),
        uncompressed(R8G8B8A8, UNORM)
            .with_gl(GlFormat::RGBA8)
            .with_vulkan(VkFormat::VK_FORMAT_R8G8B8A8_UNORM),
        uncompressed(R8G8B8A8, SNORM)
            .with_gl(GlFormat::RGBA8_SNORM)
            .with_vulkan(VkFormat::VK_FORMAT_R8G8B8A8_SNORM),
        uncompressed(R8G8B8A8, UINT)
            .with_gl(GlFormat::RGBA8UI)
            .with_vulkan(VkFormat::VK_FORMAT_R8G8B8A8_UINT),
        uncompressed(R8G8B8A8, SINT)
            .with_gl(GlFormat::RGBA8I)
            .with_vulkan(VkFormat::VK_FORMAT_R8G8B8A8_SINT),
        uncompressed(R8G8B8A8, SRGB)
            .with_gl(GlFormat::RGBA8)
            .with_vulkan(VkFormat::VK_FORMAT_R8G8B8A8_SRGB),
        uncompressed(B8G8R8A8, UNORM)
            .with_gl(GlFormat::RGBA8)
            .with_vulkan(VkFormat::VK_FORMAT_B8G8R8A8_UNORM),
        uncompressed(B8G8R8A8, SNORM)
            .with_gl(GlFormat::RGBA8_SNORM)
            .with_vulkan(VkFormat::VK_FORMAT_B8G8R8A8_SNORM),
        uncompressed(B8G8R8A8, UINT)
            .with_gl(GlFormat::RGBA8UI)
            .with_vulkan(VkFormat::VK_FORMAT_B8G8R8A8_UINT),
        uncompressed(B8G8R8A8, SINT)
            .with_gl(GlFormat::RGBA8I)
            .with_vulkan(VkFormat::VK_FORMAT_B8G8R8A8_SINT),
        uncompressed(B8G8R8A8, SRGB)
            .with_gl(GlFormat::RGBA8)
            .with_vulkan(VkFormat::VK_FORMAT_B8G8R8A8_SRGB),
        uncompressed(A8B8G8R8, UNORM)
            .with_gl(GlFormat::RGBA8)
            .with_vulkan(VkFormat::VK_FORMAT_A8B8G8R8_UNORM_PACK32),
        uncompressed(A8B8G8R8, SNORM)
            .with_gl(GlFormat::RGBA8_SNORM)
            .with_vulkan(VkFormat::VK_FORMAT_A8B8G8R8_SNORM_PACK32),
        uncompressed(A8B8G8R8, UINT)
            .with_gl(GlFormat::RGBA8UI)
            .with_vulkan(VkFormat::VK_FORMAT_A8B8G8R8_UINT_PACK32),
        uncompressed(A8B8G8R8, SINT)
            .with_gl(GlFormat::RGBA8I)
            .with_vulkan(VkFormat::VK_FORMAT_A8B8G8R8_SINT_PACK32),
        uncompressed(A8B8G8R8, SRGB)
            .with_gl(GlFormat::RGBA8)
            .with_vulkan(VkFormat::VK_FORMAT_A8B8G8R8_SRGB_PACK32),
        uncompressed(R16, UNORM)
            .with_gl(GlFormat::R16)
            .with_vulkan(VkFormat::VK_FORMAT_R16_UNORM),
        uncompressed(R16, SNORM)
            .with_gl(GlFormat::R16_SNORM)
            .with_vulkan(VkFormat::VK_FORMAT_R16_SNORM),
        uncompressed(R16, UINT)
            .with_gl(GlFormat::R16UI)
            .with_vulkan(VkFormat::VK_FORMAT_R16_UINT),
        uncompressed(R16, SINT)
            .with_gl(GlFormat::R16I)
            .with_vulkan(VkFormat::VK_FORMAT_R16_SINT),
        uncompressed(R16, SFLOAT)
            .with_gl(GlFormat::R16F)
            .with_vulkan(VkFormat::VK_FORMAT_R16_SFLOAT),
        uncompressed(R16G16, UNORM)
            .with_gl(GlFormat::RG16)
            .with_vulkan(VkFormat::VK_FORMAT_R16G16_UNORM),
        uncompressed(R16G16, SNORM)
            .with_gl(GlFormat::RG16_SNORM)
            .with_vulkan(VkFormat::VK_FORMAT_R16G16_SNORM),
        uncompressed(R16G16, UINT)
            .with_gl(GlFormat::RG16UI)
            .with_vulkan(VkFormat::VK_FORMAT_R16G16_UINT),
        uncompressed(R16G16, SINT)
            .with_gl(GlFormat::RG16I)
            .with_vulkan(VkFormat::VK_FORMAT_R16G16_SINT),
        uncompressed(R16G16, SFLOAT)
            .with_gl(GlFormat::RG16F)
            .with_vulkan(VkFormat::VK_FORMAT_R16G16_SFLOAT),
        uncompressed(R16G16B16, UNORM)
            .with_gl(GlFormat::RGB16)
            .with_vulkan(VkFormat::VK_FORMAT_R16G16B16_UNORM),
        uncompressed(R16G16B16, SNORM)
            .with_gl(GlFormat::RGB16_SNORM)
            .with_vulkan(VkFormat::VK_FORMAT_R16G16B16_SNORM),
        uncompressed(R16G16B16, UINT)
            .with_gl(GlFormat::RGB16UI)
            .with_vulkan(VkFormat::VK_FORMAT_R16G16B16_UINT),
        uncompressed(R16G16B16, SINT)
            .with_gl(GlFormat::RGB16I)
            .with_vulkan(VkFormat::VK_FORMAT_R16G16B16_SINT),
        uncompressed(R16G16B16, SFLOAT)
            .with_gl(GlFormat::RGB16F)
            .with_vulkan(VkFormat::VK_FORMAT_R16G16B16_SFLOAT),
        uncompressed(R16G16B16A16, UNORM)
            .with_gl(GlFormat::RGBA16)
            .with_vulkan(VkFormat::VK_FORMAT_R16G16B16A16_UNORM),
        uncompressed(R16G16B16A16, SNORM)
            .with_gl(GlFormat::RGBA16_SNORM)
            .with_vulkan(VkFormat::VK_FORMAT_R16G16B16A16_SNORM),
        uncompressed(R16G16B16A16, UINT)
            .with_gl(GlFormat::RGBA16UI)
            .with_vulkan(VkFormat::VK_FORMAT_R16G16B16A16_UINT),
        uncompressed(R16G16B16A16, SINT)
            .with_gl(GlFormat::RGBA16I)
            .with_vulkan(VkFormat::VK_FORMAT_R16G16B16A16_SINT),
        uncompressed(R16G16B16A16, SFLOAT)
            .with_gl(GlFormat::RGBA16F)
            .with_vulkan(VkFormat::VK_FORMAT_R16G16B16A16_SFLOAT),
        uncompressed(R32, UINT)
            .with_gl(GlFormat::R32UI)
            .with_vulkan(VkFormat::VK_FORMAT_R32_UINT),
        uncompressed(R32, SINT)
            .with_gl(GlFormat::R32I)
            .with_vulkan(VkFormat::VK_FORMAT_R32_SINT),
        uncompressed(R32, SFLOAT)
            .with_gl(GlFormat::R32F)
            .with_vulkan(VkFormat::VK_FORMAT_R32_SFLOAT),
        uncompressed(R32G32, UINT)
            .with_gl(GlFormat::RG32UI)
            .with_vulkan(VkFormat::VK_FORMAT_R32G32_UINT),
        uncompressed(R32G32, SINT)
            .with_gl(GlFormat::RG32I)
            .with_vulkan(VkFormat::VK_FORMAT_R32G32_SINT),
        uncompressed(R32G32, SFLOAT)
            .with_gl(GlFormat::RG32F)
            .with_vulkan(VkFormat::VK_FORMAT_R32G32_SFLOAT),
        uncompressed(R32G32B32, UINT)
            .with_gl(GlFormat::RGB32UI)
            .with_vulkan(VkFormat::VK_FORMAT_R32G32B32_UINT),
        uncompressed(R32G32B32, SINT)
            .with_gl(GlFormat::RGB32I)
            .with_vulkan(VkFormat::VK_FORMAT_R32G32B32_SINT),
        uncompressed(R32G32B32, SFLOAT)
            .with_gl(GlFormat::RGB32F)
            .with_vulkan(VkFormat::VK_FORMAT_R32G32B32_SFLOAT),
        uncompressed(R32G32B32A32, UINT)
            .with_gl(GlFormat::RGBA32UI)
            .with_vulkan(VkFormat::VK_FORMAT_R32G32B32A32_UINT),
        uncompressed(R32G32B32A32, SINT)
            .with_gl(GlFormat::RGBA32I)
            .with_vulkan(VkFormat::VK_FORMAT_R32G32B32A32_SINT),
        uncompressed(R32G32B32A32, SFLOAT)
            .with_gl(GlFormat::RGBA32F)
            .with_vulkan(VkFormat::VK_FORMAT_R32G32B32A32_SFLOAT),
        uncompressed(B10G11R11, UFLOAT)
            .with_gl(GlFormat::R11F_G11F_B10F)
            .with_vulkan(VkFormat::VK_FORMAT_B10G11R11_UFLOAT_PACK32),
        uncompressed(E5B9G9R9, UFLOAT)
            .with_gl(GlFormat::RGB9_E5)
            .with_vulkan(VkFormat::VK_FORMAT_E5B9G9R9_UFLOAT_PACK32),
        uncompressed(A2R10G10B10, UNORM)
            .with_gl(GlFormat::RGB10_A2)
            .with_vulkan(VkFormat::VK_FORMAT_A2R10G10B10_UNORM_PACK32),
        uncompressed(A2R10G10B10, UINT)
            .with_gl(GlFormat::RGB10_A2UI)
            .with_vulkan(VkFormat::VK_FORMAT_A2R10G10B10_UINT_PACK32),
        uncompressed(A2B10G10R10, UNORM)
            .with_gl(GlFormat::RGB10_A2)
            .with_vulkan(VkFormat::VK_FORMAT_A2B10G10R10_UNORM_PACK32),
        uncompressed(A2B10G10R10, UINT)
            .with_gl(GlFormat::RGB10_A2UI)
            .with_vulkan(VkFormat::VK_FORMAT_A2B10G10R10_UINT_PACK32),
        uncompressed(S8, UINT)
            .with_gl(GlFormat::STENCIL_INDEX8)
            .with_vulkan(VkFormat::VK_FORMAT_S8_UINT),
        uncompressed(D16, UNORM)
            .with_gl(GlFormat::DEPTH_COMPONENT16)
            .with_vulkan(VkFormat::VK_FORMAT_D16_UNORM),
        uncompressed(X8D24, UNORM)
            .with_gl(GlFormat::DEPTH_COMPONENT24)
            .with_vulkan(VkFormat::VK_FORMAT_X8_D24_UNORM_PACK32),
        uncompressed(D24S8, SPECIAL)
            .with_gl(GlFormat::DEPTH24_STENCIL8)
            .with_vulkan(VkFormat::VK_FORMAT_D24_UNORM_S8_UINT),
        uncompressed(D32, SFLOAT)
            .with_gl(GlFormat::DEPTH32F_STENCIL8)
            .with_vulkan(VkFormat::VK_FORMAT_D32_SFLOAT),
        uncompressed(D32S8, SPECIAL)
            .with_gl(GlFormat::DEPTH32F_STENCIL8)
            .with_vulkan(VkFormat::VK_FORMAT_D32_SFLOAT_S8_UINT),
    ]
}

// USCALED and SSCALED are not currently supported
#[allow(dead_code)]
fn vulkan_scaled() -> Vec<PixelFormat> {
    vec![
        uncompressed(R8, SPECIAL).with_vulkan(VkFormat::VK_FORMAT_R8_USCALED),
        uncompressed(R8, SPECIAL).with_vulkan(VkFormat::VK_FORMAT_R8_SSCALED),
        uncompressed(R8G8, SPECIAL).with_vulkan(VkFormat::VK_FORMAT_R8G8_USCALED),
        uncompressed(R8G8, SPECIAL).with_vulkan(VkFormat::VK_FORMAT_R8G8_SSCALED),
        uncompressed(R8G8B8, SPECIAL).with_vulkan(VkFormat::VK_FORMAT_R8G8B8_USCALED),
        uncompressed(R8G8B8, SPECIAL).with_vulkan(VkFormat::VK_FORMAT_R8G8B8_SSCALED),
        uncompressed(B8G8R8, SPECIAL).with_vulkan(VkFormat::VK_FORMAT_B8G8R8_USCALED),
        uncompressed(B8G8R8, SPECIAL).with_vulkan(VkFormat::VK_FORMAT_B8G8R8_SSCALED),
        uncompressed(R8G8B8A8, SPECIAL).with_vulkan(VkFormat::VK_FORMAT_R8G8B8A8_USCALED),
        uncompressed(R8G8B8A8, SPECIAL).with_vulkan(VkFormat::VK_FORMAT_R8G8B8A8_SSCALED),
        uncompressed(B8G8R8A8, SPECIAL).with_vulkan(VkFormat::VK_FORMAT_B8G8R8A8_USCALED),
        uncompressed(B8G8R8A8, SPECIAL).with_vulkan(VkFormat::VK_FORMAT_B8G8R8A8_SSCALED),
        uncompressed(A8B8G8R8, SPECIAL).with_vulkan(VkFormat::VK_FORMAT_A8B8G8R8_USCALED_PACK32),
        uncompressed(A8B8G8R8, SPECIAL).with_vulkan(VkFormat::VK_FORMAT_A8B8G8R8_SSCALED_PACK32),
        uncompressed(R16, SPECIAL).with_vulkan(VkFormat::VK_FORMAT_R16_USCALED),
        uncompressed(R16, SPECIAL).with_vulkan(VkFormat::VK_FORMAT_R16_SSCALED),
        uncompressed(R16G16, SPECIAL).with_vulkan(VkFormat::VK_FORMAT_R16G16_USCALED),
        uncompressed(R16G16, SPECIAL).with_vulkan(VkFormat::VK_FORMAT_R16G16_SSCALED),
        uncompressed(R16G16B16, SPECIAL).with_vulkan(VkFormat::VK_FORMAT_R16G16B16_USCALED),
        uncompressed(R16G16B16, SPECIAL).with_vulkan(VkFormat::VK_FORMAT_R16G16B16_SSCALED),
        uncompressed(R16G16B16A16, SPECIAL).with_vulkan(VkFormat::VK_FORMAT_R16G16B16A16_USCALED),
        uncompressed(R16G16B16A16, SPECIAL).with_vulkan(VkFormat::VK_FORMAT_R16G16B16A16_SSCALED),
        uncompressed(A2R10G10B10, SPECIAL)
            .with_vulkan(VkFormat::VK_FORMAT_A2R10G10B10_USCALED_PACK32),
        uncompressed(A2R10G10B10, SPECIAL)
            .with_vulkan(VkFormat::VK_FORMAT_A2R10G10B10_SSCALED_PACK32),
        uncompressed(A2R10G10B10, SPECIAL)
            .with_vulkan(VkFormat::VK_FORMAT_A2B10G10R10_USCALED_PACK32),
        uncompressed(A2R10G10B10, SPECIAL)
            .with_vulkan(VkFormat::VK_FORMAT_A2B10G10R10_SSCALED_PACK32),
    ]
}
