use minifb::{Key, Window, WindowOptions};
use oxiconway::framebuffer::{self, Framebuffer};
use std::{
    collections::{HashMap, HashSet},
    time::Duration,
};

type Cell = (isize, isize, isize);

struct Model {
    pub live_cells: HashSet<Cell>,
    pub framebuffer_dimensions: (usize, usize),
}

fn main() {
    let window_width = 800;
    let window_height = 800;

    let framebuffer_width = 100;
    let framebuffer_height = 100;

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
    // let frame_delay = std::time::Duration::from_millis(2500);
    let live_cells = HashSet::from([(50, 50, 0), (49, 50, 0), (51, 50, 0)]);
    let mut data = Model {
        live_cells,
        framebuffer_dimensions: (framebuffer_width, framebuffer_height),
    };

    let mut frame_count: u64 = 0;
    let frame_update_timer: u64 = 100;
    while window.is_open() {
        // listen to inputs
        if window.is_key_down(Key::Escape) {
            break;
        }

        // Clear the framebuffer
        framebuffer.set_background_color(0x333355);
        framebuffer.clear();

        // Render
        render(&mut framebuffer, &data);

        // Update the model
        if frame_count >= frame_update_timer && frame_count % frame_update_timer == 0 {
            data = update(data, HashSet::new());
        }

        // Update the window with the framebuffer contents
        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
        frame_count += 1;
    }
}

fn update(data: Model, evaluated_points: HashSet<Cell>) -> Model {
    let mut live_cells = HashSet::new();
    let framebuffer_dimensions = data.framebuffer_dimensions;
    for live_cell in data.live_cells.iter() {
        if evaluated_points.contains(live_cell) {
            continue;
        }

        let neighbors = get_cell_neighbors(live_cell, &data.framebuffer_dimensions);
        let alive_neighbors_count = neighbors
            .iter()
            .filter(|n| match n {
                None => false,
                Some(cell) => data.live_cells.contains(cell),
            })
            .count();

        if alive_neighbors_count == 2 || alive_neighbors_count == 3 {
            live_cells.insert(*live_cell);
        }
    }

    Model {
        live_cells,
        framebuffer_dimensions,
    }
}

fn get_cell_neighbors(cell: &Cell, dimensions: &(usize, usize)) -> Vec<Option<Cell>> {
    let (x, y, z) = cell;
    let (width, height) = dimensions;
    let width = *width as isize;
    let height = *height as isize;

    // From top left, clockwise
    let directions = vec![
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
    ];

    directions
        .into_iter()
        .map(|(delta_x, delta_y)| {
            let new_x = x + delta_x;
            let new_y = y + delta_y;

            if new_x < 0 || new_x >= width || new_y < 0 || new_y >= height {
                None
            } else {
                Some((new_x, new_y, *z))
            }
        })
        .collect()
}

fn render(framebuffer: &mut Framebuffer, data: &Model) {
    // Clear the framebuffer
    framebuffer.set_background_color(0x333355);
    framebuffer.clear();

    // Draw some points
    framebuffer.set_current_color(0xFFDDDD);
    data.live_cells.iter().for_each(|&(x, y, z)| {
        let _ = framebuffer.paint_point(nalgebra_glm::Vec3::new(x as f32, y as f32, z as f32));
    });

    println!("Rendered frame!");
}
