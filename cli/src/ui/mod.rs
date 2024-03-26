mod app;
mod pixel_display;
mod size_error;

use chip_8::Chip8;
use crossterm::{
    event::{self, poll, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};
use std::{
    io::{self, stdout, Stdout},
    iter,
    time::Duration,
    vec,
};

use crate::timer::Timer;

pub trait Drawable {
    fn render(&self, f: &mut Frame, position: (u16, u16));
    fn size(&self) -> (u16, u16);
}

struct Stats<'a> {
    instructions_timer: &'a Timer,
    frames_timer: &'a Timer,
}
impl<'a> Drawable for Stats<'a> {
    fn render(&self, f: &mut Frame, position: (u16, u16)) {
        let size = f.size();
        let instructions = (1f64 / self.instructions_timer.delta().as_secs_f64()).round();
        let frames = (1f64 / self.frames_timer.delta().as_secs_f64()).round();
        f.render_widget(
            Paragraph::new(format!("{} ips", instructions)).alignment(Alignment::Right),
            Rect {
                x: position.0,
                y: position.1,
                width: size.width,
                height: size.height,
            },
        );
        f.render_widget(
            Paragraph::new(format!("{} fps", frames)).alignment(Alignment::Right),
            Rect {
                x: position.0,
                y: position.1 + 1,
                width: size.width,
                height: size.height,
            },
        );
    }

    fn size(&self) -> (u16, u16) {
        (0, 1)
    }
}

pub use app::*;

//#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq)]
//pub enum AppState {
//    InProgress,
//    End,
//}
//
//#[derive(Debug)]
//pub struct App {
//    chip: Chip8,
//    state: AppState,
//    instructions_timer: Timer,
//    pub frames_timer: Timer,
//}
//
//impl App {
//    pub fn new(chip: Chip8) -> Self {
//        Self {
//            chip,
//            state: AppState::InProgress,
//            instructions_timer: Timer::new(),
//            frames_timer: Timer::new(),
//        }
//    }
//
//    fn handle_input(&mut self) -> crossterm::Result<bool> {
//        if poll(Duration::from_secs(0))? {
//            let key = event::read()?;
//            if let Event::Key(key) = key {
//                match (key.kind, key.code) {
//                    (KeyEventKind::Press, KeyCode::Char('q')) => {
//                        self.state = AppState::End;
//                    }
//                    _ => (),
//                }
//            }
//        }
//        Ok(true)
//    }
//
//    pub fn update(&mut self) {
//        self.instructions_timer.update();
//
//        if let Err(e) = self.handle_input() {
//            self.state = AppState::End;
//            println!("Error reading from input {}", e);
//        }
//
//        self.chip.advance();
//    }
//
//    pub fn state(&self) -> AppState {
//        self.state
//    }
//}
//
//impl Drawable for App {
//    fn render(&self, f: &mut Frame, _: (u16, u16)) {
//        let size = f.size();
//
//        let display = self.chip.screen();
//        let (display_x, display_y) = (display[0].len() as u16, display.len() as u16);
//        let display_y = display_y / 2;
//
//        let widget_instructions = Stats {
//            instructions_timer: &self.instructions_timer,
//            frames_timer: &self.frames_timer,
//        };
//
//        let widget_display = Chip8Display { display: &display };
//
//        let minimum_x = widget_display.size().0;
//        let minimum_y = widget_display.size().1 + widget_instructions.size().1 + 1;
//
//        if size.width < minimum_x || size.height < minimum_y {
//            SizeErrorBox {
//                min_x: minimum_x,
//                min_y: minimum_y,
//            }
//            .render(f, (0, 0));
//        } else {
//            widget_display.render(
//                f,
//                (
//                    (size.width - display_x) / 2 + widget_instructions.size().1,
//                    (size.height - display_y) / 2,
//                ),
//            );
//            widget_instructions.render(f, (0, 0));
//        }
//    }
//
//    fn size(&self) -> (u16, u16) {
//        (0, 0)
//    }
//}

pub fn start_ui() -> Result<Terminal<CrosstermBackend<Stdout>>, io::Error> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    Terminal::new(CrosstermBackend::new(stdout()))
}

pub fn end_ui() -> Result<(), io::Error> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
