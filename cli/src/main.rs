mod ui;

use std::io;

use chip_8::Chip8;
use tui::backend::CrosstermBackend;
use ui::Drawable;

fn main() -> Result<(), i32> {
    let mut terminal = if let Ok(terminal) = ui::start_ui(CrosstermBackend::new(io::stdout())) {
        terminal
    } else {
        eprintln!("{}", "Can't initialize TUI session");
        return Err(1);
    };

    let mut chip = Chip8::new();
    chip.load(include_bytes!("../../roms/ibm.ch8"));

    let mut app = ui::App::new(chip);

    loop {
        terminal.draw(|f| app.render(f)).unwrap();
        app.update();

        match app.state() {
            ui::AppState::InProgress =>
                continue,
            ui::AppState::End => {
                ui::end_ui(terminal).unwrap();
                return Ok(())
            },
        }
    }
}
