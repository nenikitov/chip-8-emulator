use chip_8::Chip8;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, poll},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io::{self, Write},
    iter, vec, time::Duration,
};
use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::Paragraph,
    Frame, Terminal,
};

use crate::timer::Timer;

pub trait Drawable {
    fn render<B: Backend>(&self, f: &mut Frame<B>, position: (u16, u16));
    fn size(&self) -> (u16, u16);
}

struct SizeErrorBox {
    min_x: u16,
    min_y: u16,
}
impl Drawable for SizeErrorBox {
    fn render<B: Backend>(&self, f: &mut Frame<B>, _: (u16, u16)) {
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

    fn size(&self) -> (u16, u16) {
        (0, 0)
    }
}

struct Chip8Display<'a> {
    display: &'a [&'a [bool]],
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
    fn render<B: Backend>(&self, f: &mut Frame<B>, position: (u16, u16)) {
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
                x: position.0,
                y: position.1,
                width: display_size.0,
                height: display_size.1 / 2,
            },
        );
    }

    fn size(&self) -> (u16, u16) {
        let size = self.display_size();
        (size.0, size.1 / 2)
    }
}


struct InstructionsPerSecond<'a> {
    timer: &'a Timer,
    instructions_per_loop: u32,
}
impl<'a> Drawable for InstructionsPerSecond<'a> {
    fn render<B: Backend>(&self, f: &mut Frame<B>, position: (u16, u16)) {
        let size = f.size();
        let instructions = (1f64 / self.timer.delta().as_secs_f64()) * self.instructions_per_loop as f64;
        f.render_widget(
            Paragraph::new(format!("{} i/s", instructions.round())).alignment(Alignment::Right),
            Rect {
                x: position.0,
                y: position.1,
                width: size.width,
                height: size.height
            }
        )
    }

    fn size(&self) -> (u16, u16) {
        (0, 1)
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
    state: AppState,
    timer: Timer,
    instructions_per_loop: u32
}

impl App {
    pub fn new(chip: Chip8) -> Self {
        Self {
            chip,
            state: AppState::InProgress,
            timer: Timer::new(),
            instructions_per_loop: 1
        }
    }

    fn handle_input(&mut self) -> crossterm::Result<bool> {
        if poll(Duration::from_secs(0))? {
            let key = event::read()?;
            if let Event::Key(key) = key {
                match (key.kind, key.code) {
                    (KeyEventKind::Press, KeyCode::Esc) =>
                        self.state = AppState::End,
                    _ => ()
                }
            }
        }
        Ok(true)
    }

    pub fn update(&mut self) {
        if let Err(e) = self.handle_input() {
            self.state = AppState::End;
            println!("Error reading from input {}", e);
        }

        self.chip.advance();

        self.timer.update();
    }

    pub fn state(&self) -> AppState {
        self.state
    }
}

impl Drawable for App {
    fn render<B: Backend>(&self, f: &mut Frame<B>, _: (u16, u16)) {
        let size = f.size();

        let display = self.chip.screen();
        let (display_x, display_y) = (display[0].len() as u16, display.len() as u16);
        let display_y = display_y / 2;

        let widget_instructions = InstructionsPerSecond {
            timer: &self.timer,
            instructions_per_loop: self.instructions_per_loop
        };

        let widget_display = Chip8Display {
            display: &display,
        };

        let minimum_x = widget_display.size().0;
        let minimum_y = widget_display.size().1 + widget_instructions.size().1 + 1;

        if size.width < minimum_x || size.height < minimum_y {
            SizeErrorBox {
                min_x: minimum_x,
                min_y: minimum_y,
            }
            .render(
                f,
                (0, 0)
            );
        } else {
            widget_display.render(
                f,
                (
                    (size.width - display_x) / 2 + widget_instructions.size().1,
                    (size.height - display_y) / 2
                )
            );
            widget_instructions.render(
                f,
                (0, 0)
            );
        }
    }

    fn size(&self) -> (u16, u16) {
        (0, 0)
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
