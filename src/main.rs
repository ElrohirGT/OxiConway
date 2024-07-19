use minifb::{Key, Window, WindowOptions};
use oxiconway::framebuffer;
use std::time::Duration;

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

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Update the window with the framebuffer contents
        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();
    }
}
