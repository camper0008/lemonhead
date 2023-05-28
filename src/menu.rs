use rodio::{Decoder, Sink};
use sdl2::rect::Rect;
use std::{f64::consts::PI, fs::File, io::BufReader, path::Path, time::Duration};

use sdl2::{
    event::Event, image::LoadTexture, keyboard::Keycode, pixels::Color, render::WindowCanvas, Sdl,
};

use crate::{globals::PIXEL_PER_DOT, rect};

pub fn main_menu(sdl_context: &Sdl, canvas: &mut WindowCanvas) -> Result<(), String> {
    let mut animation_timer = 0.0;

    let Ok((_stream, stream_handle)) = rodio::OutputStream::try_default() else {
            return Err("unable to open audio channel".to_owned());
        };
    let sink = Sink::try_new(&stream_handle).map_err(|e| e.to_string())?;

    'game_loop: loop {
        let delta_time = 1.0 / 60.0;

        let offset = 2.0 * (animation_timer - 0.5f64).abs();
        canvas.set_draw_color(Color::RGB(
            217 - (offset * 0.0) as u8,
            87 - (offset * 20.0) as u8,
            99 - (offset * 20.0) as u8,
        ));
        canvas.clear();
        let texture_creator = canvas.texture_creator();
        let logo = texture_creator.load_texture(Path::new("assets/logo.png"))?;

        let offset = (animation_timer * PI * 2.0).sin() * f64::from(PIXEL_PER_DOT) * 0.125;

        canvas.copy(
            &logo,
            rect!(0, 0, 32 * 10, 32 * 10),
            rect!(
                1.0 * (PIXEL_PER_DOT as f64),
                1.0 * (PIXEL_PER_DOT as f64) + offset,
                PIXEL_PER_DOT * 8,
                PIXEL_PER_DOT * 8
            ),
        )?;

        canvas.present();
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'game_loop Err("user quit".to_string()),
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => break 'game_loop Ok(()),
                _ => (),
            }
        }

        let file = BufReader::new(File::open("assets/lemonhead.ogg").map_err(|e| e.to_string())?);
        let source = Decoder::new(file).map_err(|e| e.to_string())?;
        sink.append(source);

        animation_timer += delta_time;
        animation_timer %= 1.0;
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
