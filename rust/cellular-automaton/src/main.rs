use std::env;

fn main() {
    let rule: u8 = env::args().nth(1).unwrap().parse().unwrap();
    let size: usize = env::args().nth(2).unwrap().parse().unwrap();
    let mut current: Vec<u8> = vec![0; size];
    current[size/2] = 1;
    let mut next: Vec<u8> = vec![0; size];
    for _ in 0..size {
        for i in 0..size {
            let fst = if i == 0 { 0 } else { current[i - 1] };
            let snd = current[i] << 1;
            let trd = if i == size - 1 { 0 } else { current[i + 1] << 2 };
            next[i] = (rule >> (fst + snd + trd)) & 1;
        }
        let s: String = current.iter().map(|n| if *n == 0 { ' ' } else { '|' }).collect();
        println!("{}", s);
        current = next.clone();
    }
}
