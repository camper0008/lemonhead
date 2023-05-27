use std::path::Path;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::{image::LoadTexture, render::WindowCanvas};

use crate::globals::{GROUND_LEVEL, PIXEL_PER_DOT};
use crate::helper::{closest_item_with_distance, draw_interact_prompt, draw_item};
use crate::state::State;
use crate::{rect, scene::Scene};

use super::Scenes;

pub struct InsideHouse {}

enum Interactables {
    Door,
    Weapon,
}

impl Default for InsideHouse {
    fn default() -> Self {
        Self {}
    }
}

impl InsideHouse {
    fn draw_house(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        let texture_creator = canvas.texture_creator();
        let door_texture = texture_creator.load_texture(Path::new("assets/door.png"))?;

        canvas.copy(
            &door_texture,
            rect!(32, 0, 32, 32),
            rect!(
                1 * PIXEL_PER_DOT,
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
                rect!(0, 32, 32, 32),
                rect!(
                    x * PIXEL_PER_DOT,
                    (GROUND_LEVEL + 1) * PIXEL_PER_DOT,
                    PIXEL_PER_DOT,
                    PIXEL_PER_DOT
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
        items.push(((PIXEL_PER_DOT * 1) as f64, Interactables::Door));
        if !state.weapon_picked_up {
            items.push(((PIXEL_PER_DOT * 9) as f64, Interactables::Weapon));
        }

        items
    }
}

impl Scene for InsideHouse {
    fn draw_scenery(
        &self,
        state: &crate::state::State,
        canvas: &mut sdl2::render::WindowCanvas,
        animation_timer: f64,
    ) -> Result<(), String> {
        canvas.set_draw_color(Color::RGB(200, 200, 200));
        canvas.clear();
        self.draw_house(canvas)?;
        self.draw_ground(canvas)?;
        if !state.front_door_key_picked_up {
            draw_item(canvas, 9, "assets/weapon.png", animation_timer)?;
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
                    Interactables::Door => state.scene_changed = Some((5.0, Scenes::OutsideHouse)),
                    Interactables::Weapon => {}
                }
            }
        }
        Ok(())
    }
}
