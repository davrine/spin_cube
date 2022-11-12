mod cube;
mod renderer;

fn main() {
    print!("{}[2J", 27 as char); // different ways to clear screen https://stackoverflow.com/questions/34837011/how-to-clear-the-terminal-screen-in-rust-after-a-new-line-is-printed
                                 // print!("\x1B[2J");
}
