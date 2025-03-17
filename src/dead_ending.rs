use crate::{
    ctx::{Ctx, Key, Music, Rgb},
    globals::GROUND_LEVEL,
    sprite::Tile,
};

pub fn dead_ending<C: Ctx>(ctx: &mut C) -> Result<(), C::Error> {
    ctx.set_music(Music::RipBozo)?;

    loop {
        ctx.fill_background(Rgb(54, 54, 54))?;
        if ctx.key_down(Key::Quit) || ctx.key_down(Key::Interact) {
            break Ok(());
        }

        let offset = ctx.seconds_elapsed() % 5.0 * 4.0;

        ctx.draw_sprite((-3.0 + offset * 1.1, 1.0), (1.0, 1.0), &Tile::Cloud0)?;
        ctx.draw_sprite((-6.0 + offset, 2.0), (1.0, 1.0), &Tile::Cloud1)?;
        ctx.draw_sprite((-8.0 + offset * 0.9, 1.0), (1.0, 1.0), &Tile::Cloud2)?;
        ctx.draw_sprite((-1.0 + offset * 1.2, 2.0), (1.0, 1.0), &Tile::Cloud3)?;
        ctx.draw_sprite((5.0, GROUND_LEVEL), (1.0, 1.0), &Tile::Cross)?;

        ctx.draw_sprite((2.0, GROUND_LEVEL), (1.0, 1.0), &Tile::TreeTrunk)?;
        ctx.draw_sprite((2.0, GROUND_LEVEL - 1.0), (1.0, 1.0), &Tile::TreeLeaves)?;
        for i in 0..10 {
            ctx.draw_sprite((f64::from(i), GROUND_LEVEL), (1.0, 1.0), &Tile::Grass)?;
        }
        ctx.draw_sprite((0.0, GROUND_LEVEL + 1.0), (10.0, 1.0), &Tile::Ground)?;
        ctx.draw_sprite(
            (0.0, GROUND_LEVEL + 2.0),
            (10.0, 10.0 - GROUND_LEVEL - 2.0),
            &Tile::Block,
        )?;

        ctx.draw_sprite((5.0, GROUND_LEVEL + 2.0), (1.0, 0.0), &Tile::LemonSkull)?;

        let angel = if ctx.seconds_elapsed() % 0.2 > 0.1 {
            Tile::LemonAngel0
        } else {
            Tile::LemonAngel1
        };

        ctx.draw_sprite(
            (
                4.5,
                GROUND_LEVEL - (1.0 - (ctx.seconds_elapsed() * 4.0).sin() * 0.15),
            ),
            (1.0, 1.0),
            &angel,
        )?;

        ctx.draw_sprite(
            (2.0, 1.0 + (ctx.seconds_elapsed() * 1.2).sin() * 0.1),
            (6.0, 3.0),
            &Tile::GameOver,
        )?;

        ctx.post_step()?;
    }
}
