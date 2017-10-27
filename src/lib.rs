//! libs.rs
//!
//! Jakob Beckmann

extern crate csv;
extern crate rand;
#[macro_use]
extern crate log;
extern crate log4rs;

pub mod io;
pub mod util;

const EPSILON: f64 = 0.0001;

#[derive(Debug, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        return (self.x - other.x).abs() < EPSILON && (self.y - other.y).abs() < EPSILON;
    }
}

impl Eq for Point {}
