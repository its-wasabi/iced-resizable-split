pub struct Style {
    pub divider_color: iced_core::Color,
    pub divider_width: f32,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            divider_color: iced_core::Color::WHITE,
            divider_width: 1.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Idle,
    Dragging,
}

pub(super) type StyleFn<'a, Theme> =
    Box<dyn Fn(&Theme, super::style::State) -> super::style::Style + 'a>;
