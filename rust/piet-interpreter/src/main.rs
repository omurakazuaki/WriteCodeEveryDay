use std::cmp::Eq;
use std::fs::File;
use std::cmp::Ordering;
use std::convert::TryInto;
use std::convert::TryFrom;

#[derive(Debug, Copy, Clone)]
pub enum DPValues {
    Right,
    Down,
    Left,
    Up,
}
impl TryFrom<i32> for DPValues {
    type Error = ();
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == DPValues::Right as i32 => Ok(DPValues::Right),
            x if x == DPValues::Down as i32 => Ok(DPValues::Down),
            x if x == DPValues::Left as i32 => Ok(DPValues::Left),
            x if x == DPValues::Up as i32 => Ok(DPValues::Up),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum CCValues {
    Left,
    Right,
}
impl TryFrom<i32> for CCValues {
    type Error = ();
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == CCValues::Right as i32 => Ok(CCValues::Right),
            x if x == CCValues::Left as i32 => Ok(CCValues::Left),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct Potion {
    x: usize,
    y: usize
}

pub struct Piet {
    stack: Vec<isize>,
    codels: Vec<RGB>,
    width: usize,
    dp: DPValues,
    cc: CCValues,
    ptr: usize,
    retry_count: u8
}

#[derive(Debug)]
pub struct RGB {
    r: u8,
    g: u8,
    b: u8
}

static BLACK: RGB = RGB {r:  0, g:  0, b:  0};
static WHITE: RGB = RGB {r:255, g:255, b:255};


impl PartialEq for RGB {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r &&
        self.g == other.g &&
        self.b == other.b
    }
}
impl Eq for RGB {}

impl Piet {
    pub fn new(frame: &gif::Frame) -> Self {
        Self {
            width: frame.width as usize,
            stack: vec![],
            codels: frame.buffer.chunks(4).map(|v|RGB{r:v[0], g:v[1], b:v[2]}).collect(),
            ptr: 0,
            dp: DPValues::Right,
            cc: CCValues::Left,
            retry_count: 0
        }
    }

    pub fn current_color_block(&self) -> Vec<usize> {
        let blocks: Vec<usize> = vec![];
        self.search_color_block(self.ptr as isize, blocks)
    }

    fn get_position(&self, index: usize) -> Potion {
        Potion{ x: index % self.width, y: index / self.width }
    }

    fn search_color_block(&self, ptr: isize, blocks: Vec<usize>) -> Vec<usize> {
        match self.codels.get(ptr as usize) {
            None => blocks,
            Some(rgb) => {
                if *rgb != *self.codels.get(self.ptr).unwrap() {
                    blocks
                } else if blocks.contains(&(ptr as usize)) {
                    blocks
                } else {
                    let mut b = blocks.clone();
                    b.push(ptr as usize);
                    let b = self.search_color_block(ptr + 1, b);
                    let b = self.search_color_block(ptr + self.width as isize, b);
                    let b = self.search_color_block(ptr - 1, b);
                    let b = self.search_color_block(ptr - self.width as isize, b);
                    b
                }
            }
        }
    }

    pub fn next(&mut self) {
        let mut blocks: Vec<Potion> = self.current_color_block()
            .into_iter()
            .map(|i|self.get_position(i))
            .collect();
        blocks.sort_by(|a, b| {
            let cmp_y = match self.cc {
                CCValues::Left  => a.y.cmp(&b.y),
                CCValues::Right => b.y.cmp(&a.y),
            };
            match self.dp {
                DPValues::Right => match b.x.cmp(&a.x) {
                    Ordering::Equal => cmp_y,
                    other => other
                },
                DPValues::Left  => match a.x.cmp(&b.x) {
                    Ordering::Equal => cmp_y,
                    other => other
                },
                DPValues::Up    => match b.x.cmp(&a.x) {
                    Ordering::Equal => cmp_y,
                    other => other
                },
                DPValues::Down  => match a.x.cmp(&b.x) {
                    Ordering::Equal => cmp_y,
                    other => other
                },
            }
        });
        let new_ptr: isize = (blocks[0].y * self.width + blocks[0].x) as isize + match self.dp {
            DPValues::Right =>  1,
            DPValues::Left  => -1,
            DPValues::Up    => -(self.width as isize),
            DPValues::Down  => self.width as isize,
        };
        match self.codels.get(new_ptr as usize) {
            None => {
                match self.retry() {
                    Err(_) => {},
                    Ok(_) => { self.next() }
                }
            },
            Some(codel) => {
                let new_ptr_u = new_ptr as usize;
                if *codel == WHITE {
                    self.ptr = new_ptr_u;
                    self.next();
                } else if new_ptr < 0 || self.ptr / self.width != new_ptr_u / self.width {
                    match self.retry() {
                        Err(_) => {},
                        Ok(_) => { self.next() }
                    }
                } else if *codel == BLACK {
                    match self.retry() {
                        Err(_) => {},
                        Ok(_) => { self.next() }
                    }
                } else {
                    self.ptr = new_ptr_u;
                }
            }
        }
    }

    pub fn retry(&mut self) -> Result<(), ()> {
        self.retry_count += 1;
        if self.retry_count > 7 {
            Err(())
        } else if self.retry_count % 2 == 0 {
            let n = (self.cc as i32 + 1) / 2 % 2;
            self.cc = n.try_into()?;
            println!("dp={:?} cc={:?} n={}", self.dp, self.cc, n);
            Ok(())
        } else {
            let n = (self.dp as i32 + 1) / 2 % 4;
            self.cc = n.try_into()?;
            println!("dp={:?} cc={:?} n={}", self.dp, self.cc, n);
            Ok(())
        }
    }

}

fn main() {
    let file = File::open("./samples/print_Piet_.gif").unwrap();
    let mut decoder = gif::DecodeOptions::new();
    decoder.set_color_output(gif::ColorOutput::RGBA);
    let mut decoder = decoder.read_info(file).unwrap();

    let frame = decoder.read_next_frame().unwrap().unwrap();
    let mut piet = Piet::new(frame);
    println!("{} {:?}", piet.ptr, piet.current_color_block().len());
    piet.next();
    println!("{} {:?}", piet.ptr, piet.current_color_block().len());
    piet.next();
    println!("{} {:?}", piet.ptr, piet.current_color_block().len());
    piet.next();
    println!("{} {:?}", piet.ptr, piet.current_color_block().len());
    piet.next();
    println!("{} {:?}", piet.ptr, piet.current_color_block().len());
    piet.next();
    println!("{} {:?}", piet.ptr, piet.current_color_block().len());
}
