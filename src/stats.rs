use std::fs::{self, File};

use ron::{ser, ser::PrettyConfig, de};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Stats {
    pub points: u32,
    pub upgrade: bool,
}

impl Stats {
    pub fn get_stats() -> Self {
        let path = format!("{}/containers/stats.ron", dirs::data_dir().expect("unable to find data dir").to_str().unwrap());
        let file = match File::open(&path) {
            Ok(f) => f,
            Err(_) => {
                return Self {
                    points: 0,
                    upgrade: false,
                };
            }
        };

        let stats: Self = match de::from_reader(file) {
            Ok(stats) => stats,
            Err(_) => Self {
                points: 0,
                upgrade: false,
            }
        };

        stats
    }

    pub fn save(&self) {
        let config = PrettyConfig::new().depth_limit(2);
        let s = ser::to_string_pretty(self, config).expect("Serialization failed");

        let _ = fs::create_dir(dirs::data_dir().expect("unable to find data dir").to_str().unwrap().to_owned() + "/containers/");
        let _ = fs::write(dirs::data_dir().expect("unable to find data dir").to_str().unwrap().to_owned() + "/containers/stats.ron", s);
    }
}
