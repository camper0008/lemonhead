use super::{InteractableId, Item, Items, Scene};
use crate::ctx::{Ctx, Effect, Music};
use crate::globals::GROUND_LEVEL;
use crate::helper::CtxHelperExt;
use crate::sprite::{Actor, ActorState, Bubble, Tile};
use crate::state::State;

use super::Scenes;

#[derive(Default)]
pub struct LivingRoom;

enum Interactables {
    ExitDoor,
    Coin0,
    Coin1,
}

impl Item for Interactables {
    fn id(&self) -> InteractableId {
        match self {
            Self::ExitDoor => InteractableId(0),
            Self::Coin0 => InteractableId(1),
            Self::Coin1 => InteractableId(2),
        }
    }
}

impl From<InteractableId> for Interactables {
    fn from(value: InteractableId) -> Self {
        match value {
            InteractableId(0) => Self::ExitDoor,
            InteractableId(1) => Self::Coin0,
            InteractableId(2) => Self::Coin1,
            InteractableId(_) => unreachable!(),
        }
    }
}

impl LivingRoom {
    fn enqueue_house<C: Ctx>(&self, ctx: &mut C, state: &State<C>) {
        ctx.enqueue_ground();
        ctx.enqueue_wallpaper(&Tile::StripeWallpaper);

        ctx.enqueue_sprite((1.0, GROUND_LEVEL), (1.0, 1.0), &Tile::DoorOpen);

        ctx.enqueue_sprite((3.0, GROUND_LEVEL), (1.0, 1.0), &Tile::TreeDayPicture);
        ctx.enqueue_sprite((4.0, GROUND_LEVEL), (1.0, 1.0), &Tile::HousePicture);
        ctx.enqueue_sprite((6.0, GROUND_LEVEL), (1.0, 1.0), &Tile::Couch);

        if !state.living_room.coins[0] {
            ctx.enqueue_item(&Tile::Coin, 3.0);
        }
        if !state.living_room.coins[1] {
            ctx.enqueue_item(&Tile::Coin, 8.0);
        }
    }

    fn enqueue_confrontation<C: Ctx>(&self, ctx: &mut C, state: &State<C>) {
        if !state.living_room.all_coins_collected() {
            return;
        }
        let bubble = {
            use Bubble::*;
            let conf = state.living_room.dad_attack_seconds * 8.0;
            [L0, L1, L2, L3, L4, L5, L6, L7]
                .into_iter()
                .enumerate()
                .find_map(|(idx, bub)| {
                    let seconds_elapsed = idx as f64 + 1.0;
                    if conf < seconds_elapsed {
                        Some(bub)
                    } else {
                        None
                    }
                })
        };

        let dad = Actor::npc_sprite(
            &ActorState::Left,
            ctx.seconds_elapsed() % 1.0 > 0.5,
            Actor::Dad,
        );
        ctx.enqueue_sprite(
            (
                14.0 - state.living_room.dad_attack_seconds * 2.0,
                GROUND_LEVEL,
            ),
            (1.0, 1.0),
            &dad,
        );

        if let Some(bubble) = bubble {
            ctx.enqueue_sprite((9.0, GROUND_LEVEL), (1.0, 1.0), &bubble);
        }
    }
}

impl<C: Ctx> Scene<C> for LivingRoom {
    fn draw(&self, ctx: &mut C, state: &crate::state::State<C>) {
        self.enqueue_house(ctx, state);
        self.enqueue_confrontation(ctx, state);
    }

    fn interact(
        &self,
        ctx: &mut C,
        state: &mut crate::state::State<C>,
        position: f64,
    ) -> Result<(), C::Error> {
        let Some(closest) = self.closest_item_within_distance(state, position) else {
            return Ok(());
        };
        ctx.play_effect(Effect::Interact)?;
        match closest.id().into() {
            Interactables::ExitDoor => {
                state.living_room.has_escaped_dad = true;
                state.scene_changed = Some((8.0, Scenes::Kitchen));
            }
            Interactables::Coin0 => {
                state.living_room.coins[0] = true;
                if state.living_room.coins[1] {
                    ctx.set_music(Music::Run)?;
                };
            }
            Interactables::Coin1 => {
                state.living_room.coins[1] = true;
                if state.living_room.coins[0] {
                    ctx.set_music(Music::Run)?;
                };
            }
        };
        Ok(())
    }

    fn prepare_items(&self, state: &State<C>) -> Items {
        let mut items = Items::new();
        if !state.living_room.coins[0] {
            items.push(3.0, Interactables::Coin0);
        }
        if !state.living_room.coins[1] {
            items.push(8.0, Interactables::Coin1);
        }
        if state.living_room.all_coins_collected() {
            items.push(1.0, Interactables::ExitDoor);
        }
        items
    }
}
