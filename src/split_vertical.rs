pub struct SplitVertical<'a, Message, Theme, Renderer> {
    left: iced_core::Element<'a, Message, Theme, Renderer>,
    right: iced_core::Element<'a, Message, Theme, Renderer>,

    state: super::state::State,
    on_drag: Box<dyn Fn(super::state::State) -> Message + 'a>,

    drag_area_size: f32,

    style: super::style::StyleFn<'a, Theme>,
}

impl<'a, Message, Theme, Renderer> SplitVertical<'a, Message, Theme, Renderer>
where
    Theme: 'a,
{
    pub fn new(
        left: impl Into<iced_core::Element<'a, Message, Theme, Renderer>>,
        right: impl Into<iced_core::Element<'a, Message, Theme, Renderer>>,
        state: super::state::State,
        message: impl Fn(super::state::State) -> Message + 'a,
    ) -> Self {
        Self {
            left: left.into(),
            right: right.into(),
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

impl<Message, Theme, Renderer> iced_core::Widget<Message, Theme, Renderer>
    for SplitVertical<'_, Message, Theme, Renderer>
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
            iced_core::widget::Tree::new(&self.left),
            iced_core::widget::Tree::new(&self.right),
        ]
    }

    fn diff(&self, tree: &mut iced_core::widget::Tree) {
        tree.diff_children(&[&self.left, &self.right]);
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

        let split_x_pos = size.width * self.state.ratio();

        let left_limits = limits.max_width(split_x_pos);
        let left_node =
            self.left
                .as_widget_mut()
                .layout(&mut tree.children[0], renderer, &left_limits);

        let right_limits = limits.max_width(size.width - split_x_pos);
        let right_node = self
            .right
            .as_widget_mut()
            .layout(&mut tree.children[1], renderer, &right_limits)
            .move_to(iced_core::Point::new(split_x_pos, 0.0));

        iced_core::layout::Node::with_children(size, vec![left_node, right_node])
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
        let divider_x_pos = bounds.width.mul_add(self.state.ratio(), bounds.x);
        let drag_rect = iced_core::Rectangle {
            x: divider_x_pos - self.drag_area_size / 2.0,
            y: bounds.y,
            width: self.drag_area_size,
            height: bounds.height,
        };

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
            ) => {
                if *is_dragging {
                    *is_dragging = false;
                    let next_state = self.state;
                    if next_state != self.state {
                        shell.publish((self.on_drag)(next_state));
                    }
                    shell.capture_event();
                    return;
                }
            }

            iced_core::Event::Mouse(iced_core::mouse::Event::CursorMoved { position })
                if *is_dragging =>
            {
                let relative_x = position.x - bounds.x;
                let new_ratio = (relative_x / bounds.width).clamp(0.0, 1.0);
                let mut next_state = self.state;
                next_state.set_ratio(new_ratio);
                if next_state != self.state {
                    shell.publish((self.on_drag)(next_state));
                }
                shell.capture_event();
                return;
            }

            _ => {}
        }

        let mut layouts = layout.children();
        self.left.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layouts.next().unwrap(),
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );

        self.right.as_widget_mut().update(
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
        let left_layout = layouts.next().unwrap();
        let right_layout = layouts.next().unwrap();

        self.left.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            left_layout,
            cursor,
            viewport,
        );

        self.right.as_widget().draw(
            &tree.children[1],
            renderer,
            theme,
            style,
            right_layout,
            cursor,
            viewport,
        );

        let bounds = layout.bounds();
        let divider_x = bounds.width.mul_add(self.state.ratio(), bounds.x);

        let is_hovering = cursor.position().is_some_and(|position| {
            let hover_rect = iced_core::Rectangle {
                x: divider_x - (self.drag_area_size / 2.0),
                y: bounds.y,
                width: self.drag_area_size,
                height: bounds.height,
            };
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
                bounds: iced_core::Rectangle {
                    x: divider_x - (style.divider_width / 2.0),
                    y: bounds.y,
                    width: style.divider_width,
                    height: bounds.height,
                },
                ..Default::default()
            },
            style.divider_color,
        );
    }
}

impl<'a, Message, Theme, Renderer> From<SplitVertical<'a, Message, Theme, Renderer>>
    for iced_core::Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: iced_core::Renderer + 'a,
{
    fn from(value: SplitVertical<'a, Message, Theme, Renderer>) -> Self {
        Self::new(value)
    }
}
