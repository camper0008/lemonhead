use sdl2::rect::Rect;
use std::{f64::consts::PI, path::Path, time::Duration};

use sdl2::{
    event::Event, image::LoadTexture, keyboard::Keycode, pixels::Color, render::WindowCanvas, Sdl,
};

use crate::{
    actor::Actor,
    audio::{audio_thread, AudioConfiguration},
    globals::PIXEL_PER_DOT,
    rect,
    tileset::Tile,
};

pub fn main_menu(sdl_context: &Sdl, canvas: &mut WindowCanvas) -> Result<(), String> {
    let mut animation_timer: f64 = 0.0;

    let music_sender = audio_thread();

    for _ in 0..500 {
        music_sender
            .send(AudioConfiguration::Play(1.0, "assets/lemonhead.ogg"))
            .unwrap();
    }

    let texture_creator = canvas.texture_creator();
    let tileset = texture_creator.load_texture(Path::new("assets/tile.png"))?;
    let mut lemonhead = Actor::new("assets/lemonhead.png");
    lemonhead.run_left();
    let mut dad = Actor::new("assets/dad.png");
    dad.run_left();
    let logo = texture_creator.load_texture(Path::new("assets/logo.png"))?;

    'game_loop: loop {
        let delta_time = 1.0 / 60.0;

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        Tile::Ground.draw(canvas, &tileset, (0.0, 9.0), (10.0, 1.0))?;

        let npc_offset = animation_timer.sin() * 6.5 + 4.5;
        let dad_offset = -animation_timer.cos();
        if dad_offset.is_sign_negative() {
            lemonhead.run_right();
            dad.run_right();
        } else {
            lemonhead.run_left();
            dad.run_left();
        }
        lemonhead.set_position((npc_offset) * PIXEL_PER_DOT, 8.0 * PIXEL_PER_DOT);
        dad.set_position(
            (npc_offset + dad_offset) * PIXEL_PER_DOT,
            8.0 * PIXEL_PER_DOT,
        );
        lemonhead.draw(canvas, animation_timer * 2.0);
        dad.draw(canvas, animation_timer * 2.0);

        let offset = (animation_timer * PI * 2.0).sin() * f64::from(PIXEL_PER_DOT) * 0.125;
        canvas.copy(
            &logo,
            rect!(0, 0, 32 * 10, 32 * 10),
            rect!(
                1.0 * (PIXEL_PER_DOT as f64),
                1.0 * (PIXEL_PER_DOT as f64) + offset,
                PIXEL_PER_DOT * 8.0,
                PIXEL_PER_DOT * 8.0
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

        animation_timer += delta_time;
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
