use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub day_temp: f32,
    pub night_temp: f32,
    pub start_time: String, // "HH:MM"
    pub end_time: String,   // "HH:MM"
    pub transition_mins: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            day_temp: 6500.0,
            night_temp: 3500.0,
            start_time: String::from("20:00"),
            end_time: String::from("06:00"),
            transition_mins: 60,
        }
    }
}
