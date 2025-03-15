use std::path::Path;

use sdl2::rect::Rect;
use sdl2::{image::LoadTexture, render::WindowCanvas};

use super::{InteractableId, Item, Items, Scene};
use crate::globals::{GROUND_LEVEL, PIXEL_PER_DOT};
use crate::helper::{draw_ground, draw_item, draw_wallpaper};
use crate::logic::Unit;
use crate::rect;
use crate::state::{all_coins_collected, State};
use crate::tileset::Tile;

use super::Scenes;

#[derive(Default)]
pub struct Kitchen;

enum Interactables {
    ExitDoor,
    LivingRoomDoor,
    Weapon,
    Coin0,
    Coin1,
    Coin2,
}

impl Item for Interactables {
    fn id(&self) -> InteractableId {
        match self {
            Interactables::ExitDoor => InteractableId(0),
            Interactables::LivingRoomDoor => InteractableId(1),
            Interactables::Weapon => InteractableId(2),
            Interactables::Coin0 => InteractableId(3),
            Interactables::Coin1 => InteractableId(4),
            Interactables::Coin2 => InteractableId(5),
        }
    }
}

impl From<InteractableId> for Interactables {
    fn from(value: InteractableId) -> Self {
        match value {
            InteractableId(0) => Self::ExitDoor,
            InteractableId(1) => Self::LivingRoomDoor,
            InteractableId(2) => Self::Weapon,
            InteractableId(3) => Self::Coin0,
            InteractableId(4) => Self::Coin1,
            InteractableId(5) => Self::Coin2,
            InteractableId(_) => unreachable!(),
        }
    }
}

impl Kitchen {
    fn draw_house(
        &self,
        canvas: &mut WindowCanvas,
        state: &State,
        animation_timer: f64,
    ) -> Result<(), String> {
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator.load_texture(Path::new("assets/tile.png"))?;
        let blood = texture_creator.load_texture(Path::new("assets/blood.png"))?;

        draw_wallpaper(canvas, &texture, &Tile::KitchenBrick)?;

        Tile::DoorOpen.draw(canvas, &texture, (1.0, GROUND_LEVEL), (1.0, 1.0))?;

        Tile::Oven.draw(canvas, &texture, (3.0, GROUND_LEVEL), (1.0, 1.0))?;

        let picture = if state.murder_living_room.dad_dead {
            Tile::LemonNightPicture
        } else {
            Tile::TreeNightPicture
        };

        picture.draw(canvas, &texture, (9.0, GROUND_LEVEL), (1.0, 1.0))?;

        let living_room_door = if all_coins_collected(&state.kitchen.coins) {
            Tile::DoorOpen
        } else {
            Tile::DoorClosed
        };

        living_room_door.draw(canvas, &texture, (8.0, GROUND_LEVEL), (1.0, 1.0))?;

        if state.murder_living_room.dad_dead {
            canvas.copy(
                &blood,
                rect!(0, 32, 64, 32),
                rect!(
                    PIXEL_PER_DOT * 3.0,
                    GROUND_LEVEL * PIXEL_PER_DOT,
                    PIXEL_PER_DOT * 2.0,
                    PIXEL_PER_DOT
                ),
            )?;

            canvas.copy(
                &blood,
                rect!(64, 0, 32, 32),
                rect!(
                    6.0 * PIXEL_PER_DOT,
                    (GROUND_LEVEL - 1.0) * PIXEL_PER_DOT,
                    PIXEL_PER_DOT,
                    PIXEL_PER_DOT
                ),
            )?;
        }

        if !state.kitchen.coins[0] {
            draw_item(canvas, &texture, &Tile::Coin, 3.0, animation_timer)?;
        }
        if !state.kitchen.coins[1] {
            draw_item(canvas, &texture, &Tile::Coin, 4.0, animation_timer)?;
        }
        if !state.kitchen.coins[2] {
            draw_item(canvas, &texture, &Tile::Coin, 5.0, animation_timer)?;
        }
        if !state.kitchen.weapon_collected {
            draw_item(canvas, &texture, &Tile::Weapon, 6.0, animation_timer)?;
        }

        Ok(())
    }
}

impl Scene for Kitchen {
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

        items.push(Unit::from_units(1), Interactables::ExitDoor);

        if !state.kitchen.coins[0] {
            items.push(Unit::from_units(3), Interactables::Coin0);
        }
        if !state.kitchen.coins[1] {
            items.push(Unit::from_units(4), Interactables::Coin1);
        }
        if !state.kitchen.coins[2] {
            items.push(Unit::from_units(5), Interactables::Coin2);
        }
        if all_coins_collected(&state.kitchen.coins) {
            items.push(Unit::from_units(8), Interactables::LivingRoomDoor);
        }
        if state.living_room.confronted && !state.kitchen.weapon_collected {
            items.push(Unit::from_units(6), Interactables::Weapon);
        }

        items
    }

    fn interact(&self, state: &mut crate::state::State, position: Unit) {
        let Some(closest) = self.closest_item_within_distance(state, position) else {
            return;
        };
        state.send_audio("assets/click.ogg");
        match closest.id().into() {
            Interactables::ExitDoor => {
                if state.living_room.confronted && !state.kitchen.weapon_collected {
                    return;
                }
                state.scene_changed = Some((8, Scenes::Entryway));
            }
            Interactables::Coin0 => state.kitchen.coins[0] = true,
            Interactables::Coin1 => state.kitchen.coins[1] = true,
            Interactables::Coin2 => state.kitchen.coins[2] = true,
            Interactables::Weapon => {
                state.kitchen.weapon_collected = true;
                state.change_background_track("assets/heartbeat.ogg");
            }
            Interactables::LivingRoomDoor => {
                if state.living_room.confronted && !state.kitchen.weapon_collected {
                    return;
                }
                let scene = if state.kitchen.weapon_collected {
                    state.murder_living_room.murderous_intent = true;
                    Scenes::MurderLivingRoom
                } else {
                    Scenes::LivingRoom
                };
                state.scene_changed = Some((1, scene));
            }
        }
    }
}
