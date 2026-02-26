use shoko_screen_timer::{active_timer, format_screen_time};

fn main() {
    active_timer(|active_seconds| {
        println!(
            "{}", format_screen_time(active_seconds)
        );
    });
}