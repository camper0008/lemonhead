use super::{InteractableId, Item, Items, Scene};
use crate::ctx::{Ctx, Effect, Music};
use crate::globals::GROUND_LEVEL;
use crate::helper::CtxHelperExt;
use crate::sprite::{Blood, Tile};
use crate::state::State;

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
    fn draw_house<C: Ctx>(&self, ctx: &mut C, state: &State<C>) {
        ctx.draw_ground();
        ctx.draw_wallpaper(&Tile::StripeWallpaper);

        ctx.draw_sprite((1.0, GROUND_LEVEL), (1.0, 1.0), &Tile::DoorOpen);

        let picture_tile = if state.child_room.child_stabs > 0 {
            Tile::LemonDayPicture
        } else {
            Tile::TreeDayPicture
        };

        ctx.draw_sprite((7.0, GROUND_LEVEL), (1.0, 1.0), &picture_tile);

        ctx.draw_sprite((2.0, GROUND_LEVEL), (1.0, 1.0), &Tile::HousePicture);

        if state.murder_living_room.dad_dead {
            ctx.draw_sprite((2.0, GROUND_LEVEL), (1.0, 1.0), &Blood::SplatterCenter);
        }

        let kitchen_door = if state.entryway.all_coins_collected() {
            Tile::DoorOpen
        } else {
            Tile::DoorClosed
        };

        ctx.draw_sprite((8.0, GROUND_LEVEL), (1.0, 1.0), &kitchen_door);

        let child_door = if state.murder_living_room.dad_dead {
            Tile::DoorOpen
        } else {
            Tile::DoorClosed
        };

        ctx.draw_sprite((4.0, GROUND_LEVEL), (1.0, 1.0), &child_door);

        if !state.murder_living_room.dad_dead {
            ctx.draw_sprite((4.0, GROUND_LEVEL), (1.0, 1.0), &Tile::ChildSticker);
        }

        if state.murder_living_room.dad_dead && state.child_room.child_stabs == 0 {
            ctx.draw_sprite((4.0, GROUND_LEVEL - 1.0), (1.0, 1.0), &Blood::Pentagram);
        }

        if !state.entryway.coins[0] {
            ctx.draw_item(&Tile::Coin, 3.0);
        }
        if !state.entryway.coins[1] {
            ctx.draw_item(&Tile::Coin, 4.0);
        }
        if !state.entryway.coins[2] {
            ctx.draw_item(&Tile::Coin, 5.0);
        }
        if !state.entryway.coins[3] {
            ctx.draw_item(&Tile::Coin, 6.0);
        }
    }
}

impl<C: Ctx> Scene<C> for Entryway {
    fn draw(&self, ctx: &mut C, state: &State<C>) {
        self.draw_house(ctx, state);
    }

    fn prepare_items(&self, state: &State<C>) -> Items {
        let mut items = Items::new();
        items.push(1.0, Interactables::ExitDoor);
        if !state.entryway.coins[0] {
            items.push(3.0, Interactables::Coin0);
        }
        if !state.entryway.coins[1] {
            items.push(4.0, Interactables::Coin1);
        }
        if !state.entryway.coins[2] {
            items.push(5.0, Interactables::Coin2);
        }
        if !state.entryway.coins[3] {
            items.push(6.0, Interactables::Coin3);
        }
        if state.entryway.all_coins_collected() {
            items.push(8.0, Interactables::KitchenDoor);
        }
        if state.murder_living_room.dad_dead {
            items.push(4.0, Interactables::ChildDoor);
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
                if state.murder_living_room.dad_dead && !state.child_room.child_dead() {
                    return Ok(());
                }
                state.scene_changed = Some((7.0, Scenes::Outside));
                if !state.child_room.child_dead() {
                    ctx.set_music(Music::Outside)?;
                }
            }
            Interactables::Coin0 => state.entryway.coins[0] = true,
            Interactables::Coin1 => state.entryway.coins[1] = true,
            Interactables::Coin2 => state.entryway.coins[2] = true,
            Interactables::Coin3 => state.entryway.coins[3] = true,
            Interactables::ChildDoor => {
                ctx.set_music(Music::HeartbeatChildWithLemon)?;
                state.scene_changed = Some((1.0, Scenes::ChildRoom));
            }
            Interactables::KitchenDoor => {
                state.scene_changed = Some((1.0, Scenes::Kitchen));
            }
        }
        Ok(())
    }
}
