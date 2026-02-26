use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_active() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}

pub fn format_screen_time(active_seconds: u64) -> String {
    let hours = active_seconds / 3600;
    let minutes = (active_seconds % 3600) / 60;
    let seconds = active_seconds % 60;

    format!(
    "{{\"text\": \"{:02}:{:02}:{:02}\", \"tooltip\": \"Screen Time\" }}",
    hours, minutes, seconds)
}