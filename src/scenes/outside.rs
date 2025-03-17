use super::{InteractableId, Item, Items, Scene};
use crate::ctx::{Ctx, Effect, Music, Rgb};
use crate::globals::GROUND_LEVEL;
use crate::helper::CtxHelperExt;
use crate::sprite::Tile;
use crate::state::{EndingChosen, State};

use super::Scenes;

const HOUSE_OFFSET: f64 = 6.0;

#[derive(Default)]
pub struct Outside;

enum Interactables {
    Bike,
    Door,
    Ascension,
    Key,
}

impl Item for Interactables {
    fn id(&self) -> InteractableId {
        match self {
            Self::Bike => InteractableId(0),
            Self::Door => InteractableId(1),
            Self::Ascension => InteractableId(2),
            Self::Key => InteractableId(3),
        }
    }
}

impl From<InteractableId> for Interactables {
    fn from(value: InteractableId) -> Self {
        match value {
            InteractableId(0) => Self::Bike,
            InteractableId(1) => Self::Door,
            InteractableId(2) => Self::Ascension,
            InteractableId(3) => Self::Key,
            InteractableId(_) => unreachable!(),
        }
    }
}

impl Outside {
    fn enqueue_house<C: Ctx>(&self, ctx: &mut C, state: &State<C>) {
        ctx.enqueue_sprite((1.0, GROUND_LEVEL), (1.0, 1.0), &Tile::Bike);

        for i in 0..=2 {
            ctx.enqueue_sprite(
                (HOUSE_OFFSET + i as f64, GROUND_LEVEL),
                (1.0, 1.0),
                &Tile::HouseBrick,
            );
        }

        for x in 0..10 {
            ctx.enqueue_sprite((x as f64, GROUND_LEVEL), (1.0, 1.0), &Tile::Grass);
        }

        let sun_tile = if state.child_room.child_dead() {
            Tile::LemonSun
        } else {
            Tile::Sun
        };
        ctx.enqueue_sprite((1.0, 1.0), (1.0, 1.0), &sun_tile);

        [Tile::LeftTriangle, Tile::Block, Tile::RightTriangle]
            .into_iter()
            .enumerate()
            .for_each(|(offset, tile)| {
                ctx.enqueue_sprite(
                    (HOUSE_OFFSET + offset as f64, GROUND_LEVEL - 1.0),
                    (1.0, 1.0),
                    &tile,
                );
            });

        let door_texture = if state.outside.key_collected {
            Tile::DoorOpen
        } else {
            Tile::DoorClosed
        };

        ctx.enqueue_sprite(
            (HOUSE_OFFSET + 1.0, GROUND_LEVEL),
            (1.0, 1.0),
            &door_texture,
        );

        if state.child_room.child_dead() {
            let ascension_offset = ctx.seconds_elapsed() % 4.0;
            let sprite = if ascension_offset < 1.0 {
                Tile::Ascension0
            } else if ascension_offset < 2.0 {
                Tile::Ascension1
            } else if ascension_offset < 3.0 {
                Tile::Ascension2
            } else {
                Tile::Ascension3
            };

            ctx.enqueue_sprite((3.0, -2.0), (1.0, 4.0), &sprite);
            ctx.enqueue_sprite((3.0, 2.0), (1.0, 4.0), &sprite);
        }

        if !state.outside.key_collected {
            ctx.enqueue_item(&Tile::Key, 3.0);
        }
    }
}

impl<C: Ctx> Scene<C> for Outside {
    fn draw(&self, ctx: &mut C, state: &crate::state::State<C>) {
        if state.child_room.child_dead() {
            ctx.enqueue_background_fill(Rgb(217, 87, 99));
        } else {
            ctx.enqueue_background_fill(Rgb(255, 255, 255));
        }
        self.enqueue_house(ctx, state);
        ctx.enqueue_ground();
    }

    fn prepare_items(&self, state: &State<C>) -> Items {
        let mut items = Items::new();
        if state.outside.key_collected {
            items.push(HOUSE_OFFSET + 1.0, Interactables::Door);
        } else {
            items.push(3.0, Interactables::Key);
        }
        if state.child_room.child_dead() && state.ending_chosen.is_none() {
            items.push(3.0, Interactables::Ascension);
        }
        if state.living_room.has_escaped_dad
            && state.ending_chosen.is_none()
            && !state.child_room.child_dead()
        {
            items.push(1.0, Interactables::Bike);
        }

        items
    }

    fn interact(&self, ctx: &mut C, state: &mut State<C>, position: f64) -> Result<(), C::Error> {
        let Some(closest) = self.closest_item_within_distance(state, position) else {
            return Ok(());
        };
        ctx.play_effect(Effect::Interact)?;
        match closest.id().into() {
            Interactables::Key => state.outside.key_collected = true,
            Interactables::Ascension => {
                state.ending_chosen = Some(EndingChosen::Ascended);
                ctx.set_music(Music::Ascend)?;
            }
            Interactables::Door => {
                state.scene_changed = Some((1.into(), Scenes::Entryway));

                if !state.living_room.has_escaped_dad {
                    ctx.set_music(Music::Lemonhead)?;
                }
            }
            Interactables::Bike => {
                state.ending_chosen = Some(EndingChosen::Escaped);
            }
        };
        Ok(())
    }
}
