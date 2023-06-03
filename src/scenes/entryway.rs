use std::path::Path;

use sdl2::rect::Rect;
use sdl2::{image::LoadTexture, render::WindowCanvas};

use crate::globals::{GROUND_LEVEL, PIXEL_PER_DOT};
use crate::helper::{closest_item_within_distance, draw_ground, draw_item, draw_wallpaper};
use crate::state::State;
use crate::tileset::Tile;
use crate::{rect, scene::Scene};

use super::Scenes;

#[derive(Default)]
pub struct Entryway;

enum Interactables {
    KitchenDoor,
    ExitDoor,
    ChildDoor,
    Coin0,
    Coin1,
    Coin2,
    Coin3,
}

impl Entryway {
    fn draw_house(
        &self,
        canvas: &mut WindowCanvas,
        state: &State,
        animation_timer: f64,
    ) -> Result<(), String> {
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator.load_texture(Path::new("assets/tile.png"))?;
        let blood = texture_creator.load_texture(Path::new("assets/blood.png"))?;

        draw_wallpaper(canvas, &texture, &Tile::StripeWallpaper)?;
        Tile::DoorOpen.draw(canvas, &texture, (1.0, GROUND_LEVEL), (1.0, 1.0))?;

        let picture_tile = if state.child_dead {
            Tile::LemonDayPicture
        } else {
            Tile::TreeDayPicture
        };

        picture_tile.draw(canvas, &texture, (7.0, GROUND_LEVEL), (1.0, 1.0))?;

        Tile::HousePicture.draw(canvas, &texture, (2.0, GROUND_LEVEL), (1.0, 1.0))?;

        if state.dad_dead {
            canvas.copy(
                &blood,
                rect!(0, 0, 32, 32),
                rect!(
                    PIXEL_PER_DOT * 2.0,
                    (GROUND_LEVEL) * PIXEL_PER_DOT,
                    PIXEL_PER_DOT,
                    PIXEL_PER_DOT
                ),
            )?;
        }

        let kitchen_door = if state.coin_0 && state.coin_1 && state.coin_2 && state.coin_3 {
            Tile::DoorOpen
        } else {
            Tile::DoorClosed
        };

        kitchen_door.draw(canvas, &texture, (8.0, GROUND_LEVEL), (1.0, 1.0))?;

        let child_door = if state.dad_dead {
            Tile::DoorOpen
        } else {
            Tile::DoorClosed
        };

        child_door.draw(canvas, &texture, (4.0, GROUND_LEVEL), (1.0, 1.0))?;

        if !state.dad_dead {
            Tile::ChildSticker.draw(canvas, &texture, (4.0, GROUND_LEVEL), (1.0, 1.0))?;
        }

        if state.dad_dead && !state.child_dead {
            canvas.copy(
                &blood,
                rect!(32, 0, 32, 32),
                rect!(
                    4.0 * PIXEL_PER_DOT,
                    (GROUND_LEVEL - 1.0) * PIXEL_PER_DOT,
                    PIXEL_PER_DOT,
                    PIXEL_PER_DOT
                ),
            )?;
        }

        if !state.coin_0 {
            draw_item(canvas, &texture, &Tile::Coin, 3.0, animation_timer)?;
        }
        if !state.coin_1 {
            draw_item(canvas, &texture, &Tile::Coin, 4.0, animation_timer)?;
        }
        if !state.coin_2 {
            draw_item(canvas, &texture, &Tile::Coin, 5.0, animation_timer)?;
        }
        if !state.coin_3 {
            draw_item(canvas, &texture, &Tile::Coin, 6.0, animation_timer)?;
        }

        Ok(())
    }

    fn prepare_items(&self, state: &State) -> Vec<(f64, Interactables)> {
        let mut items = Vec::new();
        items.push((PIXEL_PER_DOT, Interactables::ExitDoor));
        if !state.coin_0 {
            items.push(((PIXEL_PER_DOT * 3.), Interactables::Coin0));
        }
        if !state.coin_1 {
            items.push(((PIXEL_PER_DOT * 4.), Interactables::Coin1));
        }
        if !state.coin_2 {
            items.push(((PIXEL_PER_DOT * 5.), Interactables::Coin2));
        }
        if !state.coin_3 {
            items.push(((PIXEL_PER_DOT * 6.), Interactables::Coin3));
        }
        if state.coin_0 && state.coin_1 && state.coin_2 && state.coin_3 {
            items.push(((PIXEL_PER_DOT * 8.), Interactables::KitchenDoor));
        }

        if state.dad_dead {
            items.push(((PIXEL_PER_DOT * 4.), Interactables::ChildDoor));
        }

        items
    }
}

impl Scene for Entryway {
    fn draw_scenery(
        &self,
        state: &crate::state::State,
        canvas: &mut sdl2::render::WindowCanvas,
        animation_timer: f64,
    ) -> Result<(), String> {
        canvas.clear();
        self.draw_house(canvas, state, animation_timer)?;
        draw_ground(canvas)?;
        Ok(())
    }

    fn should_draw_interact_popup(&self, state: &crate::state::State, position: f64) -> bool {
        let items = self.prepare_items(state);
        let closest = closest_item_within_distance(items, position);
        closest.is_some()
    }

    fn interact(&self, state: &mut crate::state::State, position: f64) {
        let items = self.prepare_items(state);

        let closest = closest_item_within_distance(items, position);
        if let Some(item) = closest {
            state.send_audio("assets/click.ogg");
            match item {
                Interactables::ExitDoor => {
                    if state.dad_dead && !state.child_dead {
                        return;
                    }
                    state.scene_changed = Some((6.0, Scenes::Outside));
                    if !state.child_dead {
                        state.change_background_track("assets/outside.ogg");
                    }
                }
                Interactables::Coin0 => state.coin_0 = true,
                Interactables::Coin1 => state.coin_1 = true,
                Interactables::Coin2 => state.coin_2 = true,
                Interactables::Coin3 => state.coin_3 = true,
                Interactables::ChildDoor => {
                    state.change_background_track("assets/heartbeat-child-with-lemon.ogg");
                    state.scene_changed = Some((1.0, Scenes::ChildRoom));
                }
                Interactables::KitchenDoor => {
                    state.scene_changed = Some((1.0, Scenes::Kitchen));
                }
            }
        }
    }
}
