use std::{fs, thread};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn get_active() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis()).unwrap_or(0)
}

pub fn format_screen_time(active_millis: u64) -> String {
    let active_seconds = active_millis / 1000;
    let hours = active_seconds / 3600;
    let minutes = (active_seconds % 3600) / 60;
    let seconds = active_seconds % 60;
    let ms = active_millis % 1000;

    format!(
        "{{\"text\": \"{:02}:{:02}:{:02}.{:03}\", \"tooltip\": \"Screen Time\" }}",
        hours, minutes, seconds, ms)
}

pub fn active_timer(callback: fn(u64)) {
    let path = "/dev/shm/active_time.text";
    let mut active_start = match fs::read_to_string(path) {
        Ok(val) => val.as_str().trim().parse::<u128>().unwrap_or_else(|_| get_active()),
        Err(_) => {
            let active = get_active();
            let _ = fs::write(
                path,
                active.to_string()
            );
            active
        }
    };

    let mut last_active = get_active();

    loop {
        let active = get_active();
        if active > last_active + 5000 {
            let eepy_nix = active - last_active - 1000;
            active_start += eepy_nix;
            let _ = fs::write(path, active_start.to_string());
        }
        last_active = active;
        if active_start > active {
            active_start = active;
            let _ = fs::write(path, active_start.to_string());
        }
        let active_millis = active - active_start;
        callback(active_millis.try_into().unwrap_or(0));
        thread::sleep(Duration::from_millis(8));
    }

}
