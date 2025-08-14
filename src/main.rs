use cursive::event::Key;
use cursive::{
    views::{Canvas, NamedView},
    Printer, Vec2,
};
use ndarray::*;
use rand::prelude::*;

mod site;
mod state;
use site::SiteManager;
use state::{GameState, Point2, GRID_SIZE, N_SITES, N_TRIALS};

// Example function to create custom bitmaps for player sites
fn create_custom_bitmaps() -> Vec<Array2<bool>> {
    vec![
        // Simple cross pattern
        array![
            [false, true, false],
            [true, true, true],
            [false, true, false],
        ],
        // Small square
        array![[true, true], [true, true],],
        // Diagonal line
        array![
            [true, false, false],
            [false, true, false],
            [false, false, true],
        ],
        // Hollow square
        array![[true, true, true], [true, false, true], [true, true, true],],
    ]
}

// Example function to demonstrate site management
fn demonstrate_site_management() {
    let mut site_manager = SiteManager::new();

    // Add some default sites
    site_manager.add_site((10, 10));
    site_manager.add_site((20, 20));

    // Add custom sites with different bitmaps
    let custom_bitmaps = create_custom_bitmaps();
    site_manager.add_custom_site((30, 30), custom_bitmaps[0].clone());
    site_manager.add_custom_site((40, 40), custom_bitmaps[1].clone());

    println!("Total sites: {}", site_manager.total_count());
    println!("Active sites: {}", site_manager.active_count());

    // Show how to access sites
    for site in site_manager.get_active_sites() {
        println!(
            "Site at {:?} with custom bitmap: {}",
            site.position,
            site.custom_bitmap.is_some()
        );
    }
}

fn goodness(cords: &Point2, side: &Array3<bool>, bmp: &Array2<bool>) -> f32 {
    // Check if the bitmap would fit within the slice at the given coordinates
    if cords.0 + bmp.dim().0 > side.dim().0 || cords.1 + bmp.dim().1 > side.dim().1 {
        return 0.0;
    }

    // Extract the 2D slice from the 3D array (assuming we're using the first z-layer)
    let window = side.slice(s![
        cords.0..cords.0 + bmp.dim().0,
        cords.1..cords.1 + bmp.dim().1,
        0
    ]);
    let bits = xor(&window.to_owned(), bmp);

    // Count matching bits (false in XOR result means matching)
    let mut tot = 0;
    for &i in bits.iter() {
        if !i {
            tot += 1
        }
    }

    tot as f32 / (bmp.dim().0 * bmp.dim().1) as f32
}

fn xor(slice: &Array2<bool>, bmp: &Array2<bool>) -> Array2<bool> {
    // Create a result array with the same dimensions as the input arrays
    let mut result = Array2::from_elem(slice.dim(), false);
    for ((i, j), val) in result.indexed_iter_mut() {
        *val = slice[[i, j]] ^ bmp[[i, j]];
    }
    result
}

fn rand_point(n: usize) -> Vec<usize> {
    let mut ret: Vec<usize> = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..n {
        let r: f64 = rng.gen();
        let r: usize = (r * GRID_SIZE as f64).floor() as usize;
        ret.push(r);
    }
    ret
}

fn collides(s: Point2, sites: &Vec<Option<Point2>>, site_shape: (usize, usize)) -> bool {
    for site in sites.iter().flatten() {
        // Check if the two rectangles overlap
        let s_end = (s.0 + site_shape.0, s.1 + site_shape.1);
        let site_end = (site.0 + site_shape.0, site.1 + site_shape.1);

        // Check for overlap in both dimensions
        if s.0 < site_end.0 && s_end.0 > site.0 && s.1 < site_end.1 && s_end.1 > site.1 {
            return true;
        }
    }
    false
}

