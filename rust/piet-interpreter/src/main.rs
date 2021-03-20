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
    y: isize,
    width: usize,
}

impl Position {
    pub fn new(x: isize, y: isize, width: usize) -> Self {
        Position{ x: x, y: y, width: width }
    }
    pub fn from_ptr(ptr: usize, width: usize) -> Self {
        Position{ x: (ptr % width) as isize, y: (ptr / width)  as isize, width: width }
    }
    pub fn to_ptr(&self) -> isize {
        self.y * self.width as isize + self.x
    }
}

pub struct Piet {
    stack: Vec<isize>,
    codels: Vec<RGB>,
    width: usize,
    height: usize,
    dp: DPValues,
    cc: CCValues,
    ptr: usize,
    retry_count: u8,
}

#[derive(Debug)]
pub struct RGB {
    r: u8,
    g: u8,
    b: u8
}

static BLACK: RGB = RGB {r:  0, g:  0, b:  0};
static WHITE: RGB = RGB {r:255, g:255, b:255};

static COLORS: [&'static RGB; 18] = [
    &RGB{r: 255, g: 192, b: 192}, &RGB{r: 255, g: 255, b: 192}, &RGB{r: 192, g: 255, b: 192}, &RGB{r: 192, g: 255, b: 255}, &RGB{r: 192, g: 192, b: 255}, &RGB{r: 255, g: 192, b: 255},
    &RGB{r: 255, g: 000, b: 000}, &RGB{r: 255, g: 255, b: 000}, &RGB{r: 000, g: 255, b: 000}, &RGB{r: 000, g: 255, b: 255}, &RGB{r: 000, g: 000, b: 255}, &RGB{r: 255, g: 000, b: 255},
    &RGB{r: 192, g: 000, b: 000}, &RGB{r: 192, g: 192, b: 000}, &RGB{r: 000, g: 192, b: 000}, &RGB{r: 000, g: 192, b: 192}, &RGB{r: 000, g: 000, b: 192}, &RGB{r: 192, g: 000, b: 192}
];

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

    fn color_block(&self, ptr: Option<usize>) -> Vec<usize> {
        let block: Vec<usize> = vec![];
        self.search_color_block(ptr.unwrap_or(self.ptr) as isize, block)
    }

    fn search_color_block(&self, ptr: isize, block: Vec<usize>) -> Vec<usize> {
        match self.codels.get(ptr as usize) {
            None => block,
            Some(rgb) => {
                if block.contains(&(ptr as usize)) {
                    block
                } else {
                    let mut b = block.clone();
                    b.push(ptr as usize);
                    let pos = Position::from_ptr(ptr as usize, self.width);
                    vec![
                        Position::new(pos.x - 1, pos.y, self.width),
                        Position::new(pos.x + 1, pos.y, self.width),
                        Position::new(pos.x, pos.y - 1, self.width),
                        Position::new(pos.x, pos.y + 1, self.width),
                    ]
                    .iter()
                    .filter(|p| -1 < p.x && p.x < self.width as isize && -1 < p.y && p.y < self.height as isize)
                    .map(|p|p.to_ptr())
                    .filter(|p| *rgb == *self.codels.get(*p as usize).unwrap())
                    .fold(b, |b, p| self.search_color_block(p, b))
                }
            }
        }
    }

    fn next(&mut self, ptr: Option<usize>) -> bool {
        let ptr = ptr.unwrap_or(self.ptr);
        let current_codel = self.codels.get(ptr).unwrap();
        let mut block: Vec<Position> = self.color_block(Some(ptr))
            .into_iter()
            .map(|i| Position::from_ptr(i as usize, self.width))
            .map(|pos| match self.dp {
                DPValues::Right => Position::new(pos.x + 1, pos.y, self.width),
                DPValues::Left  => Position::new(pos.x - 1, pos.y, self.width),
                DPValues::Up    => Position::new(pos.x, pos.y - 1, self.width),
                DPValues::Down  => Position::new(pos.x, pos.y + 1, self.width),
            })
            .filter(|p| -1 < p.x && p.x < self.width as isize && -1 < p.y && p.y < self.height as isize)
            .filter(|p| match self.codels.get((p.to_ptr()) as usize) {
                None => false,
                Some(codel) => codel != current_codel
            })
            .collect();
        block.sort_by(|a, b| {
            match self.dp {
                DPValues::Right => match b.x.cmp(&a.x) {
                    Ordering::Equal => match self.cc {
                        CCValues::Left  => (a.to_ptr()).cmp(&(b.to_ptr())),
                        CCValues::Right => (b.to_ptr()).cmp(&(a.to_ptr())),
                    },
                    other => other
                },
                DPValues::Down => match b.y.cmp(&a.y) {
                    Ordering::Equal => match self.cc {
                        CCValues::Left  => (b.to_ptr()).cmp(&(a.to_ptr())),
                        CCValues::Right => (a.to_ptr()).cmp(&(b.to_ptr())),
                    },
                    other => other
                },
                DPValues::Left => match a.x.cmp(&b.x) {
                    Ordering::Equal => match self.cc {
                        CCValues::Left  => (b.to_ptr()).cmp(&(a.to_ptr())),
                        CCValues::Right => (a.to_ptr()).cmp(&(b.to_ptr())),
                    },
                    other => other
                },
                DPValues::Up => match a.y.cmp(&b.y) {
                    Ordering::Equal => match self.cc {
                        CCValues::Left  => (a.to_ptr()).cmp(&(b.to_ptr())),
                        CCValues::Right => (b.to_ptr()).cmp(&(a.to_ptr())),
                    },
                    other => other
                },
            }
        });
        match block.get(0) {
            None => {
                match self.retry() {
                    Err(_) => {false},
                    Ok(_) => { self.next(Some(ptr)) }
                }
            },
            Some(p) => {
                let new_ptr = p.to_ptr();
                let codel = self.codels.get(new_ptr as usize).unwrap();
                if *codel == BLACK {
                    match self.retry() {
                        Err(_) => {false},
                        Ok(_) => { self.next(Some(ptr)) }
                    }
                } else {
                    self.retry_count = 0;
                    if *codel == WHITE {
                        self.next(Some(new_ptr as usize))
                    } else {
                        self.operate(new_ptr as usize);
                        self.ptr = new_ptr as usize;
                        true
                    }
                }
            }
        }
    }

    fn retry(&mut self) -> Result<(), ()> {
        self.retry_count += 1;
        if self.retry_count > 7 {
            Err(())
        } else if self.retry_count % 2 == 1 {
            let n = (self.cc as i32 + 1) % 2;
            self.cc = n.try_into()?;
            //println!("dp={:?} cc={:?} n={}", self.dp, self.cc, n);
            Ok(())
        } else {
            let n = (self.dp as i32 + 1) % 4;
            self.dp = n.try_into()?;
            //println!("dp={:?} cc={:?} n={}", self.dp, self.cc, n);
            Ok(())
        }
    }

    fn operate(&mut self, new_ptr: usize) {
        let cur = self.codels.get(self.ptr).unwrap();
        let new = self.codels.get(new_ptr).unwrap();
        let cur_i = COLORS.iter().position(|c| *c==cur).unwrap(); // Todo: None
        let new_i = COLORS.iter().position(|c| *c==new).unwrap(); // Todo: None
        let mut hue_delta = new_i as isize % 6 - cur_i as isize % 6;
        if hue_delta < 0 {
            hue_delta += 6;
        }
        let mut lightness_delta = new_i as isize / 6 - cur_i as isize / 6;
        if lightness_delta < 0 {
            lightness_delta += 3;
        }
        match hue_delta * 3 + lightness_delta {
             1 => self.push(),
             2 => self.pop(),
             3 => self.add(),
             4 => println!("subtract"),
             5 => println!("multiply"),
             6 => println!("divide"),
             7 => self.modulo(),
             8 => println!("not"),
             9 => println!("greater"),
            10 => println!("pointer"),
            11 => println!("switch"),
            12 => self.duplicate(),
            13 => println!("roll"),
            14 => println!("in(number)"),
            15 => println!("in(char)"),
            16 => self.out_num(),
            17 => self.out_char(),
            _ => {}
        }
        //println!("{}, {}", hue_delta, lightness_delta);
    }

    fn push(&mut self) {
        self.stack.push(self.color_block(None).len() as isize);
    }

    fn pop(&mut self) {
        self.stack.pop();
    }

    fn add(&mut self) {
        let a = self.stack.pop().unwrap_or(0);
        let b = self.stack.pop().unwrap_or(0);
        self.stack.push(a + b);
    }

    fn modulo(&mut self) {
        let a = self.stack.pop().unwrap_or(0);
        let b = self.stack.pop().unwrap_or(0);
        if b != 0 {
            self.stack.push(a % b);
        }
    }

    fn duplicate(&mut self) {
        match self.stack.clone().last() {
            None => {},
            Some(v) => {self.stack.push(v.clone());}
        }
    }

    fn roll(&mut self) {
        let num = self.stack.pop().unwrap_or(0);
        let depth = self.stack.pop().unwrap_or(0);

    }

    fn out_char(&mut self) {
        let v = self.stack.pop().unwrap();
        print!("{}", std::char::from_u32(v as u32).unwrap());
    }

    fn out_num(&mut self) {
        let v = self.stack.pop().unwrap();
        print!("{}",v as u32);
    }
}

fn main() {
    let file = File::open("./samples/Piet_hello.gif").unwrap();
    let mut decoder = gif::DecodeOptions::new();
    decoder.set_color_output(gif::ColorOutput::RGBA);
    let mut decoder = decoder.read_info(file).unwrap();

    let frame = decoder.read_next_frame().unwrap().unwrap();
    let mut piet = Piet::new(frame);
    while piet.next(None) {
        //println!("-----------ptr={:?} area={:?} dp={:?} cc={:?}", Position::from_ptr(piet.ptr, piet.width), piet.color_block(None).len(), piet.dp, piet.cc);
    };
}
