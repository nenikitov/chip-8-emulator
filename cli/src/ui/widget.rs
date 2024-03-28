use ratatui::{layout::*, prelude::*};

pub trait WidgetSize {
    fn render_sized(&self, area: Rect, buf: &mut Buffer) -> Size;
}

pub struct LayoutAlign<'a> {
    pub child: &'a dyn WidgetSize,
    pub horizontal: Alignment,
    pub vertical: Alignment,
}

impl<'a> WidgetSize for LayoutAlign<'a> {
    fn render_sized(&self, area: Rect, buf: &mut Buffer) -> Size {
        let mut buf_temp = Buffer::empty(buf.area);
        let size = self.child.render_sized(buf_temp.area, &mut buf_temp);

        let x = match self.horizontal {
            Alignment::Left => area.x,
            Alignment::Center => area
                .x
                .saturating_add((area.width.saturating_sub(size.width)) / 2),
            Alignment::Right => area.x.saturating_add(area.width).saturating_sub(size.width),
        };
        let y = match self.vertical {
            Alignment::Left => area.y,
            Alignment::Center => area
                .y
                .saturating_add((area.height.saturating_sub(size.height)) / 2),
            Alignment::Right => area
                .x
                .saturating_add(area.height)
                .saturating_sub(size.height),
        };

        self.child.render_sized(
            Rect {
                x,
                y,
                width: size.width,
                height: size.height,
            },
            buf,
        );

        Size {
            width: if self.horizontal == Alignment::Left {
                size.width
            } else {
                area.width
            },
            height: if self.vertical == Alignment::Left {
                size.height
            } else {
                area.height
            },
        }
    }
}

impl<'a> Widget for LayoutAlign<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        self.render_sized(area, buf);
    }
}

pub struct LayoutLinear<'a> {
    pub children: Vec<&'a dyn WidgetSize>,
    pub direction: Direction,
    pub gap: u16,
}

impl<'a> WidgetSize for LayoutLinear<'a> {
    fn render_sized(&self, area: Rect, buf: &mut Buffer) -> Size {
        if self.children.len() == 0 {
            return Size::default();
        }

        let mut buf_temp = Buffer::empty(buf.area);
        let children_and_sizes: Vec<_> = self
            .children
            .iter()
            .map(|c| {
                // TODO(nenikitov): This is a bit wasteful to render widgets to get their sizes just to re-render them
                // Optimize this
                (*c, c.render_sized(area, &mut buf_temp))
            })
            .collect();

        // Opposite to the axis
        let width_relative = children_and_sizes
            .iter()
            .map(|(_, s)| match self.direction {
                Direction::Horizontal => s.height,
                Direction::Vertical => s.width,
            })
            .max()
            .expect("layout has children");

        let mut position = area.as_position();
        let mut height_relative = 0;
        for (c, size) in &children_and_sizes {
            match self.direction {
                Direction::Horizontal => {
                    let size = c.render_sized(
                        Rect::from((
                            position,
                            Size {
                                width: size.width,
                                height: width_relative,
                            },
                        )),
                        buf,
                    );
                    let step = size.width + self.gap;
                    position.x += step;
                    height_relative += step;
                }
                Direction::Vertical => {
                    let size = c.render_sized(
                        Rect::from((
                            position,
                            Size {
                                width: width_relative,
                                height: size.height,
                            },
                        )),
                        buf,
                    );
                    let step = size.height + self.gap;
                    position.y += step;
                    height_relative += step;
                }
            }
        }

        height_relative -= self.gap;
        match self.direction {
            Direction::Horizontal => Size {
                width: height_relative,
                height: width_relative,
            },
            Direction::Vertical => Size {
                width: width_relative,
                height: height_relative,
            },
        }
    }
}
