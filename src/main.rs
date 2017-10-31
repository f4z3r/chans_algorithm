extern crate chan;
#[macro_use]
extern crate log;
extern crate log4rs;

use chan::*;
use std::time::Instant;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let n_points: u32 = args[1].parse().expect("Point number must be a positive integer");
    let n_threads: u32 = args[2].parse().expect("Thread number must be a positive integer");

    // Set up logger for library function issues
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    if cfg!(not(feature = "animate")) {
        info!("Running with {} threads on {} points...", n_threads, n_points);
    }

    // Generate random points into file
    io::generate_points(1000 as f64, n_points);

    // Time execution without reading from files
    let now = Instant::now();

    // Get records from file
    let mut points = io::read_points();
    // util::order_points(&mut points);
    if cfg!(feature = "animate") {
        io::print_points("all_points.csv", &points);             // For animation
    }

    // Run the algorithm (For now only graham)
    let mut hull: Vec<Point> = Vec::with_capacity(1_000_000);   // TODO: this could be made smaller maybe
    // hull = util::graham::scan(&mut points);
    hull = util::chan::algorithm(&mut points, n_threads);

    // Get elapsed time and print to log
    let exec_time = now.elapsed();
    if cfg!(not(feature = "animate")) {
        info!("Took {}.{} seconds.", exec_time.as_secs(), exec_time.subsec_nanos());
    }

    // Print the hull to another file
    io::print_points("hull.csv", &hull);
}
