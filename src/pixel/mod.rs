pub mod gl;
use gl::GlFormat;

pub mod dxt10;
use dxt10::Dxt10Format;

pub mod vulkan;
use vulkan::VkFormat;

mod db;

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub enum CompLayout {
    A8,
    R8,
    R11,
    R16,
    R32,
    R64,
    R8G8,
    R11G11,
    R16G16,
    R32G32,
    R64G64,
    R5G6B5,
    R8G8B8,
    R16G16B16,
    R32G32B32,
    R64G64B64,
    R4G4B4A4,
    R5G5B5A1,
    R8G8B8A8,
    R16G16B16A16,
    R32G32B32A32,
    R64G64B64A64,
    B8G8R8,
    B10G11R11,
    B4G4R4A4,
    B5G6R5,
    B5G5R5A1,
    B8G8R8A8,
    A1R5G5B5,
    A2R10G10B10,
    A2B10G10R10,
    A8B8G8R8,
    E5B9G9R9,
    S8,
    D16,
    D32,
    D16S8,
    D24S8,
    D32S8,
    X8D24,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub enum CompContent {
    UNORM,
    SNORM,
    SFLOAT,
    UFLOAT,
    UINT,
    SINT,
    SRGB,
    SPECIAL,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Dimensions(u32, u32, u32);

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct PixelFormat {
    pub tag: &'static str,
    pub comp_layout: CompLayout,
    pub comp_content: CompContent,
    pub block_dim: Option<Dimensions>,
    pub vk_format: Option<VkFormat>,
    pub gl_format: Option<GlFormat>,
    pub four_cc: Option<u32>,
    pub dxt10_format: Option<Dxt10Format>,
}

impl std::fmt::Display for PixelFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "PixFormat({} - {:?}, {:?})",
            self.tag, self.comp_layout, self.comp_content
        )
    }
}

impl PixelFormat {
    pub fn for_gl_format(gl_format: GlFormat) -> Vec<&'static PixelFormat> {
        db::PIXEL_FORMATS
            .iter()
            .filter(|x| x.gl_format.map_or(false, |f| f == gl_format))
            .collect()
    }

    pub fn with_gl(&self, gl_format: GlFormat) -> PixelFormat {
        PixelFormat {
            gl_format: Some(gl_format),
            ..*self
        }
    }
    pub fn with_four_cc(&self, bytes: &[u8; 4]) -> PixelFormat {
        let four_cc = Some(
            (bytes[0] as u32)
                | (bytes[1] as u32) << 8
                | (bytes[2] as u32) << 16
                | (bytes[3] as u32) << 24,
        );
        PixelFormat { four_cc, ..*self }
    }
    pub fn with_dxt10(&self, dxt10_format: Dxt10Format) -> PixelFormat {
        PixelFormat {
            dxt10_format: Some(dxt10_format),
            ..*self
        }
    }
    pub fn with_vulkan(&self, vk_format: VkFormat) -> PixelFormat {
        PixelFormat {
            vk_format: Some(vk_format),
            ..*self
        }
    }
}
