use std::time::Duration;

use chip_8::Chip8;
use crossterm::event::{self, poll, Event, KeyCode, KeyEventKind};
use ratatui::{layout::Flex, prelude::*};

use crate::timer::Timer;

use super::{
    pixel_display::PixelDisplay, stats::Stat, LayoutAlign, LayoutLinear, LayoutSizeError,
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
        LayoutSizeError {
            child: &LayoutLinear {
                direction: Direction::Vertical,
                children: vec![
                    (
                        &LayoutAlign {
                            child: &Stat {
                                name: "1".to_string(),
                                value: 1.0,
                                target: 1.0,
                                precision: Some(0),
                            },
                            horizontal: Alignment::Right,
                            vertical: Alignment::Left,
                        },
                        None,
                    ),
                    (
                        &LayoutAlign {
                            child: &PixelDisplay {
                                display: self.app.chip.memory.vram.as_slice(),
                            },
                            horizontal: Alignment::Center,
                            vertical: Alignment::Center,
                        },
                        Some(Constraint::Fill(1)),
                    ),
                ],
                flex_main_axis: Flex::Start,
                flex_cross_axis: true,
                spacing: 0,
            },
        }
        .render_sized(area, buf);
    }
}
