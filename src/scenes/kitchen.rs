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
pub struct Kitchen {}

enum Interactables {
    ExitDoor,
    LivingRoomDoor,
    Weapon,
    Coin4,
    Coin5,
    Coin6,
}

impl Kitchen {
    fn draw_house(&self, canvas: &mut WindowCanvas, state: &State) -> Result<(), String> {
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator.load_texture(Path::new("assets/tile.png"))?;
        let blood = texture_creator.load_texture(Path::new("assets/blood.png"))?;

        draw_wallpaper(canvas, &texture, Tile::KitchenBrick)?;

        Tile::DoorOpen.draw(canvas, &texture, (1.0, GROUND_LEVEL), (1.0, 1.0))?;

        Tile::Oven.draw(canvas, &texture, (3.0, GROUND_LEVEL), (1.0, 1.0))?;

        let picture = if state.dad_dead {
            Tile::LemonNightPicture
        } else {
            Tile::TreeNightPicture
        };

        picture.draw(canvas, &texture, (9.0, GROUND_LEVEL), (1.0, 1.0))?;

        let living_room_door = if state.coin_4 && state.coin_5 && state.coin_6 {
            Tile::DoorOpen
        } else {
            Tile::DoorClosed
        };

        living_room_door.draw(canvas, &texture, (8.0, GROUND_LEVEL), (1.0, 1.0))?;

        if state.dad_dead {
            canvas.copy(
                &blood,
                rect!(0, 32, 64, 32),
                rect!(
                    PIXEL_PER_DOT * 3.0,
                    GROUND_LEVEL * PIXEL_PER_DOT,
                    PIXEL_PER_DOT * 2.0,
                    PIXEL_PER_DOT
                ),
            )?;

            canvas.copy(
                &blood,
                rect!(64, 0, 32, 32),
                rect!(
                    6.0 * PIXEL_PER_DOT,
                    (GROUND_LEVEL - 1.0) * PIXEL_PER_DOT,
                    PIXEL_PER_DOT,
                    PIXEL_PER_DOT
                ),
            )?;
        }

        Ok(())
    }

    fn prepare_items(&self, state: &State) -> Vec<(f64, Interactables)> {
        let mut items = Vec::new();

        items.push((f64::from(PIXEL_PER_DOT), Interactables::ExitDoor));
        if !state.coin_4 {
            items.push((f64::from(PIXEL_PER_DOT * 3.0), Interactables::Coin4));
        }
        if !state.coin_5 {
            items.push((f64::from(PIXEL_PER_DOT * 4.), Interactables::Coin5));
        }
        if !state.coin_6 {
            items.push((f64::from(PIXEL_PER_DOT * 5.), Interactables::Coin6));
        }

        if state.coin_4 && state.coin_5 && state.coin_6 {
            items.push((f64::from(PIXEL_PER_DOT * 8.), Interactables::LivingRoomDoor));
        }

        if state.confronted && !state.weapon_picked_up {
            items.push((f64::from(PIXEL_PER_DOT * 6.), Interactables::Weapon));
        }

        items
    }
}

impl Scene for Kitchen {
    fn draw_scenery(
        &self,
        state: &crate::state::State,
        canvas: &mut sdl2::render::WindowCanvas,
        animation_timer: f64,
    ) -> Result<(), String> {
        canvas.clear();
        self.draw_house(canvas, state)?;
        draw_ground(canvas)?;
        if !state.coin_4 {
            draw_item(canvas, 3.0, "assets/coin.png", animation_timer)?;
        }
        if !state.coin_5 {
            draw_item(canvas, 4.0, "assets/coin.png", animation_timer)?;
        }
        if !state.coin_6 {
            draw_item(canvas, 5.0, "assets/coin.png", animation_timer)?;
        }
        if !state.weapon_picked_up {
            draw_item(canvas, 6.0, "assets/weapon.png", animation_timer)?;
        }
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
                    if state.confronted && !state.dad_dead {
                        return;
                    }
                    state.scene_changed = Some((8.0, Scenes::Entryway));
                }
                Interactables::Coin4 => state.coin_4 = true,
                Interactables::Coin5 => state.coin_5 = true,
                Interactables::Coin6 => state.coin_6 = true,
                Interactables::Weapon => {
                    state.weapon_picked_up = true;
                    state.change_background_track("assets/heartbeat.ogg");
                }
                Interactables::LivingRoomDoor => {
                    if state.confronted && !state.weapon_picked_up {
                        return;
                    }

                    let scene = if state.weapon_picked_up {
                        Scenes::MurderLivingRoom
                    } else {
                        Scenes::LivingRoom
                    };
                    state.scene_changed = Some((1.0, scene));
                }
            }
        }
    }
}
