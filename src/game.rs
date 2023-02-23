pub struct Game {
    containers: Vec<Container>,
    player: Player,
}

pub struct Container {
    pos: (u16, u16),
}

pub struct Player {
    pos: (u16, u16),
}
