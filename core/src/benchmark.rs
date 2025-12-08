use std::time::{Duration, Instant};

fn format_duration(d: Duration) -> String {
    let nanos = d.as_nanos();
    if nanos < 1_000 {
        format!("{}.0ns", nanos)
    } else if nanos < 1_000_000 {
        format!("{:.1}µs", nanos as f64 / 1000.0)
    } else if nanos < 1_000_000_000 {
        format!("{:.1}ms", nanos as f64 / 1_000_000.0)
    } else {
        format!("{:.1}s", nanos as f64 / 1_000_000_000.0)
    }
}

pub fn benchmark_parts<F, G>(part1: F, part2: G, input: &str)
where
    F: Fn(&str),
    G: Fn(&str),
{
    if !cfg!(debug_assertions) {
        println!();
        fn calibrate<F>(f: &F, input: &str) -> usize
        where
            F: Fn(&str),
        {
            let test_start = Instant::now();
            f(input);
            let single_run = test_start.elapsed().max(Duration::from_nanos(1));

            let target_time = Duration::from_secs(1);
            let iterations =
                (target_time.as_nanos() / single_run.as_nanos()).clamp(5, 10000) as usize;

            for _ in 0..iterations.min(100) {
                f(input);
            }

            iterations
        }

        fn bench<F>(name: &str, f: &F, input: &str)
        where
            F: Fn(&str),
        {
            let iterations = calibrate(f, input);
            let mut times = Vec::with_capacity(iterations);

            for _ in 0..iterations {
                let start = Instant::now();
                f(input);
                times.push(start.elapsed());
            }

            times.sort();
            let min = times[0];
            let max = times[times.len() - 1];
            let median = times[times.len() / 2];

            println!(
                "  {}: {} median ({} min, {} max)",
                name,
                format_duration(median),
                format_duration(min),
                format_duration(max)
            );
        }

        bench("part1", &part1, input);
        bench("part2", &part2, input);
    }
}
