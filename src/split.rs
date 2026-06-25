pub struct Split<'a, Message, Theme, Renderer> {
    axis: Axis,
    first: iced_core::Element<'a, Message, Theme, Renderer>,
    second: iced_core::Element<'a, Message, Theme, Renderer>,

    state: super::state::State,
    on_drag: Box<dyn Fn(super::state::State) -> Message + 'a>,

    drag_area_size: f32,
    style: super::style::StyleFn<'a, Theme>,
}

impl<'a, Message, Theme, Renderer> Split<'a, Message, Theme, Renderer>
where
    Theme: 'a,
{
    pub fn new(
        axis: Axis,
        first: impl Into<iced_core::Element<'a, Message, Theme, Renderer>>,
        second: impl Into<iced_core::Element<'a, Message, Theme, Renderer>>,
        state: super::state::State,
        message: impl Fn(super::state::State) -> Message + 'a,
    ) -> Self {
        Self {
            axis,
            first: first.into(),
            second: second.into(),
            state,
            on_drag: Box::new(message),
            drag_area_size: super::DEFAULT_DRAG_AREA_SIZE,
            style: Box::new(|_, _| super::style::Style::default()),
        }
    }

    #[must_use]
    pub fn style(
        mut self,
        style: impl Fn(&Theme, super::style::State) -> super::style::Style + 'a,
    ) -> Self {
        self.style = Box::new(style);
        self
    }

    #[must_use]
    pub const fn drag_area_size(mut self, size: f32) -> Self {
        self.drag_area_size = size;
        self
    }
}

impl<Message, Theme, Renderer> Split<'_, Message, Theme, Renderer> {
    fn split_pos(&self, size: iced_core::Size) -> f32 {
        match self.axis {
            Axis::Vertical => size.width * self.state.ratio(),
            Axis::Horizontal => size.height * self.state.ratio(),
        }
    }

    const fn split_relative_position(&self, bounds: iced_core::Rectangle) -> f32 {
        match self.axis {
            Axis::Vertical => bounds.width.mul_add(self.state.ratio(), bounds.x),
            Axis::Horizontal => bounds.height.mul_add(self.state.ratio(), bounds.y),
        }
    }

    fn create_split_rect(&self, size: f32, bounds: iced_core::Rectangle) -> iced_core::Rectangle {
        let divider_pos = self.split_relative_position(bounds);

        match self.axis {
            Axis::Vertical => iced_core::Rectangle {
                x: divider_pos - (size / 2.0),
                y: bounds.y,
                width: size,
                height: bounds.height,
            },
            Axis::Horizontal => iced_core::Rectangle {
                x: bounds.x,
                y: divider_pos - (size / 2.0),
                width: bounds.width,
                height: size,
            },
        }
    }

    fn limit_nodes_size(
        &self,
        size: iced_core::Size,
        limits: iced_core::layout::Limits,
        split_pos: f32,
    ) -> (iced_core::layout::Limits, iced_core::layout::Limits) {
        match self.axis {
            Axis::Vertical => (
                limits.max_width(split_pos),
                limits.max_width(size.width - split_pos),
            ),
            Axis::Horizontal => (
                limits.max_height(split_pos),
                limits.max_height(size.height - split_pos),
            ),
        }
    }

    const fn second_node_position(&self, split_pos: f32) -> iced_core::Point {
        match self.axis {
            Axis::Vertical => iced_core::Point::new(split_pos, 0.0),
            Axis::Horizontal => iced_core::Point::new(0.0, split_pos),
        }
    }

    fn relative_position(&self, mouse_pos: iced_core::Point, bounds: iced_core::Rectangle) -> f32 {
        match self.axis {
            Axis::Vertical => mouse_pos.x - bounds.x,
            Axis::Horizontal => mouse_pos.y - bounds.y,
        }
    }

    fn ratio(&self, mouse_pos: iced_core::Point, bounds: iced_core::Rectangle) -> f32 {
        let relative_mouse_position = self.relative_position(mouse_pos, bounds);
        match self.axis {
            Axis::Vertical => (relative_mouse_position / bounds.width).clamp(0.0, 1.0),
            Axis::Horizontal => (relative_mouse_position / bounds.height).clamp(0.0, 1.0),
        }
    }
}

impl<Message, Theme, Renderer> iced_core::Widget<Message, Theme, Renderer>
    for Split<'_, Message, Theme, Renderer>
