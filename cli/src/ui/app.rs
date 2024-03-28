use std::time::Duration;

use chip_8::Chip8;
use crossterm::event::{self, poll, Event, KeyCode, KeyEventKind};
use ratatui::{prelude::*, widgets::*};

use crate::timer::Timer;

use super::{
    pixel_display::PixelDisplay, size_error::SizeError, stats::Stat, LayoutAlign, LayoutLinear,
    WidgetSize,
};

#[derive(PartialEq, Eq, Default, Clone, Copy)]
pub enum AppState {
    #[default]
    InProgress,
    Pause,
    End,
}

pub struct App {
    chip: Chip8,
    state: AppState,
    timer_instructions: Timer,
    timer_frames: Timer,
}

impl App {
    pub fn new(chip: Chip8) -> Self {
        Self {
            chip,
            state: AppState::default(),
            timer_instructions: Timer::new(),
            timer_frames: Timer::new(),
        }
    }

    pub fn update(&mut self) {
        self.timer_instructions.update();

        if poll(Duration::ZERO).expect("can poll terminal events") {
            match event::read().expect("can read events") {
                Event::Key(key) => match (key.kind, key.code) {
                    (KeyEventKind::Press, KeyCode::Char('q')) => self.state = AppState::End,
                    (KeyEventKind::Press, KeyCode::Char('p')) => {
                        self.state = if self.state == AppState::InProgress {
                            AppState::Pause
                        } else {
                            AppState::InProgress
                        }
                    }
                    _ => (),
                },
                _ => (),
            }
        }

        self.chip.advance();
    }

    pub fn state(&self) -> AppState {
        self.state
    }
}

pub struct AppWidget<'a> {
    pub app: &'a App,
}

impl<'a> Widget for AppWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        LayoutLinear {
            children: vec![
                &LayoutAlign {
                    child: &Stat {
                        name: "1".to_string(),
                        value: 1.0,
                        target: 1.0,
                        precision: Some(0),
                    },
                    horizontal: Alignment::Left,
                    vertical: Alignment::Left,
                },
                &LayoutAlign {
                    child: &Stat {
                        name: "2".to_string(),
                        value: 2.0,
                        target: 2.0,
                        precision: Some(5),
                    },
                    horizontal: Alignment::Left,
                    vertical: Alignment::Left,
                },
            ],
            direction: Direction::Horizontal,
            gap: 3,
        }
        .render_sized(area, buf);

        // PixelDisplay {
        //     display: self.app.chip.memory.vram.as_slice(),
        // }
        // .render(area, buf)

        // LayoutAlign {
        //     child: &Stat {
        //         name: "IPS".to_string(),
        //         value: buf.area.width as f64,
        //         target: area.width as f64,
        //         precision: Some(0),
        //     },
        //     vertical: Alignment::Center,
        //     horizontal: Alignment::Center,
        // }
        // .render(area, buf);
    }
}
