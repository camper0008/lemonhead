use super::{InteractableId, Item, Items, Scene};
use crate::ctx::{Effect, Music};
use crate::helper::CtxHelperExt;
use crate::sprite::Blood;
use crate::{ctx::Ctx, globals::GROUND_LEVEL, sprite::Tile, state::State};

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
    fn draw_house<C: Ctx>(&self, ctx: &mut C, state: &State<C>) {
        ctx.draw_ground();
        ctx.draw_wallpaper(&Tile::KitchenBrick);
        ctx.draw_sprite((1.0, GROUND_LEVEL), (1.0, 1.0), &Tile::DoorOpen);
        ctx.draw_sprite((3.0, GROUND_LEVEL), (1.0, 1.0), &Tile::Oven);

        let picture = if state.murder_living_room.dad_dead {
            Tile::LemonNightPicture
        } else {
            Tile::TreeNightPicture
        };

        ctx.draw_sprite((9.0, GROUND_LEVEL), (1.0, 1.0), &picture);

        let living_room_door = if state.kitchen.all_coins_collected() {
            Tile::DoorOpen
        } else {
            Tile::DoorClosed
        };

        ctx.draw_sprite((8.0, GROUND_LEVEL), (1.0, 1.0), &living_room_door);

        if state.murder_living_room.dad_dead {
            ctx.draw_sprite((3.0, GROUND_LEVEL), (1.0, 1.0), &Blood::SplatterRight);
            ctx.draw_sprite((4.0, GROUND_LEVEL), (1.0, 1.0), &Blood::SplatterLeft);

            ctx.draw_sprite((6.0, GROUND_LEVEL - 1.0), (1.0, 1.0), &Blood::PraiseLemon);
        }

        if !state.kitchen.coins[0] {
            ctx.draw_item(&Tile::Coin, 3.0);
        }
        if !state.kitchen.coins[1] {
            ctx.draw_item(&Tile::Coin, 4.0);
        }
        if !state.kitchen.coins[2] {
            ctx.draw_item(&Tile::Coin, 5.0);
        }
        if !state.kitchen.weapon_collected {
            ctx.draw_item(&Tile::Weapon, 6.0);
        }
    }
}

impl<C: Ctx> Scene<C> for Kitchen {
    fn draw(&self, ctx: &mut C, state: &State<C>) {
        self.draw_house(ctx, state);
    }

    fn prepare_items(&self, state: &State<C>) -> Items {
        let mut items = Items::new();

        items.push(1.0, Interactables::ExitDoor);

        if !state.kitchen.coins[0] {
            items.push(3.0, Interactables::Coin0);
        }
        if !state.kitchen.coins[1] {
            items.push(4.0, Interactables::Coin1);
        }
        if !state.kitchen.coins[2] {
            items.push(5.0, Interactables::Coin2);
        }
        if state.kitchen.all_coins_collected() {
            items.push(8.0, Interactables::LivingRoomDoor);
        }
        if state.living_room.has_escaped_dad && !state.kitchen.weapon_collected {
            items.push(6.0, Interactables::Weapon);
        }

        items
    }

    fn interact(&self, ctx: &mut C, state: &mut State<C>, position: f64) -> Result<(), C::Error> {
        let Some(closest) = self.closest_item_within_distance(state, position) else {
            return Ok(());
        };
        ctx.play_effect(Effect::Interact)?;
        match closest.id().into() {
            Interactables::ExitDoor => {
                if state.living_room.has_escaped_dad && !state.kitchen.weapon_collected {
                    return Ok(());
                }
                state.scene_changed = Some((8.0, Scenes::Entryway));
            }
            Interactables::Coin0 => state.kitchen.coins[0] = true,
            Interactables::Coin1 => state.kitchen.coins[1] = true,
            Interactables::Coin2 => state.kitchen.coins[2] = true,
            Interactables::Weapon => {
                state.kitchen.weapon_collected = true;
                ctx.set_music(Music::Heartbeat)?;
            }
            Interactables::LivingRoomDoor => {
                if state.living_room.has_escaped_dad && !state.kitchen.weapon_collected {
                    return Ok(());
                }
                let scene = if state.kitchen.weapon_collected {
                    state.murder_living_room.murderous_intent = true;
                    Scenes::MurderLivingRoom
                } else {
                    Scenes::LivingRoom
                };
                state.scene_changed = Some((1.0, scene));
            }
        }
        Ok(())
    }
}
