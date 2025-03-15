use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};

use crate::globals::PIXEL_PER_DOT;
use crate::rect;

pub enum Tile {
    LemonAngel0,
    LemonAngel1,
    Cloud0,
    Cloud1,
    Cloud2,
    Cloud3,
    GameOver,
    TreeTrunk,
    LemonSkull,
    TreeLeaves,
    Cross,
    Bike,
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
    Weapon,
    Coin,
    Key,
    IntroductionText,
    IntroductionGoalsText,
    RememberText,
    VoicesText,
}

impl Tile {
    pub fn draw<'a>(
        &self,
        canvas: &mut WindowCanvas,
        texture: &'a Texture<'a>,
        dest_position: (f64, f64),
        dest_size: (f64, f64),
    ) -> Result<(), String> {
        let src_position = self.spritesheet_offset();
        let src_size = self.size();
        canvas.copy(
            texture,
            rect!(
                src_position.0 * 32.0,
                src_position.1 * 32.0,
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
            Tile::CityLayer2 | Tile::LemonCar0 | Tile::LemonCar1 => (64, 32),
            Tile::IntroductionText => (64, 16),
            Tile::IntroductionGoalsText => (128, 16),
            Tile::RememberText => (96, 16),
            Tile::VoicesText | Tile::GameOver | Tile::LemonSkull => (32, 16),
            _ => (32, 32),
        }
    }
    fn spritesheet_offset(&self) -> (f64, f64) {
        match self {
            Tile::LemonAngel0 => (6.0, 4.0),
            Tile::LemonAngel1 => (6.0, 5.0),
            Tile::Bike => (0.0, 6.0),
            Tile::LemonSkull => (4.0, 6.0),
            Tile::GameOver => (4.0, 6.5),
            Tile::Cross => (1.0, 6.0),
            Tile::Cloud0 => (6.0, 0.0),
            Tile::Cloud1 => (6.0, 1.0),
            Tile::Cloud2 => (6.0, 2.0),
            Tile::Cloud3 => (6.0, 3.0),
            Tile::TreeTrunk => (2.0, 6.0),
            Tile::TreeLeaves => (3.0, 6.0),
            Tile::Grass => (0.0, 0.0),
            Tile::HouseBrick => (1.0, 0.0),
            Tile::StripeWallpaper => (2.0, 0.0),
            Tile::LeftTriangle => (3.0, 0.0),
            Tile::RightTriangle => (3.0, 1.0),
            Tile::Block => (1.0, 1.0),
            Tile::Computer => (4.0, 0.0),
            Tile::DoorClosed => (5.0, 0.0),
            Tile::DoorOpen => (5.0, 1.0),
            Tile::Ground => (0.0, 1.0),
            Tile::TreeDayPicture => (0.0, 2.0),
            Tile::HousePicture => (1.0, 2.0),
            Tile::TreeNightPicture => (2.0, 2.0),
            Tile::LemonDayPicture => (3.0, 2.0),
            Tile::LemonNightPicture => (4.0, 2.0),
            Tile::Sun => (0.0, 3.0),
            Tile::LemonSun => (3.0, 3.0),
            Tile::Oven => (1.0, 3.0),
            Tile::Couch => (2.0, 3.0),
            Tile::KitchenBrick => (4.0, 3.0),
            Tile::ChildPoster => (0.0, 4.0),
            Tile::ChildSticker => (1.0, 4.0),
            Tile::DotWallpaper => (2.0, 4.0),
            Tile::Bed => (3.0, 4.0),
            Tile::CityLayer0 => (5.0, 4.0),
            Tile::CityLayer1 => (4.0, 4.0),
            Tile::CityLayer2 => (4.0, 5.0),
            Tile::LemonCar0 => (0.0, 5.0),
            Tile::LemonCar1 => (2.0, 5.0),
            Tile::OfficeChair => (4.0, 1.0),
            Tile::Weapon => (5.0, 3.0),
            Tile::Coin => (5.0, 2.0),
            Tile::Key => (2.0, 1.0),
            Tile::IntroductionText => (0.0, 7.0),
            Tile::IntroductionGoalsText => (0.0, 7.5),
            Tile::RememberText => (2.0, 7.0),
            Tile::VoicesText => (4.0, 7.5),
        }
    }
}
