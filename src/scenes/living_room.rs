use std::path::Path;

use sdl2::rect::Rect;
use sdl2::{image::LoadTexture, render::WindowCanvas};

use crate::actor::Actor;
use crate::globals::{GROUND_LEVEL, PIXEL_PER_DOT};
use crate::helper::{closest_item_within_distance, draw_ground, draw_item, draw_wallpaper};
use crate::state::State;
use crate::tileset::Tile;
use crate::{rect, scene::Scene};

use super::Scenes;

#[derive(Default)]
pub struct LivingRoom {}

enum Interactables {
    ExitDoor,
    Coin7,
    Coin8,
}

impl LivingRoom {
    fn draw_house(
        &self,
        canvas: &mut WindowCanvas,
        state: &State,
        animation_timer: f64,
    ) -> Result<(), String> {
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator.load_texture(Path::new("assets/tile.png"))?;

        draw_wallpaper(canvas, &texture, &Tile::StripeWallpaper)?;

        Tile::DoorOpen.draw(canvas, &texture, (1.0, GROUND_LEVEL), (1.0, 1.0))?;

        Tile::TreeDayPicture.draw(canvas, &texture, (3.0, GROUND_LEVEL), (1.0, 1.0))?;
        Tile::HousePicture.draw(canvas, &texture, (4.0, GROUND_LEVEL), (1.0, 1.0))?;
        Tile::Couch.draw(canvas, &texture, (6.0, GROUND_LEVEL), (1.0, 1.0))?;

        if !state.coin_7 {
            draw_item(canvas, &texture, &Tile::Coin, 3.0, animation_timer)?;
        }
        if !state.coin_8 {
            draw_item(canvas, &texture, &Tile::Coin, 8.0, animation_timer)?;
        }

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
            PIXEL_PER_DOT * 14.0 - (state.confronting_animation_timer * 2.0 * PIXEL_PER_DOT),
            PIXEL_PER_DOT * GROUND_LEVEL,
        );
        dad.run_left();
        dad.draw(canvas, animation_timer)?;
        canvas.copy(
            &bubble,
            rect!(offset, 0, 32, 32),
            rect!(
                PIXEL_PER_DOT * 9.0,
                (GROUND_LEVEL) * PIXEL_PER_DOT,
                PIXEL_PER_DOT,
                PIXEL_PER_DOT
            ),
        )?;

        Ok(())
    }

    fn prepare_items(&self, state: &State) -> Vec<(f64, Interactables)> {
        let mut items = Vec::new();
        if !state.coin_7 {
            items.push(((PIXEL_PER_DOT * 3.0), Interactables::Coin7));
        }
        if !state.coin_8 {
            items.push(((PIXEL_PER_DOT * 8.0), Interactables::Coin8));
        }

        if state.coin_7 && state.coin_8 {
            items.push((PIXEL_PER_DOT, Interactables::ExitDoor));
        }

        items
    }
}

impl Scene for LivingRoom {
    fn draw_scenery(
        &self,
        state: &crate::state::State,
        canvas: &mut sdl2::render::WindowCanvas,
        animation_timer: f64,
    ) -> Result<(), String> {
        canvas.clear();
        self.draw_house(canvas, state, animation_timer)?;
        draw_ground(canvas)?;
        self.draw_confrontation(canvas, state, animation_timer)?;
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
                    state.confronted = true;
                    state.scene_changed = Some((8.0, Scenes::Kitchen));
                }
                Interactables::Coin7 => {
                    state.coin_7 = true;
                    if state.coin_8 {
                        state.change_background_track("assets/run.ogg");
                    };
                }
                Interactables::Coin8 => {
                    state.coin_8 = true;
                    if state.coin_7 {
                        state.change_background_track("assets/run.ogg");
                    };
                }
            }
        }
    }
}
