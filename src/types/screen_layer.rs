#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum ScreenLayer {
    Background,
    Objects,
    Effects,
    Ui,
}

impl From<ScreenLayer> for f32 {
    fn from(value: ScreenLayer) -> Self { value as u32 as f32 }
}
