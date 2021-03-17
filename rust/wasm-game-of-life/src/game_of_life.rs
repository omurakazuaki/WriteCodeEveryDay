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

        let gol = GameOfLife {
            cells: vec![0; width * height]
            .iter()
            .map(|_| if js_sys::Math::random() < 0.5 { 1 } else { 0 })
            .collect(),
            width: width
        };
        gol
    }

    pub fn cells(&self) -> Vec<u8> {
        self.cells.clone()
    }

    pub fn tick(&mut self) {
        let size = self.width as isize;
        let mut new_cells: Vec<u8> = vec![0; self.cells.len()];
        let mut i = 0isize;
        while i < new_cells.len() as isize {
            let sum = vec![
                    i-size-1, i-size, i-size+1,
                    i-1, i+1,
                    i+size-1, i+size, i+size+1
                ].into_iter()
                .filter(|v| {
                    -1 < *v && *v < new_cells.len() as isize &&
                    !(i % size == 0 && v % size == size - 1) &&
                    !(i % size == size - 1 && v % size == 0)
                })
                .fold(0, |sum, v| sum + self.cells.get(v as usize).unwrap());
            let cur = self.cells.get(i as usize).unwrap();
            if *cur == 0 && sum == 3 {
                new_cells[i as usize] = 1;
            } else if *cur == 1 && 1 < sum && sum < 4 {
                new_cells[i as usize] = 1;
            }
            i += 1;
        }
        self.cells = new_cells;
    }
}
