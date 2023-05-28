use std::path::Path;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::{image::LoadTexture, render::WindowCanvas};

use crate::globals::{GROUND_LEVEL, PIXEL_PER_DOT};
use crate::helper::{closest_item_within_distance, draw_item};
use crate::state::State;
use crate::{rect, scene::Scene};

use super::Scenes;

#[derive(Default)]
pub struct Kitchen {}

enum Interactables {
    ExitDoor,
    LivingRoomDoor,
    Weapon,
    Coin4,
    Coin5,
    Coin6,
}

impl Kitchen {
    fn draw_house(&self, canvas: &mut WindowCanvas, state: &State) -> Result<(), String> {
        let texture_creator = canvas.texture_creator();
        let door = texture_creator.load_texture(Path::new("assets/door.png"))?;
        let ground = texture_creator.load_texture(Path::new("assets/ground.png"))?;

        for x in 0..10 {
            for y in 0..=GROUND_LEVEL {
                canvas.copy(
                    &ground,
                    rect!(0, 64, 32, 32),
                    rect!(
                        x * PIXEL_PER_DOT,
                        y * PIXEL_PER_DOT,
                        PIXEL_PER_DOT,
                        PIXEL_PER_DOT
                    ),
                )?;
            }
        }

        canvas.copy(
            &door,
            rect!(32, 0, 32, 32),
            rect!(
                PIXEL_PER_DOT,
                GROUND_LEVEL * PIXEL_PER_DOT,
                PIXEL_PER_DOT,
                PIXEL_PER_DOT
            ),
        )?;

        canvas.copy(
            &ground,
            rect!(32, 96, 32, 32),
            rect!(
                PIXEL_PER_DOT * 3,
                (GROUND_LEVEL) * PIXEL_PER_DOT,
                PIXEL_PER_DOT,
                PIXEL_PER_DOT
            ),
        )?;

        canvas.copy(
            &ground,
            rect!(96, 64, 32, 32),
            rect!(
                PIXEL_PER_DOT * 9,
                (GROUND_LEVEL) * PIXEL_PER_DOT,
                PIXEL_PER_DOT,
                PIXEL_PER_DOT
            ),
        )?;

        let kitchen_door_offset = if state.coin_4 && state.coin_5 && state.coin_6 {
            32
        } else {
            0
        };
        canvas.copy(
            &door,
            rect!(kitchen_door_offset, 0, 32, 32),
            rect!(
                8 * PIXEL_PER_DOT,
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

        items.push((f64::from(PIXEL_PER_DOT), Interactables::ExitDoor));
        if !state.coin_4 {
            items.push((f64::from(PIXEL_PER_DOT * 3), Interactables::Coin4));
        }
        if !state.coin_5 {
            items.push((f64::from(PIXEL_PER_DOT * 4), Interactables::Coin5));
        }
        if !state.coin_6 {
            items.push((f64::from(PIXEL_PER_DOT * 5), Interactables::Coin6));
        }

        if state.coin_4 && state.coin_5 && state.coin_6 {
            items.push((f64::from(PIXEL_PER_DOT * 8), Interactables::LivingRoomDoor));
        }

        if state.confronted && !state.weapon_picked_up {
            items.push((f64::from(PIXEL_PER_DOT * 6), Interactables::Weapon));
        }

        items
    }
}

impl Scene for Kitchen {
    fn draw_scenery(
        &self,
        state: &crate::state::State,
        canvas: &mut sdl2::render::WindowCanvas,
        animation_timer: f64,
    ) -> Result<(), String> {
        canvas.set_draw_color(Color::RGB(200, 200, 200));
        canvas.clear();
        self.draw_house(canvas, state)?;
        self.draw_ground(canvas)?;
        if !state.coin_4 {
            draw_item(canvas, 3, "assets/coin.png", animation_timer)?;
        }
        if !state.coin_5 {
            draw_item(canvas, 4, "assets/coin.png", animation_timer)?;
        }
        if !state.coin_6 {
            draw_item(canvas, 5, "assets/coin.png", animation_timer)?;
        }
        if !state.weapon_picked_up {
            draw_item(canvas, 6, "assets/weapon.png", animation_timer)?;
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
                Interactables::ExitDoor => {
                    if state.confronted && !state.dad_dead {
                        return;
                    }
                    state.scene_changed = Some((8.0, Scenes::Entryway));
                }
                Interactables::Coin4 => state.coin_4 = true,
                Interactables::Coin5 => state.coin_5 = true,
                Interactables::Coin6 => state.coin_6 = true,
                Interactables::Weapon => state.weapon_picked_up = true,
                Interactables::LivingRoomDoor => {
                    if state.confronted && !state.weapon_picked_up {
                        return;
                    }

                    let scene = if state.weapon_picked_up {
                        Scenes::MurderLivingRoom
                    } else {
                        Scenes::LivingRoom
                    };
                    state.scene_changed = Some((1.0, scene));
                }
            }
        }
    }
}
