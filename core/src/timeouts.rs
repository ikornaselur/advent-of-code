use std::thread;
use std::time::{Duration, Instant};

pub fn run_with_timeout<F, T>(part_name: &str, f: F, input: &str) -> T
where
    F: FnOnce(&str) -> T + Send + 'static,
    T: Send + 'static,
{
    let input = input.to_string();
    let part_name = part_name.to_string();

    let handle = thread::spawn(move || f(&input));

    let start = Instant::now();
    let warning_threshold = Duration::from_secs(5);
    let mut warned = false;

    while !handle.is_finished() {
        thread::sleep(Duration::from_millis(100));

        let elapsed = start.elapsed();
        if elapsed >= warning_threshold && !warned {
            println!("Warning: {} is taking longer than 5 seconds...", part_name);
            warned = true;
        }
    }

    handle.join().unwrap()
}
