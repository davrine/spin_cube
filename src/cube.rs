pub struct Angles {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

impl Angles {
    pub fn calculate_x(&self, i: &f64, j: &f64, k: &f64) -> f64 {
        j * self.a.sin() * self.b.sin() * self.c.cos()
            - k * self.a.cos() * self.b.sin() * self.c.cos()
            + j * self.a.cos() * self.c.sin()
            + k * self.a.sin() * self.c.sin()
            + i * self.b.cos() * self.c.cos()
    }

    pub fn calculate_y(&self, i: &f64, j: &f64, k: &f64) -> f64 {
        j * self.a.cos() * self.c.cos() + k * self.a.sin() * self.c.cos()
            - j * self.a.sin() * self.b.sin() * self.c.sin()
            + k * self.a.cos() * self.b.sin() * self.c.sin()
            - i * self.a.cos() * self.c.sin()
    }

    pub fn calculate_z(&self, i: &f64, j: &f64, k: &f64) -> f64 {
        k * self.a.cos() * self.b.cos() - j * self.a.sin() * self.b.cos() + i * self.b.sin()
    }
}
