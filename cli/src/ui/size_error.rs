use ratatui::{layout::*, prelude::*, widgets::*};

use super::WidgetSize;

pub struct SizeError {
    pub min: Size,
}

impl WidgetSize for SizeError {
    fn render_sized(&self, area: Rect, buf: &mut Buffer) -> Size {
        let size_area = area.as_size();
        let size_minimum = self.minimum_size();

        if size_area.width >= size_minimum.width && size_area.height >= size_minimum.height {
            let color_width = if area.width < self.min.width {
                Color::LightRed
            } else {
                Color::LightGreen
            };
            let color_height = if area.height < self.min.height {
                Color::LightRed
            } else {
                Color::LightGreen
            };

            let paragraph = Paragraph::new(vec![
                Line::styled(
                    "Terminal window is too small",
                    Style::default().add_modifier(Modifier::BOLD),
                ),
                Line::styled(
                    format!("Width = {} (needed {})", area.width, self.min.width),
                    Style::default().fg(color_width),
                ),
                Line::styled(
                    format!("Height = {} (needed {})", area.height, self.min.height),
                    Style::default().fg(color_height),
                ),
            ])
            .style(Style::default().bg(Color::Black))
            .alignment(Alignment::Center);

            paragraph.render(area, buf);

            area.as_size()
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
