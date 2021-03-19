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
pub struct Position {
    x: isize,
    y: isize
}

pub struct Piet {
    stack: Vec<isize>,
    codels: Vec<RGB>,
    width: usize,
    height: usize,
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

// static COLORS: Vec<RGB> = vec![
//     RGB{r: 255, g: 192, b: 192}, RGB{r: 255, g: 255, b: 192}, RGB{r: 192, g: 255, b: 192}, RGB{r: 192, g: 255, b: 255}, RGB{r: 192, g: 192, b: 255}, RGB{r: 255, g: 192, b: 255},
//     RGB{r: 255, g: 000, b: 000}, RGB{r: 255, g: 255, b: 000}, RGB{r: 000, g: 255, b: 000}, RGB{r: 000, g: 255, b: 255}, RGB{r: 000, g: 000, b: 255}, RGB{r: 255, g: 000, b: 255},
//     RGB{r: 192, g: 000, b: 000}, RGB{r: 192, g: 192, b: 000}, RGB{r: 000, g: 192, b: 000}, RGB{r: 000, g: 192, b: 192}, RGB{r: 000, g: 000, b: 192}, RGB{r: 192, g: 000, b: 192}
// ];

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
            height: frame.height as usize,
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

    fn get_position(&self, index: usize) -> Position {
        Position{ x: (index % self.width) as isize, y: (index / self.width)  as isize }
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
                    let pos = self.get_position(ptr as usize);
                    vec![
                        Position{x: pos.x - 1, y: pos.y},
                        Position{x: pos.x + 1, y: pos.y},
                        Position{x: pos.x, y: pos.y - 1},
                        Position{x: pos.x, y: pos.y + 1},
                    ]
                    .iter()
                    .filter(|p| -1 < p.x && p.x < self.width as isize && -1 < p.y && p.y < self.height as isize)
                    .fold(b, |b, p| self.search_color_block(p.y * self.width as isize + p.x, b))
                }
            }
        }
    }

    pub fn next(&mut self) {
        let current_codel = self.codels.get(self.ptr).unwrap();
        let mut blocks: Vec<Position> = self.current_color_block()
            .into_iter()
            .map(|i| self.get_position(i))
            .map(|p| match self.dp {
                DPValues::Right => Position{x: p.x + 1, y: p.y},
                DPValues::Left  => Position{x: p.x - 1, y: p.y},
                DPValues::Up    => Position{x: p.x, y: p.y - 1},
                DPValues::Down  => Position{x: p.x, y: p.y + 1},
            })
            .filter(|p| -1 < p.x && p.x < self.width as isize && -1 < p.y && p.y < self.height as isize)
            .filter(|p| match self.codels.get((p.y * self.width as isize + p.x) as usize) {
                None => false,
                Some(codel) => codel != current_codel
            })
            .collect();
        blocks.sort_by(|a, b| {
            match self.dp {
                DPValues::Right => match b.x.cmp(&a.x) {
                    Ordering::Equal => match self.cc {
                        CCValues::Left  => a.y.cmp(&b.y),
                        CCValues::Right => b.y.cmp(&a.y),
                    },
                    other => other
                },
                DPValues::Left => match a.x.cmp(&b.x) {
                    Ordering::Equal => match self.cc {
                        CCValues::Left  => a.y.cmp(&b.y),
                        CCValues::Right => b.y.cmp(&a.y),
                    },
                    other => other
                },
                DPValues::Up => match a.y.cmp(&b.y) {
                    Ordering::Equal => match self.cc {
                        CCValues::Left  => a.x.cmp(&b.x),
                        CCValues::Right => b.x.cmp(&a.x),
                    },
                    other => other
                },
                DPValues::Down => match b.y.cmp(&a.y) {
                    Ordering::Equal => match self.cc {
                        CCValues::Left  => a.x.cmp(&b.x),
                        CCValues::Right => b.x.cmp(&a.x),
                    },
                    other => other
                },
            }
        });
        match blocks.get(0) {
            None => {
                match self.retry() {
                    Err(_) => {},
                    Ok(_) => { self.next() }
                }
            },
            Some(p) => {
                let new_ptr = p.y * self.width as isize + p.x;
                let codel = self.codels.get(new_ptr as usize).unwrap();
                if *codel == BLACK {
                    match self.retry() {
                        Err(_) => {},
                        Ok(_) => { self.next() }
                    }
                } else {
                    self.retry_count = 0;
                    self.ptr = new_ptr as usize;
                    if *codel == WHITE {
                        self.next();
                    }
                }
            }
        }
    }

    pub fn retry(&mut self) -> Result<(), ()> {
        self.retry_count += 1;
        if self.retry_count > 7 {
            Err(())
        } else if self.retry_count % 2 == 0 {
            let n = self.retry_count as i32 / 2 % 2;
            self.cc = n.try_into()?;
            println!("dp={:?} cc={:?} n={}", self.dp, self.cc, n);
            Ok(())
        } else {
            let n = self.retry_count as i32 / 2 % 4;
            self.dp = n.try_into()?;
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
    println!("{:?} {:?}", piet.get_position(piet.ptr), piet.current_color_block().len());
    piet.next();
    println!("{:?} {:?}", piet.get_position(piet.ptr), piet.current_color_block().len());
    piet.next();
    println!("{:?} {:?}", piet.get_position(piet.ptr), piet.current_color_block().len());
    piet.next();
    println!("{:?} {:?}", piet.get_position(piet.ptr), piet.current_color_block().len());
    piet.next();
    println!("{:?} {:?}", piet.get_position(piet.ptr), piet.current_color_block().len());
    piet.next();
    println!("{:?} {:?}", piet.get_position(piet.ptr), piet.current_color_block().len());
    piet.next();
    println!("{:?} {:?}", piet.get_position(piet.ptr), piet.current_color_block().len());
    piet.next();
    println!("{:?} {:?}", piet.get_position(piet.ptr), piet.current_color_block().len());
    piet.next();
    println!("{:?} {:?}", piet.get_position(piet.ptr), piet.current_color_block().len());
    piet.next();
    println!("{:?} {:?}", piet.get_position(piet.ptr), piet.current_color_block().len());
    piet.next();
    println!("{:?} {:?}", piet.get_position(piet.ptr), piet.current_color_block().len());
    piet.next();
    println!("{:?} {:?}", piet.get_position(piet.ptr), piet.current_color_block().len());
    piet.next();
    println!("{:?} {:?}", piet.get_position(piet.ptr), piet.current_color_block().len());
    piet.next();
    println!("{:?} {:?}", piet.get_position(piet.ptr), piet.current_color_block().len());
    piet.next();
    println!("{:?} {:?}", piet.get_position(piet.ptr), piet.current_color_block().len());
    piet.next();
    println!("{:?} {:?}", piet.get_position(piet.ptr), piet.current_color_block().len());
    piet.next();
    println!("{:?} {:?}", piet.get_position(piet.ptr), piet.current_color_block().len());
    piet.next();
    println!("{:?} {:?}", piet.get_position(piet.ptr), piet.current_color_block().len());
    piet.next();
    println!("{:?} {:?}", piet.get_position(piet.ptr), piet.current_color_block().len());
}
