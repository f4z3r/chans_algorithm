use std::process;
use std::fs::File;
use std::io::prelude::*;

use csv;

use rand;
use rand::distributions::{
    IndependentSample,
    Range,
};

pub fn read_points() -> Vec<::Point> {
    let mut points = Vec::new();

    // Create the reader
    let mut reader = match csv::Reader::from_path("static/points_init.csv") {
        Ok(file)    => file,
        Err(_)      => {
            warn!("Could not open file containing initial points");
            process::exit(1);
        }
    };

    // Get the points in the csv file
    for result in reader.records() {
        match result {
            Err(err)    => warn!("Error reading record in csv file: {}", err),
            Ok(record)  => {
                if let Some(field0) = record.get(0) {
                    let x = match field0.parse::<f64>() {
                        Ok(res) => res,
                        Err(_)  => {
                            warn!("{:?} => x is not float. Using 0.", record);
                            0 as f64
                        }
                    };
                    if let Some(field1) = record.get(1) {
                        let y = match field1.parse::<f64>() {
                            Ok(res) => res,
                            Err(_)  => {
                                warn!("{:?} => y is not float. Using 0.", record);
                                0 as f64
                            }
                        };
                        points.push(::Point::new(x, y));
                    } else {
                        warn!("Record {:?} does not contain a float for y position.", record);
                    }
                } else {
                    warn!("Record {:?} does not contain a float for x position.", record);
                }
            }
        }
    }
    return points;
}

pub fn generate_points(radius: f64, n_points: i32) {
    let mut file = match File::create("static/points_init.csv") {
        Ok(res)     => res,
        Err(_)      => {
            warn!("Could not open file to generate random points.");
            process::exit(1);
        }
    };
    writeln!(file, "{},{}", "x value", "y value");
    let range = Range::new(-radius, radius);
    let mut rng = rand::thread_rng();
    let mut counter = 0;

    while counter < n_points {
        let x = range.ind_sample(&mut rng);
        let y = range.ind_sample(&mut rng);

        if x.powi(2) + y.powi(2) <= radius.powi(2) {
            writeln!(file, "{},{}", x, y);
            counter += 1;
        }
    }
}

pub fn print_points(filename: &str, points: &Vec<::Point>) {
    let mut file = match File::create(format!("static/{}", filename)) {
        Ok(res)     => res,
        Err(_)      => {
            warn!("Could not open file to print points.");
            process::exit(1);
        }
    };
    writeln!(file, "{},{}", "x value", "y value");
    for point in points {
        writeln!(file, "{},{}", point.x, point.y);
    }
}
