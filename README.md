# Resizable split for iced
as name suggests...
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
            AppMessage::SplitDragged(new_state) => self.split_state = new_state,
        }
    }

    fn view(&self) -> iced::Element<'_, AppMessage> {
        iced_resizable_split::SplitHorizontal::new(
            iced::widget::text("TOP split"),
            iced::widget::text("BOTTOM split"),
            self.split_state,
            AppMessage::SplitDragged,
        )
        .into()
    }
}

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view).run()
}
```
