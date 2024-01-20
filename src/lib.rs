use wasm_bindgen::prelude::*;

mod tsp;

#[wasm_bindgen]
#[derive(Clone)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

#[wasm_bindgen]
#[derive(Default, Debug, Clone, Copy)]
pub struct Connection {
    pub i1: usize,
    pub i2: usize,
}

#[wasm_bindgen]
impl Vec2 {
    #[wasm_bindgen]
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}


#[wasm_bindgen]
pub fn solve(starting_point: Vec2, points: Vec<Vec2>) -> Vec<usize> {
    tsp::solve(starting_point, points.into_iter())
}
