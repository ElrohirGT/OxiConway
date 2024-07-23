use minifb::{Key, Window, WindowOptions};
use oxiconway::framebuffer::{self, Framebuffer};
use std::{
    collections::{HashMap, HashSet},
    time::Duration,
    usize,
};

type Cell = (isize, isize, isize);

struct Model {
    pub live_cells: HashSet<Cell>,
    pub framebuffer_dimensions: (usize, usize),
}

fn main() {
    let window_width = 800;
    let window_height = 800;

    let framebuffer_width = 500;
    let framebuffer_height = 500;

    let mut framebuffer = framebuffer::Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Rust Graphics - Framebuffer Example",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    let frame_delay = std::time::Duration::from_millis(1000 / 120);
    let rle = "72bo85bo$71b3o83b3o$52boboo14bo3bo14boobo45boboo14bo3bo14boobo$51boob
ooboobo10b3o10boboobooboo43boobooboobo10b3o10boboobooboo$50bobboboboob
oo8b5o8booboobobobbo41bobbobobooboo8b5o8booboobobobbo$51bo6bo5bo4boo3b
oo4bo5bo6bo43bo6bo5bo4boo3boo4bo5bo6bo$59bo3b4obo7bob4o3bo59bo3b4obo7b
ob4o3bo$63bo3b3obobob3o3bo67bo3b3obobob3o3bo$58boobobbo3boo5boo3bobbob
oo57boobobbo3boo5boo3bobboboo$57bo4bobbobobbooboobbobobbo4bo55bo4bobbo
bobbooboobbobobbo4bo$56boobobo6boo5boo6boboboo53boobobo6boo5boo6bobob
oo$54boobobo6booboboboboboo6boboboo49boobobo6booboboboboboo6boboboo$
52boobo9bo3bobbobbo3bo9boboo45boobo9bo3bobbobbo3bo9boboo$51bo3bo9bobob
oo3boobobo9bo3bo43bo3bo9boboboo3boobobo9bo3bo$52bobo11bobobo3bobobo11b
obo45bobo11bobobo3bobobo11bobo$69bobobobo79bobobobo$66bo11bo73bo11bo$
64boobo3bobo3boboo69boobo3bobo3boboo$67bo4bo4bo75bo4bo4bo$67bobooboboo
bo75bobooboboobo$66boobo5boboo73boobo5boboo$65bobobo5bobobo71bobobo5bo
bobo$64boobobob3oboboboo69boobobob3oboboboo$67bobo5bobo75bobo5bobo$64b
ooboboo3booboboo69booboboo3booboboo$43boboo24b3o24boobo27boboo24b3o24b
oobo$39b3obooboboo18b9o18booboboob3o19b3obooboboo18b9o18booboboob3o$
38bo6boobb4o15boo5boo15b4obboo6bo17bo6boobb4o15boo5boo15b4obboo6bo$39b
oo3bo3bo4bo12boo9boo12bo4bo3bo3boo19boo3bo3bo4bo12boo9boo12bo4bo3bo3b
oo$51b3o5boo9bobobo9boo5b3o43b3o5boo9bobobo9boo5b3o$57boboo5bobboo3boo
bbo5boobo55boboo5bobboo3boobbo5boobo$54boboo3boo19boo3boobo49boboo3boo
19boo3boobo$53booboobobobo4bob5obo4bobobobooboo47booboobobobo4bob5obo
4bobobobooboo$53boobo4bob3ob11ob3obo4boboo47boobo4bob3ob11ob3obo4boboo
$57boobo3bob13obo3boboo55boobo3bob13obo3boboo$64bo15bo69bo15bo$65bo13b
o71bo13bo$$68b9o77b9o$$69boo3boo13bo51bo13boo3boo$72bo12b3oboo49boob3o
12bo$67bo9bo6bo5b3o45b3o5bo6bo9bo$67bo9bo7boo7boo39boo7boo7bo9bo$66bo
3bobobo3bo10bo4boo39boo4bo10bo3bobobo3bo$66bo4bobo4bo9bo4b3o39b3o4bo9b
o4bobo4bo$65bo4bo3bo4bo7boo3boo5b3o5bo15bo5b3o5boo3boo7bo4bo3bo4bo$65b
5o5b5o5boobboobb3obbo6b3ob3o7b3ob3o6bobb3obboobboo5b5o5b5o$85b3oboo6b
oo5boo6bo5bo6boo5boo6boob3o$86booboobboo3b4obobbo3boo7boo3bobbob4o3boo
bbooboo$$64b3o11b3o20bo27bo20b3o11b3o$69bo5bo25bobo23bobo25bo5bo$68boo
5boo77boo5boo$66bo3bobobo3bo73bo3bobobo3bo$66bobbo5bobbo73bobbo5bobbo$
66bo3bobobo3bo73bo3bobobo3bo$67bobo5bobo75bobo5bobo$68boo5boo77boo5boo
3$69boo3boo79boo3boo$69boo3boo79boo3boo$$78bo73bo$78bobo69bobo$78boo
71boo$$69boo3boo79boo3boo$69boo3boo79boo3boo$$69boo3boo79boo3boo$69boo
3boo79boo3boo4$75boo77boo$74boboo75boobo$73boobb3o71b3obboo$73b3ob3o
71b3ob3o$73bobboboo71boobobbo$73bobbobo73bobobbo$73boobobo73boboboo$
71bobbobobo73bobobobbo$70bobo5boo71boo5bobo$$70b3o8bo67bo8b3o$81bo67bo
$81bo67bo$$77b3o71b3o5$45boob3o129b3oboo$45boo4boo125boo4boo$44bobbobo
3bo20bobo77bobo20bo3bobobbo$51b3obb3o3bobo9bobo77bobo9bobo3b3obb3o$53b
3obobobbooboo8bobbo73bobbo8booboobbobob3o$53bo4bo8bobo6b3o73b3o6bobo8b
o4bo$54boo7boboo3boo87boo3boobo7boo$64b3obo8boo73boo8bob3o$66boboo7b3o
71b3o7boobo$66bobo10bo71bo10bobo$67bo11boo69boo11bo$79bo71bo$70bo10bo
21booboo15booboo21bo10bo$70bo9bobo18boobobooboo9boobooboboo18bobo9bo$
69boo4boo3boo16b3obobb3obb4o3b4obb3obbob3o16boo3boo4boo$68boo5boo3bo
13b3o4b3o5bo4bobo4bo5b3o4b3o13bo3boo5boo$68bobobboboo16bobobob3obo8boo
3boo8bob3obobobo16boobobbobo$72bobbo17bobobob4o25b4obobobo17bobbo$72b
3o19bobo4bobbo21bobbo4bobo19b3o$74bo28bo23bo28bo$73bo21bo39bo21bo$70bo
bbo21bo39bo21bobbo7$104b3oboo11boob3o$102boo4boo11boo4boo$101bo3bobobb
o9bobbobo3bo$96b3obb3o23b3obb3o$95bobob3o27b3obobo$86boo8bo4bo7boo9boo
7bo4bo8boo$86booboo8boo5booboo9booboo5boo8booboo$80boo4boobb4o9b4obbob
o7bobobb4o9b4obboo4boo$80booboo4bo4bo7bo4bo4bo5bo4bo4bo7bo4bo4booboo$
79bo3boo7boo9boo7bo5bo7boo9boo7boo3bo$83boo27bo5bo27boo$108b3o9b3o$
107bo15bo$108boboo7boobo$112bo5bo$108boo11boo$107bo3bo7bo3bo$110bo9bo$
108bo13bo$108bo13bo$114b3o$108boo4b3o4boo$113bo3bo$107b3o11b3o$66boo
41bobbo5bobbo41boo$66boo40bo3boo3boo3bo40boo$114bobo$107b3o5bo5b3o$
107b4obooboboob4o$109bobb7obbo$110b4o3b4o$109b3o7b3o$108bo4bo3bo4bo$
107bo3bo3bo3bo3bo$65bo46bob3obo46bo$65boo40bob4o5b4obo40boo$64bo42bo4b
3ob3o4bo42bo$65b3o42boo3bo3boo42b3o$66bo46bo3bo46bo$48bo60boobbooboobb
oo60bo$44b3ob3o57bobobbooboobbobo57b3ob3o$43bo6boo56bo6bo6bo56boo6bo$
44boo3bobbo3bo50boo6bo6boo50bo3bobbo3boo$54b4o49bob3obbobobb3obo49b4o$
54bo3bo52bo7bo52bo3bo$52bobbobo52bobo5bobo52bobobbo$53bo53b3obbob3obo
bb3o53bo$41boob3o59boboboboo3boobobobo59b3oboo$41boo4boo32b3o22bobobob
oo3boobobobo22b3o32boo4boo$40bobbobo3bo33bo23boob3obobob3oboo23bo33bo
3bobobbo$47b3obb3o27bo25boo3booboo3boo25bo27b3obb3o$49b3obobo54b3obobo
b3o54bobob3o$49bo4bo52bo4bobobobo4bo52bo4bo$50boo60bobobobo60boo$66boo
46bobo46boo$66boo36boo8bobo8boo36boo$103bobo7booboo7bobo$105bo19bo$
113bo3bo$112bo5bo$51boob3o20b3oboo30bo3bo30boob3o20b3oboo$51boo4boo16b
oo4boo65boo4boo16boo4boo$50bobbobo3bo14bo3bobobbo63bobbobo3bo14bo3bobo
bbo$57b3obb3o4b3obb3o77b3obb3o4b3obb3o$59b3obobobbobob3o81b3obobobbobo
b3o$59bo4bo4bo4bo12bo55bo12bo4bo4bo4bo$60boo10boo9b3ob3o51b3ob3o9boo
10boo12bo$82bo6boo49boo6bo33bobo$83boo3bobbo3bo39bo3bobbo3boo18bo14bo
3bo14bo$93b4o37b4o24b3ob3o13b3o13b3ob3o$93bo3bo35bo3bo23bo6bob3o21b3ob
o6bo$46b3o42bobbobo37bobobbo22boo3bo5bo6boo3boo6bo5bo3boo$46b3o43bo45b
o31b7o3boo3boo3b7o$26boob3o13bo3bo13b3oboo99bobboo3boobb3ob3obboo3boo
bbo$26boo4booboo21booboo4boo100boobo4boo9boo4boboo$25bobbobo4boo7bo5bo
7boo4bobobbo105b4obbooboobb4o$32boobobboo4booboboo4boobboboo104boob4o
7booboo7b4oboo$38boboo11boobo109boboboo5boobob3oboboo5boobobo$33b3oboo
bbo3bo3bo3bobboob3o101boobo10boobo5boboo10boboo$33bo3b3obo4bobo4bob3o
3bo101boobo9bobobo5bobobo9boboo$31booboboo4bobbo3bobbo4booboboo99bobo
10bo3bo5bo3bo10bobo$30bobobbo8bobobobo8bobbobo117bobo$28b3obobbo5boobo
bbobboboo5bobbob3o109bo3b3ob3o3bo$27boobo9bobobob3obobobo9boboo108bob
5ob5obo$26bo3bo9bobobbooboobbobo9bo3bo107bobo9bobo$41b3obbobobb3o124bo
booboboobo$42boboo3boobo124boobob3oboboo$40boo3bo3bo3boo123bobob3obobo
$40bobo4bo4bobo120booboboo3booboboo$42bobbobobobbo122boobobo5boboboo$
42bob3ob3obo123bobobo5bobobo$42bobo5bobo104bo21boo5boo21bo$39boobobobb
obboboboo94boob3ob3o20b7o20b3ob3oboo$39boobobobbobboboboo94boo4bobboob
oo19bo19booboobbo4boo$42bobob3obobo96bobboboo3bobboo14bobobbobbobo14b
oobbo3boobobbo$41bobb7obbo107bobbo5bo6boobo5boboo6bo5bobbo$15boboob3o
49b3oboobo85bobb4o5boo3bobo3boo5b4obbo$14booboo4boobo16bo7bo16boboo4b
ooboo87bo3bo6boo5boo6bo3bo$13bobbobobbo3b3o18b3o18b3o3bobbobobbo81b3o
bboboboo5b9o5boobobobb3o$14bo5bo3bo3bo13b3o5b3o13bo3bo3bo5bo85bobboobo
boo15booboboobbo$27boo4b3o5boob3ob3oboo5b3o4boo100boobo3boboo9boobo3bo
boo$27boobboobo9b3ob3o9boboobboo107bobob9obobo13b3o$28boo4boboo9bo9boo
bo4boo109b15o13bo$33bobbobo3boo7boo3bobobbo116b11o14boo$28boobobbobobo
bo13bobobobobboboo130boboobboobb3o$24boo6bo8bo11bo8bo133boobooboo5bo4b
oo$23boobo12bob13obo122boo3bo3boo6bobboo3boboo4booboo$23boobo20bo129bo
booboboboobo11b4o6boo3bo$21b3oboo3bo16bo128boo11boo6boboboo8boo$16bo3b
o4bobb3ob3o10booboo127bobo7bobo9bo$15b4o4b3obboo5bo8bobobobo127boo7boo
7boo18bo$14bo3bo9boobobbo7bobooboboobo59bo5bo60bobo3bobo7boobbo4boobo
3bo3boob3o$15bobobbo4boo4bo12bobobobo61bo5bo58boo9boo7b3oboobobooboob
oobob3obbo3b3o$19bo9bob3o8bo9bo59bo5bo58boo9boo3bobbo5boboo6bo9bobbobo
boo$14bo16bo3bo5boo9boo123bobooboboboobo4bob4obo3b3o3bobobbo3bobobbobo
3bo$14b3obbo6bo3bobobbo3bo15bo124bo5bo12b3obboo10bo4boobbobbobo$3boo4b
5o4bobob3ob3o6boo3bobo9bobo126bo3bo13boo4bo16b5o$ooboo3b3obbo4bobo3boo
bbo3booboo5bo9bo129b3o15boboobo17boo$oo3bo4b5obobo3boo12bo3bobo9bobo$o
bobbobbo3boo12bobboobobbo4boo11boo117bo28bobbo$3bob4o16boobboobbo137b
4o26bobbo$25boboo142boboo$30bo11b3o5b3o$26b4o14bo5bo129boo$27boo17b3o
10bo120boo$46b3o9bobo$47bo$177boo$176bobbo$49boo125bobo18b3ob3o$49boo
126bo16b4obo4boobo$190bo3boo3boobo3b3o$189b4o5boobo3bo3bo$28booboo19b
oo134bo3bobo3bo9bo$25boobooboboo16bobbo133bobobobo3bo$22b4obb3obbob3o
14bobo122bo5boo4b3o4bobo$21bo4bo5b3o4b3o11bo122boo4bobbo4bo13b3ob3o$
22boo8bob3obobobo133boo5b3o15b4obo4boobo$33b4obobobo154bo3boo3boobo3b
3o$31bobbo4bobo117bo36b4o5boobo3bo3bo$21booboo6bo122b3ob3o33bo3bobo3bo
9bo$18boobooboboo12bo4b3o106bo6boo14bo17bobobobo3bo$15b4obb3obbob3o9bo
3bo3bo106boo3bobbo3bo7booboo7boo7b3o4bobo$14bo4bo5b3o4b3o8bo5bo115b4o
6booboo6bobbo7bo$15boo8bob3obobobo8bo3bo21b3oboo89bo3bo5booboo6boob5o
15b3oboo$26b4obobobo9b3o4b3o13boo4boo87bobbobo8bo8boobb4o13boo4boo$24b
obbo4bobo16bo3bo11bo3bobobbo87bo24b3obboo10bo3bobobbo$25bo15bobo6bo5bo
5b3obb3o118b3obobbo5b3obb3o$17boboo12bo15bo3bo3bo3bobob3o116boo3boo4bo
4bobob3o$16boobooboo9bo10bo4bobbobobbo4bo4bo116b3obo4boo6bo4bo$15bobbo
bobbo12boo11bo3bo3bo7boo117boboo16boo$16bo5boo4boo13boobo3bo5bo128boo
bbo$25booboo3boobo3bobobo6bo3bo128bobb4o$25boo3bobbobboobobboobbo5b3o
124boo4bo5bo$25boo6bobbobboo5boo131boo5b5o$34bobobbo6bo140b3o4$50boo$
38boboo8boo$39b3o84bo26bo$40bo81b3ob3o11b4obo5b4o$121bo6bob3o7bobobboo
3boo3bo$122boo3bo5bobboobobbo4bo5boboo$130boobobbobobboobobbo3b3obobo
26b3o$76b3o6booboo13b3oboo23boobboboobbobbobbo4boboboboobo15b3o4boo$
77boboo3bo3boboo6booboo4boo23boo3b3obbooboobobbobobobobooboo14bobboobb
o$74boobo3bobo4bo4bo4boo4bobobbo25b3obbobbobo3bobobobobobo4bo13booboo
bboo$73bobobb5o3b3o3b3obboboboo30bobbo5bo3bo4bobobobo5b3o14boboboo12b
3oboo$71b3obobo3boobbobbo3bo4boo36bo17boboboboobb4o14b4o11boo4boo$67b
3obobobobobbo4b3o3bobbo60bo5b3obbo15boo11bo3bobobbo$45b3oboboobo11bo6b
obobob3o3bobo3bobbo3bo54bo7bo4bo23b3obb3o$28boboo13boo3bo3bo12booboobo
bobobo3bobobobo4bo3bo48boo4bo3b4obboboo22bobob3o$27boobooboo10b4o3bobo
9boo7bobobobo15boo37boboo8bobbooboobbobobobooboo23bo4bo$26bobbobobbo8b
oob6o12boobboobbooboo56boobooboo4bo3bobboobo8bo28boo$27bo5boo4boo4bo
20boobobboobo56bobbobobbo5b4o13boo$36booboo21bobb3oboboobb3o4bo50bo5b
oo4bobbobobboo$36boo3bo21bobo9b3o3boboo8bo48boobobbobobboo$36boo29bobb
obobbo3boboboo6b3ob3o44boo4bobo4bo$66bo10bo3boobbo4boo6bo45bobobobobob
o9boo$67bo12bo8bobbo3boo65boo$76boobbobobb3o77bo$75bob4ob4obo$76bobobo
bobobobbo62b3o$66b3o83bo$68bo84bo$67bo74bo$78bo62boo$78boo61bobo$77bob
o$130boo$89boo39bobo$88bobo39bo$90bo$119boo$100boo16boo$101boo17bo$
100bo$$111b3o$113bo$112bo20$179boo$178bobo$180bo21$246b3o$248bo$247bo!";
    let mut data = generate_live_board_cells(rle, &framebuffer_width, &framebuffer_height);

    let mut frame_count: u64 = 0;
    let frame_update_timer: u64 = 1;
    framebuffer.set_background_color(0x040b05);
    framebuffer.set_current_color(0xc35817);
    while window.is_open() {
        // listen to inputs
        if window.is_key_down(Key::Escape) {
            break;
        }

        // Render
        render(&mut framebuffer, &data);

        // Update the model
        if frame_count >= frame_update_timer && frame_count % frame_update_timer == 0 {
            data = update(data);
            // framebuffer
            //     .save(&format!("image_{}.bmp", frame_count))
            //     .unwrap();
        }

        // Update the window with the framebuffer contents
        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
        frame_count += 1;
    }
}

