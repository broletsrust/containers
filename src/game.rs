use std::time::Instant;

use rand::Rng;
use tui::style::Color;

pub struct Game {
    pub containers: Vec<Container>,
    pub player: Player,
    timer: Timer,
    container_falling: bool,
    pub over: bool,
    pub points: u8,
}

impl Game {
    pub fn new() -> Self {
        Self {
            containers: vec![],
            player: Player {
                pos: (4, 14),
                extra: (2, 0),
                timer: Timer::new(0),
                movement: Movement::None,
                jump_timer: Timer::new(100),
                jumping: false,
                falling: false,
            },
            timer: Timer::new(3000),
            container_falling: false,
            over: false,
            points: 0,
        }
    }

    pub fn update(&mut self) {
        if self.has_container_at(self.player.pos.0, self.player.pos.1) || self.player.extra.0 > 4 && self.has_container_at(self.player.pos.0 + 1, self.player.pos.1) ||
            self.player.extra.1 > 0 && self.has_container_at(self.player.pos.0, self.player.pos.1 + 1) || self.player.extra.1 > 0 && self.player.extra.0 > 4 && self.has_container_at(self.player.pos.0 + 1, self.player.pos.1 + 1) {
            self.over = true;
        }

        if self.over {
            return;
        }

        if self.timer.is_done() && self.container_falling {
            if self.containers.last().unwrap().is_on_ground(self) {
                self.points += 1;
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

        if self.timer.is_done() && !self.container_falling {
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

        let old_pos = self.player.pos;
        let old_extra = self.player.extra;
        self.player.update();

        if self.has_container_at(self.player.pos.0, self.player.pos.1) || self.player.extra.0 > 4 && self.has_container_at(self.player.pos.0 + 1, self.player.pos.1) ||
            self.player.extra.1 > 0 && self.has_container_at(self.player.pos.0, self.player.pos.1 + 1) || self.player.extra.1 > 0 && self.player.extra.0 > 4 && self.has_container_at(self.player.pos.0 + 1, self.player.pos.1 + 1) {
            self.player.pos = old_pos;
            self.player.extra = old_extra;
        }

        if !self.player.jumping && self.player.jump_timer.is_done() && !self.has_container_at(self.player.pos.0, self.player.pos.1 + 1) && self.player.pos.1 < 14 {
            if self.player.extra.0 <= 4 || self.player.extra.0 > 4 && !self.has_container_at(self.player.pos.0 + 1, self.player.pos.1 + 1) {
                if self.player.extra.1 >= 2 {
                    self.player.pos.1 += 1;
                    self.player.extra.1 = 0;
                } else {
                    self.player.extra.1 += 1;
                }
                self.player.falling = true;
            } else {
                self.player.falling = false;
            }
        } else {
            self.player.falling = false;
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
    pub extra: (u16, u16),
    timer: Timer,
    movement: Movement,
    jump_timer: Timer,
    jumping: bool,
    falling: bool,
}

impl Player {
    pub fn update(&mut self) {
        if self.timer.is_done() {
            match self.movement {
                Movement::Left => {
                    if self.extra.0 == 0 {
                        if self.pos.0 != 0 {
                            self.pos.0 -= 1;
                            self.extra.0 = 6;
                        }
                    } else {
                        self.extra.0 -= 1;
                    }
                }
                Movement::Right => {
                    if self.extra.0 == 6 && self.pos.0 < 9 {
                        self.pos.0 += 1;
                        self.extra.0 = 0;
                    } else if self.pos.0 == 9 {
                        if self.extra.0 < 4 {
                            self.extra.0 += 1;
                        }
                    } else {
                        self.extra.0 += 1;
                    }
                }
                Movement::None => {}
            }
            self.movement = Movement::None;
        }

        if self.jumping && self.jump_timer.is_done() {
            if self.extra.1 == 2 {
                self.extra.1 += 1;
                self.jumping = false;
                self.falling = true;
            } else {
                if self.extra.1 == 0 {
                    self.pos.1 = self.pos.1.saturating_sub(1);
                    self.extra.1 = 3
                }
                if self.extra.1 == 1 {
                    self.pos.1 = self.pos.1.saturating_sub(1);
                    self.extra.1 = 4;
                }
                self.extra.1 -= 2;
            }

            self.jump_timer.reset();
        }
    }

    pub fn move_left(&mut self) {
        if self.timer.is_done() {
            self.movement = Movement::Left;
            self.timer.len = 10;
            self.timer.reset();
        }
    }

    pub fn move_right(&mut self) {
        if self.timer.is_done() {
            self.movement = Movement::Right;
            self.timer.len = 10;
            self.timer.reset();
        }
    }

    pub fn jump(&mut self) {
        if !self.jumping && !self.falling && self.jump_timer.is_done() {
            self.jumping = true;
            self.jump_timer.reset();
        }
    }
}

#[derive(PartialEq)]
enum Movement {
    Left,
    Right,
    None,
}
