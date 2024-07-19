use minifb::{Key, Window, WindowOptions};
use oxiconway::framebuffer::{self, Framebuffer};
use std::time::Duration;

struct Model {
    pub position: nalgebra_glm::Vec3,
    pub velocity: nalgebra_glm::Vec3,
}

fn main() {
    let window_width = 800;
    let window_height = 600;

    let framebuffer_width = 80;
    let framebuffer_height = 60;

    let mut framebuffer = framebuffer::Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Rust Graphics - Framebuffer Example",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    // Clear the framebuffer
    framebuffer.set_background_color(0x333355);
    framebuffer.clear();

    // Draw a point
    framebuffer.set_current_color(0xFFDDDD);
    framebuffer
        .paint_point(nalgebra_glm::Vec3::new(1.0, 1.0, 1.0))
        .unwrap();

    // Update the window with the framebuffer contents
    window
        .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
        .unwrap();

    let frame_delay = std::time::Duration::from_millis(1000 / 60);
    let mut data = Model {
        position: nalgebra_glm::Vec3::new(20.0, 40.0, 0.0),
        velocity: nalgebra_glm::Vec3::new(0.5, 0.5, 0.0),
    };

    while window.is_open() {
        // listen to inputs
        if window.is_key_down(Key::Escape) {
            break;
        }

        // Clear the framebuffer
        framebuffer.set_background_color(0x333355);
        framebuffer.clear();

        data.position += data.velocity;

        // render
        render(&mut framebuffer, &mut data);

        // Update the window with the framebuffer contents
        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}

fn render(framebuffer: &mut Framebuffer, data: &mut Model) {
    // Clear the framebuffer
    framebuffer.set_background_color(0x333355);
    framebuffer.clear();

    // Draw some points
    framebuffer.set_current_color(0xFFDDDD);
    let _ = framebuffer.paint_point(data.position);
}
