mod framebuffer;
use framebuffer::*;

mod colors;
use colors::*;

mod constructs;
use constructs::*;

use minifb::{Key, Window, WindowOptions};

fn main() {
    const WIDTH: usize = 1200;
    const HEIGHT: usize = 800;

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    let mut window = Window::new("Neorenderer", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| panic!("{}", e));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        framebuffer.clear(Colors::BLACK);

        // Drawing a triangle
        /*
        framebuffer.draw_line(200.0, 600.0, 1000.0, 600.0, Colors::RED);
        framebuffer.draw_line(1000.0, 600.0, 600.0, 200.0, Colors::RED);
        framebuffer.draw_line(600.0, 200.0, 200.0, 600.0, Colors::RED);
        */
        let triangle = Triangle::new([200.0, 600.0], [1000.0, 600.0], [600.0, 200.0], Colors::RED);
        triangle.draw(&mut framebuffer);

        window
            .update_with_buffer(&framebuffer.buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
