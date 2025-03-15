use std::path::Path;

use sdl2::rect::Rect;
use sdl2::{image::LoadTexture, render::WindowCanvas};

use crate::actor::Actor;
use crate::globals::{GROUND_LEVEL, PIXEL_PER_DOT};
use crate::helper::{draw_ground, draw_item, draw_wallpaper};
use crate::scene::{Id, Item, Items};
use crate::state::State;
use crate::tileset::Tile;
use crate::{rect, scene::Scene};

use super::Scenes;

#[derive(Default)]
pub struct LivingRoom;

enum Interactables {
    ExitDoor,
    Coin0,
    Coin1,
}

impl Item for Interactables {
    fn id(&self) -> Id {
        match self {
            Self::ExitDoor => Id(0),
            Self::Coin0 => Id(1),
            Self::Coin1 => Id(2),
        }
    }
}

impl From<Id> for Interactables {
    fn from(value: Id) -> Self {
        match value {
            Id(0) => Self::ExitDoor,
            Id(1) => Self::Coin0,
            Id(2) => Self::Coin1,
            Id(_) => unreachable!(),
        }
    }
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

        if !state.living_room.coins[0] {
            draw_item(canvas, &texture, &Tile::Coin, 3.0, animation_timer)?;
        }
        if !state.living_room.coins[1] {
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
        if !(state.living_room.coins.iter().all(|v| *v)) {
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
}

impl Scene for LivingRoom {
    fn draw(
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

    fn interact(&self, state: &mut crate::state::State, position: f64) {
        let Some(closest) = self.closest_item_within_distance(state, position) else {
            return;
        };
        state.send_audio("assets/click.ogg");
        match closest.id().into() {
            Interactables::ExitDoor => {
                state.living_room.confronted = true;
                state.scene_changed = Some((8, Scenes::Kitchen));
            }
            Interactables::Coin0 => {
                state.living_room.coins[0] = true;
                if state.living_room.coins[1] {
                    state.change_background_track("assets/run.ogg");
                };
            }
            Interactables::Coin1 => {
                state.living_room.coins[1] = true;
                if state.living_room.coins[0] {
                    state.change_background_track("assets/run.ogg");
                };
            }
        }
    }

    fn prepare_items(&self, state: &State) -> Items {
        let mut items = Items::new();
        if !state.living_room.coins[0] {
            items.push(3, Interactables::Coin0);
        }
        if !state.living_room.coins[1] {
            items.push(8, Interactables::Coin1);
        }

        if state.living_room.coins.iter().all(|v| !v) {
            items.push(1, Interactables::ExitDoor);
        }

        items
    }
}
