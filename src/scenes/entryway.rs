use std::path::Path;

use sdl2::rect::Rect;
use sdl2::{image::LoadTexture, render::WindowCanvas};

use super::{InteractableId, Item, Items, Scene};
use crate::globals::{GROUND_LEVEL, PIXEL_PER_DOT};
use crate::helper::{draw_ground, draw_item, draw_wallpaper};
use crate::rect;
use crate::state::{all_coins_collected, State};
use crate::tileset::Tile;

use super::Scenes;

#[derive(Default)]
pub struct Entryway;

enum Interactables {
    KitchenDoor,
    ExitDoor,
    ChildDoor,
    Coin0,
    Coin1,
    Coin2,
    Coin3,
}

impl Item for Interactables {
    fn id(&self) -> InteractableId {
        match self {
            Interactables::KitchenDoor => InteractableId(0),
            Interactables::ExitDoor => InteractableId(1),
            Interactables::ChildDoor => InteractableId(2),
            Interactables::Coin0 => InteractableId(3),
            Interactables::Coin1 => InteractableId(4),
            Interactables::Coin2 => InteractableId(5),
            Interactables::Coin3 => InteractableId(6),
        }
    }
}

impl From<InteractableId> for Interactables {
    fn from(value: InteractableId) -> Self {
        match value {
            InteractableId(0) => Self::KitchenDoor,
            InteractableId(1) => Self::ExitDoor,
            InteractableId(2) => Self::ChildDoor,
            InteractableId(3) => Self::Coin0,
            InteractableId(4) => Self::Coin1,
            InteractableId(5) => Self::Coin2,
            InteractableId(6) => Self::Coin3,
            InteractableId(_) => unreachable!(),
        }
    }
}

impl Entryway {
    fn draw_house(
        &self,
        canvas: &mut WindowCanvas,
        state: &State,
        animation_timer: f64,
    ) -> Result<(), String> {
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator.load_texture(Path::new("assets/tile.png"))?;
        let blood = texture_creator.load_texture(Path::new("assets/blood.png"))?;

        draw_wallpaper(canvas, &texture, &Tile::StripeWallpaper)?;
        Tile::DoorOpen.draw(canvas, &texture, (1.0, GROUND_LEVEL), (1.0, 1.0))?;

        let picture_tile = if state.child_room.child_stabs > 0 {
            Tile::LemonDayPicture
        } else {
            Tile::TreeDayPicture
        };

        picture_tile.draw(canvas, &texture, (7.0, GROUND_LEVEL), (1.0, 1.0))?;

        Tile::HousePicture.draw(canvas, &texture, (2.0, GROUND_LEVEL), (1.0, 1.0))?;

        if state.murder_living_room.dad_dead {
            canvas.copy(
                &blood,
                rect!(0, 0, 32, 32),
                rect!(
                    PIXEL_PER_DOT * 2.0,
                    (GROUND_LEVEL) * PIXEL_PER_DOT,
                    PIXEL_PER_DOT,
                    PIXEL_PER_DOT
                ),
            )?;
        }

        let kitchen_door = if all_coins_collected(&state.entryway.coins) {
            Tile::DoorOpen
        } else {
            Tile::DoorClosed
        };

        kitchen_door.draw(canvas, &texture, (8.0, GROUND_LEVEL), (1.0, 1.0))?;

        let child_door = if state.murder_living_room.dad_dead {
            Tile::DoorOpen
        } else {
            Tile::DoorClosed
        };

        child_door.draw(canvas, &texture, (4.0, GROUND_LEVEL), (1.0, 1.0))?;

        if !state.murder_living_room.dad_dead {
            Tile::ChildSticker.draw(canvas, &texture, (4.0, GROUND_LEVEL), (1.0, 1.0))?;
        }

        if state.murder_living_room.dad_dead && state.child_room.child_stabs == 0 {
            canvas.copy(
                &blood,
                rect!(32, 0, 32, 32),
                rect!(
                    4.0 * PIXEL_PER_DOT,
                    (GROUND_LEVEL - 1.0) * PIXEL_PER_DOT,
                    PIXEL_PER_DOT,
                    PIXEL_PER_DOT
                ),
            )?;
        }

        if !state.entryway.coins[0] {
            draw_item(canvas, &texture, &Tile::Coin, 3.0, animation_timer)?;
        }
        if !state.entryway.coins[1] {
            draw_item(canvas, &texture, &Tile::Coin, 4.0, animation_timer)?;
        }
        if !state.entryway.coins[2] {
            draw_item(canvas, &texture, &Tile::Coin, 5.0, animation_timer)?;
        }
        if !state.entryway.coins[3] {
            draw_item(canvas, &texture, &Tile::Coin, 6.0, animation_timer)?;
        }

        Ok(())
    }
}

impl Scene for Entryway {
    fn draw(
        &self,
        state: &crate::state::State,
        canvas: &mut sdl2::render::WindowCanvas,
        animation_timer: f64,
    ) -> Result<(), String> {
        canvas.clear();
        self.draw_house(canvas, state, animation_timer)?;
        draw_ground(canvas)?;
        Ok(())
    }

    fn prepare_items(&self, state: &State) -> Items {
        let mut items = Items::new();
        items.push(1, Interactables::ExitDoor);
        if !state.entryway.coins[0] {
            items.push(3, Interactables::Coin0);
        }
        if !state.entryway.coins[1] {
            items.push(4, Interactables::Coin1);
        }
        if !state.entryway.coins[2] {
            items.push(5, Interactables::Coin2);
        }
        if !state.entryway.coins[3] {
            items.push(6, Interactables::Coin3);
        }
        if all_coins_collected(&state.entryway.coins) {
            items.push(8, Interactables::KitchenDoor);
        }
        if state.murder_living_room.dad_dead {
            items.push(4, Interactables::ChildDoor);
        }
        items
    }

    fn interact(&self, state: &mut crate::state::State, position: f64) {
        let Some(closest) = self.closest_item_within_distance(state, position) else {
            return;
        };
        state.send_audio("assets/click.ogg");
        match closest.id().into() {
            Interactables::ExitDoor => {
                if state.murder_living_room.dad_dead && !state.child_room.child_stabs > 0 {
                    return;
                }
                state.scene_changed = Some((7, Scenes::Outside));
                if state.child_room.child_stabs == 0 {
                    state.change_background_track("assets/outside.ogg");
                }
            }
            Interactables::Coin0 => state.entryway.coins[0] = true,
            Interactables::Coin1 => state.entryway.coins[1] = true,
            Interactables::Coin2 => state.entryway.coins[2] = true,
            Interactables::Coin3 => state.entryway.coins[3] = true,
            Interactables::ChildDoor => {
                state.change_background_track("assets/heartbeat-child-with-lemon.ogg");
                state.scene_changed = Some((1, Scenes::ChildRoom));
            }
            Interactables::KitchenDoor => {
                state.scene_changed = Some((1, Scenes::Kitchen));
            }
        }
    }
}
