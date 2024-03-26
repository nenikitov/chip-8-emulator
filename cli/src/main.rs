mod timer;
mod ui;
mod waiter;

use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use chip_8::Chip8;
use ui::Drawable;
use waiter::Waiter;

const INSTRUCTIONS_PER_SECOND: usize = 200;
const FRAMES_PER_SECOND: usize = 60;

fn main() -> Result<(), i32> {
    let mut chip = Chip8::new();
    chip.load(include_bytes!("../../roms/ibm.ch8"));

    let mut terminal = ui::start_ui().map_err(|_| 1)?;

    let app = Arc::new(Mutex::new(ui::App::new(chip)));

    let draw_handle = {
        let app_draw = app.clone();
        let mut waiter = Waiter::new(Duration::from_secs_f64(1f64 / FRAMES_PER_SECOND as f64));
        thread::spawn(move || loop {
            waiter.start();

            {
                let mut app = app_draw.lock().expect("handle on the app in draw loop");
                app.frames_timer.update();
                if app.state() == ui::AppState::End {
                    ui::end_ui().expect("draw end");
                    break;
                }
                terminal.draw(|f| app.render(f, (0, 0))).expect("draw loop");
            }

            waiter.end();
            waiter.cycle();
        })
    };

    {
        let mut waiter = Waiter::new(Duration::from_secs_f64(
            1f64 / INSTRUCTIONS_PER_SECOND as f64,
        ));

        loop {
            waiter.start();

            {
                let mut app = app.lock().expect("handle on the app in update loop");

                app.update();
                if app.state() == ui::AppState::End {
                    break;
                }
            }

            waiter.end();
            waiter.cycle();
        }
    };

    draw_handle.join().map_err(|_| 2)?;

    Ok(())
}
