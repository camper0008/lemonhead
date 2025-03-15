use std::path::Path;

use sdl2::rect::Rect;
use sdl2::{image::LoadTexture, render::WindowCanvas};

use crate::globals::{GROUND_LEVEL, PIXEL_PER_DOT};
use crate::helper::{draw_ground, draw_wallpaper};
use crate::scene::{Id, Item, Items};
use crate::state::State;
use crate::tileset::Tile;
use crate::{rect, scene::Scene};

use super::Scenes;

#[derive(Default)]
pub struct MurderLivingRoom;

enum Interactables {
    ExitDoor,
    Dad,
}

impl Item for Interactables {
    fn id(&self) -> Id {
        match self {
            Self::ExitDoor => Id(0),
            Self::Dad => Id(1),
        }
    }
}

impl From<Id> for Interactables {
    fn from(value: Id) -> Self {
        match value {
            Id(0) => Self::ExitDoor,
            Id(1) => Self::Dad,
            Id(_) => unreachable!(),
        }
    }
}

impl MurderLivingRoom {
    fn draw_house(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator.load_texture(Path::new("assets/tile.png"))?;

        draw_wallpaper(canvas, &texture, &Tile::StripeWallpaper)?;

        Tile::DoorOpen.draw(canvas, &texture, (1.0, GROUND_LEVEL), (1.0, 1.0))?;

        Tile::TreeDayPicture.draw(canvas, &texture, (3.0, GROUND_LEVEL), (1.0, 1.0))?;
        Tile::HousePicture.draw(canvas, &texture, (4.0, GROUND_LEVEL), (1.0, 1.0))?;
        Tile::Couch.draw(canvas, &texture, (6.0, GROUND_LEVEL), (1.0, 1.0))?;

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
                (GROUND_LEVEL) * PIXEL_PER_DOT,
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
                    (GROUND_LEVEL) * PIXEL_PER_DOT,
                    PIXEL_PER_DOT,
                    PIXEL_PER_DOT
                ),
            )?;
            canvas.copy(
                &blood,
                rect!(0, 0, 32, 32),
                rect!(
                    PIXEL_PER_DOT * 5.0,
                    (GROUND_LEVEL) * PIXEL_PER_DOT,
                    PIXEL_PER_DOT,
                    PIXEL_PER_DOT
                ),
            )?;
            canvas.copy(
                &blood,
                rect!(32, 32, 32, 32),
                rect!(
                    PIXEL_PER_DOT * 6.0,
                    (GROUND_LEVEL) * PIXEL_PER_DOT,
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

    fn interact(&self, state: &mut crate::state::State, position: f64) {
        let Some(closest) = self.closest_item_within_distance(state, position) else {
            return;
        };
        match closest.id().into() {
            Interactables::ExitDoor => {
                state.send_audio("assets/click.ogg");
                state.scene_changed = Some((8, Scenes::Kitchen));
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
        items.push(5, Interactables::Dad);
        if state.murder_living_room.dad_dead {
            items.push(1, Interactables::ExitDoor);
        }
        items
    }
}
