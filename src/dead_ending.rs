use std::{path::Path, sync::mpsc::Sender, time::Duration};

use sdl2::{
    event::Event, image::LoadTexture, keyboard::Keycode, pixels::Color, render::WindowCanvas, Sdl,
};

use crate::{audio::Configuration, globals::GROUND_LEVEL, tileset::Tile};

pub fn dead_ending(
    sdl_context: &Sdl,
    canvas: &mut WindowCanvas,
    music_sender: &Sender<Configuration>,
) -> Result<(), String> {
    let mut animation_timer = 0.0;

    music_sender
        .send(Configuration::Play(0.75, "assets/ripbozo.ogg"))
        .map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture(Path::new("assets/tile.png"))?;

    'game_loop: loop {
        let delta_time = 1.0 / 60.0;

        canvas.set_draw_color(Color::RGB(54, 54, 54));
        canvas.clear();

        let offset = animation_timer % 5.0 * 4.0;

        Tile::Cloud0.draw(canvas, &texture, (-3.0 + offset * 1.1, 1.0), (1.0, 1.0))?;
        Tile::Cloud1.draw(canvas, &texture, (-6.0 + offset, 2.0), (1.0, 1.0))?;
        Tile::Cloud2.draw(canvas, &texture, (-8.0 + offset * 0.9, 1.0), (1.0, 1.0))?;
        Tile::Cloud3.draw(canvas, &texture, (-1.0 + offset * 1.2, 2.0), (1.0, 1.0))?;
        Tile::Cross.draw(canvas, &texture, (5.0, GROUND_LEVEL), (1.0, 1.0))?;

        Tile::TreeTrunk.draw(canvas, &texture, (2.0, GROUND_LEVEL), (1.0, 1.0))?;
        Tile::TreeLeaves.draw(canvas, &texture, (2.0, GROUND_LEVEL - 1.0), (1.0, 1.0))?;
        for i in 0..10 {
            Tile::Grass.draw(canvas, &texture, (i as f64, GROUND_LEVEL), (1.0, 1.0))?;
        }
        Tile::Ground.draw(canvas, &texture, (0.0, GROUND_LEVEL + 1.0), (10.0, 1.0))?;
        Tile::Block.draw(
            canvas,
            &texture,
            (0.0, GROUND_LEVEL + 2.0),
            (10.0, 10.0 - GROUND_LEVEL - 2.0),
        )?;

        Tile::LemonSkull.draw(canvas, &texture, (5.0, GROUND_LEVEL + 2.0), (1.0, 1.0))?;

        let angel = if animation_timer % 0.2 > 0.1 {
            Tile::LemonAngel0
        } else {
            Tile::LemonAngel1
        };

        angel.draw(
            canvas,
            &texture,
            (
                4.5,
                GROUND_LEVEL - 1.0 - (animation_timer * 4.0).sin() * 0.15,
            ),
            (1.0, 1.0),
        )?;

        Tile::GameOver.draw(
            canvas,
            &texture,
            (2.0, 1.0 + (animation_timer * 1.2).sin() * 0.1),
            (6.0, 6.0),
        )?;

        canvas.present();
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'game_loop Ok(()),
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
