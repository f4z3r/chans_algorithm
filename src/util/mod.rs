use std::cmp::Ordering;

pub mod graham;

pub fn vector_length_squared(vector: (f64, f64)) -> f64 {
    let (x, y) = vector;
    return x.powi(2) + y.powi(2);
}


pub fn order_points(points: &mut Vec<::Point>) {
    // Find the points with lowest y coordinate and if two points with same lowest y coordinate take lowest x as well
    let mut lowest = points[0].clone();
    let mut lowest_idx = 0;
    for (idx, point) in points.iter().enumerate() {
        if (point.y - lowest.y).abs() < ::EPSILON {
            if point.x < lowest.x {
                lowest = point.clone();
                lowest_idx = idx;
            }
        } else if point.y < lowest.y {
            lowest = point.clone();
            lowest_idx = idx;
        }
    }

    // Remove lowest point
    points.swap_remove(lowest_idx);

    // Sort points according to lowest point and reinsert lowest point as first point
    points.sort_unstable_by(|a, b| compare(a, b, &lowest));
    points.insert(0, lowest);
}

fn compare(a: &::Point, b: &::Point, base: &::Point) -> Ordering {
    let vec_a_base    = (base.x - a.x, base.y - a.y);
    let vec_a_b   = (b.x - a.x, b.y - a.y);
    let cross_prod = (vec_a_base.0 * vec_a_b.1) - (vec_a_base.1 * vec_a_b.0);
    // If the cross product is negative, `other` has larger polar angle
    if cross_prod > ::EPSILON {
        // self > other
        return Ordering::Greater;
    } else if cross_prod < - ::EPSILON {
        // self < other
        return Ordering::Less;
    }
    return Ordering::Equal;
}


#[cfg(test)]
mod tests {
    use super::*;

    // Unit testing for order_points
    #[test]
    fn test_order_points() {
        unimplemented!();
    }

    // Unit testing for compare
    #[test]
    fn test_compare() {
        let p1 = ::Point::new(0 as f64, 0 as f64);
        let p2 = ::Point::new(1 as f64, 1 as f64);
        let p3 = ::Point::new(2 as f64, 2 as f64);
        assert_eq!(compare(p1, p2, p3), Ordering::Equal);

        let base = ::Point::new(0 as f64, 0 as f64);
        let p1 = ::Point::new(1 as f64, 1 as f64);
        let p2 = ::Point::new(0 as f64, 1 as f64);
        assert_eq!(compare(p1, p2, base), Ordering::Greater);

        assert_eq!(compare(p2, p1, base), Ordering::Less);
    }
}
