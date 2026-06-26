#![doc = include_str!("../README.md")]

mod split;
pub mod state;
pub mod style;

pub use state::State;
pub use style::Status;
pub use style::Style;

const DEFAULT_DRAG_AREA_SIZE: f32 = 12.0;

#[allow(unused)]
pub fn split_horizontal<'a, Message, Theme, Renderer>(
    top: impl Into<iced_core::Element<'a, Message, Theme, Renderer>>,
    bottom: impl Into<iced_core::Element<'a, Message, Theme, Renderer>>,
    state: state::State,
    message: impl Fn(state::State) -> Message + 'a,
) -> split::Split<'a, Message, Renderer, Theme>
where
    Theme: style::Catalog,
{
    split::Split::new(split::Axis::Horizontal, top, bottom, state, message)
}

#[allow(unused)]
pub fn split_vertical<'a, Message, Theme, Renderer>(
    left: impl Into<iced_core::Element<'a, Message, Theme, Renderer>>,
    right: impl Into<iced_core::Element<'a, Message, Theme, Renderer>>,
    state: state::State,
    message: impl Fn(state::State) -> Message + 'a,
) -> split::Split<'a, Message, Renderer, Theme>
where
    Theme: style::Catalog,
{
    split::Split::new(split::Axis::Vertical, left, right, state, message)
}
