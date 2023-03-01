pub struct Stats {
    pub points: u32,
    pub upgrade: bool,
}

impl Stats {
    pub fn get_stats() -> Self {
        Self {
            points: 0,
            upgrade: false,
        }
    }
}
