use std::env;

fn main() {
    let rule: i8 = env::args().nth(1).unwrap().parse().unwrap();
    let size: usize = env::args().nth(2).unwrap().parse().unwrap();
    let mut current: Vec<i8> = vec![0; size];
    current[0] = 1;
    let mut next: Vec<i8> = vec![0; size];
    for _ in 0..size {
        if current[size - 1] == 1 {
            break;
        }
        for i in 0..size {
            let fst = if i == 0 { 0 } else { current[i - 1] };
            let snd = current[i] << 1;
            let trd = if i == size - 1 { 0 } else { current[i + 1] << 2 };
            next[i] = (rule >> (fst + snd + trd)) & 1;
        }
        let s: String = current.iter().map(|n| if *n == 0 { ' ' } else { '\\' }).collect();
        println!("{}", s);
        current = next.clone();
    }
}
