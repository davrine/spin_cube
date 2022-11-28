use crate::cube::*;
use crate::renderer::*;
use std::{thread, time, time::Instant};
pub struct CubeRenderer {
    cube_width: usize,
    renderer: Renderer,
    angle: Angles,
    dt: f64,
    fps: i32,
}
impl CubeRenderer {
    pub fn new(width: usize, renderer: Renderer, fps: i32) -> Self {
        let angle = Angles {
            a: 0f64,
            b: 0f64,
            c: 0f64,
        };
        Self {
            cube_width: width,
            renderer,
            angle,
            dt: 0.2f64,
            fps,
        }
    }
    pub fn render(&mut self) {
        let bg = ' ' as u8;
        let c = 1f64 * self.cube_width as f64;
        loop {
            self.renderer.buffer.fill(bg);
            self.renderer.z_buffer.fill(0f64);
            let start = Instant::now();

            let mut cube_x = -1f64 * self.cube_width as f64;
            while cube_x < self.cube_width as f64 {
                let mut cube_y = -1f64 * self.cube_width as f64;
                while cube_y < self.cube_width as f64 {
                    cube_y += self.dt;
                    let i = -cube_x;
                    let j = cube_y;
                    let k = c;

                    let ch = '^' as u8;
                    self.renderer
                        .calculate_surface(&self.angle, &i, &j, &k, &ch);

                    let i = cube_x;
                    let j = cube_y;
                    let k = -c;

                    let ch = '$' as u8;
                    self.renderer
                        .calculate_surface(&self.angle, &i, &j, &k, &ch);

                    let i = -c;
                    let j = cube_y;
                    let c = -cube_x;
                    let ch = ',' as u8;
                    self.renderer
                        .calculate_surface(&self.angle, &i, &j, &k, &ch);
                    let i = c;
                    let j = cube_y;
                    let k = cube_x;
                    let ch = '%' as u8;
                    self.renderer
                        .calculate_surface(&self.angle, &i, &j, &k, &ch);

                    let i = cube_x;
                    let j = -c;
                    let k = -cube_y;
                    let ch = '/' as u8;
                    self.renderer
                        .calculate_surface(&self.angle, &i, &j, &k, &ch);

                    let i = cube_x;
                    let j = c;
                    let k = cube_y;
                    let ch = '#' as u8;
                    self.renderer
                        .calculate_surface(&self.angle, &i, &j, &k, &ch);
                }
                cube_x += self.dt;
            }
            self.renderer.render();
            self.angle.a += 0.05;
            self.angle.b += 0.05;
            self.angle.c += 0.01;

            let elapsed = start.elapsed();
            let frame_time = 1100 / self.fps; // Code is used to control speed of rotation

            let wait_time =
                time::Duration::from_millis((frame_time - elapsed.as_millis() as i32) as u64);
            thread::sleep(wait_time);
        }
    }
}
