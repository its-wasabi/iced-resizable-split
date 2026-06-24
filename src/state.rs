#[derive(Clone, Copy, PartialEq)]
pub struct State {
    ratio: f32,

    first_split_min: f32,
    second_split_min: f32,

    is_dragging: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            ratio: 0.5,
            first_split_min: 0.05,
            second_split_min: 0.95,
            is_dragging: false,
        }
    }
}

impl State {
    #[must_use]
    pub const fn new(initial_ratio: f32, first_split_min: f32, second_split_min: f32) -> Self {
        Self {
            ratio: initial_ratio,
            first_split_min,
            second_split_min,
            is_dragging: false,
        }
    }

    pub(super) const fn ratio(&self) -> f32 {
        self.ratio
    }

    pub(super) const fn set_ratio(&mut self, ratio: f32) {
        self.ratio = ratio.clamp(self.first_split_min, self.second_split_min);
    }

    pub(super) const fn start_drag(&mut self) {
        self.is_dragging = true;
    }

    pub(super) const fn stop_drag(&mut self) {
        self.is_dragging = false;
    }

    pub(super) const fn is_dragging(&self) -> bool {
        self.is_dragging
    }
}
