# Example usage
```rust
struct App {
    split_state: iced_resizable_split::State,
}

enum AppMessage {
    SplitDragged(iced_resizable_split::State),
}

impl App {
    const fn new() -> Self {
        Self {
            split_state: iced_resizable_split::State::new(0.5, 0.1, 0.9),
        }
    }

    const fn update(&mut self, msg: AppMessage) {
        match msg {
            AppMessage::SplitDragged(state) => self.split_state.update(state),
        }
    }

    fn view(&self) -> iced::Element<'_, AppMessage> {
        iced_resizable_split::split_vertical(
            iced::widget::text("TOP split"),
            iced::widget::text("BOTTOM split"),
            self.split_state,
            AppMessage::SplitDragged,
        )
        .style(|_theme, state| iced_resizable_split::Style {
            divider_width: 1.0,
            divider_color: if state == iced_resizable_split::style::State::Dragging {
                iced::Color::WHITE
            } else if state == iced_resizable_split::style::State::Hovering {
                iced::Color::WHITE.scale_alpha(0.5)
            } else {
                iced::Color::BLACK
            },
        })
        .into()
    }
}

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title("Split")
        .theme(iced::theme::Theme::Dark)
        .run()
}
```
