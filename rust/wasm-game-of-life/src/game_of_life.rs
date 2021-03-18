use wasm_bindgen::prelude::*;
use js_sys;

#[wasm_bindgen]
pub struct GameOfLife {
    cells: Vec<u8>,
    width: usize
}

#[wasm_bindgen]
impl GameOfLife {
    pub fn new(width: usize, height: usize) -> GameOfLife {
        GameOfLife {
            cells: vec![0; width * height]
            .iter()
            .map(|_| if js_sys::Math::random() < 0.5 { 1 } else { 0 })
            .collect(),
            width: width
        }
    }

    pub fn cells(&self) -> Vec<u8> {
        self.cells.clone()
    }

    pub fn tick(&mut self) {
        let width = self.width as isize;
        self.cells = self.cells
            .iter()
            .enumerate()
            .map(|e| {
                let i = e.0 as isize;
                let v = e.1 as &u8;
                let sum = vec![
                    i - width - 1, i - width, i - width + 1,
                    i - 1, i + 1,
                    i + width - 1, i + width, i + width + 1
                ].into_iter()
                .filter(|j| {
                    -1 < *j && *j < self.cells.len() as isize &&
                    !(i % width == 0 && j % width == width - 1) &&
                    !(i % width == width - 1 && j % width == 0)
                })
                .fold(0, |sum, j| sum + self.cells.get(j as usize).unwrap());
                if *v == 0 && sum == 3 { 1 }
                else if *v == 1 && 1 < sum && sum < 4 { 1 }
                else { 0 }
            })
            .collect();
    }
}
