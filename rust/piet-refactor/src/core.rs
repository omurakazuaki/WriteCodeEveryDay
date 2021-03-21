#[derive(Debug, Copy, Clone)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r &&
        self.g == other.g &&
        self.b == other.b
    }
}
impl Eq for Color {}

#[derive(Debug, Copy, Clone)]
pub struct Codel {
    color: Color,
    pos: (usize, usize),
}

impl Codel {
    pub fn new(color: Color, width: usize, ptr: usize) -> Self {
        Codel{
            color: color,
            pos: (ptr % width, ptr / width)
        }
    }
}
impl PartialEq for Codel {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}
impl Eq for Codel {}


pub struct Piet {
    stack: Vec<isize>,
    codels: Vec<Codel>,
    ptr: usize,
}

impl Piet {
    pub fn new(frame: &gif::Frame) -> Self {
        let colors: Vec<Color> = frame.buffer.chunks(4).map(|v|Color{r:v[0], g:v[1], b:v[2]}).collect();
        Self {
            stack: vec![],
            codels: colors.iter().enumerate().map(|(ptr, color)|Codel::new(*color, frame.width as usize, ptr)).collect(),
            ptr: 0,
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
}
