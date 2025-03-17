use std::f64::consts::PI;

use crate::{
    ctx::Ctx,
    globals::GROUND_LEVEL,
    sprite::{Sprite, Tile},
};

pub trait CtxHelperExt<C: Ctx> {
    fn draw_item(&mut self, sprite: &impl Sprite, position: f64);
    fn draw_ground(&mut self);
    fn draw_wallpaper(&mut self, sprite: &impl Sprite);
}

impl<C: Ctx> CtxHelperExt<C> for C {
    fn draw_item(&mut self, sprite: &impl Sprite, position: f64) {
        let offset = (self.seconds_elapsed() * PI * 1.5).sin() * 0.125;
        self.draw_sprite((position, GROUND_LEVEL + offset), (1.0, 1.0), sprite);
    }

    fn draw_ground(&mut self) {
        self.draw_sprite((0.0, GROUND_LEVEL + 1.0), (10.0, 1.0), &Tile::Ground);
        self.draw_sprite(
            (0.0, GROUND_LEVEL + 2.0),
            (10.0, 10.0 - GROUND_LEVEL - 2.0),
            &Tile::Block,
        );
    }

    fn draw_wallpaper(&mut self, sprite: &impl Sprite) {
        for x in 0..10 {
            for y in 0..=GROUND_LEVEL as u32 {
                self.draw_sprite((x as f64, y as f64), (1.0, 1.0), sprite);
            }
        }
    }
}
