pub struct SplitVertical<'a, Message, Theme, Renderer> {
    left: iced_core::Element<'a, Message, Theme, Renderer>,
    right: iced_core::Element<'a, Message, Theme, Renderer>,

    state: super::state::State,
    on_drag: Box<dyn Fn(super::state::State) -> Message + 'a>,
}

impl<'a, Message, Theme, Renderer> SplitVertical<'a, Message, Theme, Renderer> {
    pub fn new(
        top: impl Into<iced_core::Element<'a, Message, Theme, Renderer>>,
        bottom: impl Into<iced_core::Element<'a, Message, Theme, Renderer>>,
        state: super::state::State,
        message: impl Fn(super::state::State) -> Message + 'a,
    ) -> Self {
        Self {
            left: top.into(),
            right: bottom.into(),
            state,
            on_drag: Box::new(message),
        }
    }
}

impl<Message, Theme, Renderer> iced_core::Widget<Message, Theme, Renderer>
    for SplitVertical<'_, Message, Theme, Renderer>
where
    Renderer: iced_core::renderer::Renderer,
{
    fn tag(&self) -> iced_core::widget::tree::Tag {
        iced_core::widget::tree::Tag::of::<()>()
    }

    fn state(&self) -> iced_core::widget::tree::State {
        iced_core::widget::tree::State::None
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
            .left
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
        let bounds = layout.bounds();
        let divider_x_pos = bounds.width.mul_add(self.state.ratio(), bounds.x);
        let drag_rect = iced_core::Rectangle {
            x: divider_x_pos - super::DRAG_AREA_SIZE / 2.0,
            y: bounds.y,
            width: super::DRAG_AREA_SIZE,
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
                    let mut next_state = self.state;
                    next_state.start_drag();
                    shell.publish((self.on_drag)(next_state));
                    shell.capture_event();
                    return;
                }
            }

            iced_core::Event::Mouse(iced_core::mouse::Event::ButtonReleased(
                iced_core::mouse::Button::Left,
            )) => {
                if self.state.is_dragging() {
                    let mut next_state = self.state;
                    next_state.stop_drag();
                    shell.publish((self.on_drag)(next_state));
                    shell.capture_event();
                    return;
                }
            }

            iced_core::Event::Mouse(iced_core::mouse::Event::CursorMoved { position }) => {
                if self.state.is_dragging() {
                    let relative_x = position.x - bounds.x;
                    let new_ratio = relative_x / bounds.width;
                    let mut next_state = self.state;
                    next_state.set_ratio(new_ratio);
                    shell.publish((self.on_drag)(next_state));
                    shell.capture_event();
                    return;
                }
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
        let divider_x = bounds.x + (bounds.width * self.state.ratio());

        renderer.fill_quad(
            iced_core::renderer::Quad {
                bounds: iced_core::Rectangle {
                    x: divider_x - (super::DRAG_AREA_SIZE / 2.0),
                    y: bounds.y,
                    width: super::DRAG_AREA_SIZE,
                    height: bounds.height,
                },
                ..iced_core::renderer::Quad::default()
            },
            iced_core::Color::from_rgb8(255, 0, 0),
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
