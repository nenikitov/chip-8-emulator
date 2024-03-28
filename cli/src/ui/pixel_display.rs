use ratatui::{layout::Size, prelude::*, widgets::*};
use std::{iter, ops::Deref};

use super::WidgetSize;

fn generate_style(top: bool, bottom: bool) -> Style {
    Style::default()
        .fg(if top { Color::White } else { Color::Black })
        .bg(if bottom { Color::White } else { Color::Black })
}

pub struct PixelDisplay<Outer: ?Sized, Inner>
where
    Outer: Deref<Target = [Inner]>,
    Inner: AsRef<[bool]>,
{
    pub display: Outer,
}

impl<Outer: ?Sized, Inner> WidgetSize for PixelDisplay<Outer, Inner>
where
    Outer: Deref<Target = [Inner]>,
    Inner: AsRef<[bool]>,
{
    fn render_sized(&self, area: Rect, buf: &mut Buffer) -> layout::Size {
        let lines: Vec<Line> = self
            .display
            .array_chunks::<2>()
            .map(|[row_1, row_2]| iter::zip(row_1.as_ref(), row_2.as_ref()))
            .map(|row_pairs| -> Vec<Span> {
                row_pairs
                    .map(|(top, bottom)| Span::styled("▀", generate_style(*top, *bottom)))
                    .collect()
            })
            .map(Line::from)
            .collect();

        let size = Size {
            width: lines[0].width() as u16,
            height: lines.len() as u16,
        };
        Paragraph::new(lines).render(area, buf);

        size
    }
}
