mod framebuffer;
use framebuffer::*;

mod colors;
use colors::*;

mod constructs;

mod gourad;
use gourad::*;

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
        /*
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
        */
        // Cool looking Gouraud triangle
        let v0 = ColoredVertex {
            pos: [600.0, 300.0],
            color: Colors::RED,
        };
        let v1 = ColoredVertex {
            pos: [500.0, 500.0],
            color: Colors::GREEN,
        };
        let v2 = ColoredVertex {
            pos: [700.0, 500.0],
            color: Colors::BLUE,
        };

        ColoredVertex::draw_gourad_triangle(&mut framebuffer, v0, v1, v2);

        // Makes a really cool decagon
        /*
        let center = ColoredVertex {
            pos: [600.0, 400.0],
            color: Colors::WHITE,
        };

        let vertices = vec![
            (
                [
                    600.0 + 200.0 * (0.0_f64.to_radians()).cos(),
                    400.0 + 200.0 * (0.0_f64.to_radians()).sin(),
                ],
                Colors::RED,
            ),
            (
                [
                    600.0 + 200.0 * (36.0_f64.to_radians()).cos(),
                    400.0 + 200.0 * (36.0_f64.to_radians()).sin(),
                ],
                Colors::GREEN,
            ),
            (
                [
                    600.0 + 200.0 * (72.0_f64.to_radians()).cos(),
                    400.0 + 200.0 * (72.0_f64.to_radians()).sin(),
                ],
                Colors::BLUE,
            ),
            (
                [
                    600.0 + 200.0 * (108.0_f64.to_radians()).cos(),
                    400.0 + 200.0 * (108.0_f64.to_radians()).sin(),
                ],
                Colors::RED,
            ),
            (
                [
                    600.0 + 200.0 * (144.0_f64.to_radians()).cos(),
                    400.0 + 200.0 * (144.0_f64.to_radians()).sin(),
                ],
                Colors::GREEN,
            ),
            (
                [
                    600.0 + 200.0 * (180.0_f64.to_radians()).cos(),
                    400.0 + 200.0 * (180.0_f64.to_radians()).sin(),
                ],
                Colors::BLUE,
            ),
            (
                [
                    600.0 + 200.0 * (216.0_f64.to_radians()).cos(),
                    400.0 + 200.0 * (216.0_f64.to_radians()).sin(),
                ],
                Colors::RED,
            ),
            (
                [
                    600.0 + 200.0 * (252.0_f64.to_radians()).cos(),
                    400.0 + 200.0 * (252.0_f64.to_radians()).sin(),
                ],
                Colors::GREEN,
            ),
            (
                [
                    600.0 + 200.0 * (288.0_f64.to_radians()).cos(),
                    400.0 + 200.0 * (288.0_f64.to_radians()).sin(),
                ],
                Colors::BLUE,
            ),
            (
                [
                    600.0 + 200.0 * (324.0_f64.to_radians()).cos(),
                    400.0 + 200.0 * (324.0_f64.to_radians()).sin(),
                ],
                Colors::RED,
            ),
        ];

        for i in 0..vertices.len() {
            let (pos1, color1) = vertices[i];
            let (pos2, color2) = vertices[(i + 1) % vertices.len()];

            let v1 = ColoredVertex {
                pos: pos1,
                color: color1,
            };
            let v2 = ColoredVertex {
                pos: pos2,
                color: color2,
            };

            ColoredVertex::draw_gourad_triangle(&mut framebuffer, center, v1, v2);
        }
        */

        window
            .update_with_buffer(&framebuffer.buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
