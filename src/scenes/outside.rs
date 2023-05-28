use std::path::Path;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::{image::LoadTexture, render::WindowCanvas};

use crate::globals::{GROUND_LEVEL, PIXEL_PER_DOT};
use crate::helper::{closest_item_within_distance, draw_item};
use crate::state::State;
use crate::{rect, scene::Scene};

use super::Scenes;

const HOUSE_OFFSET: i32 = 5;

#[derive(Default)]
pub struct Outside {}

enum Interactables {
    Door,
    Key,
}

impl Outside {
    fn draw_house(&self, canvas: &mut WindowCanvas, door_open: bool) -> Result<(), String> {
        let texture_creator = canvas.texture_creator();
        let door = texture_creator.load_texture(Path::new("assets/door.png"))?;
        let ground = texture_creator.load_texture(Path::new("assets/ground.png"))?;

        for x in HOUSE_OFFSET..HOUSE_OFFSET + 3 {
            canvas.copy(
                &ground,
                rect!(32, 0, 32, 32),
                rect!(
                    x * PIXEL_PER_DOT,
                    GROUND_LEVEL * PIXEL_PER_DOT,
                    PIXEL_PER_DOT,
                    PIXEL_PER_DOT
                ),
            )?;
        }

        canvas.copy(
            &ground,
            rect!(0, 96, 32, 32),
            rect!(PIXEL_PER_DOT, PIXEL_PER_DOT, PIXEL_PER_DOT, PIXEL_PER_DOT),
        )?;

        canvas.copy(
            &ground,
            rect!(96, 0, 32, 32),
            rect!(
                HOUSE_OFFSET * PIXEL_PER_DOT,
                (GROUND_LEVEL - 1) * PIXEL_PER_DOT,
                PIXEL_PER_DOT,
                PIXEL_PER_DOT
            ),
        )?;

        canvas.copy(
            &ground,
            rect!(32, 32, 32, 32),
            rect!(
                (HOUSE_OFFSET + 1) * PIXEL_PER_DOT,
                (GROUND_LEVEL - 1) * PIXEL_PER_DOT,
                PIXEL_PER_DOT,
                PIXEL_PER_DOT
            ),
        )?;

        canvas.copy(
            &ground,
            rect!(96, 32, 32, 32),
            rect!(
                (HOUSE_OFFSET + 2) * PIXEL_PER_DOT,
                (GROUND_LEVEL - 1) * PIXEL_PER_DOT,
                PIXEL_PER_DOT,
                PIXEL_PER_DOT
            ),
        )?;

        let door_offset = if door_open { 32 } else { 0 };

        canvas.copy(
            &door,
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
                f64::from(PIXEL_PER_DOT * (HOUSE_OFFSET + 1)),
                Interactables::Door,
            ));
        }
        if !state.front_door_key_picked_up {
            items.push((f64::from(PIXEL_PER_DOT * 3), Interactables::Key));
        }

        items
    }
}

impl Scene for Outside {
    fn draw_scenery(
        &self,
        state: &crate::state::State,
        canvas: &mut sdl2::render::WindowCanvas,
        animation_timer: f64,
    ) -> Result<(), String> {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        self.draw_house(canvas, state.front_door_opened)?;
        self.draw_ground(canvas)?;
        if !state.front_door_key_picked_up {
            draw_item(canvas, 3, "assets/key.png", animation_timer)?;
        }
        Ok(())
    }

    fn should_draw_interact_popup(&self, state: &crate::state::State, position: f64) -> bool {
        let items = self.prepare_items(state);
        let closest = closest_item_within_distance(items, position);
        closest.is_some()
    }

    fn interact(&self, state: &mut crate::state::State, position: f64) {
        let items = self.prepare_items(state);

        let closest = closest_item_within_distance(items, position);
        if let Some(item) = closest {
            state.send_audio("assets/click.ogg");
            match item {
                Interactables::Key => state.front_door_key_picked_up = true,
                Interactables::Door => {
                    if state.front_door_opened {
                        state.scene_changed = Some((1.0, Scenes::Entryway));

                        if !(state.confronted) {
                            state.change_background_track("assets/lemonhead.ogg");
                        }
                    } else {
                        state.front_door_opened = true;
                    }
                }
            }
        }
    }
}
