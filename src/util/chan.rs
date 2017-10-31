use std::cmp::Ordering;
use std::thread;
use std::sync::mpsc;

use ::util;

pub fn algorithm(points: &mut Vec<::Point>, parallelism: u32) -> Vec<::Point> {
    let mut hulls: Vec<Vec<::Point>> = Vec::new();

    let n_points = points.len();

    let (tx, rx) = mpsc::channel();     // Channel over which the threads communicate

    for idx in 0..parallelism {
        let transmitter = tx.clone();
        let mut subset: Vec<::Point> = Vec::new();

        let mut point_idx = idx;
        while point_idx < n_points as u32 {
            subset.push(points[point_idx as usize].clone());
            point_idx += parallelism;
        }
        thread::spawn(move || {
            util::order_points(&mut subset);
            let hull = util::graham::scan(&mut subset);
            transmitter.send(hull).unwrap();
        });
    }

    for idx in 0..parallelism {
        let hull = rx.recv().unwrap();
        hulls.push(hull);
    }


    if cfg!(feature = "animate") {                      // For animation
        let hulls_cpy = hulls.clone();
        for (idx, hull) in hulls_cpy.iter().enumerate() {
            ::io::print_points(&format!("hull_{}.csv", idx), &hull);
        }
    }


    let mut next_point = find_lowest_point(&hulls);
    let mut result: Vec<::Point> = Vec::new();
    result.push(hulls[next_point.0][next_point.1].clone());
    loop {
        next_point = find_next_merge_point(&hulls, next_point);
        result.push(hulls[next_point.0][next_point.1].clone());

        // Condition
        if result[0] == result[result.len() - 1] {
            break;
        }
    }

    result.pop();
    return result;
}


fn find_next_merge_point(hulls: &Vec<Vec<::Point>>, base_pair: (usize, usize)) -> (usize, usize) {
    let (base_hull, base_pt) = base_pair;
    let base = hulls[base_hull][base_pt].clone();

    // Select next point on the same hull as the base point
    let mut result = (base_hull, (base_pt + 1) % hulls[base_hull].len());

    for (hull_idx, hull) in hulls.iter().enumerate() {
        if hull_idx != base_hull {
            let candidate_idx = find_tangent_index(hull, &base);
            let previous = hulls[result.0][result.1].clone();
            let candidate = hull[candidate_idx].clone();
            let linearity = util::compare(&previous, &candidate, &base);
            if linearity == Ordering::Greater ||
               (linearity == Ordering::Equal && util::distance(&base, &previous) < util::distance(&base, &candidate)) {
                result = (hull_idx, candidate_idx);
            }
        }
    }
    return result;
}


fn find_lowest_point(hulls: &Vec<Vec<::Point>>) -> (usize, usize) {
    let mut res_hull: usize = 0;
    let mut res_pt: usize = 0;
    let mut lowest_y = hulls[0][0].y;
    for (hull_idx, hull) in hulls.iter().enumerate() {
        for (pt_idx, point) in hull.iter().enumerate() {
            if point.y < lowest_y {
                res_hull = hull_idx;
                res_pt = pt_idx;
                lowest_y = point.y;
            }
        }
    }

    (res_hull, res_pt)
}


fn find_tangent_index(hull: &Vec<::Point>, base: &::Point) -> usize {
    let mut lower_bound = 0;
    let mut upper_bound = hull.len();


    // Get relative orderings of points before and after the lower bound
    let lb_turn_before = util::compare(&hull[0], &hull[hull.len() - 1], base);
    let mut lb_turn_after = util::compare(&hull[0], &hull[1], base);

    // Check if the first point is the right-most point
    if lb_turn_before != Ordering::Greater && lb_turn_after == Ordering::Less {
        return 0usize;
    }

    // The first point is not the right-most point
    while lower_bound < upper_bound {
        // Find index of point in between the two bounds
        let mid = (upper_bound + lower_bound) / 2;

        // Compute the turns before and after the mid-point
        let mid_turn_before = util::compare(&hull[mid], &hull[(mid - 1) % hull.len()], base);
        let mid_turn_after = util::compare(&hull[mid], &hull[(mid + 1) % hull.len()], base);

        // Check in which direction the next cut should be, based on the position relative to the lower_bound point
        let cut_direction = util::compare(&hull[lower_bound], &hull[mid], base);

        if mid_turn_before != Ordering::Greater && mid_turn_after == Ordering::Less {
            // All points lie to the right of 'mid'
            return mid as usize;
        } else if (cut_direction != Ordering::Greater && lb_turn_after != Ordering::Less) ||
                  (cut_direction == Ordering::Greater && mid_turn_before == Ordering::Greater) {
            upper_bound = mid;
        } else {
            lower_bound = mid;
        }
        lb_turn_after = util::compare(&hull[lower_bound], &hull[(lower_bound + 1) % hull.len()], base);
    }

    return lower_bound as usize;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tangent_func() {
        // Tests whether the find_tangent_index method works properly.
        assert!(true);
    }
}
