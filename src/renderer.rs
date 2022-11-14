use crate::cube::Angles;
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

    pub fn calculate_surface(&mut self, angle: Angles, i: f64, j: f64, k: f64, ch: &u8) {
        let x = angle.calculate_x(i, j, k);
        let y = angle.calculate_y(i, j, k);
        let z = angle.calculate_z(i, j, k);

        let ooz = 1f64 / z;
        let xp = self.width as isize / 2 + self.offset + (50f64 * ooz * x * 2f64) as isize;
        let yp = self.height as isize / 2 + (50f64 * ooz * y) as isize;
        let idx = xp as usize + (yp as usize * self.width);
        if idx < (self.width * self.height) {
            if ooz > self.z_buffer[idx] {
                self.z_buffer[idx] = ooz;
                self.buffer[idx] = *ch;
            }
        }
    }
}