fn init_state() -> (Array3<bool>, Vec<Option<Point2>>, Array2<bool>) {
    // Initialize the bitmap
    let bmp: Array2<bool> = array![
        [
            false, false, false, false, false, false, false, true, true, true, true, true, true,
            true, false, false, false, false, false, false, false
        ],
        [
            false, false, false, false, false, true, true, false, false, false, true, false, false,
            false, true, true, false, false, false, false, false
        ],
        [
            false, false, false, false, true, false, false, false, false, false, true, false,
            false, false, false, false, true, false, false, false, false
        ],
        [
            false, false, false, true, false, false, false, false, false, false, true, false,
            false, false, false, false, false, true, false, false, false
        ],
        [
            false, false, true, false, false, false, false, false, false, false, true, false,
            false, false, false, false, false, false, true, false, false
        ],
        [
            false, true, false, false, false, false, false, false, false, false, true, false,
            false, false, false, false, false, false, false, true, false
        ],
        [
            false, true, false, false, false, false, false, false, false, false, true, false,
            false, false, false, false, false, false, false, true, false
        ],
        [
            true, false, false, false, false, false, false, false, false, false, true, false,
            false, false, false, false, false, false, false, false, true
        ],
        [
            true, false, false, false, false, false, false, false, false, false, true, false,
            false, false, false, false, false, false, false, false, true
        ],
        [
            true, false, false, false, false, false, false, false, false, false, true, false,
            false, false, false, false, false, false, false, false, true
        ],
        [
            true, false, false, false, false, false, false, false, false, false, true, false,
            false, false, false, false, false, false, false, false, true
        ],
        [
            true, false, false, false, false, false, false, false, false, true, true, true, false,
            false, false, false, false, false, false, false, true
        ],
        [
            true, false, false, false, false, false, false, false, false, true, true, true, false,
            false, false, false, false, false, false, false, true
        ],
        [
            true, false, false, false, false, false, false, false, true, false, true, false, true,
            false, false, false, false, false, false, false, true
        ],
        [
            false, true, false, false, false, false, false, true, false, false, true, false, false,
            true, false, false, false, false, false, true, false
        ],
        [
            false, true, false, false, false, false, false, true, false, false, true, false, false,
            true, false, false, false, false, false, true, false
        ],
        [
            false, false, true, false, false, false, true, false, false, false, true, false, false,
            false, true, false, false, false, true, false, false
        ],
        [
            false, false, false, true, false, true, false, false, false, false, true, false, false,
            false, false, true, false, true, false, false, false
        ],
        [
            false, false, false, false, true, true, false, false, false, false, true, false, false,
            false, false, true, true, false, false, false, false
        ],
        [
            false, false, false, false, true, true, true, false, false, false, true, false, false,
            false, true, true, true, false, false, false, false
        ],
        [
            false, false, false, false, false, false, false, true, true, true, true, true, true,
            true, false, false, false, false, false, false, false
        ],
    ];
    assert!(
        bmp.dim().0 == bmp.dim().1,
        "number of arrays == length of first array"
    );

    // Count true values in bmp
    let tot: usize = bmp.iter().map(|&b| b as usize).sum();
    //println!("total number of true bits in bmp: {}", tot);
    let r: f64 = tot as f64 / (bmp.dim().0 * bmp.dim().1) as f64;

    // Initialize state with random bits
    let mut state = Array3::<bool>::from_elem((GRID_SIZE, GRID_SIZE, GRID_SIZE), false);
    let mut i = 0;
    //println!("GRID_SIZE.pow(3) * r: {}", GRID_SIZE.pow(3) as f64 * r);

    // flip some bits
    while (i as f64) < GRID_SIZE.pow(3) as f64 * r {
        let points = rand_point(3);
        let (x, y, z) = (points[0], points[1], points[2]);

        if !state[[x, y, z]] {
            state[[x, y, z]] = true;
            i += 1;
        }
    }
    //println!("attempted to flip bits {} times.", i);

    // Initialize sites
    let mut sites: Vec<Option<Point2>> = Vec::new(); // XXX why is the Option necessary?

    for _ in 0..N_SITES {
        let (mut best_site, mut best_goodness): (Option<Point2>, f32) = (None, 0.0);

        for _ in 0..N_TRIALS {
            let point = rand_point(2);
            let s: Point2 = (point[0], point[1]);

            let g = if collides(s, &sites, (bmp.dim().0, bmp.dim().1)) {
                0.0
            } else {
                goodness(&s, &state, &bmp)
            };

            if g > best_goodness {
                best_goodness = g;
                best_site = Some(s);
            }
        }

        sites.push(best_site);
    }

    (state, sites, bmp)
}

fn run_sim(game_state: GameState) {
    // Initialize visualization with cursive
    let siv = cursive::default();
    let mut siv = siv.into_runner();

    // Create Canvas with initial state and player position
    let (initial_state, player_pos) = game_state.get_render_data_with_player();
    let canvas = Canvas::new((initial_state, player_pos))
        .with_draw(
            |(grid, player_pos): &(Array2<bool>, Point2), printer: &Printer| {
                // Draw the grid
                for (pos, value) in grid.indexed_iter() {
                    let ch = if *value { "█" } else { "." }; // XXX add another condition or fix this code to hilight player position
                    printer.print((pos.1, pos.0), ch);
                }

                // Render player position in red
                printer.with_color(
                    cursive::theme::ColorStyle::new(
                        cursive::theme::Color::Rgb(255, 0, 0), // Red
                        cursive::theme::Color::Rgb(255, 0, 0), // Red background
                    ),
                    |printer| {
                        printer.print((player_pos.1, player_pos.0), "█");
                    },
                );
            },
        )
        .with_required_size(|_, _| Vec2::new(GRID_SIZE, GRID_SIZE));

    siv.add_layer(NamedView::new("canvas", canvas));
    siv.add_global_callback('q', |s| s.quit());

    siv.set_user_data(game_state);

    // add WASD inputs
    siv.add_global_callback('w', |s| {
        s.with_user_data(|game_state: &mut GameState| {
            game_state.move_player('w');
        });
    });
    siv.add_global_callback('a', |s| {
        s.with_user_data(|game_state: &mut GameState| {
            game_state.move_player('a');
        });
    });
    siv.add_global_callback('s', |s| {
        s.with_user_data(|game_state: &mut GameState| {
            game_state.move_player('s');
        });
    });
    siv.add_global_callback('d', |s| {
        s.with_user_data(|game_state: &mut GameState| {
            game_state.move_player('d');
        });
    });

    // press enter to force a new site at player
    siv.add_global_callback(Key::Enter, |s| {
        s.with_user_data(|game_state: &mut GameState| {
            game_state.force_site();
        });
    });

    siv.refresh();

    while siv.is_running() {
        // Perform simulation step and get update flag
        let (should_update, render_data) = siv
            .with_user_data(|game_state: &mut GameState| {
                game_state.step();
                let should_update = game_state.should_update_display();
                let render_data = if should_update {
                    Some(game_state.get_render_data_with_player())
                } else {
                    None
                };
                (should_update, render_data)
            })
            .unwrap_or((false, None));

        // Update canvas when needed
        if should_update {
            if let Some(render_data) = render_data {
                if let Some(mut canvas) = siv.find_name::<Canvas<(Array2<bool>, Point2)>>("canvas")
                {
                    *canvas.state_mut() = render_data;
                }
            }
            siv.step();
            siv.refresh();
        }
    }
}

#[cfg(test)]
mod tests;

fn main() {
    // Demonstrate the new site system
    demonstrate_site_management();

    let (state, sites, bmp) = init_state();
    let game_state = GameState::new(state, sites, bmp);
    run_sim(game_state);
}
