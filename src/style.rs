pub struct Style {
    pub divider_color: iced_core::Color,
    pub divider_width: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Idle,
    Hovering,
    Dragging,
}

pub trait Catalog {
    type Class<'a>;

    fn default<'a>() -> Self::Class<'a>;
    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style;
}

pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

impl Catalog for iced_core::Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default_style)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

fn default_style(theme: &iced_core::Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    match status {
        Status::Idle => Style {
            divider_color: palette.background.strong.color,
            divider_width: 1.0,
        },
        Status::Hovering => Style {
            divider_color: palette.primary.base.color,
            divider_width: 1.0,
        },
        Status::Dragging => Style {
            divider_color: palette.primary.strong.color,
            divider_width: 1.0,
        },
    }
}
