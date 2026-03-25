#[repr(C)]
#[derive(Debug, Clone, Copy,)]
pub struct FLinearColor {
    pub R: f32,
    pub G: f32,
    pub B: f32,
    pub A: f32,
}

impl FLinearColor {
    pub const PINK: Self = Self::from_rgba(1.0, 0.4, 0.7, 1.0,);
    pub const LIGHT_BLUE: Self = Self::from_rgba(0.6, 0.8, 1.0, 1.0,);
    pub const RED: Self = Self::from_rgba(1.0, 0.0, 0.0, 1.0,);
    pub const LIGHT_RED: Self = Self::from_rgba(1.0, 0.4, 0.4, 1.0,);
    pub const GREEN: Self = Self::from_rgba(0.0, 1.0, 0.0, 1.0,);
    pub const BLUE: Self = Self::from_rgba(0.0, 0.0, 1.0, 1.0,);
    pub const YELLOW: Self = Self::from_rgba(1.0, 1.0, 0.0, 1.0,);
    pub const PURPLE: Self = Self::from_rgba(1.0, 0.0, 1.0, 1.0,);
    pub const CYAN: Self = Self::from_rgba(0.0, 1.0, 1.0, 1.0,);
    pub const ORANGE: Self = Self::from_rgba(1.0, 0.5, 0.0, 1.0,);
    pub const WHITE: Self = Self::from_rgba(1.0, 1.0, 1.0, 1.0,);

    #[inline]
    pub const fn zero() -> Self {
        unsafe { ::core::mem::zeroed() }
    }

    #[inline]
    pub const fn from_rgba(r: f32, g: f32, b: f32, a: f32,) -> Self {
        Self { R: r, G: g, B: b, A: a, }
    }
}
