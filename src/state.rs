#[derive(Clone, Copy, PartialEq)]
pub struct State {
    ratio: f32,

    first_split_min: f32,
    second_split_min: f32,
}

impl Default for State {
    fn default() -> Self {
        Self {
            ratio: 0.5,
            first_split_min: 0.05,
            second_split_min: 0.95,
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub(super) struct InternalState {
    pub(super) is_dragging: bool,
    pub(super) is_hovering: bool,
}

impl State {
    #[must_use]
    pub const fn new(initial_ratio: f32, first_split_min: f32, second_split_min: f32) -> Self {
        Self {
            ratio: initial_ratio,
            first_split_min,
            second_split_min,
        }
    }

    pub const fn update(&mut self, new_state: Self) {
        *self = new_state;
    }

    pub(super) const fn copy_with_new_ratio(&self, ratio: f32) -> Self {
        Self { ratio, ..*self }
    }

    pub(super) const fn ratio(&self) -> f32 {
        self.ratio
    }
}
