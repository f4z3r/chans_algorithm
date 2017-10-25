extern crate chan;
#[macro_use]
extern crate log;
extern crate log4rs;

use chan::*;
use std::time::Instant;

fn main() {
    // Set up logger for library function issues
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    // Generate random points into file
    let n_points = 1_000;
    io::generate_points(10 as f64, n_points);

    // Time execution without reading from files
    let now = Instant::now();

    // Get records from file
    let mut points = io::read_points();
    util::order_points(&mut points);

    // Run the algorithm (For now only graham)
    let mut hull: Vec<Point> = Vec::with_capacity(1_000_000);
    hull = util::graham::scan(&mut points);

    // Get elapsed time and print to log
    let exec_time = now.elapsed();
    info!("Run with {} points. Took {}.{} seconds.", n_points, exec_time.as_secs(), exec_time.subsec_nanos());

    // Print the hull to another file
    io::print_points("hull.csv", &hull);
}
