use ratatui::{layout::*, prelude::*, widgets::*};

use crate::ui::size_error::SizeError;

pub trait WidgetSize {
    fn render_sized(&self, area: Rect, buf: &mut Buffer) -> Size;
    fn minimum_size(&self) -> Size;
}

pub struct LayoutAlign<'a> {
    pub child: &'a dyn WidgetSize,
    pub horizontal: Alignment,
    pub vertical: Alignment,
}

impl<'a> WidgetSize for LayoutAlign<'a> {
    fn render_sized(&self, area: Rect, buf: &mut Buffer) -> Size {
        let child_size = self.child.minimum_size();

        let x = match self.horizontal {
            Alignment::Left => area.x,
            Alignment::Center => area
                .x
                .saturating_add((area.width.saturating_sub(child_size.width)) / 2),
            Alignment::Right => area
                .x
                .saturating_add(area.width)
                .saturating_sub(child_size.width),
        };
        let y = match self.vertical {
            Alignment::Left => area.y,
            Alignment::Center => area
                .y
                .saturating_add((area.height.saturating_sub(child_size.height)) / 2),
            Alignment::Right => area
                .x
                .saturating_add(area.height)
                .saturating_sub(child_size.height),
        };

        self.child.render_sized(
            Rect {
                x,
                y,
                width: child_size.width,
                height: child_size.height,
            },
            buf,
        );

        area.as_size()
    }

    fn minimum_size(&self) -> Size {
        self.child.minimum_size()
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
    pub direction: Direction,
    pub children: Vec<(&'a dyn WidgetSize, Option<Constraint>)>,
    pub flex_main_axis: Option<Flex>,
    pub flex_cross_axis: bool,
    pub spacing: u16,
}

impl<'a> WidgetSize for LayoutLinear<'a> {
    fn render_sized(&self, area: Rect, buf: &mut Buffer) -> Size {
        let constraints: Vec<_> = self
            .children
            .iter()
            .map(|(child, constraint)| {
                if let Some(constraint) = constraint {
                    *constraint
                } else {
                    Constraint::Length(match self.direction {
                        Direction::Horizontal => child.minimum_size().width,
                        Direction::Vertical => child.minimum_size().height,
                    })
                }
            })
            .collect();

        let mut target_area = area;
        if !self.flex_cross_axis {
            match self.direction {
                Direction::Horizontal => {
                    target_area.height = u16::min(target_area.height, self.minimum_size().height)
                }
                Direction::Vertical => {
                    target_area.width = u16::min(target_area.width, self.minimum_size().width)
                }
            }
        }

        let mut layout = Layout::default()
            .direction(self.direction)
            .spacing(self.spacing)
            .constraints(constraints);
        if let Some(flex) = self.flex_main_axis {
            layout = layout.flex(flex);
        }

        for (c, &a) in self
            .children
            .iter()
            .map(|(c, _)| *c)
            .zip(layout.split(target_area).as_ref())
        {
            c.render_sized(a, buf);
        }

        target_area.as_size()
    }

    fn minimum_size(&self) -> Size {
        if self.children.len() == 0 {
            Size::default()
        } else {
            let sizes: Vec<_> = self
                .children
                .iter()
                .map(|(c, _)| c.minimum_size())
                .collect();

            let main_axis = sizes
                .iter()
                .map(|s| match self.direction {
                    Direction::Horizontal => s.width,
                    Direction::Vertical => s.height,
                })
                .sum::<u16>()
                + self.spacing * (self.children.len() as u16 - 1);
            let cross_axis = sizes
                .iter()
                .map(|s| match self.direction {
                    Direction::Horizontal => s.height,
                    Direction::Vertical => s.width,
                })
                .max()
                .unwrap();

            match self.direction {
                Direction::Horizontal => Size {
                    width: main_axis,
                    height: cross_axis,
                },
                Direction::Vertical => Size {
                    width: cross_axis,
                    height: main_axis,
                },
            }
        }
    }
}

pub struct LayoutOverlay<'a> {
    pub children: Vec<&'a dyn WidgetSize>,
}

impl<'a> WidgetSize for LayoutOverlay<'a> {
    fn render_sized(&self, area: Rect, buf: &mut Buffer) -> Size {
        for c in &self.children {
            c.render_sized(area, buf);
        }

        area.as_size()
    }

    fn minimum_size(&self) -> Size {
        if self.children.len() == 0 {
            Size::default()
        } else {
            let sizes: Vec<_> = self.children.iter().map(|c| c.minimum_size()).collect();

            Size {
                width: sizes.iter().map(|s| s.width).max().unwrap(),
                height: sizes.iter().map(|s| s.height).max().unwrap(),
            }
        }
    }
}

pub struct LayoutSizeError<'a> {
    pub child: &'a dyn WidgetSize,
}

impl<'a> WidgetSize for LayoutSizeError<'a> {
    fn render_sized(&self, area: Rect, buf: &mut Buffer) -> Size {
        let size_area = area.as_size();
        let size_minimum = self.minimum_size();

        if size_area.width >= size_minimum.width && size_area.height >= size_minimum.height {
            self.child.render_sized(area, buf)
        } else {
            SizeError { min: size_minimum }.render_sized(area, buf)
        }
    }

    fn minimum_size(&self) -> Size {
        self.child.minimum_size()
    }
}

impl<'a> WidgetSize for &'a Paragraph<'_> {
    fn render_sized(&self, area: Rect, buf: &mut Buffer) -> Size {
        self.render(area, buf);
        self.minimum_size()
    }

    fn minimum_size(&self) -> Size {
        let width = self.line_width() as u16;
        let height = self.line_count(width) as u16;

        Size { width, height }
    }
}