fn generate_live_board_cells(
    rle: &str,
    framebuffer_width: &usize,
    framebuffer_height: &usize,
) -> Model {
    // d o d
    // d d o
    // o o o
    let mut buffer: String = "".to_owned();
    let x_0 = ((*framebuffer_width as f32) * 0.2).round() as isize;
    let y_0 = ((*framebuffer_height as f32) * 0.3).round() as isize;

    let mut x = x_0;
    let mut y = y_0;

    let mut live_cells = HashSet::new();
    for char in rle.chars() {
        println!("Parsing {}...", char);
        match char {
            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => buffer.push(char),
            'b' => {
                let n = buffer.parse().unwrap_or(1);
                x += n;
                buffer.clear();
            }
            'o' => {
                let n = buffer.parse().unwrap_or(1);
                println!("Generating from {}..{}+{}", x, x, n);
                (x..(x + n)).for_each(|xi| {
                    println!("Generating ({}, {})", xi, y);
                    live_cells.insert((xi, y, 0));
                });
                x += n;
                buffer.clear();
            }
            '$' => {
                let n = buffer.parse().unwrap_or(1);

                x = x_0;
                y += n;

                buffer.clear();
                println!("New Line... ({}, {})", x, y);
            }
            _ => {}
        }
    }

    println!("Foud live cells: {:?}", live_cells);

    Model {
        live_cells,
        framebuffer_dimensions: (*framebuffer_width, *framebuffer_height),
    }
}

