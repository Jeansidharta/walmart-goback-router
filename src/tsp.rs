use ordered_float::NotNan;

use crate::{Route, Vec2};

pub fn dist_squared(pos1: &Vec2, pos2: &Vec2) -> f64 {
    (pos2.x - pos1.x).powi(2) + (pos2.y - pos1.y).powi(2)
}

// pub fn dist(pos1: &Position, pos2: &Position) -> f64 {
//     dist_squared(pos1, pos2).sqrt()
// }

fn find_path(route: &Route, p1: usize, p2: usize) -> Option<(Vec<usize>, NotNan<f64>)> {
    pathfinding::directed::astar::astar(
        &p1,
        |p| {
            route
                .connections_dict
                .get(p)
                .unwrap()
                .iter()
                .map(|next| {
                    (
                        *next,
                        NotNan::new(dist_squared(&route.points[*p], &route.points[*next])).unwrap(),
                    )
                })
                .collect::<Vec<(usize, NotNan<f64>)>>()
        },
        |p| NotNan::new(dist_squared(&route.points[*p], &route.points[p2])).unwrap(),
        |p| *p == p2,
    )
}

pub fn solve(route: Route, starting_point: usize, points: Vec<usize>) -> Vec<(usize, Vec<usize>)> {
    let mut current_point = starting_point;
    let mut remaining_points = points;
    let mut path = vec![];
    while !remaining_points.is_empty() {
        let mut closest_point_index = 0usize;
        let mut smallest_distance = std::f64::MAX;
        let mut smallest_path = vec![];
        remaining_points
            .iter()
            .enumerate()
            .for_each(|(index, point)| {
                let (path, distance) = find_path(&route, current_point, *point).unwrap();
                if *distance < smallest_distance {
                    smallest_distance = *distance;
                    closest_point_index = index;
                    smallest_path = path;
                }
            });
        current_point = remaining_points.remove(closest_point_index);
        path.push((current_point, smallest_path));
    }
    path
}
