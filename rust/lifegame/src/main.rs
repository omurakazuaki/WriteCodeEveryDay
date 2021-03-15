use std::fmt;
use std::time::Duration;
use std::thread::sleep;
use rand::{thread_rng, Rng};
use termion::clear;

#[derive(Debug, Clone)]
struct LifeGame {
    raw: Vec<u8>,
    size: usize
}

struct LifeGameIterator {
    current: LifeGame
}

impl LifeGame {
    fn new(size: usize) -> LifeGame {
        let mut cells = LifeGame {
            raw: vec![0; size.pow(2)],
            size: size
        };
        let mut rng = thread_rng();
        let mut n = 0;
        while n < size * 5 {
            let i: usize = rng.gen_range(0..size.pow(2));
            cells.raw[i] = 1;
            n += 1;
        }
        cells
    }

    fn iter(&mut self) -> LifeGameIterator {
        LifeGameIterator {current: self.clone()}
    }
}

impl fmt::Display for LifeGame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        println!("{}", clear::All);
        let cells_as_str: Vec<String> = self.raw
            .chunks(self.size)
            .map(|row|row.into_iter().map(|v|if *v == 0u8 {"　"} else {"■ "}).collect::<String>())
            .collect();
        write!(f, "{}|{}|", clear::All, cells_as_str.join("|\n|"))
    }
}

impl Iterator for LifeGameIterator {
    type Item = LifeGame;
    fn next(&mut self) -> Option<LifeGame> {
        if self.current.raw.iter().fold(0usize, |sum, v| sum + *v as usize) == 0 {
            None
        } else {
            let size = self.current.size as isize;
            let mut new_cells: Vec<u8> = vec![0; (size as usize).pow(2)];
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
                    .fold(0, |sum, v| sum + self.current.raw.get(v as usize).unwrap());
                let cur = self.current.raw.get(i as usize).unwrap();
                if *cur == 0 && sum == 3 {
                    new_cells[i as usize] = 1;
                } else if *cur == 1 && 1 < sum && sum < 4 {
                    new_cells[i as usize] = 1;
                }
                i += 1;
            }
            self.current.raw = new_cells;
            Some(self.current.clone())
        }
    }
}

fn main() {
    for lg in LifeGame::new(32).iter() {
        sleep(Duration::from_millis(100));
        println!("{}", lg);
    }
}
