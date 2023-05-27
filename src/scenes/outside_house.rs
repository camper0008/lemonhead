use std::path::Path;

use sdl2::rect::Rect;
use sdl2::{image::LoadTexture, render::WindowCanvas};

use crate::globals::{GROUND_LEVEL, PIXEL_PER_DOT};
use crate::helper::{closest_item_with_distance, draw_interact_prompt, draw_item};
use crate::state::State;
use crate::{rect, scene::Scene};

const HOUSE_OFFSET: i32 = 5;

pub struct OutsideHouse {}

enum Interactables {
    Door,
    Key,
}

impl Default for OutsideHouse {
    fn default() -> Self {
        Self {}
    }
}

impl OutsideHouse {
    fn draw_house(&self, canvas: &mut WindowCanvas, door_open: bool) -> Result<(), String> {
        let texture_creator = canvas.texture_creator();
        let brick_texture = texture_creator.load_texture(Path::new("assets/bricks.png"))?;
        let door_texture = texture_creator.load_texture(Path::new("assets/door.png"))?;
        let roof_texture = texture_creator.load_texture(Path::new("assets/roof.png"))?;

        for x in HOUSE_OFFSET..HOUSE_OFFSET + 3 {
            canvas.copy(
                &brick_texture,
                rect!(0, 0, 32, 32),
                rect!(
                    x * PIXEL_PER_DOT,
                    GROUND_LEVEL * PIXEL_PER_DOT,
                    PIXEL_PER_DOT,
                    PIXEL_PER_DOT
                ),
            )?;
        }

        canvas.copy(
            &roof_texture,
            rect!(0, 0, 96, 32),
            rect!(
                HOUSE_OFFSET * PIXEL_PER_DOT,
                (GROUND_LEVEL - 1) * PIXEL_PER_DOT,
                PIXEL_PER_DOT * 3,
                PIXEL_PER_DOT
            ),
        )?;

        let door_offset = if door_open { 32 } else { 0 };

        canvas.copy(
            &door_texture,
            rect!(door_offset, 0, 32, 32),
            rect!(
                (HOUSE_OFFSET + 1) * PIXEL_PER_DOT,
                GROUND_LEVEL * PIXEL_PER_DOT,
                PIXEL_PER_DOT,
                PIXEL_PER_DOT
            ),
        )?;
        Ok(())
    }

    fn draw_ground(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        let texture_creator = canvas.texture_creator();
        let ground_texture = texture_creator.load_texture(Path::new("assets/ground.png"))?;

        for x in 0..10 {
            canvas.copy(
                &ground_texture,
                rect!(0, 0, 32, 64),
                rect!(
                    x * PIXEL_PER_DOT,
                    GROUND_LEVEL * PIXEL_PER_DOT,
                    PIXEL_PER_DOT,
                    PIXEL_PER_DOT * 2
                ),
            )?;
        }

        for x in 0..10 {
            for y in (GROUND_LEVEL + 2)..10 {
                canvas.copy(
                    &ground_texture,
                    rect!(32, 32, 32, 32),
                    rect!(
                        x * PIXEL_PER_DOT,
                        y * PIXEL_PER_DOT,
                        PIXEL_PER_DOT,
                        PIXEL_PER_DOT
                    ),
                )?;
            }
        }

        Ok(())
    }

    fn prepare_items(&self, state: &State) -> Vec<(f64, Interactables)> {
        let mut items = Vec::new();
        if state.front_door_key_picked_up {
            items.push((
                (PIXEL_PER_DOT * (HOUSE_OFFSET + 1)) as f64,
                Interactables::Door,
            ));
        }
        if !state.front_door_key_picked_up {
            items.push(((PIXEL_PER_DOT * 3) as f64, Interactables::Key));
        }

        items
    }
}

impl Scene for OutsideHouse {
    fn draw_scenery(
        &self,
        state: &crate::state::State,
        canvas: &mut sdl2::render::WindowCanvas,
        animation_timer: f64,
    ) -> Result<(), String> {
        self.draw_house(canvas, state.front_door_opened)?;
        self.draw_ground(canvas)?;
        if !state.front_door_key_picked_up {
            draw_item(canvas, 3, "assets/key.png", animation_timer)?;
        }
        Ok(())
    }

    fn draw_interact_popup(
        &self,
        state: &crate::state::State,
        canvas: &mut sdl2::render::WindowCanvas,
        position: f64,
        animation_timer: f64,
    ) -> Result<(), String> {
        let items = self.prepare_items(state);
        let closest = closest_item_with_distance(items, position);
        if let Some((difference, _)) = closest {
            if difference <= PIXEL_PER_DOT.into() {
                draw_interact_prompt(canvas, animation_timer)?;
            }
        }

        Ok(())
    }

    fn interact(&self, state: &mut crate::state::State, position: f64) -> Result<(), String> {
        let items = self.prepare_items(state);

        let closest = closest_item_with_distance(items, position);
        if let Some((difference, item)) = closest {
            if difference <= PIXEL_PER_DOT.into() {
                state.send_audio("assets/click.ogg");
                match item {
                    Interactables::Key => {
                        state.front_door_key_picked_up = true;
                    }
                    Interactables::Door => {
                        if !state.front_door_opened {
                            state.front_door_opened = true;
                        } else {
                            // teleport the user
                            state.front_door_opened = false;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