where
    Renderer: iced_core::renderer::Renderer,
{
    fn tag(&self) -> iced_core::widget::tree::Tag {
        iced_core::widget::tree::Tag::of::<super::state::IsDragging>()
    }

    fn state(&self) -> iced_core::widget::tree::State {
        iced_core::widget::tree::State::new(false)
    }

    fn children(&self) -> Vec<iced_core::widget::Tree> {
        vec![
            iced_core::widget::Tree::new(&self.first),
            iced_core::widget::Tree::new(&self.second),
        ]
    }

    fn diff(&self, tree: &mut iced_core::widget::Tree) {
        tree.diff_children(&[&self.first, &self.second]);
    }

    fn size(&self) -> iced_core::Size<iced_core::Length> {
        iced_core::Size::new(iced_core::Length::Fill, iced_core::Length::Fill)
    }

    fn layout(
        &mut self,
        tree: &mut iced_core::widget::Tree,
        renderer: &Renderer,
        limits: &iced_core::layout::Limits,
    ) -> iced_core::layout::Node {
        let limits = limits
            .width(iced_core::Length::Fill)
            .height(iced_core::Length::Fill);
        let size = limits.resolve(
            iced_core::Length::Fill,
            iced_core::Length::Fill,
            iced_core::Size::ZERO,
        );

        let split_pos = self.split_pos(size);
        let (first_limits, second_limits) = self.limit_nodes_size(size, limits, split_pos);

        let first_node =
            self.first
                .as_widget_mut()
                .layout(&mut tree.children[0], renderer, &first_limits);

        let second_node = self
            .second
            .as_widget_mut()
            .layout(&mut tree.children[1], renderer, &second_limits)
            .move_to(self.second_node_position(split_pos));

        iced_core::layout::Node::with_children(size, vec![first_node, second_node])
    }

    fn update(
        &mut self,
        tree: &mut iced_core::widget::Tree,
        event: &iced_core::Event,
        layout: iced_core::Layout<'_>,
        cursor: iced_core::mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn iced_core::Clipboard,
        shell: &mut iced_core::Shell<'_, Message>,
        viewport: &iced_core::Rectangle,
    ) {
        let is_dragging = tree.state.downcast_mut::<super::state::IsDragging>();

        let bounds = layout.bounds();
        let drag_rect = self.create_split_rect(self.drag_area_size, bounds);

        // TODO: Implement touch events
        match event {
            iced_core::Event::Mouse(iced_core::mouse::Event::ButtonPressed(
                iced_core::mouse::Button::Left,
            )) => {
                if let Some(cursor_pos) = cursor.position()
                    && drag_rect.contains(cursor_pos)
                {
                    *is_dragging = true;
                    shell.capture_event();
                    return;
                }
            }

            iced_core::Event::Mouse(
                iced_core::mouse::Event::CursorLeft
                | iced_core::mouse::Event::ButtonReleased(iced_core::mouse::Button::Left),
            ) if *is_dragging => {
                *is_dragging = false;

                shell.request_redraw();
                shell.capture_event();
                return;
            }

            iced_core::Event::Mouse(iced_core::mouse::Event::CursorMoved { position })
                if *is_dragging =>
            {
                let new_ratio = self.ratio(*position, bounds);

                let next_state = self.state.copy_with_new_ratio(new_ratio);
                if next_state != self.state {
                    shell.publish((self.on_drag)(next_state));
                }

                shell.capture_event();
                return;
            }

            _ => {}
        }

        let mut layouts = layout.children();
        self.first.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layouts.next().unwrap(),
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );

        self.second.as_widget_mut().update(
            &mut tree.children[1],
            event,
            layouts.next().unwrap(),
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );
    }

    fn draw(
        &self,
        tree: &iced_core::widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &iced_core::renderer::Style,
        layout: iced_core::Layout<'_>,
        cursor: iced_core::mouse::Cursor,
        viewport: &iced_core::Rectangle,
    ) {
        let mut layouts = layout.children();
        let first_layout = layouts.next().unwrap();
        let second_layout = layouts.next().unwrap();

        self.first.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            first_layout,
            cursor,
            viewport,
        );

        self.second.as_widget().draw(
            &tree.children[1],
            renderer,
            theme,
            style,
            second_layout,
            cursor,
            viewport,
        );

        let bounds = layout.bounds();
        let is_hovering = cursor.position().is_some_and(|position| {
            let hover_rect = self.create_split_rect(self.drag_area_size, bounds);
            hover_rect.contains(position)
        });

        let status = if *tree.state.downcast_ref::<super::state::IsDragging>() {
            super::style::State::Dragging
        } else if is_hovering {
            super::style::State::Hovering
        } else {
            super::style::State::Idle
        };

        let style = (self.style)(theme, status);

        renderer.fill_quad(
            iced_core::renderer::Quad {
                bounds: self.create_split_rect(style.divider_width, bounds),
                ..Default::default()
            },
            style.divider_color,
        );
    }

    fn mouse_interaction(
        &self,
        _tree: &iced_core::widget::Tree,
        layout: iced_core::Layout<'_>,
        cursor: iced_core::mouse::Cursor,
        _viewport: &iced_core::Rectangle,
        _renderer: &Renderer,
    ) -> iced_core::mouse::Interaction {
        let drag_rect = self.create_split_rect(self.drag_area_size, layout.bounds());
        if let Some(position) = cursor.position()
            && drag_rect.contains(position)
        {
            match self.axis {
                Axis::Vertical => iced_core::mouse::Interaction::ResizingHorizontally,
                Axis::Horizontal => iced_core::mouse::Interaction::ResizingVertically,
            }
        } else {
            iced_core::mouse::Interaction::None
        }
    }
}

impl<'a, Message, Theme, Renderer> From<Split<'a, Message, Theme, Renderer>>
    for iced_core::Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: iced_core::Renderer + 'a,
{
    fn from(value: Split<'a, Message, Theme, Renderer>) -> Self {
        Self::new(value)
    }
}

pub enum Axis {
    Vertical,
    Horizontal,
}
