pub fn kelvin_to_rgb(kelvin: f32) -> (f32, f32, f32) {
    let t = kelvin / 100.0;
    let r: f32 = 1.0;
    let g = (99.4708025861 * t.ln() - 161.1195681661) / 255.0;
    let b = if kelvin <= 1900.0 { 0.0 } else { (138.5177312231 * (t - 10.0).ln() - 305.0447927307) / 255.0 };
    (r.clamp(0.0, 1.0), g.clamp(0.0, 1.0), b.clamp(0.0, 1.0))
}

pub fn calculate_current_temp(config: &crate::config::Config, current_time: chrono::NaiveTime) -> f32 {
    let start = chrono::NaiveTime::parse_from_str(&config.start_time, "%H:%M").unwrap_or_else(|_| chrono::NaiveTime::from_hms_opt(20, 0, 0).unwrap());
    let end = chrono::NaiveTime::parse_from_str(&config.end_time, "%H:%M").unwrap_or_else(|_| chrono::NaiveTime::from_hms_opt(6, 0, 0).unwrap());
    
    if current_time >= start || current_time < end {
        let diff = if current_time >= start {
            current_time.signed_duration_since(start)
        } else {
            current_time.signed_duration_since(chrono::NaiveTime::from_hms_opt(0,0,0).unwrap()) + chrono::NaiveTime::from_hms_opt(23,59,59).unwrap().signed_duration_since(start)
        };
        
        let diff_mins = diff.num_minutes() as u32;
        if diff_mins < config.transition_mins {
            let progress = diff_mins as f32 / config.transition_mins as f32;
            let smooth = progress * progress * (3.0 - 2.0 * progress);
            config.day_temp + (config.night_temp - config.day_temp) * smooth
        } else {
            config.night_temp
        }
    } else {
        config.day_temp
    }
}
