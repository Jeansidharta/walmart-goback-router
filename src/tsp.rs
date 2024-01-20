use crate::Vec2;

#[derive(Clone)]
struct Vertice {
    pos: Vec2,
    visited: bool,
}

pub fn dist_squared(pos1: &Vec2, pos2: &Vec2) -> f64 {
    (pos2.x - pos1.x).powi(2) + (pos2.y - pos1.y).powi(2)
}

// pub fn dist(pos1: &Position, pos2: &Position) -> f64 {
//     dist_squared(pos1, pos2).sqrt()
// }

pub fn solve(starting_point: Vec2, points: impl Iterator<Item = Vec2>) -> Vec<usize> {
    let mut vertices = points
        .map(|pos| Vertice {
            pos,
            visited: false,
        })
        .collect::<Vec<Vertice>>();

    let mut route = vec![];
    let mut current_vertice = Vertice {
        pos: starting_point,
        visited: true,
    };
    loop {
        let mut unvisited = vertices
            .iter_mut()
            .enumerate()
            .filter(|(_index, v)| !v.visited)
            .peekable();

        if unvisited.peek().is_none() {
            break;
        }

        let closest = unvisited
            .reduce(|left, right| {
                if dist_squared(&left.1.pos, &current_vertice.pos)
                    < dist_squared(&right.1.pos, &current_vertice.pos)
                {
                    left
                } else {
                    right
                }
            })
            .unwrap();

        closest.1.visited = true;
        current_vertice = closest.1.clone();
        route.push(closest.0);
    }
    route
}
