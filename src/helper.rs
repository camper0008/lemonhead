use sdl2::{image::LoadTexture, rect::Rect};
use std::{f64::consts::PI, path::Path};

use sdl2::render::WindowCanvas;

use crate::{
    globals::{GROUND_LEVEL, PIXEL_PER_DOT},
    state::State,
};

#[macro_export]
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

pub fn draw_item(
    canvas: &mut WindowCanvas,
    position: i32,
    path: &'static str,
    animation_timer: f64,
) -> Result<(), String> {
    let texture_creator = canvas.texture_creator();
    let door_texture = texture_creator.load_texture(Path::new(path))?;

    let offset = (animation_timer * PI * 2.0).sin() * f64::from(PIXEL_PER_DOT) * 0.125;

    canvas.copy(
        &door_texture,
        rect!(0, 0, 32, 32),
        rect!(
            position * PIXEL_PER_DOT,
            GROUND_LEVEL * PIXEL_PER_DOT + offset as i32,
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
        .filter(|(dist, _)| dist < &f64::from(PIXEL_PER_DOT / 2))
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

    let x_offset = if state.dad_dead && !state.child_dead {
        32
    } else {
        0
    };

    let (x_size, x_position, y_offset) = if state.confronted && !state.dad_dead {
        (2, 3.5, 0)
    } else {
        (1, 4.5, 16)
    };

    canvas.copy(
        &texture,
        rect!(x_offset, y_offset, x_size * 32, 16),
        rect!(
            (x_position * PIXEL_PER_DOT as f64),
            9 * PIXEL_PER_DOT + offset as i32,
            PIXEL_PER_DOT * x_size * 2,
            PIXEL_PER_DOT
        ),
    )?;
    Ok(())
}
