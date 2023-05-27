mod actor;
mod globals;
mod helper;
mod scene;
mod scenes;
mod state;

use actor::Actor;
use rodio::{Decoder, OutputStream, Sink};
use scenes::inside_house::InsideHouse;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;

use globals::{GROUND_LEVEL, PIXEL_PER_DOT};
use scene::Scene;
use scenes::outside_house::OutsideHouse;
use sdl2::event::Event;
use sdl2::image::InitFlag;
use sdl2::keyboard::Keycode;
use sdl2::video::Window;
use sdl2::Sdl;
use state::State;

fn prepare_window(sdl_context: &Sdl) -> Result<Window, String> {
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    let window = video_subsystem
        .window(
            "the adventures of lemonhead",
            PIXEL_PER_DOT as u32 * 10,
            PIXEL_PER_DOT as u32 * 10,
        )
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    Ok(window)
}

pub fn run() -> Result<(), String> {
    let mut keys_down: HashMap<Keycode, bool> = Default::default();

    let (audio_sender, audio_reciever): (Sender<&'static str>, Receiver<&'static str>) =
        mpsc::channel();

    thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        loop {
            let Ok(path) = audio_reciever.recv() else {
                break;
            };
            let file = BufReader::new(File::open(path).unwrap());
            let source = Decoder::new(file).unwrap();
            sink.append(source);
        }
    });

    let sdl_context = sdl2::init()?;
    let window = prepare_window(&sdl_context)?;
    let mut animation_timer = 0.0;

    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())?;

    let outside_house = OutsideHouse::default();
    let inside_house = InsideHouse::default();
    let mut scene: &dyn Scene = &outside_house;
    let mut state = State::new(audio_sender);
    let mut lemonhead = Actor::new("assets/lemonhead.png");
    lemonhead.set_position(PIXEL_PER_DOT as f64, (PIXEL_PER_DOT * GROUND_LEVEL).into());

    'mainloop: loop {
        let delta_time = 1.0 / 60.0;
        canvas.clear();
        scene.draw_scenery(&state, &mut canvas, animation_timer)?;
        scene.draw_interact_popup(&state, &mut canvas, lemonhead.x(), animation_timer)?;
        lemonhead.idle();
        if *keys_down.get(&Keycode::A).unwrap_or(&false) {
            lemonhead.offset_position(PIXEL_PER_DOT as f64 * -1.5, 0.0, delta_time);
            lemonhead.run_left();
        }
        if *keys_down.get(&Keycode::D).unwrap_or(&false) {
            lemonhead.offset_position(PIXEL_PER_DOT as f64 * 1.5, 0.0, delta_time);
            lemonhead.run_right();
        }
        if *keys_down.get(&Keycode::Space).unwrap_or(&false) {
            scene.interact(&mut state, lemonhead.x())?;
            keys_down.insert(Keycode::Space, false);
        }

        lemonhead.draw(&mut canvas, animation_timer);
        canvas.present();
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'mainloop,
                Event::KeyDown {
                    keycode: Some(k @ Keycode::A | k @ Keycode::D | k @ Keycode::Space),
                    ..
                } => {
                    keys_down.insert(k, true);
                }
                Event::KeyUp {
                    keycode: Some(k @ Keycode::A | k @ Keycode::D | k @ Keycode::Space),
                    ..
                } => {
                    keys_down.insert(k, false);
                }
                _ => {}
            }
        }

        match state.scene_changed {
            None => (),
            Some((position, ref new_scene)) => {
                match new_scene {
                    scenes::Scenes::InsideHouse => scene = &inside_house,
                    scenes::Scenes::OutsideHouse => scene = &outside_house,
                };
                lemonhead.set_position(
                    PIXEL_PER_DOT as f64 * position,
                    (PIXEL_PER_DOT * GROUND_LEVEL).into(),
                );
                state.scene_changed = None;
            }
        }

        animation_timer += delta_time;
        animation_timer %= 1.0;
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

fn main() -> Result<(), String> {
    run()
}
