use crate::ctx::{Ctx, Effect};
use crate::globals::GROUND_LEVEL;
use crate::helper::CtxHelperExt;
use crate::sprite::{Actor, Blood, Npc, Tile};
use crate::state::State;

use super::{InteractableId, Item, Items, Scene};

use super::Scenes;

#[derive(Default)]
pub struct ChildRoom;

enum Interactables {
    ExitDoor,
    Child,
}

impl Item for Interactables {
    fn id(&self) -> InteractableId {
        match self {
            Interactables::ExitDoor => InteractableId(0),
            Interactables::Child => InteractableId(1),
        }
    }
}

impl From<InteractableId> for Interactables {
    fn from(value: InteractableId) -> Self {
        match value {
            InteractableId(0) => Self::ExitDoor,
            InteractableId(1) => Self::Child,
            InteractableId(_) => unreachable!(),
        }
    }
}

impl ChildRoom {
    fn draw_house<C: Ctx>(&self, ctx: &mut C) -> Result<(), C::Error> {
        ctx.draw_ground()?;
        ctx.draw_wallpaper(&Tile::DotWallpaper)?;
        ctx.draw_sprite((1.0, GROUND_LEVEL), (1.0, 1.0), &Tile::DoorOpen)?;

        ctx.draw_sprite((3.0, GROUND_LEVEL), (1.0, 1.0), &Tile::ChildPoster)?;
        ctx.draw_sprite((4.0, GROUND_LEVEL), (1.0, 1.0), &Tile::Computer)?;
        ctx.draw_sprite((4.0, GROUND_LEVEL), (1.0, 1.0), &Tile::OfficeChair)?;
        ctx.draw_sprite((6.0, GROUND_LEVEL), (1.0, 1.0), &Tile::Bed)?;

        Ok(())
    }

    fn draw_child<C: Ctx>(&self, ctx: &mut C, state: &State<C>) -> Result<(), C::Error> {
        let child = if state.child_room.child_dead() {
            Actor::Child(Npc::Dead)
        } else if ctx.seconds_elapsed() % 1.0 < 0.5 {
            Actor::Child(Npc::Idle)
        } else {
            Actor::Child(Npc::IdleAlt)
        };

        ctx.draw_sprite((5.0, GROUND_LEVEL), (1.0, 1.0), &child)?;

        if state.child_room.child_stabs > 0 {
            ctx.draw_sprite((5.0, GROUND_LEVEL), (1.0, 1.0), &Blood::SplatterCenter)?;
        }
        if state.child_room.child_stabs > 1 {
            ctx.draw_sprite((4.0, GROUND_LEVEL), (1.0, 1.0), &Blood::SplatterRight)?;
        }
        if state.child_room.child_stabs > 2 {
            ctx.draw_sprite((6.0, GROUND_LEVEL), (1.0, 1.0), &Blood::SplatterLeft)?;
        }

        Ok(())
    }
}

impl<C: Ctx> Scene<C> for ChildRoom {
    fn draw(&self, ctx: &mut C, state: &crate::state::State<C>) -> Result<(), C::Error> {
        self.draw_house(ctx)?;
        self.draw_child(ctx, state)?;
        Ok(())
    }

    fn prepare_items(&self, state: &State<C>) -> Items {
        let mut items = Items::new();
        if state.child_room.child_stabs < 3 {
            items.push(5.0, Interactables::Child);
        }
        if state.child_room.child_stabs > 0 {
            items.push(1.0, Interactables::ExitDoor);
        }
        items
    }

    fn interact(&self, ctx: &mut C, state: &mut State<C>, position: f64) -> Result<(), C::Error> {
        let Some(closest) = self.closest_item_within_distance(state, position) else {
            return Ok(());
        };
        match closest.id().into() {
            Interactables::ExitDoor => {
                ctx.play_effect(Effect::Interact)?;
                if state.child_room.child_stabs < 3 {
                    return Ok(());
                }
                state.scene_changed = Some((4.0, Scenes::Entryway));
            }
            Interactables::Child => {
                ctx.play_effect(Effect::Stab)?;
                state.child_room.child_stabs += 1;
                if state.child_room.child_stabs == 1 {
                    ctx.set_music(crate::ctx::Music::HeartbeatChild)?;
                } else if state.child_room.child_stabs == 2 {
                    ctx.set_music(crate::ctx::Music::Heartbeat)?;
                } else if state.child_room.child_stabs == 3 {
                    ctx.stop_music()?;
                } else {
                    unreachable!();
                }
            }
        };
        Ok(())
    }
}
