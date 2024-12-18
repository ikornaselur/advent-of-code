use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

pub fn run_with_timeout<F, T>(part_name: &str, f: F, input: &str) -> T
where
    F: FnOnce(&str) -> T,
    T: Send + 'static,
{
    let input = input.to_string();
    let part_name = part_name.to_string();

    let done = Arc::new(AtomicBool::new(false));
    let done_clone = done.clone();

    let monitor_name = part_name.clone();
    let monitor = thread::spawn(move || {
        let start = Instant::now();
        let warning_threshold = Duration::from_secs(5);
        let mut warned = false;

        while !done_clone.load(Ordering::Relaxed) {
            thread::sleep(Duration::from_millis(100));

            let elapsed = start.elapsed();
            if elapsed >= warning_threshold && !warned {
                println!(
                    "Warning: {} is taking longer than 5 seconds...",
                    monitor_name
                );
                warned = true;
            }
        }
    });

    let result = f(&input);

    done.store(true, Ordering::Relaxed);

    let _ = monitor.join();

    result
}