fn update(data: Model) -> Model {
    let Model {
        live_cells,
        framebuffer_dimensions,
    } = data;
    let mut already_evaluated_cells = HashSet::new();

    let live_cells = live_cells
        .iter()
        .flat_map(|lc| {
            let mut neighbors = get_cell_neighbors(lc, &data.framebuffer_dimensions);
            neighbors.append(&mut vec![Some(*lc)]);
            neighbors
        })
        .flatten()
        .filter_map(|cell| {
            if already_evaluated_cells.insert(cell) {
                evaluate_cell(&cell, &live_cells, &framebuffer_dimensions).cloned()
            } else {
                None
            }
        })
        .collect();

    Model {
        live_cells,
        framebuffer_dimensions,
    }
}

fn evaluate_cell<'a>(
    cell: &'a Cell,
    live_cells: &HashSet<Cell>,
    dimensions: &(usize, usize),
) -> Option<&'a Cell> {
    let neighbors = get_cell_neighbors(cell, dimensions);
    let alive_neighbors_count = neighbors
        .iter()
        .filter(|n| match n {
            None => false,
            Some(cell) => live_cells.contains(cell),
        })
        .count();

    let is_alive = live_cells.contains(cell);
    if is_alive {
        if alive_neighbors_count == 2 || alive_neighbors_count == 3 {
            Some(cell)
        } else {
            None
        }
    } else if alive_neighbors_count == 3 {
        Some(cell)
    } else {
        None
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
    framebuffer.clear();

    // Draw some points
    data.live_cells.iter().for_each(|&(x, y, z)| {
        let _ = framebuffer.paint_point(nalgebra_glm::Vec3::new(x as f32, y as f32, z as f32));
    });
}
