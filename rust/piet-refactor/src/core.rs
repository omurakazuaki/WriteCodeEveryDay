use std::fmt;
use std::cmp::Ordering;
use std::convert::TryInto;
use std::convert::TryFrom;

#[derive(Debug, Copy, Clone)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8
}

impl Color {
    fn diff_to(&self, other: &Self) -> (usize, usize) {
        let self_i = COLORS.iter().position(|c|*c==self).unwrap();
        let other_i = COLORS.iter().position(|c|*c==other).unwrap();
        let mut hue_delta = self_i as isize % 6 - other_i as isize % 6;
        if hue_delta < 0 {
            hue_delta += 6;
        }
        let mut lightness_delta = self_i as isize / 6 - other_i as isize / 6;
        if lightness_delta < 0 {
            lightness_delta += 3;
        }
        (hue_delta as usize, lightness_delta as usize)
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r &&
        self.g == other.g &&
        self.b == other.b
    }
}
impl Eq for Color {}

static BLACK: Color = Color {r:  0, g:  0, b:  0};
static COLORS: [&'static Color; 18] = [
    &Color{r: 255, g: 192, b: 192}, &Color{r: 255, g: 255, b: 192}, &Color{r: 192, g: 255, b: 192}, &Color{r: 192, g: 255, b: 255}, &Color{r: 192, g: 192, b: 255}, &Color{r: 255, g: 192, b: 255},
    &Color{r: 255, g: 000, b: 000}, &Color{r: 255, g: 255, b: 000}, &Color{r: 000, g: 255, b: 000}, &Color{r: 000, g: 255, b: 255}, &Color{r: 000, g: 000, b: 255}, &Color{r: 255, g: 000, b: 255},
    &Color{r: 192, g: 000, b: 000}, &Color{r: 192, g: 192, b: 000}, &Color{r: 000, g: 192, b: 000}, &Color{r: 000, g: 192, b: 192}, &Color{r: 000, g: 000, b: 192}, &Color{r: 192, g: 000, b: 192}
];

#[derive(Debug, Copy, Clone)]
pub struct Codel {
    color: Color,
    pos: (usize, usize),
    ptr: usize
}

impl Codel {
    pub fn new(color: Color, width: usize, ptr: usize) -> Self {
        Codel{
            color: color,
            pos: (ptr % width, ptr / width),
            ptr: ptr
        }
    }
}
impl PartialEq for Codel {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}
impl Eq for Codel {}

#[derive(Debug, Copy, Clone)]
pub enum DP {
    Right,
    Down,
    Left,
    Up,
}
impl TryFrom<i32> for DP {
    type Error = ();
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == DP::Right as i32 => Ok(DP::Right),
            x if x == DP::Down as i32 => Ok(DP::Down),
            x if x == DP::Left as i32 => Ok(DP::Left),
            x if x == DP::Up as i32 => Ok(DP::Up),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum CC {
    Left,
    Right,
}
impl TryFrom<i32> for CC {
    type Error = ();
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == CC::Right as i32 => Ok(CC::Right),
            x if x == CC::Left as i32 => Ok(CC::Left),
            _ => Err(()),
        }
    }
}

pub struct Piet {
    stack: Vec<isize>,
    codels: Vec<Codel>,
    ptr: usize,
    dp: DP,
    cc: CC,
}

impl Piet {
    pub fn new(frame: &gif::Frame) -> Self {
        let colors: Vec<Color> = frame.buffer.chunks(4).map(|v|Color{r:v[0], g:v[1], b:v[2]}).collect();
        Self {
            stack: vec![],
            codels: colors.iter().enumerate().map(|(ptr, color)|Codel::new(*color, frame.width as usize, ptr)).collect(),
            ptr: 0,
            dp: DP::Right,
            cc: CC::Left,
        }
    }

    fn get_codel(&self, x: isize, y: isize) -> Option<&Codel> {
        if x < 0 || y < 0 {
            None
        } else {
            self.codels.iter().find(|codel| codel.pos == (x as usize, y as usize))
        }
    }

    pub fn color_block(&self) -> Vec<Codel> {
        let block: Vec<Codel> = vec![];
        self.search_color_block(self.codels.get(self.ptr).unwrap(), block)
    }

