use std::path::Path;

use sdl2::rect::Rect;
use sdl2::{image::LoadTexture, render::WindowCanvas};

use crate::globals::{GROUND_LEVEL, PIXEL_PER_DOT};
use crate::helper::closest_item_within_distance;
use crate::state::State;
use crate::{rect, scene::Scene};

use super::Scenes;

#[derive(Default)]
pub struct ChildRoom {}

enum Interactables {
    ExitDoor,
    Child,
}

impl ChildRoom {
    fn draw_house(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        let texture_creator = canvas.texture_creator();
        let door = texture_creator.load_texture(Path::new("assets/door.png"))?;
        let ground = texture_creator.load_texture(Path::new("assets/ground.png"))?;

        for x in 0..10 {
            for y in 0..=GROUND_LEVEL {
                canvas.copy(
                    &ground,
                    rect!(64, 128, 32, 32),
                    rect!(
                        x * PIXEL_PER_DOT,
                        y * PIXEL_PER_DOT,
                        PIXEL_PER_DOT,
                        PIXEL_PER_DOT
                    ),
                )?;
            }
        }

        canvas.copy(
            &door,
            rect!(32, 0, 32, 32),
            rect!(
                PIXEL_PER_DOT,
                GROUND_LEVEL * PIXEL_PER_DOT,
                PIXEL_PER_DOT,
                PIXEL_PER_DOT
            ),
        )?;

        canvas.copy(
            &ground,
            rect!(0, 128, 32, 32),
            rect!(
                PIXEL_PER_DOT * 3,
                (GROUND_LEVEL) * PIXEL_PER_DOT,
                PIXEL_PER_DOT,
                PIXEL_PER_DOT
            ),
        )?;

        canvas.copy(
            &ground,
            rect!(128, 0, 32, 32),
            rect!(
                PIXEL_PER_DOT * 4,
                (GROUND_LEVEL) * PIXEL_PER_DOT,
                PIXEL_PER_DOT,
                PIXEL_PER_DOT
            ),
        )?;

        canvas.copy(
            &ground,
            rect!(128, 32, 32, 32),
            rect!(
                PIXEL_PER_DOT * 4,
                (GROUND_LEVEL) * PIXEL_PER_DOT,
                PIXEL_PER_DOT,
                PIXEL_PER_DOT
            ),
        )?;

        canvas.copy(
            &ground,
            rect!(96, 128, 32, 32),
            rect!(
                PIXEL_PER_DOT * 6,
                (GROUND_LEVEL) * PIXEL_PER_DOT,
                PIXEL_PER_DOT,
                PIXEL_PER_DOT
            ),
        )?;

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

        let offset = if state.child_dead {
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
                PIXEL_PER_DOT * 5,
                (GROUND_LEVEL) * PIXEL_PER_DOT,
                PIXEL_PER_DOT,
                PIXEL_PER_DOT
            ),
        )?;

        if state.child_stabs > 0 {
            canvas.copy(
                &blood,
                rect!(0, 0, 32, 32),
                rect!(
                    PIXEL_PER_DOT * 5,
                    (GROUND_LEVEL) * PIXEL_PER_DOT,
                    PIXEL_PER_DOT,
                    PIXEL_PER_DOT
                ),
            )?;
        }
        if state.child_stabs > 1 {
            canvas.copy(
                &blood,
                rect!(0, 32, 32, 32),
                rect!(
                    PIXEL_PER_DOT * 4,
                    (GROUND_LEVEL) * PIXEL_PER_DOT,
                    PIXEL_PER_DOT,
                    PIXEL_PER_DOT
                ),
            )?;
        }
        if state.child_stabs > 2 {
            canvas.copy(
                &blood,
                rect!(32, 32, 32, 32),
                rect!(
                    PIXEL_PER_DOT * 6,
                    (GROUND_LEVEL) * PIXEL_PER_DOT,
                    PIXEL_PER_DOT,
                    PIXEL_PER_DOT
                ),
            )?;
        }

        Ok(())
    }

    fn draw_ground(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        let texture_creator = canvas.texture_creator();
        let ground_texture = texture_creator.load_texture(Path::new("assets/ground.png"))?;

        for x in 0..10 {
            canvas.copy(
                &ground_texture,
                rect!(0, 32, 32, 32),
                rect!(
                    x * PIXEL_PER_DOT,
                    (GROUND_LEVEL + 1) * PIXEL_PER_DOT,
                    PIXEL_PER_DOT,
                    PIXEL_PER_DOT
                ),
            )?;
        }

        for x in 0..10 {
            for y in (GROUND_LEVEL + 2)..10 {
                canvas.copy(
                    &ground_texture,
                    rect!(32, 32, 32, 32),
                    rect!(
                        x * PIXEL_PER_DOT,
                        y * PIXEL_PER_DOT,
                        PIXEL_PER_DOT,
                        PIXEL_PER_DOT
                    ),
                )?;
            }
        }

        Ok(())
    }

    fn prepare_items(&self, state: &State) -> Vec<(f64, Interactables)> {
        let mut items = Vec::new();
        if state.child_stabs < 3 {
            items.push((f64::from(PIXEL_PER_DOT * 5), Interactables::Child));
        }

        if state.child_dead {
            items.push((f64::from(PIXEL_PER_DOT), Interactables::ExitDoor));
        }

        items
    }
}

impl Scene for ChildRoom {
    fn draw_scenery(
        &self,
        state: &crate::state::State,
        canvas: &mut sdl2::render::WindowCanvas,
        animation_timer: f64,
    ) -> Result<(), String> {
        canvas.clear();
        self.draw_house(canvas)?;
        self.draw_ground(canvas)?;
        self.draw_child(canvas, state, animation_timer)?;
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
            match item {
                Interactables::ExitDoor => {
                    state.send_audio("assets/click.ogg");
                    if state.child_stabs < 3 {
                        return;
                    }
                    state.scene_changed = Some((4.0, Scenes::Entryway));
                }
                Interactables::Child => {
                    state.send_audio("assets/stab.ogg");
                    state.child_dead = true;
                    state.child_stabs += 1;
                    if state.child_stabs == 1 {
                        state.change_background_track("assets/heartbeat-child.ogg");
                    } else if state.child_stabs == 2 {
                        state.change_background_track("assets/heartbeat.ogg");
                    } else if state.child_stabs == 3 {
                        state.stop_background_track();
                    }
                }
            }
        }
    }
}
