use sdl2::{image::LoadTexture, rect::Rect, render::Texture};
use std::{f64::consts::PI, path::Path};

use sdl2::render::WindowCanvas;

use crate::{
    globals::{GROUND_LEVEL, PIXEL_PER_DOT},
    logic::Unit,
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
    texture: &Texture,
    tile: &Tile,
    position: impl Into<Unit>,
    animation_timer: f64,
) -> Result<(), String> {
    let offset = (animation_timer * PI * 2.0).sin() * 0.125;

    tile.draw(
        canvas,
        texture,
        (position, GROUND_LEVEL + Unit::new_decimal(offset)),
        (1, 1),
    )?;

    Ok(())
}

pub fn draw_interact_prompt(
    canvas: &mut WindowCanvas,
    state: &State,
    animation_timer: f64,
) -> Result<(), String> {
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture(Path::new("assets/text.png"))?;

    let offset = (animation_timer * PI * 2.0).sin() * PIXEL_PER_DOT * 0.05;

    let murderous_intent_while_dad_alive =
        state.murder_living_room.murderous_intent && !state.murder_living_room.dad_dead;

    let intent_undecided = state.living_room.confronted && !state.kitchen.weapon_collected;

    let x_size = if murderous_intent_while_dad_alive || intent_undecided {
        2.0
    } else {
        1.0
    };

    let x_offset = if murderous_intent_while_dad_alive || intent_undecided {
        0
    } else if state.murder_living_room.dad_dead && !state.child_room.child_dead()
        || state.child_room.child_stabs >= 3
    {
        1
    } else {
        0
    };

    let y_offset = if intent_undecided {
        1
    } else if murderous_intent_while_dad_alive {
        0
    } else if state.child_room.child_dead() {
        3
    } else {
        2
    };

    let x_position = 5.0 - x_size;

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

    Tile::Ground.draw(canvas, &tileset, (0, GROUND_LEVEL + 1.into()), (10, 1))?;

    Tile::Block.draw(
        canvas,
        &tileset,
        (0, GROUND_LEVEL + 2.into()),
        (10, Unit::new(10) - GROUND_LEVEL - 2.into()),
    )?;

    Ok(())
}

pub fn draw_wallpaper(
    canvas: &mut WindowCanvas,
    texture: &Texture,
    tile: &Tile,
) -> Result<(), String> {
    for x in 0..10 {
        for y in 0..=GROUND_LEVEL.milliunits() / 1000 {
            tile.draw(canvas, texture, (x, y), (1, 1))?;
        }
    }

    Ok(())
}
