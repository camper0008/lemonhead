use std::path::Path;

use sdl2::render::Texture;
use sdl2::{image::LoadTexture, render::WindowCanvas};

use super::{InteractableId, Item, Items, Scene};
use crate::globals::GROUND_LEVEL;
use crate::helper::{draw_ground, draw_item};
use crate::logic::Unit;
use crate::state::State;
use crate::sprite::Generic;

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
        Generic::Bike.draw(
            canvas,
            texture,
            (Unit::new(8), GROUND_LEVEL),
            (Unit::new(1), Unit::new(1)),
        )?;

        for x in 0..10 {
            Generic::Grass.draw(
                canvas,
                texture,
                (Unit::new(x), GROUND_LEVEL),
                (Unit::new(1), Unit::new(1)),
            )?;
        }

        Generic::Sun.draw(
            canvas,
            texture,
            (Unit::new(1), Unit::new(1)),
            (Unit::new(1), Unit::new(1)),
        )?;

        Ok(())
    }

    fn draw_text(
        &self,
        canvas: &mut WindowCanvas,
        texture: &Texture,
        state: &State,
    ) -> Result<(), String> {
        if !state.tutorial.coin {
            Generic::IntroductionText.draw(
                canvas,
                texture,
                (Unit::new(3), Unit::new(2)),
                (Unit::new(4), Unit::new(1)),
            )?;
            Generic::IntroductionGoalsText.draw(
                canvas,
                texture,
                (Unit::new(1), Unit::new(3)),
                (Unit::new(8), Unit::new(1)),
            )?;
        } else {
            Generic::RememberText.draw(
                canvas,
                texture,
                (Unit::new(2), Unit::new_decimal(2.5)),
                (Unit::new(6), Unit::new(1)),
            )?;
            Generic::VoicesText.draw(
                canvas,
                texture,
                (Unit::new(6), Unit::new_decimal(9.25)),
                (Unit::new(1), Unit::new_decimal(0.5)),
            )?;
        }
        Generic::Bike.draw(
            canvas,
            texture,
            (Unit::new(8), GROUND_LEVEL),
            (Unit::new(1), Unit::new(1)),
        )?;

        for x in 0..10 {
            Generic::Grass.draw(
                canvas,
                texture,
                (Unit::new(x), GROUND_LEVEL),
                (Unit::new(1), Unit::new(1)),
            )?;
        }

        Generic::Sun.draw(
            canvas,
            texture,
            (Unit::new(1), Unit::new(1)),
            (Unit::new(1), Unit::new(1)),
        )?;

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
            draw_item(canvas, &texture, &Generic::Coin, 4, animation_timer)?;
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

    fn interact(&self, state: &mut State, position: Unit) {
        let closest = self.closest_item_within_distance(state, position);
        let Some(item) = closest else { return };
        state.send_audio("assets/click.ogg");
        match Interactables::from(item.id()) {
            Interactables::Coin => state.tutorial.coin = true,
            Interactables::Bike => {
                state.scene_changed = Some((1.into(), Scenes::Outside));
            }
        }
    }
}
