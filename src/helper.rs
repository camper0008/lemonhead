use sdl2::{image::LoadTexture, rect::Rect, render::Texture};
use std::{f64::consts::PI, path::Path};

use sdl2::render::WindowCanvas;

use crate::{
    globals::{GROUND_LEVEL, PIXEL_PER_DOT},
    state::State,
    tileset::Tile,
};

#[macro_export]
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

pub fn draw_item(
    canvas: &mut WindowCanvas,
    position: f64,
    path: &'static str,
    animation_timer: f64,
) -> Result<(), String> {
    let texture_creator = canvas.texture_creator();
    let item_texture = texture_creator.load_texture(Path::new(path))?;

    let offset = (animation_timer * PI * 2.0).sin() * f64::from(PIXEL_PER_DOT) * 0.125;

    canvas.copy(
        &item_texture,
        rect!(0, 0, 32, 32),
        rect!(
            position * PIXEL_PER_DOT,
            GROUND_LEVEL * PIXEL_PER_DOT + offset,
            PIXEL_PER_DOT,
            PIXEL_PER_DOT
        ),
    )?;
    Ok(())
}

pub fn closest_item_within_distance<T>(items: Vec<(f64, T)>, position: f64) -> Option<T> {
    if items.is_empty() {
        return None;
    }

    items
        .into_iter()
        .map(|(dist, item)| ((dist - position).abs(), item))
        .filter(|(dist, _)| dist < &f64::from(PIXEL_PER_DOT / 2.0))
        .min_by(|x, y| (x.0).total_cmp(&y.0))
        .map(|(_dist, item)| item)
}

pub fn draw_interact_prompt(
    canvas: &mut WindowCanvas,
    state: &State,
    animation_timer: f64,
) -> Result<(), String> {
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture(Path::new("assets/text.png"))?;

    let offset = (animation_timer * PI * 2.0).sin() * f64::from(PIXEL_PER_DOT) * 0.05;

    let x_size = if state.confronted && !state.dad_dead {
        2.0
    } else {
        1.0
    };

    let x_offset = if state.confronted && !state.dad_dead {
        0
    } else if state.dad_dead && !state.child_dead || state.child_dead && state.child_stabs > 2 {
        1
    } else {
        0
    };

    let y_offset = if state.confronted && !state.weapon_picked_up {
        1
    } else if state.weapon_picked_up && !state.dad_dead {
        0
    } else if state.child_dead {
        3
    } else {
        2
    };

    let x_position = 5.5 - x_size as f64;

    canvas.copy(
        &texture,
        rect!(x_offset * 32, y_offset * 16, x_size * 32.0, 16),
        rect!(
            (x_position * PIXEL_PER_DOT),
            9.0 * PIXEL_PER_DOT + offset,
            PIXEL_PER_DOT * x_size * 2.0,
            PIXEL_PER_DOT
        ),
    )?;
    Ok(())
}

pub fn draw_ground(canvas: &mut WindowCanvas) -> Result<(), String> {
    let texture_creator = canvas.texture_creator();
    let tileset = texture_creator.load_texture(Path::new("assets/tile.png"))?;

    Tile::Ground.draw(canvas, &tileset, (0.0, GROUND_LEVEL + 1.0), (10.0, 1.0))?;

    Tile::Block.draw(
        canvas,
        &tileset,
        (0.0, (GROUND_LEVEL + 2.0)),
        (10.0, (10.0 - GROUND_LEVEL - 2.0)),
    )?;

    Ok(())
}

pub fn draw_wallpaper<'a>(
    canvas: &mut WindowCanvas,
    texture: &Texture<'a>,
    tile: Tile,
) -> Result<(), String> {
    for x in 0..10 {
        for y in 0..=GROUND_LEVEL as usize {
            tile.draw(canvas, texture, (x as f64, y as f64), (1.0, 1.0))?;
        }
    }

    Ok(())
}
