use ratatui::{layout::Size, prelude::*, widgets::*};

use super::WidgetSize;

pub struct Stat {
    pub name: String,
    pub value: f64,
    pub target: f64,
    pub precision: Option<usize>,
}

impl Stat {
    fn format(&self) -> String {
        let value = if let Some(precision) = self.precision {
            format!("{:.1$}", self.value, precision)
        } else {
            format!("{}", self.value)
        };
        let target = if let Some(precision) = self.precision {
            format!("{:.1$}", self.target, precision)
        } else {
            format!("{}", self.target)
        };

        format!("{}: {value} / {target}", self.name)
    }
}

impl WidgetSize for Stat {
    fn render_sized(&self, area: Rect, buf: &mut Buffer) -> Size {
        let color = match self.value / self.target {
            x if x < 0.5 => Color::LightRed,
            x if x < 0.9 => Color::LightYellow,
            x if x < 1.1 => Color::LightGreen,
            x if x < 1.5 => Color::LightBlue,
            _ => Color::LightMagenta,
        };

        let text = self.format();
        let width = text.len() as u16;

        Paragraph::new(text)
            .style(Style::default().fg(color))
            .render(area, buf);

        Size { height: 1, width }
    }
}
