use std::path::Path;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::{image::LoadTexture, render::WindowCanvas};

use super::{InteractableId, Item, Items, Scene};
use crate::globals::{GROUND_LEVEL, PIXEL_PER_DOT};
use crate::helper::{draw_ground, draw_item};
use crate::logic::Unit;
use crate::rect;
use crate::state::{EndingChosen, State};
use crate::sprite::Generic;

use super::Scenes;

const HOUSE_OFFSET: Unit = Unit::new(6);

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
    fn draw_house(
        &self,
        canvas: &mut WindowCanvas,
        state: &State,
        animation_timer: f64,
    ) -> Result<(), String> {
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator.load_texture(Path::new("assets/tile.png"))?;
        let ascension = texture_creator.load_texture(Path::new("assets/ascension.png"))?;

        Generic::Bike.draw(canvas, &texture, (1, GROUND_LEVEL), (1, 1))?;

        for i in (0..=2).map(Unit::new) {
            Generic::HouseBrick.draw(canvas, &texture, ((HOUSE_OFFSET + i), GROUND_LEVEL), (1, 1))?;
        }

        for x in 0..10 {
            Generic::Grass.draw(canvas, &texture, (x, GROUND_LEVEL), (1, 1))?;
        }

        if state.child_room.child_dead() {
            Generic::LemonSun.draw(canvas, &texture, (1, 1), (1, 1))?;
        } else {
            Generic::Sun.draw(canvas, &texture, (1, 1), (1, 1))?;
        }

        Generic::LeftTriangle.draw(
            canvas,
            &texture,
            (HOUSE_OFFSET, GROUND_LEVEL - 1.into()),
            (1, 1),
        )?;
        Generic::Block.draw(
            canvas,
            &texture,
            ((HOUSE_OFFSET + 1.into()), GROUND_LEVEL - Unit::new(1)),
            (1, 1),
        )?;
        Generic::RightTriangle.draw(
            canvas,
            &texture,
            ((HOUSE_OFFSET + 2.into()), GROUND_LEVEL - Unit::new(1)),
            (1, 1),
        )?;

        let door_texture = if state.outside.key_collected {
            Generic::DoorOpen
        } else {
            Generic::DoorClosed
        };

        door_texture.draw(
            canvas,
            &texture,
            ((HOUSE_OFFSET + 1.into()), GROUND_LEVEL),
            (1, 1),
        )?;

        let ascension_offset = (animation_timer * 4.0).floor() * 32.0;

        if state.child_room.child_dead() {
            canvas.copy(
                &ascension,
                rect!(ascension_offset, 0, 32, 128),
                rect!(3.0 * PIXEL_PER_DOT, 0, PIXEL_PER_DOT, PIXEL_PER_DOT * 4.0),
            )?;

            canvas.copy(
                &ascension,
                rect!(ascension_offset, 0, 32, 128),
                rect!(
                    3.0 * PIXEL_PER_DOT,
                    4.0 * PIXEL_PER_DOT,
                    PIXEL_PER_DOT,
                    PIXEL_PER_DOT * 4.0
                ),
            )?;
        }

        if !state.outside.key_collected {
            draw_item(canvas, &texture, &Generic::Key, 3, animation_timer)?;
        }

        Ok(())
    }
}

impl Scene for Outside {
    fn draw(
        &self,
        state: &crate::state::State,
        canvas: &mut sdl2::render::WindowCanvas,
        animation_timer: f64,
    ) -> Result<(), String> {
        if state.child_room.child_dead() {
            canvas.set_draw_color(Color::RGB(217, 87, 99));
        } else {
            canvas.set_draw_color(Color::RGB(255, 255, 255));
        }
        canvas.clear();
        self.draw_house(canvas, state, animation_timer)?;
        draw_ground(canvas)?;

        Ok(())
    }

    fn prepare_items(&self, state: &State) -> Items {
        let mut items = Items::new();
        if state.outside.key_collected {
            items.push(HOUSE_OFFSET + 1.into(), Interactables::Door);
        } else {
            items.push(3, Interactables::Key);
        }

        if state.child_room.child_dead() && state.ending_chosen.is_none() {
            items.push(3, Interactables::Ascension);
        }

        if state.living_room.has_escaped_dad
            && state.ending_chosen.is_none()
            && !state.child_room.child_dead()
        {
            items.push(1, Interactables::Bike);
        }

        items
    }

    fn interact(&self, state: &mut crate::state::State, position: Unit) {
        let Some(closest) = self.closest_item_within_distance(state, position) else {
            return;
        };
        state.send_audio("assets/click.ogg");
        match closest.id().into() {
            Interactables::Key => state.outside.key_collected = true,
            Interactables::Ascension => {
                state.ending_chosen = Some(EndingChosen::Ascended);
                state.play_ascension_track();
            }
            Interactables::Door => {
                state.scene_changed = Some((1.into(), Scenes::Entryway));

                if !state.living_room.has_escaped_dad {
                    state.change_background_track("assets/lemonhead.ogg");
                }
            }
            Interactables::Bike => {
                state.ending_chosen = Some(EndingChosen::Escaped);
            }
        }
    }
}
