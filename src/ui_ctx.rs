enum Actor {
    Idle0,
    Idle1,
    Left0,
    Left1,
    Right0,
    Right1,
}

enum Sprite {
    Tile,
    Lemon(Actor),
    Father(Actor),
}

trait UiCtx {}
