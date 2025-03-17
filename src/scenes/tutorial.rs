use super::{InteractableId, Item, Items, Scene};
use crate::ctx::Ctx;
use crate::ctx::Effect;
use crate::ctx::Rgb;
use crate::globals::GROUND_LEVEL;
use crate::helper::CtxHelperExt;
use crate::sprite::Tile;
use crate::state::State;

use super::Scenes;

#[derive(Default)]
pub struct Tutorial;

enum Interactables {
    Bike,
    Coin,
}

impl Item for Interactables {
    fn id(&self) -> InteractableId {
        match self {
            Interactables::Bike => InteractableId(0),
            Interactables::Coin => InteractableId(1),
        }
    }
}

impl From<InteractableId> for Interactables {
    fn from(value: InteractableId) -> Self {
        match value {
            InteractableId(0) => Interactables::Bike,
            InteractableId(1) => Interactables::Coin,
            InteractableId(_) => unreachable!(),
        }
    }
}

impl Tutorial {
    fn enqueue_scenery<C: Ctx>(&self, ctx: &mut C) {
        ctx.enqueue_sprite((8.0, GROUND_LEVEL), (1.0, 1.0), &Tile::Bike);

        for x in 0..10 {
            ctx.enqueue_sprite((x as f64, GROUND_LEVEL), (1.0, 1.0), &Tile::Grass);
        }

        ctx.enqueue_sprite((1.0, 1.0), (1.0, 1.0), &Tile::Sun);
    }

    fn enqueue_text<C: Ctx>(&self, ctx: &mut C, state: &State<C>) {
        if !state.tutorial.coin {
            ctx.enqueue_sprite((3.0, 2.0), (4.0, 1.0), &Tile::IntroductionText);
            ctx.enqueue_sprite((1.0, 3.0), (8.0, 1.0), &Tile::IntroductionGoalsText);
        } else {
            ctx.enqueue_sprite((2.0, 2.5), (6.0, 1.0), &Tile::RememberText);
            ctx.enqueue_sprite((6.0, 9.25), (1.0, 0.5), &Tile::VoicesText);
        }
    }
}

impl<C: Ctx> Scene<C> for Tutorial {
    fn draw(&self, ctx: &mut C, state: &crate::state::State<C>) {
        ctx.enqueue_background_fill(Rgb(255, 255, 255));
        ctx.enqueue_ground();
        self.enqueue_scenery(ctx);
        self.enqueue_text(ctx, state);
        if !state.tutorial.coin {
            ctx.enqueue_item(&Tile::Coin, 4.0);
        }
    }

    fn prepare_items(&self, state: &State<C>) -> Items {
        let mut items = Items::new();
        if state.tutorial.coin {
            items.push(8.0, Interactables::Bike);
        } else {
            items.push(4.0, Interactables::Coin);
        };
        items
    }

    fn interact(&self, ctx: &mut C, state: &mut State<C>, position: f64) -> Result<(), C::Error> {
        let closest = self.closest_item_within_distance(state, position);
        let Some(item) = closest else { return Ok(()) };
        ctx.play_effect(Effect::Interact)?;
        match Interactables::from(item.id()) {
            Interactables::Coin => state.tutorial.coin = true,
            Interactables::Bike => {
                state.scene_changed = Some((1.0, Scenes::Outside));
            }
        }
        Ok(())
    }
}
