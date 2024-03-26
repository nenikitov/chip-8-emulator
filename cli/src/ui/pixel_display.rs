use ratatui::{prelude::*, widgets::*};
use std::{iter, ops::Deref};

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

impl<Outer: ?Sized, Inner> Widget for PixelDisplay<Outer, Inner>
where
    Outer: Deref<Target = [Inner]>,
    Inner: AsRef<[bool]>,
{
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let lines: Vec<Line> = self
            .display
            .array_chunks::<2>()
            .map(|[row_1, row_2]| iter::zip(row_1.as_ref(), row_2.as_ref()))
            .map(|row_pairs| -> Vec<Span> {
                row_pairs
                    .map(|(top, bottom)| Span::styled("▀", generate_style(*top, *bottom)))
                    .chain(iter::once(Span::raw("\n")))
                    .collect()
            })
            .map(Line::from)
            .collect();

        Paragraph::new(lines).render(area, buf);
    }
}
