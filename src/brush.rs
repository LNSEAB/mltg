use crate::*;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SolidColorBrush(ID2D1SolidColorBrush);

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct GradientStop {
    pub position: f32,
    pub color: Rgba,
}

impl GradientStop {
    #[inline]
    pub fn new(position: f32, color: impl Into<Rgba>) -> Self {
        Self {
            position,
            color: color.into(),
        }
    }
}

impl From<GradientStop> for D2D1_GRADIENT_STOP {
    #[inline]
    fn from(src: GradientStop) -> D2D1_GRADIENT_STOP {
        D2D1_GRADIENT_STOP {
            position: src.position,
            color: src.color.into(),
        }
    }
}

impl<T> From<(f32, T)> for GradientStop
where
    T: Into<Rgba>,
{
    #[inline]
    fn from(src: (f32, T)) -> GradientStop {
        GradientStop {
            position: src.0,
            color: src.1.into(),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GradientStopCollection(ID2D1GradientStopCollection);

impl GradientStopCollection {
    pub(crate) fn new<T>(dc: &ID2D1DeviceContext, stops: &[T]) -> windows::Result<Self>
    where
        T: Into<GradientStop> + Clone,
    {
        let stops = stops
            .iter()
            .cloned()
            .map(|stop| stop.into().into())
            .collect::<Vec<_>>();
        let collection = unsafe {
            let mut p = None;
            dc.CreateGradientStopCollection(
                stops.as_ptr(),
                stops.len() as _,
                D2D1_GAMMA_2_2,
                D2D1_EXTEND_MODE_WRAP,
                &mut p,
            )
            .and_some(p)?
        };
        Ok(Self(collection))
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LinearGradientBrush(ID2D1LinearGradientBrush);

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RadialGradientBrush(ID2D1RadialGradientBrush);

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Brush {
    SolidColor(SolidColorBrush),
    LinearGradient(LinearGradientBrush),
    RadialGradient(RadialGradientBrush),
}

impl Brush {
    #[inline]
    pub(crate) fn solid_color(
        dc: &ID2D1DeviceContext,
        color: impl Into<Rgba>,
    ) -> windows::Result<Self> {
        let color: D2D1_COLOR_F = color.into().into();
        let brush = unsafe {
            let mut p = None;
            dc.CreateSolidColorBrush(&color, std::ptr::null(), &mut p)
                .and_some(p)?
        };
        Ok(Self::SolidColor(SolidColorBrush(brush)))
    }

    #[inline]
    pub(crate) fn linear_gradient(
        dc: &ID2D1DeviceContext,
        start: impl Into<Point>,
        end: impl Into<Point>,
        stop_collection: &GradientStopCollection,
    ) -> windows::Result<Self> {
        let start = start.into();
        let end = end.into();
        let brush = unsafe {
            let mut p = None;
            dc.CreateLinearGradientBrush(
                &D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES {
                    startPoint: start.into(),
                    endPoint: end.into(),
                },
                std::ptr::null(),
                &stop_collection.0,
                &mut p,
            )
            .and_some(p)?
        };
        Ok(Self::LinearGradient(LinearGradientBrush(brush)))
    }

    #[inline]
    pub(crate) fn radial_gradient(
        dc: &ID2D1DeviceContext,
        center: impl Into<Point>,
        offset: impl Into<Point>,
        radius: impl Into<Vector>,
        stop_collection: &GradientStopCollection,
    ) -> windows::Result<Self> {
        let center = center.into();
        let offset = offset.into();
        let radius = radius.into();
        let brush = unsafe {
            let mut p = None;
            dc.CreateRadialGradientBrush(
                &D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES {
                    center: center.into(),
                    gradientOriginOffset: offset.into(),
                    radiusX: radius.x,
                    radiusY: radius.y,
                },
                std::ptr::null(),
                &stop_collection.0,
                &mut p,
            )
            .and_some(p)?
        };
        Ok(Self::RadialGradient(RadialGradientBrush(brush)))
    }

    #[inline]
    pub fn handle(&self) -> ID2D1Brush {
        match self {
            Self::SolidColor(b) => b.0.clone().into(),
            Self::LinearGradient(b) => b.0.clone().into(),
            Self::RadialGradient(b) => b.0.clone().into(),
        }
    }
}