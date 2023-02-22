use chip_8::Chip8;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, event::{self, Event, KeyEventKind, KeyCode},
};
use std::{
    io::{self, Write},
    vec,
};
use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::{Span, Spans, Text},
    widgets::{Paragraph, Wrap},
    Frame, Terminal,
};

pub trait Drawable {
    fn render<B: Backend>(&self, f: &mut Frame<B>);
}

struct SizeErrorBox {
    min_x: u16,
    min_y: u16,
}
impl Drawable for SizeErrorBox {
    fn render<B: Backend>(&self, f: &mut Frame<B>) {
        let size = f.size();
        let color_x = if size.width < self.min_x {
            Color::LightRed
        } else {
            Color::LightGreen
        };
        let color_y = if size.height < self.min_y {
            Color::LightRed
        } else {
            Color::LightGreen
        };

        let text = vec![
            Spans::from(Span::raw("Terminal window is too small")),
            Spans::from(vec![
                Span::raw("Width = "),
                Span::styled(format!("{}", size.width), Style::default().fg(color_x)),
                Span::raw(format!(" (needed {})", self.min_x)),
            ]),
            Spans::from(vec![
                Span::raw("Height = "),
                Span::styled(format!("{}", size.height), Style::default().fg(color_y)),
                Span::raw(format!(" (needed {})", self.min_y)),
            ]),
        ];

        let paragraph = Paragraph::new(text)
            .style(Style::default().bg(Color::Black))
            .alignment(Alignment::Center);
        f.render_widget(paragraph, size);
    }
}

#[derive(
    Debug,
    Clone, Copy,
    PartialEq, PartialOrd, Eq
)]
pub enum AppState {
    InProgress,
    End
}

#[derive(Debug)]
pub struct App {
    chip: Chip8,
    state: AppState
}

impl App {
    pub fn new(chip: Chip8) -> Self {
        Self {
            chip,
            state: AppState::InProgress
        }
    }

    pub fn update(&mut self) {
        if let Ok(event) = event::read() {
            if let Event::Key(key) = event {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Esc =>
                            self.state = AppState::End,
                        _ => ()
                    }
                }
            }
        }

        // TODO make this asynchronous
        for _ in 0..256 {
           self.chip.advance();
        }
    }

    pub fn state(&self) -> AppState {
        self.state
    }
}

impl Drawable for App {
    fn render<B: Backend>(&self, f: &mut Frame<B>) {
        let size = f.size();
        let (display_x, display_y) = self.chip.display_size();
        if size.width <= display_x || size.height <= display_y {
            SizeErrorBox {
                min_x: display_x + 1,
                min_y: display_y + 1,
            }
            .render(f);
        } else {
            let display =
                self.chip.display().chunks(display_x as usize)
                .map(
                    |r|
                    r.iter().map(|p| if *p { '█' } else { ' ' }).collect::<String>()
                )
                .collect::<Vec<_>>()
                .join("\n");
            let display = Paragraph::new(Text::from(display));
            let x = (size.width - display_x) / 2;
            let y = (size.height - display_y) / 2;
            f.render_widget(
                display,
                Rect {
                    x,
                    y,
                    width: display_x,
                    height: display_y,
                },
            );
        }
    }
}

pub fn start_ui<B: Backend>(backend: B) -> Result<Terminal<B>, io::Error>
where
    B: Backend,
{
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let terminal = Terminal::new(backend)?;

    Ok(terminal)
}

pub fn end_ui<B: Backend + Write>(mut terminal: Terminal<B>) -> Result<(), io::Error> {
    disable_raw_mode()?;
    let backend = terminal.backend_mut();
    execute!(backend, LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
