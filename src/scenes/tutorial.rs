use std::path::Path;

use sdl2::render::Texture;
use sdl2::{image::LoadTexture, render::WindowCanvas};

use crate::globals::{GROUND_LEVEL, PIXEL_PER_DOT};
use crate::helper::{closest_item_within_distance, draw_ground, draw_item};
use crate::scene::Scene;
use crate::state::State;
use crate::tileset::Tile;

use super::Scenes;

#[derive(Default)]
pub struct Tutorial;

enum Interactables {
    Bike,
    Coin,
}

impl Tutorial {
    fn draw_scenery(&self, canvas: &mut WindowCanvas, texture: &Texture) -> Result<(), String> {
        Tile::Bike.draw(canvas, texture, (8.0, GROUND_LEVEL), (1.0, 1.0))?;

        for x in 0..10 {
            Tile::Grass.draw(canvas, texture, (f64::from(x), GROUND_LEVEL), (1.0, 1.0))?;
        }

        Tile::Sun.draw(canvas, texture, (1.0, 1.0), (1.0, 1.0))?;

        Ok(())
    }

    fn draw_text(
        &self,
        canvas: &mut WindowCanvas,
        texture: &Texture,
        state: &State,
    ) -> Result<(), String> {
        if !state.tutorial_coin {
            Tile::IntroductionText.draw(canvas, texture, (3.0, 2.0), (4.0, 1.0))?;
            Tile::IntroductionGoalsText.draw(canvas, texture, (1.0, 3.0), (8.0, 1.0))?;
        } else {
            Tile::RememberText.draw(canvas, texture, (2.0, 2.5), (6.0, 1.0))?;
            Tile::VoicesText.draw(canvas, texture, (6.0, 9.25), (1.0, 0.5))?;
        }
        Tile::Bike.draw(canvas, texture, (8.0, GROUND_LEVEL), (1.0, 1.0))?;

        for x in 0..10 {
            Tile::Grass.draw(canvas, texture, (f64::from(x), GROUND_LEVEL), (1.0, 1.0))?;
        }

        Tile::Sun.draw(canvas, texture, (1.0, 1.0), (1.0, 1.0))?;

        Ok(())
    }

    fn prepare_items(&self, state: &State) -> Vec<(f64, Interactables)> {
        let mut items = Vec::new();
        if state.tutorial_coin {
            items.push(((PIXEL_PER_DOT * 8.0), Interactables::Bike));
        } else {
            items.push(((PIXEL_PER_DOT * 4.0), Interactables::Coin));
        };

        items
    }
}

impl Scene for Tutorial {
    fn draw_scenery(
        &self,
        state: &crate::state::State,
        canvas: &mut sdl2::render::WindowCanvas,
        animation_timer: f64,
    ) -> Result<(), String> {
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator.load_texture(Path::new("assets/tile.png"))?;

        draw_ground(canvas)?;
        self.draw_scenery(canvas, &texture)?;
        self.draw_text(canvas, &texture, state)?;
        if !state.tutorial_coin {
            draw_item(canvas, &texture, &Tile::Coin, 4.0, animation_timer)?;
        }

        Ok(())
    }

    fn should_draw_interact_popup(&self, state: &crate::state::State, position: f64) -> bool {
        let items = self.prepare_items(state);
        let closest = closest_item_within_distance(items, position);
        closest.is_some()
    }

    fn interact(&self, state: &mut State, position: f64) {
        let items = self.prepare_items(state);

        let closest = closest_item_within_distance(items, position);
        if let Some(item) = closest {
            state.send_audio("assets/click.ogg");
            match item {
                Interactables::Coin => state.tutorial_coin = true,
                Interactables::Bike => {
                    state.scene_changed = Some((1.0, Scenes::Outside));
                }
            }
        }
    }
}
