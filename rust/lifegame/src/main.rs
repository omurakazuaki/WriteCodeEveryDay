use std::time::Duration;
use std::thread::sleep;
use rand::{thread_rng, Rng};
use termion::clear;

fn create_cells(size: usize) -> Vec<u8> {
    let mut cells: Vec<u8> = vec![0; size.pow(2)];
    let mut rng = thread_rng();
    let mut n = 0;
    while n < size * 2 {
        let i: usize = rng.gen_range(0..size.pow(2));
        cells[i] = 1;
        n += 1;
    }
    cells
}

fn print_cells(cells: &Vec<u8>, size: usize) {
    println!("{}", clear::All);
    for row in cells.chunks(size) {
        println!("{}", row.into_iter().map(|v|if *v == 0u8 {" "} else {"*"}).collect::<String>());
    }
}

fn next(cells: &Vec<u8>, size: isize) -> Vec<u8> {
    let mut new_cells: Vec<u8> = vec![0; size.pow(2) as usize];
    let mut i = 0;
    while i < size.pow(2) {
        let indexes: Vec<isize> = vec![
            i-size-1, i-size, i-size+1,
            i-1,      i,      i+1,
            i+size-1, i+size, i+size+1
        ];
        let sum = indexes.into_iter().fold(0, |sum, v| sum + cells.get(v as usize).unwrap_or_else(||&0u8));
        if 2 < sum && sum < 5 {
            new_cells[i as usize] = 1;
        }
        i += 1;
    }
    new_cells
}

fn main() {
    let size: isize = 32;
    let mut cells: Vec<u8> = create_cells(size as usize);
    let mut n = 0;
    while n < 100 {
        sleep(Duration::from_millis(250));
        print_cells(&cells, size as usize);
        cells = next(&cells, size);
        n += 1;
    }
}
