pub struct Renderer {
    pub z_buffer: Vec<f64>,
    pub buffer: Vec<u8>,
    pub width: usize,
    pub height: usize,
    pub offset: isize,
}

impl Renderer {
    pub fn new(height: usize, width: usize, offset: isize) -> Self {
        Self {
            z_buffer: vec![0f64; height * width],
            buffer: vec![' ' as u8; height * width],
            width,
            height,
            offset,
        }
    }
    pub fn render(&self) {
        print!("{}[1;1H", 27 as char);
        for i in 0..&self.width * self.height {
            if i % &self.width == 0 {
                println!();
            } else {
                print!("{}", *self.buffer.get(i).unwrap() as char);
            }
        }
    }
}
