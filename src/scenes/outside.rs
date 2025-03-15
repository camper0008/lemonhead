use std::path::Path;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::{image::LoadTexture, render::WindowCanvas};

use super::{InteractableId, Item, Items, Scene};
use crate::globals::{GROUND_LEVEL, PIXEL_PER_DOT};
use crate::helper::{draw_ground, draw_item};
use crate::rect;
use crate::state::State;
use crate::tileset::Tile;

use super::Scenes;

const HOUSE_OFFSET: u8 = 6;

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

        Tile::Bike.draw(canvas, &texture, (1.0, GROUND_LEVEL), (1.0, 1.0))?;

        for i in 0..=2 {
            Tile::HouseBrick.draw(
                canvas,
                &texture,
                ((HOUSE_OFFSET + i) as f64, GROUND_LEVEL),
                (1.0, 1.0),
            )?;
        }

        for x in 0..10 {
            Tile::Grass.draw(canvas, &texture, (f64::from(x), GROUND_LEVEL), (1.0, 1.0))?;
        }

        if state.child_room.child_dead() {
            Tile::LemonSun.draw(canvas, &texture, (1.0, 1.0), (1.0, 1.0))?;
        } else {
            Tile::Sun.draw(canvas, &texture, (1.0, 1.0), (1.0, 1.0))?;
        }

        Tile::LeftTriangle.draw(
            canvas,
            &texture,
            (HOUSE_OFFSET as f64, GROUND_LEVEL - 1.0),
            (1.0, 1.0),
        )?;
        Tile::Block.draw(
            canvas,
            &texture,
            ((HOUSE_OFFSET + 1) as f64, GROUND_LEVEL - 1.0),
            (1.0, 1.0),
        )?;
        Tile::RightTriangle.draw(
            canvas,
            &texture,
            ((HOUSE_OFFSET + 2) as f64, GROUND_LEVEL - 1.0),
            (1.0, 1.0),
        )?;

        let door_texture = if state.outside.key_collected {
            Tile::DoorOpen
        } else {
            Tile::DoorClosed
        };

        door_texture.draw(
            canvas,
            &texture,
            ((HOUSE_OFFSET + 1) as f64, GROUND_LEVEL),
            (1.0, 1.0),
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
            draw_item(canvas, &texture, &Tile::Key, 3.0, animation_timer)?;
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
            items.push(HOUSE_OFFSET + 1, Interactables::Door);
        } else {
            items.push(3, Interactables::Key);
        }

        if state.child_room.child_dead() && !state.ascended {
            items.push(3, Interactables::Ascension);
        }

        if state.living_room.confronted && !state.escaped && !state.child_room.child_dead() {
            items.push(1, Interactables::Bike);
        }

        items
    }

    fn interact(&self, state: &mut crate::state::State, position: f64) {
        let Some(closest) = self.closest_item_within_distance(state, position) else {
            return;
        };
        state.send_audio("assets/click.ogg");
        match closest.id().into() {
            Interactables::Key => state.outside.key_collected = true,
            Interactables::Ascension => {
                state.ascended = true;
                state.play_ascension_track();
            }
            Interactables::Door => {
                state.scene_changed = Some((1, Scenes::Entryway));

                if !state.living_room.confronted {
                    state.change_background_track("assets/lemonhead.ogg");
                }
            }
            Interactables::Bike => {
                state.escaped = true;
            }
        }
    }
}
