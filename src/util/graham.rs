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

    for point in points[2..].iter() {
        inc_hull(&mut hull, point);
    }

    return hull;
}

fn inc_hull(hull: &mut Vec<::Point>, new: &::Point) {
    let mut last_idx = hull.len() - 1;
    // Note that we compare the anticlockwise rotation of last_idx and new based at last_idx - 1.
    // Also note that compare(last_idx, new, _) == Greater here means that last_idx > new.
    while hull.len() > 2 && ::util::compare(&hull[last_idx], &new, &hull[last_idx - 1]) != Ordering::Less {
        hull.pop();
        last_idx -= 1;
    }
    hull.push(new.clone());
}
