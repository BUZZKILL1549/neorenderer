mod framebuffer;
use framebuffer::*;

mod colors;
use colors::*;

mod constructs;

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
        let triangle = Triangle::new([200.0, 600.0], [1000.0, 600.0], [600.0, 200.0], Colors::RED);
        triangle.draw(&mut framebuffer);
        */

        // Drawing a square
        /*
        let square = Square::new(
            [400.0, 200.0],
            [800.0, 200.0],
            [800.0, 600.0],
            [400.0, 600.0],
            Colors::RED,
        );
        square.draw(&mut framebuffer);
        */

        // Drawing a polygon
        // This maeks a 10 sided star. pretty cool, no?
        let polygon = vec![
            [600.0, 100.0],
            [650.0, 300.0],
            [800.0, 300.0],
            [675.0, 400.0],
            [750.0, 600.0],
            [600.0, 475.0],
            [450.0, 600.0],
            [525.0, 400.0],
            [400.0, 300.0],
            [550.0, 300.0],
        ];

        let triangles = framebuffer.ear_clip(polygon);
        for triangle in triangles {
            framebuffer.draw_filled_triangle(triangle[0], triangle[1], triangle[2], Colors::WHITE);
        }

        window
            .update_with_buffer(&framebuffer.buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
