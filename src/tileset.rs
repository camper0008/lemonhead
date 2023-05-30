use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};

use crate::globals::PIXEL_PER_DOT;
use crate::rect;

pub enum Tile {
    Grass,
    HouseBrick,
    StripeWallpaper,
    LeftTriangle,
    RightTriangle,
    Block,
    Computer,
    OfficeChair,
    DoorClosed,
    DoorOpen,
    Ground,
    TreeDayPicture,
    HousePicture,
    TreeNightPicture,
    LemonDayPicture,
    LemonNightPicture,
    Sun,
    LemonSun,
    Oven,
    Couch,
    KitchenBrick,
    ChildPoster,
    ChildSticker,
    DotWallpaper,
    Bed,
    CityLayer0,
    CityLayer1,
    CityLayer2,
    LemonCar0,
    LemonCar1,
}

impl Tile {
    pub fn draw<'a>(
        &self,
        canvas: &mut WindowCanvas,
        texture: &'a Texture<'a>,
        dest_position: (f64, f64),
        dest_size: (f64, f64),
    ) -> Result<(), String> {
        let src_position = self.position();
        let src_size = self.size();
        canvas.copy(
            texture,
            rect!(
                src_position.0 * 32,
                src_position.1 * 32,
                src_size.0,
                src_size.1
            ),
            rect!(
                dest_position.0 * PIXEL_PER_DOT,
                dest_position.1 * PIXEL_PER_DOT,
                dest_size.0 * PIXEL_PER_DOT,
                dest_size.1 * PIXEL_PER_DOT
            ),
        )
    }

    fn size(&self) -> (u32, u32) {
        match self {
            Tile::CityLayer2 => (64, 32),
            Tile::LemonCar0 => (64, 32),
            Tile::LemonCar1 => (64, 32),
            _ => (32, 32),
        }
    }
    fn position(&self) -> (u32, u32) {
        match self {
            Tile::Grass => (0, 0),
            Tile::HouseBrick => (1, 0),
            Tile::StripeWallpaper => (2, 0),
            Tile::LeftTriangle => (3, 0),
            Tile::RightTriangle => (3, 1),
            Tile::Block => (1, 1),
            Tile::Computer => (4, 0),
            Tile::DoorClosed => (5, 0),
            Tile::DoorOpen => (5, 1),
            Tile::Ground => (0, 1),
            Tile::TreeDayPicture => (0, 2),
            Tile::HousePicture => (1, 2),
            Tile::TreeNightPicture => (2, 2),
            Tile::LemonDayPicture => (3, 2),
            Tile::LemonNightPicture => (4, 2),
            Tile::Sun => (0, 3),
            Tile::LemonSun => (3, 3),
            Tile::Oven => (1, 3),
            Tile::Couch => (2, 3),
            Tile::KitchenBrick => (4, 3),
            Tile::ChildPoster => (0, 4),
            Tile::ChildSticker => (1, 4),
            Tile::DotWallpaper => (2, 4),
            Tile::Bed => (3, 4),
            Tile::CityLayer0 => (5, 4),
            Tile::CityLayer1 => (4, 4),
            Tile::CityLayer2 => (4, 5),
            Tile::LemonCar0 => (0, 5),
            Tile::LemonCar1 => (2, 5),
            Tile::OfficeChair => (4, 1),
        }
    }
}
