use std::path::Path;

use sdl2::render::Texture;
use sdl2::{image::LoadTexture, render::WindowCanvas};

use super::{InteractableId, Item, Items, Scene};
use crate::globals::GROUND_LEVEL;
use crate::helper::{draw_ground, draw_item};
use crate::state::State;
use crate::tileset::Tile;

use super::Scenes;

#[derive(Default)]
pub struct Tutorial;

enum Interactables {
    Bike,
    Coin,
}

impl Item for Interactables {
    fn id(&self) -> InteractableId {
        match self {
            Interactables::Bike => InteractableId(0),
            Interactables::Coin => InteractableId(1),
        }
    }
}

impl From<InteractableId> for Interactables {
    fn from(value: InteractableId) -> Self {
        match value {
            InteractableId(0) => Interactables::Bike,
            InteractableId(1) => Interactables::Coin,
            InteractableId(_) => unreachable!(),
        }
    }
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
        if !state.tutorial.coin {
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
}

impl Scene for Tutorial {
    fn draw(
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
        if !state.tutorial.coin {
            draw_item(canvas, &texture, &Tile::Coin, 4.0, animation_timer)?;
        }

        Ok(())
    }

    fn prepare_items(&self, state: &State) -> Items {
        let mut items = Items::new();
        if state.tutorial.coin {
            items.push(8, Interactables::Bike);
        } else {
            items.push(4, Interactables::Coin);
        };
        items
    }

    fn interact(&self, state: &mut State, position: f64) {
        let closest = self.closest_item_within_distance(state, position);
        let Some(item) = closest else { return };
        state.send_audio("assets/click.ogg");
        match Interactables::from(item.id()) {
            Interactables::Coin => state.tutorial.coin = true,
            Interactables::Bike => {
                state.scene_changed = Some((1, Scenes::Outside));
            }
        }
    }
}
