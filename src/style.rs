pub struct Style {
    pub divider_color: iced_core::Color,
    pub divider_width: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Idle,
    Hovering,
    Dragging,
}

pub(crate) type StyleFn<'a> =
    Box<dyn Fn(&iced_core::Theme, super::style::State) -> super::style::Style + 'a>;
