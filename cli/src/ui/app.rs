use std::{cell::RefCell, time::Duration};

use chip_8::Chip8;
use crossterm::event::{self, poll, Event, KeyCode, KeyEventKind};
use ratatui::{layout::Flex, prelude::*};

use crate::timer::Timer;

use super::{
    pixel_display::PixelDisplay,
    stats::{Stat, StatBias},
    LayoutAlign, LayoutLinear, LayoutSizeError, WidgetSize,
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
    target_instructions: usize,
    timer_frames: RefCell<Timer>,
    target_frames: usize,
}

impl App {
    pub fn new(chip: Chip8, target_instructions: usize, target_frames: usize) -> Self {
        Self {
            chip,
            state: AppState::default(),
            timer_instructions: Timer::new(),
            timer_frames: RefCell::new(Timer::new()),
            target_instructions,
            target_frames,
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
        // TODO(nenikitov): This internal mutability automatically updates the timer since last rendered frame
        // Maybe find a more elegant solution
        self.app.timer_frames.borrow_mut().update();

        let ips = Stat {
            name: "IPS".to_string(),
            value: 1f64 / self.app.timer_instructions.delta().as_secs_f64(),
            target: self.app.target_instructions as f64,
            bias: StatBias::HigherBetter,
            precision: Some(0),
        };
        let ips_secs = Stat {
            name: "sec".to_string(),
            value: self.app.timer_instructions.delta().as_secs_f64(),
            target: 1f64 / self.app.target_instructions as f64,
            bias: StatBias::LowerBetter,
            precision: Some(4),
        };
        let ips_stats = LayoutLinear {
            direction: Direction::Vertical,
            children: vec![(&ips, None), (&ips_secs, None)],
            flex_main_axis: None,
            flex_cross_axis: false,
            spacing: 0,
        };

        let fps = Stat {
            name: "FPS".to_string(),
            value: 1f64 / &self.app.timer_frames.borrow().delta().as_secs_f64(),
            target: self.app.target_frames as f64,
            bias: StatBias::HigherBetter,
            precision: Some(0),
        };
        let fps_secs = Stat {
            name: "sec".to_string(),
            value: self.app.timer_frames.borrow().delta().as_secs_f64(),
            target: 1f64 / self.app.target_frames as f64,
            bias: StatBias::LowerBetter,
            precision: Some(4),
        };
        let fps_stats = LayoutLinear {
            direction: Direction::Vertical,
            children: vec![(&fps, None), (&fps_secs, None)],
            flex_main_axis: None,
            flex_cross_axis: false,
            spacing: 0,
        };

        let stats = LayoutLinear {
            direction: Direction::Horizontal,
            children: vec![(&ips_stats, None), (&fps_stats, None)],
            flex_main_axis: Some(Flex::SpaceBetween),
            flex_cross_axis: false,
            spacing: 0,
        };

        let screen = PixelDisplay {
            display: self.app.chip.memory.vram.as_slice(),
        };

        let emulator = LayoutAlign {
            child: &screen,
            horizontal: Alignment::Center,
            vertical: Alignment::Center,
        };

        LayoutSizeError {
            child: &LayoutLinear {
                direction: Direction::Vertical,
                children: vec![(&stats, None), (&emulator, Some(Constraint::Fill(1)))],
                flex_main_axis: None,
                flex_cross_axis: true,
                spacing: 0,
            },
        }
        .render_sized(area, buf);
    }
}
