use super::{InteractableId, Item, Items, Scene};
use crate::ctx::{Ctx, Effect, Music};
use crate::globals::GROUND_LEVEL;
use crate::helper::CtxHelperExt;
use crate::sprite::{Actor, Blood, Npc, Tile};
use crate::state::State;

use super::Scenes;

#[derive(Default)]
pub struct MurderLivingRoom;

enum Interactables {
    ExitDoor,
    Dad,
}

impl Item for Interactables {
    fn id(&self) -> InteractableId {
        match self {
            Self::ExitDoor => InteractableId(0),
            Self::Dad => InteractableId(1),
        }
    }
}

impl From<InteractableId> for Interactables {
    fn from(value: InteractableId) -> Self {
        match value {
            InteractableId(0) => Self::ExitDoor,
            InteractableId(1) => Self::Dad,
            InteractableId(_) => unreachable!(),
        }
    }
}

impl MurderLivingRoom {
    fn draw_house<C: Ctx>(&self, ctx: &mut C) {
        ctx.draw_wallpaper(&Tile::StripeWallpaper);
        ctx.draw_sprite((1.0, GROUND_LEVEL), (1.0, 1.0), &Tile::DoorOpen);
        ctx.draw_sprite((3.0, GROUND_LEVEL), (1.0, 1.0), &Tile::TreeDayPicture);
        ctx.draw_sprite((4.0, GROUND_LEVEL), (1.0, 1.0), &Tile::HousePicture);
        ctx.draw_sprite((6.0, GROUND_LEVEL), (1.0, 1.0), &Tile::Couch);
    }

    fn draw_dad<C: Ctx>(&self, ctx: &mut C, state: &State<C>) {
        let dad = if state.murder_living_room.dad_dead {
            Actor::Dad(Npc::Dead)
        } else if ctx.seconds_elapsed() % 1.0 < 0.5 {
            Actor::Dad(Npc::Idle)
        } else {
            Actor::Dad(Npc::IdleAlt)
        };

        ctx.draw_sprite((5.0, GROUND_LEVEL), (1.0, 1.0), &dad);

        if state.murder_living_room.dad_dead {
            ctx.draw_sprite((4.0, GROUND_LEVEL), (1.0, 1.0), &Blood::SplatterRight);
            ctx.draw_sprite((5.0, GROUND_LEVEL), (1.0, 1.0), &Blood::SplatterCenter);
            ctx.draw_sprite((6.0, GROUND_LEVEL), (1.0, 1.0), &Blood::SplatterLeft);
        }
    }
}

impl<C: Ctx> Scene<C> for MurderLivingRoom {
    fn draw(&self, ctx: &mut C, state: &crate::state::State<C>) {
        self.draw_house(ctx);
        ctx.draw_ground();
        self.draw_dad(ctx, state);
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
        match closest.id().into() {
            Interactables::ExitDoor => {
                ctx.play_effect(Effect::Interact)?;
                state.scene_changed = Some((8.0, Scenes::Kitchen));
            }
            Interactables::Dad => {
                ctx.play_effect(Effect::Stab)?;
                if !state.murder_living_room.dad_dead {
                    state.murder_living_room.dad_dead = true;
                    ctx.set_music(Music::HeartbeatChild)?;
                }
            }
        };
        Ok(())
    }

    fn prepare_items(&self, state: &State<C>) -> Items {
        let mut items = Items::new();
        items.push(5.0, Interactables::Dad);
        if state.murder_living_room.dad_dead {
            items.push(1.0, Interactables::ExitDoor);
        }
        items
    }
}
