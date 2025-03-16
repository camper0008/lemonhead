pub trait Sprite {
    fn size(&self) -> (u32, u32);
    fn offset(&self) -> (u32, u32);
    fn path(&self) -> &'static str;
}

pub enum Lemonhead {
    Idle,
    IdleAlt,
    Left,
    LeftAlt,
    Right,
    RightAlt,
}

pub enum Npc {
    Idle,
    IdleAlt,
    Left,
    LeftAlt,
    Right,
    RightAlt,
    Dead,
}

pub enum Actor {
    Lemonhead(Lemonhead),
    Dad(Npc),
    Child(Npc),
}
pub enum ActorState {
    Idle,
    Left,
    Right,
}

impl Actor {
    pub fn animated_lemonhead(state: ActorState, seconds_elapsed: f64) -> Actor {
        let use_normal = seconds_elapsed % 1.0 < 0.5;
        let v = if use_normal {
            match state {
                ActorState::Idle => Lemonhead::Idle,
                ActorState::Left => Lemonhead::Left,
                ActorState::Right => Lemonhead::Right,
            }
        } else {
            match state {
                ActorState::Idle => Lemonhead::IdleAlt,
                ActorState::Left => Lemonhead::LeftAlt,
                ActorState::Right => Lemonhead::RightAlt,
            }
        };
        Actor::Lemonhead(v)
    }
    pub fn animated_npc<F: Fn(Npc) -> Actor>(
        npc: F,
        state: ActorState,
        seconds_elapsed: f64,
    ) -> Actor {
        let use_normal = seconds_elapsed % 1.0 < 0.5;
        let v = if use_normal {
            match state {
                ActorState::Idle => Npc::Idle,
                ActorState::Left => Npc::Left,
                ActorState::Right => Npc::Right,
            }
        } else {
            match state {
                ActorState::Idle => Npc::IdleAlt,
                ActorState::Left => Npc::LeftAlt,
                ActorState::Right => Npc::RightAlt,
            }
        };
        npc(v)
    }
}

impl Sprite for Actor {
    fn size(&self) -> (u32, u32) {
        (32, 32)
    }

    fn offset(&self) -> (u32, u32) {
        let x = match self {
            Actor::Lemonhead(lemon) => match lemon {
                Lemonhead::Idle => 0,
                Lemonhead::IdleAlt => 1,
                Lemonhead::Right => 2,
                Lemonhead::RightAlt => 3,
                Lemonhead::Left => 4,
                Lemonhead::LeftAlt => 5,
            },
            Actor::Child(npc) | Actor::Dad(npc) => match npc {
                Npc::Idle => 0,
                Npc::IdleAlt => 1,
                Npc::Right => 2,
                Npc::RightAlt => 3,
                Npc::Left => 4,
                Npc::LeftAlt => 5,
                Npc::Dead => 6,
            },
        };
        (x * 32, 0)
    }

    fn path(&self) -> &'static str {
        match self {
            Actor::Lemonhead(_) => "assets/lemonhead.png",
            Actor::Dad(_) => "assets/dad.png",
            Actor::Child(_) => "assets/child.png",
        }
    }
}

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
    Logo,
}

impl Sprite for Tile {
    fn size(&self) -> (u32, u32) {
        let (x, y) = match self {
            Tile::Logo => (4, 4),
            Tile::CityLayer2 | Tile::LemonCar0 | Tile::LemonCar1 => (4, 1),
            Tile::IntroductionText => (4, 1),
            Tile::IntroductionGoalsText => (8, 1),
            Tile::RememberText => (6, 1),
            Tile::VoicesText | Tile::GameOver | Tile::LemonSkull => (2, 1),
            _ => (2, 2),
        };
        let (x, y) = (x * 16, y * 16);
        (x, y)
    }

    fn offset(&self) -> (u32, u32) {
        let (x, y) = match self {
            Tile::LemonAngel0 => (12, 8),
            Tile::LemonAngel1 => (12, 10),
            Tile::Bike => (0, 12),
            Tile::LemonSkull => (8, 12),
            Tile::GameOver => (8, 13),
            Tile::Cross => (2, 12),
            Tile::Cloud0 => (12, 0),
            Tile::Cloud1 => (12, 2),
            Tile::Cloud2 => (12, 4),
            Tile::Cloud3 => (12, 6),
            Tile::TreeTrunk => (4, 12),
            Tile::TreeLeaves => (6, 12),
            Tile::Grass => (0, 0),
            Tile::HouseBrick => (2, 0),
            Tile::StripeWallpaper => (4, 0),
            Tile::LeftTriangle => (6, 0),
            Tile::RightTriangle => (6, 2),
            Tile::Block => (2, 2),
            Tile::Computer => (8, 0),
            Tile::DoorClosed => (10, 0),
            Tile::DoorOpen => (10, 2),
            Tile::Ground => (0, 2),
            Tile::TreeDayPicture => (0, 4),
            Tile::HousePicture => (2, 4),
            Tile::TreeNightPicture => (4, 4),
            Tile::LemonDayPicture => (6, 4),
            Tile::LemonNightPicture => (8, 4),
            Tile::Sun => (0, 6),
            Tile::LemonSun => (6, 6),
            Tile::Oven => (2, 6),
            Tile::Couch => (4, 6),
            Tile::KitchenBrick => (8, 6),
            Tile::ChildPoster => (0, 8),
            Tile::ChildSticker => (2, 8),
            Tile::DotWallpaper => (4, 8),
            Tile::Bed => (6, 8),
            Tile::CityLayer0 => (10, 8),
            Tile::CityLayer1 => (8, 8),
            Tile::CityLayer2 => (8, 10),
            Tile::LemonCar0 => (0, 10),
            Tile::LemonCar1 => (4, 10),
            Tile::OfficeChair => (8, 2),
            Tile::Weapon => (10, 6),
            Tile::Coin => (10, 4),
            Tile::Key => (4, 2),
            Tile::IntroductionText => (0, 14),
            Tile::IntroductionGoalsText => (0, 15),
            Tile::RememberText => (4, 14),
            Tile::VoicesText => (8, 15),
            Tile::Logo => (10, 12),
        };
        let (x, y) = (x * 16, y * 16);
        (x, y)
    }

    fn path(&self) -> &'static str {
        "assets/tile.png"
    }
}
