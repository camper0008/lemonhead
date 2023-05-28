use std::path::Path;

use sdl2::rect::Rect;
use sdl2::{image::LoadTexture, render::WindowCanvas};

use crate::globals::{GROUND_LEVEL, PIXEL_PER_DOT};
use crate::helper::{closest_item_within_distance, draw_item};
use crate::state::State;
use crate::{rect, scene::Scene};

use super::Scenes;

#[derive(Default)]
pub struct Entryway {}

enum Interactables {
    KitchenDoor,
    ExitDoor,
    ChildDoor,
    Coin0,
    Coin1,
    Coin2,
    Coin3,
}

impl Entryway {
    fn draw_house(&self, canvas: &mut WindowCanvas, state: &State) -> Result<(), String> {
        let texture_creator = canvas.texture_creator();
        let door = texture_creator.load_texture(Path::new("assets/door.png"))?;
        let ground = texture_creator.load_texture(Path::new("assets/ground.png"))?;
        let hint = texture_creator.load_texture(Path::new("assets/blood.png"))?;

        for x in 0..10 {
            for y in 0..=GROUND_LEVEL {
                canvas.copy(
                    &ground,
                    rect!(64, 0, 32, 32),
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
            rect!(32, 64, 32, 32),
            rect!(
                PIXEL_PER_DOT * 7,
                (GROUND_LEVEL) * PIXEL_PER_DOT,
                PIXEL_PER_DOT,
                PIXEL_PER_DOT
            ),
        )?;

        canvas.copy(
            &ground,
            rect!(64, 64, 32, 32),
            rect!(
                PIXEL_PER_DOT * 2,
                (GROUND_LEVEL) * PIXEL_PER_DOT,
                PIXEL_PER_DOT,
                PIXEL_PER_DOT
            ),
        )?;

        let kitchen_door_offset = if state.coin_0 && state.coin_1 && state.coin_2 && state.coin_3 {
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

        let child_door_offset = if state.dad_dead { 32 } else { 0 };

        canvas.copy(
            &door,
            rect!(child_door_offset, 0, 32, 32),
            rect!(
                4 * PIXEL_PER_DOT,
                GROUND_LEVEL * PIXEL_PER_DOT,
                PIXEL_PER_DOT,
                PIXEL_PER_DOT
            ),
        )?;

        if !state.dad_dead {
            canvas.copy(
                &ground,
                rect!(32, 128, 32, 32),
                rect!(
                    4 * PIXEL_PER_DOT,
                    GROUND_LEVEL * PIXEL_PER_DOT,
                    PIXEL_PER_DOT,
                    PIXEL_PER_DOT
                ),
            )?;
        }

        if state.dad_dead && !state.child_dead {
            canvas.copy(
                &hint,
                rect!(32, 0, 32, 32),
                rect!(
                    4 * PIXEL_PER_DOT,
                    (GROUND_LEVEL - 1) * PIXEL_PER_DOT,
                    PIXEL_PER_DOT,
                    PIXEL_PER_DOT
                ),
            )?;
        }

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
        if !state.coin_0 {
            items.push((f64::from(PIXEL_PER_DOT * 3), Interactables::Coin0));
        }
        if !state.coin_1 {
            items.push((f64::from(PIXEL_PER_DOT * 4), Interactables::Coin1));
        }
        if !state.coin_2 {
            items.push((f64::from(PIXEL_PER_DOT * 5), Interactables::Coin2));
        }
        if !state.coin_3 {
            items.push((f64::from(PIXEL_PER_DOT * 6), Interactables::Coin3));
        }
        if state.coin_0 && state.coin_1 && state.coin_2 && state.coin_3 {
            items.push((f64::from(PIXEL_PER_DOT * 8), Interactables::KitchenDoor));
        }

        if state.dad_dead {
            items.push((f64::from(PIXEL_PER_DOT * 4), Interactables::ChildDoor));
        }

        items
    }
}

impl Scene for Entryway {
    fn draw_scenery(
        &self,
        state: &crate::state::State,
        canvas: &mut sdl2::render::WindowCanvas,
        animation_timer: f64,
    ) -> Result<(), String> {
        canvas.clear();
        self.draw_house(canvas, state)?;
        self.draw_ground(canvas)?;
        if !state.coin_0 {
            draw_item(canvas, 3, "assets/coin.png", animation_timer)?;
        }
        if !state.coin_1 {
            draw_item(canvas, 4, "assets/coin.png", animation_timer)?;
        }
        if !state.coin_2 {
            draw_item(canvas, 5, "assets/coin.png", animation_timer)?;
        }
        if !state.coin_3 {
            draw_item(canvas, 6, "assets/coin.png", animation_timer)?;
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
                    if state.confronted && !state.child_dead {
                        return;
                    }
                    state.scene_changed = Some((6.0, Scenes::Outside));
                    if !state.confronted {
                        state.change_background_track("assets/outside.ogg");
                    }
                }
                Interactables::Coin0 => state.coin_0 = true,
                Interactables::Coin1 => state.coin_1 = true,
                Interactables::Coin2 => state.coin_2 = true,
                Interactables::Coin3 => state.coin_3 = true,
                Interactables::ChildDoor => {
                    state.change_background_track("assets/heartbeat-child-with-lemon.ogg");
                    state.scene_changed = Some((1.0, Scenes::ChildRoom));
                }
                Interactables::KitchenDoor => {
                    state.scene_changed = Some((1.0, Scenes::Kitchen));
                }
            }
        }
    }
}
