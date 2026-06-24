#![doc = include_str!("../README.md")]

mod split_horizontal;
mod split_vertical;
mod state;
pub mod style;

pub use split_horizontal::SplitHorizontal;
pub use split_vertical::SplitVertical;
pub use state::State;
pub use style::Style;

const DEFAULT_DRAG_AREA_SIZE: f32 = 8.0;

#[allow(unused)]
fn split_horizontal<'a, Message, Theme, Renderer>(
    top: impl Into<iced_core::Element<'a, Message, Theme, Renderer>>,
    bottom: impl Into<iced_core::Element<'a, Message, Theme, Renderer>>,
    state: state::State,
    message: impl Fn(state::State) -> Message + 'a,
) -> split_horizontal::SplitHorizontal<'a, Message, Theme, Renderer>
where
    Theme: 'a,
{
    split_horizontal::SplitHorizontal::new(top, bottom, state, message)
}

#[allow(unused)]
fn split_vertical<'a, Message, Theme, Renderer>(
    top: impl Into<iced_core::Element<'a, Message, Theme, Renderer>>,
    bottom: impl Into<iced_core::Element<'a, Message, Theme, Renderer>>,
    state: state::State,
    message: impl Fn(state::State) -> Message + 'a,
) -> split_vertical::SplitVertical<'a, Message, Theme, Renderer>
where
    Theme: 'a,
{
    split_vertical::SplitVertical::new(top, bottom, state, message)
}
