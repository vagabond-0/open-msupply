use std::sync::Mutex;

use chrono::*;

pub struct BenchPoint {
    // file!() but would require bench point to be a macro
    // file: String,
    // line!() but would require bench point to be a macro
    // line: String,
    identifier: String,
    time: DateTime<Utc>,
}

// Make the bench library yow !
pub static BENCH_POINTS: Mutex<Vec<BenchPoint>> = Mutex::new(Vec::new());

// ideally this would be a macro to capture file and location
pub fn bench_point(identifier: &str) {
    BENCH_POINTS.lock().unwrap().push(BenchPoint {
        identifier: identifier.to_string(),
        time: Utc::now(),
    })
}

pub fn bench_clear(identifier: &str) {
    BENCH_POINTS.lock().unwrap().push(BenchPoint {
        identifier: identifier.to_string(),
        time: Utc::now(),
    })
}

pub fn bench_results() {
    let mut bench_points = BENCH_POINTS.lock().unwrap();
    bench_points.sort_by(|a, b| a.time.cmp(&b.time));

    let Some(start_time) = bench_points.first() else {
        return;
    };

    let mut previous_time = start_time.time;

    for BenchPoint { identifier, time } in bench_points.iter() {
        let diff_from_start = *time - start_time.time;
        let diff_from_previous = *time - previous_time;
        println!("{identifier: <30}\t{diff_from_start: <20}\t{diff_from_previous: <20}");
        previous_time = *time;
    }
}
