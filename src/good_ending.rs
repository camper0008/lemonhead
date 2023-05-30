use sdl2::render::Texture;
use std::{f64::consts::PI, path::Path, time::Duration};

use sdl2::{
    event::Event, image::LoadTexture, keyboard::Keycode, pixels::Color, render::WindowCanvas, Sdl,
};

use crate::{
    audio::{audio_thread, AudioConfiguration},
    tileset::Tile,
};

fn draw_layer_0(
    canvas: &mut WindowCanvas,
    texture: &Texture,
    animation_timer: f64,
) -> Result<(), String> {
    let animation_timer = animation_timer % 10.0;

    for i in 0..2 {
        let position = -animation_timer + (i * 10) as f64;
        Tile::CityLayer0.draw(canvas, texture, (position, 1.0), (10.0, 9.0))?;
    }

    Ok(())
}

fn draw_layer_1(
    canvas: &mut WindowCanvas,
    texture: &Texture,
    animation_timer: f64,
) -> Result<(), String> {
    let animation_timer = (animation_timer * 2.5) % 10.0;

    for i in 0..2 {
        let position = -animation_timer + (i * 10) as f64;
        Tile::CityLayer1.draw(canvas, texture, (position, 1.0), (10.0, 9.0))?;
    }

    Ok(())
}

fn draw_layer_2(
    canvas: &mut WindowCanvas,
    texture: &Texture,
    animation_timer: f64,
) -> Result<(), String> {
    let animation_timer = animation_timer * 5.0 % 16.0;

    for i in 0..3 {
        let position = -animation_timer + (i * 16) as f64;
        Tile::CityLayer2.draw(canvas, texture, (position, 1.0), (16.0, 8.0))?;
    }

    Ok(())
}

pub fn good_ending(sdl_context: &Sdl, canvas: &mut WindowCanvas) -> Result<(), String> {
    let mut animation_timer = 0.0;

    let music_sender = audio_thread();

    for _ in 0..500 {
        music_sender
            .send(AudioConfiguration::Play(1.0, "assets/rich.ogg"))
            .unwrap();
    }

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture(Path::new("assets/tile.png"))?;

    'game_loop: loop {
        let delta_time = 1.0 / 60.0;

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        draw_layer_0(canvas, &texture, animation_timer)?;
        draw_layer_1(canvas, &texture, animation_timer)?;
        draw_layer_2(canvas, &texture, animation_timer)?;

        let x_offset = (animation_timer % 1.0 * PI * 2.0).sin() * 0.125;
        let car = if animation_timer % 0.2 < 0.1 {
            Tile::LemonCar0
        } else {
            Tile::LemonCar1
        };

        car.draw(canvas, &texture, (4.0 + x_offset, 8.0), (2.0, 1.0))?;

        Tile::Ground.draw(canvas, &texture, (0.0, 9.0), (10.0, 1.0))?;

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
