mod app;
mod debug_screen;
mod pixel_display;
mod size_error;
mod stats;
mod widget;

use crossterm::{
    cursor::Show,
    event::{KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::*;
use std::io::{self, stdout, Stdout};

pub use app::*;
pub use widget::*;

pub fn start_ui() -> Result<Terminal<CrosstermBackend<Stdout>>, io::Error> {
    let mut stdout = stdout();

    enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(PushKeyboardEnhancementFlags(
        KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
            | KeyboardEnhancementFlags::REPORT_EVENT_TYPES,
    ))?;

    Terminal::new(CrosstermBackend::new(stdout))
}

pub fn end_ui() -> Result<(), io::Error> {
    let mut stdout = stdout();

    disable_raw_mode()?;
    stdout.execute(LeaveAlternateScreen)?;
    stdout.execute(PopKeyboardEnhancementFlags)?;
    stdout.execute(Show)?;

    Ok(())
}

pub fn panic_hook() {
    let original_hook = std::panic::take_hook();

    std::panic::set_hook(Box::new(move |panic| {
        end_ui().unwrap();
        original_hook(panic);
    }))
}
