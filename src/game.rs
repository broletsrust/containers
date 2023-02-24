pub struct Game {
    pub containers: Vec<Container>,
    pub player: Player,
}

impl Game {
    pub fn new() -> Self {
        Self {
            containers: vec![],
            player: Player {
                pos: (0, 0),
            },
        }
    }

    pub fn update(&mut self) {

    }
}

pub struct Container {
    pub pos: (u16, u16),
}

pub struct Player {
    pub pos: (u16, u16),
}
