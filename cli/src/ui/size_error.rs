use ratatui::{layout::*, prelude::*, widgets::*};

use super::{LayoutAlign, LayoutOverlay, WidgetSize};

pub struct SizeError {
    pub min: Size,
}

impl WidgetSize for SizeError {
    fn render_sized(&self, area: Rect, buf: &mut Buffer) -> Size {
        let size_area = area.as_size();
        let size_minimum = self.minimum_size();

        if size_area.width >= size_minimum.width && size_area.height >= size_minimum.height {
            let lack_width = area.width < self.min.width;
            let lack_height = area.height < self.min.height;

            let paragraph = Paragraph::new(vec![
                Line::styled(
                    format!("Width = {} (needed {})", area.width, self.min.width),
                    Style::default().fg(if lack_width {
                        Color::LightRed
                    } else {
                        Color::LightGreen
                    }),
                ),
                Line::styled(
                    format!("Height = {} (needed {})", area.height, self.min.height),
                    Style::default().fg(if lack_height {
                        Color::LightRed
                    } else {
                        Color::LightGreen
                    }),
                ),
            ]);

            let background = Block::new()
                .title_top(
                    Line::styled(
                        "Terminal window is too small",
                        Style::default().white().add_modifier(Modifier::BOLD),
                    )
                    .centered(),
                )
                .borders(
                    if lack_width {
                        Borders::LEFT | Borders::RIGHT
                    } else {
                        Borders::NONE
                    } | if lack_height {
                        Borders::TOP | Borders::BOTTOM
                    } else {
                        Borders::NONE
                    },
                )
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Red));

            LayoutOverlay {
                children: vec![
                    &background,
                    &LayoutAlign {
                        child: &paragraph,
                        horizontal: Alignment::Center,
                        vertical: Alignment::Center,
                    },
                ],
            }
            .render_sized(area, buf)
        } else {
            Size {
                width: 25,
                height: 3,
            }
        }
    }

    fn minimum_size(&self) -> Size {
        Size::default()
    }
}

impl WidgetSize for Paragraph<'_> {
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

impl WidgetSize for Block<'_> {
    fn render_sized(&self, area: Rect, buf: &mut Buffer) -> Size {
        self.render(area, buf);
        area.as_size()
    }

    fn minimum_size(&self) -> Size {
        Size {
            width: 3,
            height: 3,
        }
    }
}
