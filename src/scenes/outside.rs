use std::path::Path;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::{image::LoadTexture, render::WindowCanvas};

use crate::globals::{GROUND_LEVEL, PIXEL_PER_DOT};
use crate::helper::{closest_item_within_distance, draw_ground, draw_item};
use crate::state::State;
use crate::tileset::Tile;
use crate::{rect, scene::Scene};

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
                (HOUSE_OFFSET + f64::from(i), GROUND_LEVEL),
                (1.0, 1.0),
            )?;
        }

        for x in 0..10 {
            Tile::Grass.draw(canvas, &texture, (f64::from(x), GROUND_LEVEL), (1.0, 1.0))?;
        }

        if state.child_dead {
            Tile::LemonSun.draw(canvas, &texture, (1.0, 1.0), (1.0, 1.0))?;
        } else {
            Tile::Sun.draw(canvas, &texture, (1.0, 1.0), (1.0, 1.0))?;
        }

        Tile::LeftTriangle.draw(
            canvas,
            &texture,
            (HOUSE_OFFSET, GROUND_LEVEL - 1.0),
            (1.0, 1.0),
        )?;
        Tile::Block.draw(
            canvas,
            &texture,
            (HOUSE_OFFSET + 1.0, GROUND_LEVEL - 1.0),
            (1.0, 1.0),
        )?;
        Tile::RightTriangle.draw(
            canvas,
            &texture,
            (HOUSE_OFFSET + 2.0, GROUND_LEVEL - 1.0),
            (1.0, 1.0),
        )?;

        let door_texture = if state.front_door_key_picked_up {
            Tile::DoorOpen
        } else {
            Tile::DoorClosed
        };

        door_texture.draw(
            canvas,
            &texture,
            (HOUSE_OFFSET + 1.0, GROUND_LEVEL),
            (1.0, 1.0),
        )?;

        let ascension_offset = (animation_timer * 4.0).floor() * 32.0;

        if state.child_dead {
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

        if !state.front_door_key_picked_up {
            draw_item(canvas, &texture, &Tile::Key, 3.0, animation_timer)?;
        }

        Ok(())
    }

    fn prepare_items(&self, state: &State) -> Vec<(f64, Interactables)> {
        let mut items = Vec::new();
        if state.front_door_key_picked_up {
            items.push(((PIXEL_PER_DOT * (HOUSE_OFFSET + 1.0)), Interactables::Door));
        }
        if !state.front_door_key_picked_up {
            items.push(((PIXEL_PER_DOT * 3.0), Interactables::Key));
        }

        if state.child_dead && !state.ascended {
            items.push(((PIXEL_PER_DOT * 3.0), Interactables::Ascension));
        }

        if state.confronted && !state.escaped && !state.child_dead {
            items.push(((PIXEL_PER_DOT * 1.0), Interactables::Bike));
        }

        items
    }
}

impl Scene for Outside {
    fn draw_scenery(
        &self,
        state: &crate::state::State,
        canvas: &mut sdl2::render::WindowCanvas,
        animation_timer: f64,
    ) -> Result<(), String> {
        if state.child_dead {
            canvas.set_draw_color(Color::RGB(217, 87, 99));
        } else {
            canvas.set_draw_color(Color::RGB(255, 255, 255));
        }
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
                Interactables::Key => state.front_door_key_picked_up = true,
                Interactables::Ascension => {
                    state.ascended = true;
                    state.play_ascension_track();
                }
                Interactables::Door => {
                    state.scene_changed = Some((1.0, Scenes::Entryway));

                    if !(state.confronted) {
                        state.change_background_track("assets/lemonhead.ogg");
                    }
                }
                Interactables::Bike => {
                    state.escaped = true;
                }
            }
        }
    }
}
