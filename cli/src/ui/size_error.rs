use ratatui::{prelude::*, widgets::*};

pub struct SizeError {
    pub min_x: u16,
    pub min_y: u16,
}

impl Widget for SizeError {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let color_x = if area.width < self.min_x {
            Color::LightRed
        } else {
            Color::LightGreen
        };
        let color_y = if area.height < self.min_y {
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
                format!("Width = {} (needed {})", area.width, self.min_x),
                Style::default().fg(color_x),
            ),
            Line::styled(
                format!("Height = {} (needed {})", area.height, self.min_y),
                Style::default().fg(color_y),
            ),
        ])
        .style(Style::default().bg(Color::Black))
        .alignment(Alignment::Center);

        paragraph.render(area, buf);
    }
}
