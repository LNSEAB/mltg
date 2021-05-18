mod bindings {
    ::windows::include_bindings!();
}
mod context;
pub mod d2d;
pub mod d3d11;
pub mod d3d12;
mod path;
mod shape;
mod stroke_style;
mod text;
mod utility;

use bindings::Windows::Win32::Graphics::Direct2D::*;
pub use context::*;
pub use d2d::Direct2D;
pub use d3d11::Direct3D11;
pub use d3d12::Direct3D12;
pub use gecl::{circle, point, rect, rgba, size, vector};
pub use path::*;
pub use shape::*;
pub use stroke_style::*;
pub use text::*;
pub use utility::*;

#[derive(Clone, PartialEq, Eq)]
pub struct Brush(ID2D1Brush);

pub trait Target {
    fn bitmap(&self) -> &ID2D1Bitmap1;
}

pub trait Fill {
    fn fill(&self, dc: &ID2D1DeviceContext, brush: &ID2D1Brush);
}

pub trait Stroke {
    fn stroke(&self, dc: &ID2D1DeviceContext, brush: &ID2D1Brush, width: f32, style: Option<ID2D1StrokeStyle>);
}
