use std::{fs, thread};
use std::time::Duration;

use shoko_screen_timer::{get_active, format_screen_time};

fn main() {
    let path = "/dev/shm/active_time.text";
    let  mut active_start= match fs::read_to_string(path) {
        Ok(val) => val.as_str().trim().parse::<u64>().unwrap_or_else(|_| get_active()),
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
        if active > last_active + 5 {
            let eepy_nix = active - last_active - 1;
            active_start += eepy_nix;
            let _ = fs::write(path, active_start.to_string());
        }
        last_active = active;
        if active_start > active { active_start = active;
            let _ = fs::write(path, active_start.to_string()); }
        let active_seconds = active - active_start;

        println!(
            "{}", format_screen_time(active_seconds)
        );
        thread::sleep(Duration::from_secs(1));
    }
}