    fn search_color_block(&self, current: &Codel, block: Vec<Codel>) -> Vec<Codel> {
        if !block.contains(current) {
            let mut block = block.clone();
            block.push(*current);
            vec![
                self.get_codel(current.pos.0 as isize - 1, current.pos.1 as isize),
                self.get_codel(current.pos.0 as isize + 1, current.pos.1 as isize),
                self.get_codel(current.pos.0 as isize, current.pos.1 as isize - 1),
                self.get_codel(current.pos.0 as isize, current.pos.1 as isize + 1),
            ]
            .iter()
            .fold(block, |b, op| match op {
                None => {b},
                Some(next) => {
                    if current.color == next.color {
                        self.search_color_block(next, b)
                    } else {
                        b
                    }
                }
            })
        } else {
            block
        }
    }

    pub fn try_step(&mut self) -> bool {
        let mut retry_count = 0;
        while retry_count < 8 {
            if self.step() {
                return true;
            }
            if retry_count % 2 == 0 {
                self.switch_cc(1);
            } else {
                self.rotate_dp(1);
            }
            retry_count += 1;
        }
        return false;
    }

    fn step(&mut self) -> bool {
        let next_codel = self.color_block()
            .iter()
            .map(|codel| match self.dp {
                DP::Right => self.get_codel(codel.pos.0 as isize + 1, codel.pos.1 as isize),
                DP::Left  => self.get_codel(codel.pos.0 as isize - 1, codel.pos.1 as isize),
                DP::Up    => self.get_codel(codel.pos.0 as isize, codel.pos.1 as isize - 1),
                DP::Down  => self.get_codel(codel.pos.0 as isize, codel.pos.1 as isize + 1),
            })
            .filter(|op|op.is_some() && op.unwrap().color != self.codels.get(self.ptr).unwrap().color)
            .map(|op|op.unwrap())
            .min_by(|a, b| {
                match self.dp {
                    DP::Right => match b.pos.0.cmp(&a.pos.0) {
                        Ordering::Equal => match self.cc {
                            CC::Left  => (a.pos.1).cmp(&(b.pos.1)),
                            CC::Right => (b.pos.1).cmp(&(a.pos.1)),
                        },
                        other => other
                    },
                    DP::Down => match b.pos.1.cmp(&a.pos.1) {
                        Ordering::Equal => match self.cc {
                            CC::Left  => (b.pos.0).cmp(&(a.pos.0)),
                            CC::Right => (a.pos.0).cmp(&(b.pos.0)),
                        },
                        other => other
                    },
                    DP::Left => match a.pos.0.cmp(&b.pos.0) {
                        Ordering::Equal => match self.cc {
                            CC::Left  => (b.pos.1).cmp(&(a.pos.1)),
                            CC::Right => (a.pos.1).cmp(&(b.pos.1)),
                        },
                        other => other
                    },
                    DP::Up => match a.pos.1.cmp(&b.pos.1) {
                        Ordering::Equal => match self.cc {
                            CC::Left  => (a.pos.0).cmp(&(b.pos.0)),
                            CC::Right => (b.pos.0).cmp(&(a.pos.0)),
                        },
                        other => other
                    },
                }
            });
        match next_codel {
            None => false,
            Some(codel) => {
                if codel.color == BLACK {
                    false
                } else {
                    let codel = *codel;
                     match COLORS.iter().position(|c|**c == codel.color) {
                        None => {
                            self.try_slide(codel)
                        },
                        Some(_) => {
                            self.operate(codel);
                            true
                        }
                     }
                }
            }
        }
    }

    fn try_slide(&mut self, codel: Codel) -> bool {
        let mut retry_count = 0;
        while retry_count < 8 {
            if self.slide(codel) {
                return true;
            }
            if retry_count % 2 == 0 {
                self.switch_cc(1);
            } else {
                self.rotate_dp(1);
            }
            retry_count += 1;
        }
        self.ptr = codel.ptr;
        return false;
    }

    fn slide(&mut self, codel: Codel) -> bool {
        let op = match self.dp {
            DP::Right => self.get_codel(codel.pos.0 as isize + 1, codel.pos.1 as isize),
            DP::Left  => self.get_codel(codel.pos.0 as isize - 1, codel.pos.1 as isize),
            DP::Up    => self.get_codel(codel.pos.0 as isize, codel.pos.1 as isize - 1),
            DP::Down  => self.get_codel(codel.pos.0 as isize, codel.pos.1 as isize + 1),
        };
        match op {
            None => false,
            Some(next_codel) => {
                if next_codel.color == BLACK || self.ptr == next_codel.ptr {
                    false
                } else {
                    match COLORS.iter().position(|c|**c == next_codel.color) {
                        None => {
                            let codel = *next_codel;
                            self.slide(codel)
                        },
                        Some(_) => {
                            self.ptr = next_codel.ptr;
                            true
                        }
                    }
                }
            }
        }
    }

