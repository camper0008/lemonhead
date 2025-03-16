use std::path::Path;

use sdl2::rect::Rect;
use sdl2::{image::LoadTexture, render::WindowCanvas};

use super::{InteractableId, Item, Items, Scene};
use crate::globals::{GROUND_LEVEL, PIXEL_PER_DOT};
use crate::helper::{draw_ground, draw_wallpaper};
use crate::logic::Unit;
use crate::rect;
use crate::state::State;
use crate::sprite::Generic;

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
    fn draw_house(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator.load_texture(Path::new("assets/tile.png"))?;

        draw_wallpaper(canvas, &texture, &Generic::StripeWallpaper)?;

        Generic::DoorOpen.draw(canvas, &texture, (Unit::new(1), GROUND_LEVEL), (1, 1))?;

        Generic::TreeDayPicture.draw(canvas, &texture, (3, GROUND_LEVEL), (1, 1))?;
        Generic::HousePicture.draw(canvas, &texture, (4, GROUND_LEVEL), (1, 1))?;
        Generic::Couch.draw(canvas, &texture, (6, GROUND_LEVEL), (1, 1))?;

        Ok(())
    }

    fn draw_dad(
        &self,
        canvas: &mut WindowCanvas,
        state: &State,
        animation_timer: f64,
    ) -> Result<(), String> {
        let texture_creator = canvas.texture_creator();
        let dad = texture_creator.load_texture(Path::new("assets/dad.png"))?;
        let blood = texture_creator.load_texture(Path::new("assets/blood.png"))?;

        let offset = if state.murder_living_room.dad_dead {
            192
        } else if animation_timer < 0.5 {
            0
        } else {
            32
        };

        canvas.copy(
            &dad,
            rect!(offset, 0, 32, 32),
            rect!(
                PIXEL_PER_DOT * 5.0,
                GROUND_LEVEL.decimal() * PIXEL_PER_DOT,
                PIXEL_PER_DOT,
                PIXEL_PER_DOT
            ),
        )?;

        if state.murder_living_room.dad_dead {
            canvas.copy(
                &blood,
                rect!(0, 32, 32, 32),
                rect!(
                    PIXEL_PER_DOT * 4.0,
                    GROUND_LEVEL.decimal() * PIXEL_PER_DOT,
                    PIXEL_PER_DOT,
                    PIXEL_PER_DOT
                ),
            )?;
            canvas.copy(
                &blood,
                rect!(0, 0, 32, 32),
                rect!(
                    PIXEL_PER_DOT * 5.0,
                    GROUND_LEVEL.decimal() * PIXEL_PER_DOT,
                    PIXEL_PER_DOT,
                    PIXEL_PER_DOT
                ),
            )?;
            canvas.copy(
                &blood,
                rect!(32, 32, 32, 32),
                rect!(
                    PIXEL_PER_DOT * 6.0,
                    GROUND_LEVEL.decimal() * PIXEL_PER_DOT,
                    PIXEL_PER_DOT,
                    PIXEL_PER_DOT
                ),
            )?;
        }

        Ok(())
    }
}

impl Scene for MurderLivingRoom {
    fn draw(
        &self,
        state: &crate::state::State,
        canvas: &mut sdl2::render::WindowCanvas,
        animation_timer: f64,
    ) -> Result<(), String> {
        canvas.clear();
        self.draw_house(canvas)?;
        draw_ground(canvas)?;
        self.draw_dad(canvas, state, animation_timer)?;
        Ok(())
    }

    fn interact(&self, state: &mut crate::state::State, position: Unit) {
        let Some(closest) = self.closest_item_within_distance(state, position) else {
            return;
        };
        match closest.id().into() {
            Interactables::ExitDoor => {
                state.send_audio("assets/click.ogg");
                state.scene_changed = Some((8.into(), Scenes::Kitchen));
            }
            Interactables::Dad => {
                state.send_audio("assets/stab.ogg");
                if !state.murder_living_room.dad_dead {
                    state.murder_living_room.dad_dead = true;
                    state.change_background_track("assets/heartbeat-child.ogg");
                }
            }
        }
    }

    fn prepare_items(&self, state: &State) -> Items {
        let mut items = Items::new();
        items.push(Unit::new(5), Interactables::Dad);
        if state.murder_living_room.dad_dead {
            items.push(Unit::new(1), Interactables::ExitDoor);
        }
        items
    }
}
