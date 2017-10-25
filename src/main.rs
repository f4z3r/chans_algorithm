extern crate chan;
extern crate log;
extern crate log4rs;

use chan::*;

fn main() {
    // Set up logger for library function issues
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    // Generate random points into file
    io::generate_points(5 as f64, 1000);

    // Get records from file
    let mut points = io::read_points();
    util::order_points(&mut points);

    // Run the algorithm (For now only graham)
    let hull: Vec<Point> = util::graham::scan(&mut points);

    // Print the hull to another file
    io::print_points("hull.csv", &hull);
}
