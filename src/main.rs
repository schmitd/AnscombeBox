use cursive::event::Key;
use cursive::{
    views::{Canvas, NamedView},
    Printer, Vec2,
};
use ndarray::*;
use rand::prelude::*;

mod bitmap_loader;
mod player;
mod site;
mod state;
use bitmap_loader::{load_bitmap_from_bmp, load_bitmaps_from_directory};
use player::Player;
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

// Function to load bitmaps from files
fn load_bitmaps_from_files() -> Vec<Array2<bool>> {
    let mut bitmaps = Vec::new();
    
    // Try to load bitmaps from a "bitmaps" directory if it exists
    if let Ok(loaded_bitmaps) = load_bitmaps_from_directory("bitmaps") {
        println!("Loaded {} bitmaps from 'bitmaps' directory", loaded_bitmaps.len());
        bitmaps.extend(loaded_bitmaps);
    } else {
        println!("No 'bitmaps' directory found, using default bitmaps");
    }
    
    // If no bitmaps were loaded from files, use the default ones
    if bitmaps.is_empty() {
        println!("Using default custom bitmaps");
        bitmaps = create_custom_bitmaps();
    }
    
    bitmaps
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



fn init_state() -> (Array3<bool>, SiteManager, Array2<bool>, Array2<bool>) {
    // Initialize the main bitmap (try to load from file first)
    let bmp: Array2<bool> = if let Ok(loaded_bmp) = load_bitmap_from_bmp("main_bitmap.bmp") {
        println!("Loaded main bitmap from 'main_bitmap.bmp'");
        loaded_bmp
    } else {
        panic!("No 'main_bitmap.bmp' found!");
    };
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
    let mut sites = SiteManager::new();

    for _ in 0..N_SITES {
        let (mut best_site, mut best_goodness): (Option<Point2>, f32) = (None, 0.0);

        for _ in 0..N_TRIALS {
            let point = rand_point(2);
            let s: Point2 = (point[0], point[1]);

            let g = if sites.collides_with_sites(s, (bmp.dim().0, bmp.dim().1), &bmp) {
                0.0
            } else {
                goodness(&s, &state, &bmp)
            };

            if g > best_goodness {
                best_goodness = g;
                best_site = Some(s);
            }
        }

        if let Some(site_pos) = best_site {
            sites.add_site(site_pos);
        }
    }

    // Initialize the player bitmap (try to load from file first)
    let player_bmp: Array2<bool> = if let Ok(loaded_player_bmp) = load_bitmap_from_bmp("player_bitmap.bmp") {
        println!("Loaded player bitmap from 'player_bitmap.bmp'");
        loaded_player_bmp
    } else {
        panic!("No 'player_bitmap.bmp' found!");
    };

    (state, sites, bmp, player_bmp)
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
#[cfg(test)]
mod test_bitmap_loading;

fn main() {
    let (state, sites, bmp, player_bmp) = init_state();
    let player = Player::new((0, 0), player_bmp);
    let game_state = GameState::new(state, sites, bmp, player);
    run_sim(game_state);
}
