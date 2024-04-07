#![feature(array_chunks)]
#![feature(iter_array_chunks)]

mod timer;
mod ui;
mod waiter;

use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use chip_8::Chip8;
use ui::AppWidget;
use waiter::Waiter;

const INSTRUCTIONS_PER_SECOND: usize = 10;
const FRAMES_PER_SECOND: usize = 60;

fn main() -> Result<(), i32> {
    let mut chip = Chip8::default();
    chip.load(include_bytes!("../../roms/test_opcode.ch8"));

    let mut terminal = ui::start_ui().map_err(|_| 1)?;
    ui::panic_hook();

    let app = Arc::new(Mutex::new(ui::App::new(
        chip,
        INSTRUCTIONS_PER_SECOND,
        FRAMES_PER_SECOND,
    )));

    let draw_handle = {
        let app_draw = app.clone();
        let mut waiter = Waiter::new(Duration::from_secs_f64(1f64 / FRAMES_PER_SECOND as f64));

        thread::spawn(move || loop {
            waiter.start();

            {
                let app = app_draw.lock().expect("handle on the app in draw loop");
                if app.state() == ui::AppState::End {
                    ui::end_ui().expect("draw end");
                    break;
                }
                terminal
                    .draw(|f| {
                        f.render_widget(AppWidget { app: &app }, f.size());
                    })
                    .expect("draw loop");
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
