#![doc = include_str!("../README.md")]

mod split;
pub mod state;
pub mod style;

pub use state::State;
pub use style::Style;

const DEFAULT_DRAG_AREA_SIZE: f32 = 12.0;

#[allow(unused)]
pub fn split_horizontal<'a, Message, Renderer>(
    top: impl Into<iced_core::Element<'a, Message, iced_core::Theme, Renderer>>,
    bottom: impl Into<iced_core::Element<'a, Message, iced_core::Theme, Renderer>>,
    state: state::State,
    message: impl Fn(state::State) -> Message + 'a,
) -> split::Split<'a, Message, Renderer>
where
{
    split::Split::new(split::Axis::Horizontal, top, bottom, state, message)
}

#[allow(unused)]
pub fn split_vertical<'a, Message, Renderer>(
    left: impl Into<iced_core::Element<'a, Message, iced_core::Theme, Renderer>>,
    right: impl Into<iced_core::Element<'a, Message, iced_core::Theme, Renderer>>,
    state: state::State,
    message: impl Fn(state::State) -> Message + 'a,
) -> split::Split<'a, Message, Renderer>
where
{
    split::Split::new(split::Axis::Vertical, left, right, state, message)
}
