use std::time::Instant;

use rand::Rng;
use tui::style::Color;

pub struct Game {
    pub containers: Vec<Container>,
    pub player: Player,
    timer: Timer,
    container_falling: bool,
}

impl Game {
    pub fn new() -> Self {
        Self {
            containers: vec![],
            player: Player {
                pos: (4, 14),
            },
            timer: Timer::new(3000),
            container_falling: false,
        }
    }

    pub fn update(&mut self) {
        if self.timer.is_done() && self.container_falling {
            if self.containers.last().unwrap().is_on_ground(self) {
                self.container_falling = false;
                self.timer.len = 3000;
                self.timer.reset();
            } else {
                if self.containers.last().unwrap().extra_fall_height < 2 {
                    self.containers.last_mut().unwrap().extra_fall_height += 1;
                } else {
                    self.containers.last_mut().unwrap().extra_fall_height = 0;
                    self.containers.last_mut().unwrap().pos.1 += 1;
                }
                self.timer.reset();
            }
        }

        if self.timer.is_done() && !self.container_falling && self.containers.len() < 150 {
            let mut x = rand::thread_rng().gen_range(0..10);
            while self.has_container_at(x, 0) {
                x = rand::thread_rng().gen_range(0..10);
            }
            self.containers.push(Container {
                pos: (x, 0),
                extra_fall_height: 0,
                color: Color::Rgb(rand::thread_rng().gen_range(0..=255), rand::thread_rng().gen_range(0..=255), rand::thread_rng().gen_range(0..=255)),
            });
            self.timer.len = 30;
            self.timer.reset();
            self.container_falling = true;
        }
    }

    pub fn has_container_at(&self, x: u16, y: u16) -> bool {
        for con in self.containers.iter() {
            if con.pos.0 == x && con.pos.1 == y {
                return true;
            }
        }
        false
    }
}

pub struct Container {
    pub pos: (u16, u16),
    pub extra_fall_height: u16,
    pub color: Color,
}

impl Container {
    pub fn is_on_ground(&self, game: &Game) -> bool {
        let mut on_ground = self.pos.1 >= 14;
        for con in game.containers.iter() {
            if on_ground {
                break;
            }
            if con.pos.0 == self.pos.0 && con.pos.1 == self.pos.1 + 1 {
                on_ground = true;
            }
        }
        on_ground
    }
}

pub struct Player {
    pub pos: (u16, u16),
}

struct Timer {
    start: Instant,
    len: u128,
}

impl Timer {
    pub fn new(len: u128) -> Self {
        Self {
            start: Instant::now(),
            len,
        }
    }

    pub fn is_done(&self) -> bool {
        self.start.elapsed().as_millis() >= self.len
    }

    pub fn reset(&mut self) {
        self.start = Instant::now();
    }
}
