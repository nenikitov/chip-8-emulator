use chip_8::Chip8;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io::{self, Write},
    iter, vec,
};
use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::Paragraph,
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

struct Chip8Display<'a> {
    display: &'a [&'a [bool]],
    position: (u16, u16),
}
impl<'a> Chip8Display<'a> {
    fn display_size(&self) -> (u16, u16) {
        (
            self.display[0].len() as u16,
            self.display.len() as u16
        )
    }

    fn generate_style(top: bool, bottom: bool) -> Style {
        match (top, bottom) {
            (false, false) => Style::default().fg(Color::Black).bg(Color::Black),
            (false, true) => Style::default().fg(Color::Black).bg(Color::White),
            (true, false) => Style::default().fg(Color::White).bg(Color::Black),
            (true, true) => Style::default().fg(Color::White).bg(Color::White),
        }
    }
}
impl<'a> Drawable for Chip8Display<'a> {
    fn render<B: Backend>(&self, f: &mut Frame<B>) {
        let display_size = self.display_size();
        let display: Vec<Spans> =
            // Take every 2 rows
            self.display.chunks(2)
            // Zip them together
            .map(|c| iter::zip(c[0], c[1]))
            // Compose text rows
            .map(
                |i| i
                // Generate pixels for current 2 rows
                .map(|(t, b)| Span::styled("▀", Self::generate_style(*t, *b)))
                // Add line break after current rows
                .chain(iter::once(Span::raw("\n")))
                .collect::<Vec<Span>>()
            )
            .map(|r| Spans::from(r))
            .collect();
        f.render_widget(
            Paragraph::new(display),
            Rect {
                x: self.position.0,
                y: self.position.1,
                width: display_size.0,
                height: display_size.1 / 2,
            },
        );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq)]
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
                        KeyCode::Esc => self.state = AppState::End,
                        _ => (),
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
        let display = self.chip.display();
        let (display_x, display_y) = (display[0].len() as u16, display.len() as u16);
        let display_y = display_y / 2;
        if size.width < display_x || size.height < display_y {
            SizeErrorBox {
                min_x: display_x,
                min_y: display_y,
            }
            .render(f);
        } else {
            let x = (size.width - display_x) / 2;
            let y = (size.height - display_y) / 2;
            Chip8Display {
                display: &display,
                position: (x, y)
            }
            .render(f)
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
