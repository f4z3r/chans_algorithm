use std::cmp::Ordering;

pub fn scan(points: &mut Vec<::Point>) -> Vec<::Point> {
    // If less than 5 points, all points must be on hull
    if points.len() < 5 {
        return points.clone();
    }

    let mut hull: Vec<::Point> = Vec::new();

    // First two points must be on hull
    hull.push(points[0].clone());
    hull.push(points[1].clone());
    if cfg!(feature = "animate") {
        info!("{},{}", 0, 1);                     // For animation
        info!("{},{}", 1, 1);                     // For animation
    }

    for (idx, point) in points[2..].iter().enumerate() {
        if cfg!(feature = "animate") {
            info!("{},{}", idx+2, 1);                     // For animation
        }
        inc_hull(&mut hull, point);
    }

    return hull;
}

fn inc_hull(hull: &mut Vec<::Point>, new: &::Point) {
    let mut last_idx = hull.len() - 1;
    // Note that we compare the anticlockwise rotation of last_idx and new based at last_idx - 1.
    // Also note that compare(last_idx, new, _) == Greater here means that last_idx > new.
    while hull.len() > 2 && ::util::compare(&hull[last_idx], &new, &hull[last_idx - 1]) != Ordering::Less {
        if cfg!(feature = "animate") {
            info!("{},{}", last_idx, -1);                  // For animation
        }
        hull.pop();
        last_idx -= 1;
    }
    hull.push(new.clone());
}
