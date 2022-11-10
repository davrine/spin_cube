struct Angles {
    a: f64,
    b: f64,
    c: f64,
}

impl Angles {
    pub fn new(a: f64, b: f64, c: f64) -> Angles {
        Angles { a, b, c }
    }
    pub fn calculate_x(self, i: f64, j: f64, k: f64) -> f64 {
        j * self.a.sin() * self.b.sin() * self.c.cos()
            - k * self.a.cos() * self.b.sin() * self.c.cos()
            + j * self.a.cos() * self.c.sin()
            + k * self.a.sin() * self.c.sin()
            + i * self.b.cos() * self.c.cos()
    }
}
