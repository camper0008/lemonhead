use sdl2::{rect::Rect, render::Texture};
use std::{f64::consts::PI, path::Path, time::Duration};

use sdl2::{
    event::Event, image::LoadTexture, keyboard::Keycode, pixels::Color, render::WindowCanvas, Sdl,
};

use crate::{
    audio::{audio_thread, AudioConfiguration},
    globals::PIXEL_PER_DOT,
    rect,
};

fn draw_layer_0(
    canvas: &mut WindowCanvas,
    ground: &Texture,
    animation_timer: f64,
) -> Result<(), String> {
    let animation_timer = animation_timer % 10.0;

    for i in 0..2 {
        canvas.copy(
            &ground,
            rect!(160, 128, 32, 32),
            rect!(
                (-animation_timer + i as f64 * 10.0) * PIXEL_PER_DOT as f64,
                PIXEL_PER_DOT * 1,
                PIXEL_PER_DOT * 10,
                PIXEL_PER_DOT * 10
            ),
        )?;
    }

    Ok(())
}

fn draw_layer_1(
    canvas: &mut WindowCanvas,
    ground: &Texture,
    animation_timer: f64,
) -> Result<(), String> {
    let animation_timer = (animation_timer * 2.5) % 10.0;

    for i in 0..3 {
        canvas.copy(
            &ground,
            rect!(128, 128, 32, 32),
            rect!(
                (-animation_timer + i as f64 * 10.0) * PIXEL_PER_DOT as f64,
                0,
                PIXEL_PER_DOT as f64 * 10.0,
                PIXEL_PER_DOT * 9
            ),
        )?;
    }

    Ok(())
}

fn draw_layer_2(
    canvas: &mut WindowCanvas,
    ground: &Texture,
    animation_timer: f64,
) -> Result<(), String> {
    let animation_timer = (animation_timer * 5.0) % 16.0;

    for i in 0..2 {
        canvas.copy(
            &ground,
            rect!(128, 160, 64, 32),
            rect!(
                (-animation_timer + i as f64 * 16.0) * PIXEL_PER_DOT as f64,
                PIXEL_PER_DOT * 1,
                PIXEL_PER_DOT * 16,
                PIXEL_PER_DOT * 8
            ),
        )?;
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
    let ground = texture_creator.load_texture(Path::new("assets/ground.png"))?;

    'game_loop: loop {
        let delta_time = 1.0 / 60.0;

        //canvas.set_draw_color(Color::RGB(217, 87, 99));
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        draw_layer_0(canvas, &ground, animation_timer)?;
        draw_layer_1(canvas, &ground, animation_timer)?;
        draw_layer_2(canvas, &ground, animation_timer)?;

        let x_offset = (animation_timer * PI * 2.0).sin() * f64::from(PIXEL_PER_DOT) * 0.125;
        let animation_offset = (animation_timer % 0.25 * 4.0).round() * 64.0;

        canvas.copy(
            &ground,
            rect!(animation_offset, 160, 64, 32),
            rect!(
                4.0 * (PIXEL_PER_DOT as f64) + x_offset,
                8 * PIXEL_PER_DOT,
                PIXEL_PER_DOT * 2,
                PIXEL_PER_DOT
            ),
        )?;

        canvas.copy(
            &ground,
            rect!(0, 32, 32, 32),
            rect!(0, PIXEL_PER_DOT * 9, PIXEL_PER_DOT * 10, PIXEL_PER_DOT * 1),
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
