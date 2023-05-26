mod actor;
mod helper;

use std::collections::HashMap;

use sdl2::event::Event;
use sdl2::image::InitFlag;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

pub fn run() -> Result<(), String> {
    let mut keys_down: HashMap<Keycode, bool> = Default::default();
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    let window = video_subsystem
        .window("rust-sdl2 demo: Video", 640, 640)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())?;

    let mut lemonhead = actor::Actor::new("assets/lemonhead.png");
    lemonhead.offset_position(320, 32 * 9 * 4);
    'mainloop: loop {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        lemonhead.idle();
        if *keys_down.get(&Keycode::A).unwrap_or(&false) {
            lemonhead.offset_position(-1, 0);
            lemonhead.run_left();
        }
        if *keys_down.get(&Keycode::D).unwrap_or(&false) {
            lemonhead.offset_position(1, 0);
            lemonhead.run_right();
        }

        lemonhead.present(&mut canvas);
        canvas.present();
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'mainloop,
                Event::KeyDown {
                    keycode: Some(k @ Keycode::A | k @ Keycode::D),
                    ..
                } => {
                    keys_down.insert(k, true);
                }
                Event::KeyUp {
                    keycode: Some(k @ Keycode::A | k @ Keycode::D),
                    ..
                } => {
                    keys_down.insert(k, false);
                }
                _ => {}
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), String> {
    run()?;

    Ok(())
}
