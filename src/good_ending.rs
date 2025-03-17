use std::f64::consts::PI;

use crate::{
    ctx::{Ctx, Key, Music, Rgb},
    sprite::Tile,
};

fn draw_layer_0<C: Ctx>(ctx: &mut C) {
    let animation_timer = ctx.seconds_elapsed() % 10.0;

    for i in 0..2 {
        let position = -animation_timer + f64::from(i * 10);
        ctx.draw_sprite((position, 1.0), (10.0, 9.0), &Tile::CityLayer0);
    }
}

fn draw_layer_1<C: Ctx>(ctx: &mut C) {
    let animation_timer = (ctx.seconds_elapsed() * 2.5) % 10.0;

    for i in 0..2 {
        let position = -animation_timer + f64::from(i * 10);
        ctx.draw_sprite((position, 1.0), (10.0, 9.0), &Tile::CityLayer1);
    }
}

fn draw_layer_2<C: Ctx>(ctx: &mut C) {
    let animation_timer = ctx.seconds_elapsed() * 5.0 % 16.0;

    for i in 0..3 {
        let position = -animation_timer + f64::from(i * 16);
        ctx.draw_sprite((position, 1.0), (16.0, 8.0), &Tile::CityLayer2);
    }
}

pub fn good_ending<C: Ctx>(ctx: &mut C) -> Result<(), C::Error> {
    ctx.set_music(Music::Rich)?;

    loop {
        ctx.setup()?;
        if ctx.key_down(Key::Quit) || ctx.key_down(Key::Interact) {
            break Ok(());
        }
        ctx.draw_background_fill(Rgb(255, 255, 255));

        draw_layer_0(ctx);
        draw_layer_1(ctx);
        draw_layer_2(ctx);

        let x_offset = (ctx.seconds_elapsed() % 1.0 * PI * 2.0).sin() * 0.125;
        let car = if ctx.seconds_elapsed() % 0.2 < 0.1 {
            Tile::LemonCar0
        } else {
            Tile::LemonCar1
        };

        ctx.draw_sprite((4.0 + x_offset, 8.0), (2.0, 1.0), &car);
        ctx.draw_sprite((0.0, 9.0), (10.0, 1.0), &Tile::Ground);
        ctx.finish()?;
    }
}
