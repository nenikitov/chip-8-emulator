use ratatui::{layout::Size, prelude::*, widgets::*};

use super::WidgetSize;

pub enum StatBias {
    HigherBetter,
    LowerBetter,
}

pub struct Stat {
    pub name: String,
    pub value: f64,
    pub target: f64,
    pub bias: StatBias,
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
        let delta = (self.value - self.target) / self.target;

        let color = match match self.bias {
            StatBias::HigherBetter => delta,
            StatBias::LowerBetter => -delta,
        } {
            x if x < -0.5 => Color::LightRed,
            x if x < -0.1 => Color::LightYellow,
            x if x < 0.1 => Color::LightGreen,
            x if x < 0.5 => Color::LightBlue,
            _ => Color::LightMagenta,
        };

        Paragraph::new(self.format())
            .style(Style::default().fg(color))
            .render(area, buf);

        self.minimum_size()
    }

    fn minimum_size(&self) -> Size {
        let text = self.format();
        Size {
            height: 1,
            width: text.len() as u16,
        }
    }
}
