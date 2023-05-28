use std::path::Path;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::{image::LoadTexture, render::WindowCanvas};

use crate::actor::Actor;
use crate::globals::{GROUND_LEVEL, PIXEL_PER_DOT};
use crate::helper::{closest_item_within_distance, draw_item};
use crate::state::State;
use crate::{rect, scene::Scene};

use super::Scenes;

#[derive(Default)]
pub struct MurderLivingRoom {}

enum Interactables {
    ExitDoor,
    Coin7,
    Coin8,
}

impl MurderLivingRoom {
    fn draw_house(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        let texture_creator = canvas.texture_creator();
        let door = texture_creator.load_texture(Path::new("assets/door.png"))?;
        let ground = texture_creator.load_texture(Path::new("assets/ground.png"))?;

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
                PIXEL_PER_DOT * 3,
                (GROUND_LEVEL) * PIXEL_PER_DOT,
                PIXEL_PER_DOT,
                PIXEL_PER_DOT
            ),
        )?;

        canvas.copy(
            &ground,
            rect!(64, 64, 32, 32),
            rect!(
                PIXEL_PER_DOT * 4,
                (GROUND_LEVEL) * PIXEL_PER_DOT,
                PIXEL_PER_DOT,
                PIXEL_PER_DOT
            ),
        )?;

        canvas.copy(
            &ground,
            rect!(64, 96, 32, 32),
            rect!(
                PIXEL_PER_DOT * 6,
                (GROUND_LEVEL) * PIXEL_PER_DOT,
                PIXEL_PER_DOT,
                PIXEL_PER_DOT
            ),
        )?;

        Ok(())
    }

    fn draw_confrontation(
        &self,
        canvas: &mut WindowCanvas,
        state: &State,
        animation_timer: f64,
    ) -> Result<(), String> {
        if !(state.coin_7 && state.coin_8) {
            return Ok(());
        }
        let texture_creator = canvas.texture_creator();
        let bubble = texture_creator.load_texture(Path::new("assets/bubble.png"))?;
        let offset = (state.confronting_animation_timer * 8.0).round() * 32.0;

        let mut dad = Actor::new("assets/dad.png");
        dad.set_position(
            PIXEL_PER_DOT as f64 * 14.0
                - (state.confronting_animation_timer * 2.0 * PIXEL_PER_DOT as f64),
            (PIXEL_PER_DOT * GROUND_LEVEL).into(),
        );
        dad.run_left();
        dad.draw(canvas, animation_timer);
        canvas.copy(
            &bubble,
            rect!(offset, 0, 32, 32),
            rect!(
                PIXEL_PER_DOT * 9,
                (GROUND_LEVEL) * PIXEL_PER_DOT,
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
        if !state.coin_7 {
            items.push((f64::from(PIXEL_PER_DOT * 3), Interactables::Coin7));
        }
        if !state.coin_8 {
            items.push((f64::from(PIXEL_PER_DOT * 8), Interactables::Coin8));
        }

        if state.coin_7 && state.coin_8 {
            items.push((f64::from(PIXEL_PER_DOT), Interactables::ExitDoor));
        }

        items
    }
}

impl Scene for MurderLivingRoom {
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
        self.draw_confrontation(canvas, state, animation_timer)?;
        if !state.coin_7 {
            draw_item(canvas, 3, "assets/coin.png", animation_timer)?;
        }
        if !state.coin_8 {
            draw_item(canvas, 8, "assets/coin.png", animation_timer)?;
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
                    state.scene_changed = Some((8.0, Scenes::Kitchen));
                }
                Interactables::Coin7 => state.coin_7 = true,
                Interactables::Coin8 => state.coin_8 = true,
            }
        }
    }
}
