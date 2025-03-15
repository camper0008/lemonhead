use std::path::Path;

use sdl2::rect::Rect;
use sdl2::{image::LoadTexture, render::WindowCanvas};

use crate::globals::{GROUND_LEVEL, PIXEL_PER_DOT};
use crate::helper::{draw_ground, draw_wallpaper};
use crate::rect;
use crate::state::State;
use crate::tileset::Tile;

use crate::scene::{Id, Item, Items, Scene};

use super::Scenes;

#[derive(Default)]
pub struct ChildRoom;

enum Interactables {
    ExitDoor,
    Child,
}

impl Item for Interactables {
    fn id(&self) -> Id {
        match self {
            Interactables::ExitDoor => Id(0),
            Interactables::Child => Id(1),
        }
    }
}

impl From<Id> for Interactables {
    fn from(value: Id) -> Self {
        match value {
            Id(0) => Self::ExitDoor,
            Id(1) => Self::Child,
            Id(_) => unreachable!(),
        }
    }
}

impl ChildRoom {
    fn draw_house(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator.load_texture(Path::new("assets/tile.png"))?;

        draw_wallpaper(canvas, &texture, &Tile::DotWallpaper)?;
        Tile::DoorOpen.draw(canvas, &texture, (1.0, GROUND_LEVEL), (1.0, 1.0))?;

        Tile::ChildPoster.draw(canvas, &texture, (3.0, GROUND_LEVEL), (1.0, 1.0))?;
        Tile::Computer.draw(canvas, &texture, (4.0, GROUND_LEVEL), (1.0, 1.0))?;
        Tile::OfficeChair.draw(canvas, &texture, (4.0, GROUND_LEVEL), (1.0, 1.0))?;
        Tile::Bed.draw(canvas, &texture, (6.0, GROUND_LEVEL), (1.0, 1.0))?;

        Ok(())
    }

    fn draw_child(
        &self,
        canvas: &mut WindowCanvas,
        state: &State,
        animation_timer: f64,
    ) -> Result<(), String> {
        let texture_creator = canvas.texture_creator();
        let child = texture_creator.load_texture(Path::new("assets/child.png"))?;
        let blood = texture_creator.load_texture(Path::new("assets/blood.png"))?;

        let offset = if state.child_room.child_dead() {
            192
        } else if animation_timer < 0.5 {
            0
        } else {
            32
        };

        canvas.copy(
            &child,
            rect!(offset, 0, 32, 32),
            rect!(
                PIXEL_PER_DOT * 5.0,
                (GROUND_LEVEL) * PIXEL_PER_DOT,
                PIXEL_PER_DOT,
                PIXEL_PER_DOT
            ),
        )?;

        if state.child_room.child_stabs > 0 {
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
        }
        if state.child_room.child_stabs > 1 {
            canvas.copy(
                &blood,
                rect!(0, 32, 32, 32),
                rect!(
                    PIXEL_PER_DOT * 4.,
                    (GROUND_LEVEL) * PIXEL_PER_DOT,
                    PIXEL_PER_DOT,
                    PIXEL_PER_DOT
                ),
            )?;
        }
        if state.child_room.child_stabs > 2 {
            canvas.copy(
                &blood,
                rect!(32, 32, 32, 32),
                rect!(
                    PIXEL_PER_DOT * 6.,
                    (GROUND_LEVEL) * PIXEL_PER_DOT,
                    PIXEL_PER_DOT,
                    PIXEL_PER_DOT
                ),
            )?;
        }

        Ok(())
    }
}

impl Scene for ChildRoom {
    fn draw(
        &self,
        state: &crate::state::State,
        canvas: &mut sdl2::render::WindowCanvas,
        animation_timer: f64,
    ) -> Result<(), String> {
        canvas.clear();
        self.draw_house(canvas)?;
        draw_ground(canvas)?;
        self.draw_child(canvas, state, animation_timer)?;
        Ok(())
    }

    fn prepare_items(&self, state: &State) -> Items {
        let mut items = Items::new();
        if state.child_room.child_stabs < 3 {
            items.push(5, Interactables::Child);
        }
        if state.child_room.child_stabs > 0 {
            items.push(1, Interactables::ExitDoor);
        }
        items
    }

    fn interact(&self, state: &mut crate::state::State, position: f64) {
        let Some(closest) = self.closest_item_within_distance(state, position) else {
            return;
        };
        match closest.id().into() {
            Interactables::ExitDoor => {
                state.send_audio("assets/click.ogg");
                if state.child_room.child_stabs < 3 {
                    return;
                }
                state.scene_changed = Some((4, Scenes::Entryway));
            }
            Interactables::Child => {
                state.send_audio("assets/stab.ogg");
                state.child_room.child_stabs += 1;
                if state.child_room.child_stabs == 1 {
                    state.change_background_track("assets/heartbeat-child.ogg");
                } else if state.child_room.child_stabs == 2 {
                    state.change_background_track("assets/heartbeat.ogg");
                } else if state.child_room.child_stabs == 3 {
                    state.stop_background_track();
                } else {
                    unreachable!();
                }
            }
        }
    }
}
