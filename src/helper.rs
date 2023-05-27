use sdl2::{image::LoadTexture, rect::Rect};
use std::{f64::consts::PI, path::Path};

use sdl2::render::WindowCanvas;

use crate::globals::{GROUND_LEVEL, PIXEL_PER_DOT};

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

    let offset = (animation_timer * PI * 2.0).sin() * PIXEL_PER_DOT as f64 * 0.125;

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

pub fn closest_item_with_distance<T>(items: Vec<(f64, T)>, position: f64) -> Option<(f64, T)> {
    if items.len() == 0 {
        return None;
    }

    items
        .into_iter()
        .map(|(dist, item)| ((dist - position).abs(), item))
        .min_by(|x, y| (x.0).total_cmp(&y.0))
}

pub fn draw_interact_prompt(canvas: &mut WindowCanvas, animation_timer: f64) -> Result<(), String> {
    let texture_creator = canvas.texture_creator();
    let door_texture = texture_creator.load_texture(Path::new("assets/space.png"))?;

    let offset = (animation_timer * PI * 2.0).sin() * PIXEL_PER_DOT as f64 * 0.05;

    canvas.copy(
        &door_texture,
        rect!(0, 0, 32, 16),
        rect!(
            (4 * PIXEL_PER_DOT),
            9 * PIXEL_PER_DOT + offset as i32,
            PIXEL_PER_DOT * 2,
            PIXEL_PER_DOT
        ),
    )?;
    Ok(())
}
