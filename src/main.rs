mod cube;
mod cube_renderer;
mod renderer;

const CUBE_WIDTH: usize = 10;

//screen
const SCREEN_WIDTH: usize = 160;
const SCREEN_HEIGHT: usize = 50;

const HORIZONTAL_OFFSET: isize = -2 * CUBE_WIDTH as isize;

use cube_renderer::CubeRenderer;
use renderer::Renderer;

fn main() {
    let renderer = Renderer::new(SCREEN_HEIGHT, SCREEN_WIDTH, HORIZONTAL_OFFSET);
    let mut cube_renderer = CubeRenderer::new(CUBE_WIDTH, renderer, 10);
    cube_renderer.render();
}
