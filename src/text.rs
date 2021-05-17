use crate::bindings::Windows::Win32::{Graphics::DirectWrite::*, System::SystemServices::*};
use crate::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum FontWeight {
    Thin = DWRITE_FONT_WEIGHT_THIN.0,
    UltraLight = DWRITE_FONT_WEIGHT_ULTRA_LIGHT.0,
    Light = DWRITE_FONT_WEIGHT_LIGHT.0,
    SemiLight = DWRITE_FONT_WEIGHT_SEMI_LIGHT.0,
    Regular = DWRITE_FONT_WEIGHT_REGULAR.0,
    Medium = DWRITE_FONT_WEIGHT_MEDIUM.0,
    SemiBold = DWRITE_FONT_WEIGHT_SEMI_BOLD.0,
    Bold = DWRITE_FONT_WEIGHT_BOLD.0,
    UltraBold = DWRITE_FONT_WEIGHT_ULTRA_BOLD.0,
    Heavy = DWRITE_FONT_WEIGHT_HEAVY.0,
    UltraBlack = DWRITE_FONT_WEIGHT_ULTRA_BLACK.0,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum FontStyle {
    Normal = DWRITE_FONT_STYLE_NORMAL.0,
    Oblique = DWRITE_FONT_STYLE_OBLIQUE.0,
    Italic = DWRITE_FONT_STYLE_ITALIC.0,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum FontStretch {
    Undefined = DWRITE_FONT_STRETCH_UNDEFINED.0,
    UltraCondensed = DWRITE_FONT_STRETCH_ULTRA_CONDENSED.0,
    ExtraCondensed = DWRITE_FONT_STRETCH_EXTRA_CONDENSED.0,
    Condensed = DWRITE_FONT_STRETCH_CONDENSED.0,
    SemiCondensed = DWRITE_FONT_STRETCH_SEMI_CONDENSED.0,
    Medium = DWRITE_FONT_STRETCH_MEDIUM.0,
    SemiExpanded = DWRITE_FONT_STRETCH_SEMI_EXPANDED.0,
    Expanded = DWRITE_FONT_STRETCH_EXPANDED.0,
    ExtraExpanded = DWRITE_FONT_STRETCH_EXTRA_EXPANDED.0,
    UltraExpanded = DWRITE_FONT_STRETCH_ULTRA_EXPANDED.0,
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct FontPoint(pub f32);

impl From<FontPoint> for f32 {
    #[inline]
    fn from(src: FontPoint) -> f32 {
        src.0 * 96.0 / 72.0
    }
}

#[inline]
pub fn font_point(value: f32) -> FontPoint {
    FontPoint(value)
}

#[derive(Debug)]
pub struct TextStyle {
    weight: FontWeight,
    style: FontStyle,
    stretch: FontStretch,
}

impl Default for TextStyle {
    #[inline]
    fn default() -> Self {
        Self {
            weight: FontWeight::Regular,
            style: FontStyle::Normal,
            stretch: FontStretch::Medium,
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct TextFormat(IDWriteTextFormat);

impl TextFormat {
    #[inline]
    pub(crate) fn new(
        factory: &IDWriteFactory,
        font_name: &str,
        size: f32,
        style: &TextStyle,
    ) -> windows::Result<Self> {
        let format = unsafe {
            let mut p = None;
            factory
                .CreateTextFormat(
                    font_name,
                    None,
                    DWRITE_FONT_WEIGHT(style.weight as _),
                    DWRITE_FONT_STYLE(style.style as _),
                    DWRITE_FONT_STRETCH(style.stretch as _),
                    size,
                    "",
                    &mut p,
                )
                .and_some(p)?
        };
        Ok(Self(format))
    }
}

#[derive(Clone)]
pub struct TextLayout {
    layout: IDWriteTextLayout,
    size: Size,
}

impl TextLayout {
    #[inline]
    pub(crate) fn new(
        factory: &IDWriteFactory,
        text: &str,
        format: &TextFormat,
    ) -> windows::Result<Self> {
        let (layout, max_size) = unsafe {
            let text = text.encode_utf16().chain(Some(0)).collect::<Vec<_>>();
            let mut p = None;
            let layout = factory
                .CreateTextLayout(
                    PWSTR(text.as_ptr() as _),
                    text.len() as _,
                    &format.0,
                    std::f32::MAX,
                    std::f32::MAX,
                    &mut p,
                )
                .and_some(p)?;
            let size: Size = {
                let mut metrics = Default::default();
                layout.GetMetrics(&mut metrics).unwrap();
                (metrics.width, metrics.height).into()
            };
            layout.SetMaxWidth(size.width).unwrap();
            layout.SetMaxHeight(size.height).unwrap();
            (layout, size)
        };
        Ok(Self {
            layout,
            size: max_size,
        })
    }

    #[inline]
    pub(crate) fn draw(&self, dc: &ID2D1DeviceContext, brush: &Brush, origin: Point) {
        unsafe {
            let origin: D2D_POINT_2F = origin.into();
            dc.DrawTextLayout(
                origin,
                &self.layout,
                &brush.0,
                D2D1_DRAW_TEXT_OPTIONS_ENABLE_COLOR_FONT | D2D1_DRAW_TEXT_OPTIONS_CLIP,
            );
        }
    }

    #[inline]
    pub fn size(&self) -> Size {
        self.size
    }
}