    fn switch_cc(&mut self, times: i32) {
        let n = (self.cc as i32 + times) % 2;
        self.cc = n.try_into().unwrap();
    }

    fn rotate_dp(&mut self, times: i32) {
        let mut n = (self.dp as i32 + times) % 4;
        if n < 0 {
            n += 4;
        }
        self.dp = n.try_into().unwrap();
    }

    fn operate(&mut self, new: Codel) {
        let cur = self.codels.get(self.ptr).unwrap();
        match new.color.diff_to(&cur.color) {
            (0, 1) => self.push(),
            (0, 2) => self.pop(),
            (1, 0) => self.add(),
            (1, 1) => self.subtract(),
            (1, 2) => self.multiply(),
            (2, 0) => self.divide(),
            (2, 1) => self.modulo(),
            (2, 2) => self.not(),
            (3, 0) => self.greater(),
            (3, 1) => self.pointer(),
            (3, 2) => self.switch(),
            (4, 0) => self.duplicate(),
            (4, 1) => self.roll(),
            (4, 2) => self.in_num(),
            (5, 0) => self.in_char(),
            (5, 1) => self.out_num(),
            (5, 2) => self.out_char(),
            _ => {}
        }
        self.ptr = new.ptr as usize;
    }

    fn push(&mut self) {
        self.stack.push(self.color_block().len() as isize);
    }

    fn pop(&mut self) {
        self.stack.pop();
    }

    fn add(&mut self) {
        if self.stack.len() < 2 {
            return;
        }
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        self.stack.push(a + b);
    }

    fn subtract(&mut self) {
        if self.stack.len() < 2 {
            return;
        }
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        self.stack.push(b - a);
    }

    fn multiply(&mut self) {
        if self.stack.len() < 2 {
            return;
        }
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        self.stack.push(a * b);
    }

    fn divide(&mut self) {
        if self.stack.len() < 2 {
            return;
        }
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        if b != 0 {
            self.stack.push(b / a);
        }
    }

    fn modulo(&mut self) {
        if self.stack.len() < 2 {
            return;
        }
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        if b != 0 {
            self.stack.push(b % a);
        }
    }

    fn not(&mut self) {
        if self.stack.len() < 1 {
            return;
        }
       let v = self.stack.pop().unwrap();
       self.stack.push(if v != 0 { 0 } else { 1 });
    }

    fn greater(&mut self) {
        if self.stack.len() < 2 {
            return;
        }
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        self.stack.push(if b > a { 1 } else { 0 });
    }

    fn pointer(&mut self) {
        if self.stack.len() < 1 {
            return;
        }
        let v = self.stack.pop().unwrap();
        self.rotate_dp(v as i32);
    }

    fn switch(&mut self) {
        match self.stack.pop() {
            None => {},
            Some(v) => { self.switch_cc(v as i32) }
        }
    }

    fn duplicate(&mut self) {
        match self.stack.clone().last() {
            None => {},
            Some(v) => {self.stack.push(v.clone());}
        }
    }

    fn roll(&mut self) {
        if self.stack.len() < 2 {
            return;
        }
        let mut times = self.stack.pop().unwrap();
        let depth = self.stack.pop().unwrap();
        if depth < 0 {
            self.stack.push(depth);
            self.stack.push(times);
            return;
        }
        let i = self.stack.len() as isize - depth;
        if i < 0 {
            self.stack.push(depth);
            self.stack.push(times);
            return;
        }
        while times != 0 {
            if times > 0 {
                let e = self.stack.pop().unwrap();
                self.stack.insert(i as usize, e);
                times -= 1;
            } else {
                let e = self.stack.remove(i as usize);
                self.stack.push(e);
                times += 1;
            }
        }

    }

    fn in_char(&mut self) {
        self.stack.push(0); // todo
    }

    fn in_num(&mut self) {
        self.stack.push(0); // todo
    }

    fn out_char(&mut self) {
        match self.stack.pop() {
            None => {},
            Some(v) => {print!("{}", std::char::from_u32(v as u32).unwrap())}
        }
    }

    fn out_num(&mut self) {
        match self.stack.pop() {
            None => {},
            Some(v) => {print!("{}", v as u32)}
        }
    }
}

impl fmt::Display for Piet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "codel={:?} dp={:?}, cc={:?}", self.codels[self.ptr], self.dp, self.cc)
    }
}
