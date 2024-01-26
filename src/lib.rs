use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

mod tsp;

// lifted from the `console_log` example
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

#[wasm_bindgen]
#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Connection {
    pub i1: usize,
    pub i2: usize,
}

#[wasm_bindgen]
impl Connection {
    #[wasm_bindgen]
    pub fn new(i1: usize, i2: usize) -> Self {
        Self { i1, i2 }
    }
}

#[wasm_bindgen]
impl Vec2 {
    #[wasm_bindgen]
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Route {
    pub points: Vec<Vec2>,
    pub connections_dict: HashMap<usize, Vec<usize>>,
    pub connections: Vec<Connection>,
}

#[wasm_bindgen]
pub fn solve(
    route_points: Vec<Vec2>,
    connections: Vec<Connection>,
    starting_point: usize,
    points: Vec<usize>,
) -> Vec<JsValue> {
    let connections_dict = connections.iter().fold(
        HashMap::<usize, Vec<usize>>::new(),
        |mut hash, Connection { i1, i2 }| {
            if let Some(vec) = hash.get_mut(i1) {
                vec.push(*i2);
            } else {
                hash.insert(*i1, vec![*i2]);
            }
            if let Some(vec) = hash.get_mut(i2) {
                vec.push(*i1);
            } else {
                hash.insert(*i2, vec![*i1]);
            }
            hash
        },
    );
    let route = Route {
        points: route_points,
        connections_dict,
        connections,
    };
    tsp::solve(route, starting_point, points)
        .into_iter()
        .map(|s| serde_wasm_bindgen::to_value(&s).unwrap())
        .collect()
}
