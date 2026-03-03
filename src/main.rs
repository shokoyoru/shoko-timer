use shoko_screen_timer::{active_timer, format_screen_time};
use std::io::{self, Write};

fn main() {
    active_timer(|active_millis| {
        print!(
            "\r{}", format_screen_time(active_millis)
        );
        io::stdout().flush().ok();
    });
}