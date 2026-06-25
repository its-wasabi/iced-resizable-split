pub struct SplitHorizontal<'a, Message, Theme, Renderer> {
    top: iced_core::Element<'a, Message, Theme, Renderer>,
    bottom: iced_core::Element<'a, Message, Theme, Renderer>,

    state: super::state::State,
    on_drag: Box<dyn Fn(super::state::State) -> Message + 'a>,

    drag_area_size: f32,

    style: super::style::StyleFn<'a, Theme>,
}

impl<'a, Message, Theme, Renderer> SplitHorizontal<'a, Message, Theme, Renderer>
where
    Theme: 'a,
{
    pub fn new(
        top: impl Into<iced_core::Element<'a, Message, Theme, Renderer>>,
        bottom: impl Into<iced_core::Element<'a, Message, Theme, Renderer>>,
        state: super::state::State,
        message: impl Fn(super::state::State) -> Message + 'a,
    ) -> Self {
        Self {
            top: top.into(),
            bottom: bottom.into(),
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
    for SplitHorizontal<'_, Message, Theme, Renderer>
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
            iced_core::widget::Tree::new(&self.top),
            iced_core::widget::Tree::new(&self.bottom),
        ]
    }

    fn diff(&self, tree: &mut iced_core::widget::Tree) {
        tree.diff_children(&[&self.top, &self.bottom]);
    }

    fn size(&self) -> iced_core::Size<iced_core::Length> {
        iced_core::Size {
            width: iced_core::Length::Fill,
            height: iced_core::Length::Fill,
        }
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

        let split_y_pos = size.height * self.state.ratio();

        let top_limits = limits.max_height(split_y_pos);
        let top_node =
            self.top
                .as_widget_mut()
                .layout(&mut tree.children[0], renderer, &top_limits);

        let bottom_limits = limits.max_height(size.height - split_y_pos);
        let bottom_node = self
            .bottom
            .as_widget_mut()
            .layout(&mut tree.children[1], renderer, &bottom_limits)
            .move_to(iced_core::Point::new(0.0, split_y_pos));

        iced_core::layout::Node::with_children(size, vec![top_node, bottom_node])
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
        let divider_y_pos = bounds.height.mul_add(self.state.ratio(), bounds.y);
        let drag_rect = iced_core::Rectangle {
            y: divider_y_pos - self.drag_area_size / 2.0,
            x: bounds.x,
            height: self.drag_area_size,
            width: bounds.width,
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
                let relative_y = position.y - bounds.y;
                let new_ratio = (relative_y / bounds.height).clamp(0.0, 1.0);
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
        self.top.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layouts.next().unwrap(),
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );

        self.bottom.as_widget_mut().update(
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
        let top_layout = layouts.next().unwrap();
        let bottom_layout = layouts.next().unwrap();

        self.top.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            top_layout,
            cursor,
            viewport,
        );

        self.bottom.as_widget().draw(
            &tree.children[1],
            renderer,
            theme,
            style,
            bottom_layout,
            cursor,
            viewport,
        );

        let bounds = layout.bounds();
        let divider_y = bounds.height.mul_add(self.state.ratio(), bounds.y);

        let is_hovering = cursor.position().is_some_and(|position| {
            let hover_rect = iced_core::Rectangle {
                y: divider_y - (self.drag_area_size / 2.0),
                x: bounds.x,
                height: self.drag_area_size,
                width: bounds.width,
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
                    y: divider_y - (style.divider_width / 2.0),
                    x: bounds.x,
                    height: style.divider_width,
                    width: bounds.width,
                },
                ..iced_core::renderer::Quad::default()
            },
            style.divider_color,
        );
    }
}

impl<'a, Message, Theme, Renderer> From<SplitHorizontal<'a, Message, Theme, Renderer>>
    for iced_core::Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: iced_core::Renderer + 'a,
{
    fn from(value: SplitHorizontal<'a, Message, Theme, Renderer>) -> Self {
        Self::new(value)
    }
}
