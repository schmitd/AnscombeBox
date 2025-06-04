use cursive::{views::{NamedView, Canvas}, View, Printer, Vec2};
use ndarray::*;
use rand::prelude::*;
use std::vec;

type Point2 = (usize, usize);
type Point3 = (usize, usize, usize);
const L: usize = 60;
const N_SITES: usize = 6;
const N_TRIALS: usize = 100;
/*
const BMP : [[bool; 5]; 5] =
           [[false, false, true,  false, false],
            [false, false, true,  false, false],
            [true,  true,  true,  true,  true],
            [false, false, true,  false, false],
            [false, false, true,  false, false]];
*/

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
        let r: usize = (r * L as f64).floor() as usize;
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

fn random_direct_neighbor(point: Point3, state: &Array3<bool>) -> Option<Point3> {
    let (x, y, z) = point;
    let mut rng = rand::thread_rng();
    let directions = [
        (1, 0, 0), (-1, 0, 0),
        (0, 1, 0), (0, -1, 0),
        (0, 0, 1), (0, 0, -1)
    ];
    
    // Filter valid directions (within bounds)
    let valid_directions: Vec<_> = directions.iter()
        .filter_map(|&(dx, dy, dz)| {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            let nz = z as isize + dz;
            
            if nx >= 0 && nx < state.dim().0 as isize && 
               ny >= 0 && ny < state.dim().1 as isize && 
               nz >= 0 && nz < state.dim().2 as isize {
                Some((nx as usize, ny as usize, nz as usize))
            } else {
                None
            }
        })
        .collect();
    
    if valid_directions.is_empty() {
        None
    } else {
        // Choose a random valid direction
        let idx = rng.gen_range(0..valid_directions.len());
        Some(valid_directions[idx])
    }
}

fn try_exchange(
    p_anyway: f64,
    p_exchange: f64,
    sites: &mut Vec<Option<Point2>>,
    point: Point3,
    neighbor: Point3,
    state: &mut Array3<bool>,
    bmp: &Array2<bool>,
) {
    let mut rng = rand::thread_rng();
    let a: f64 = rng.gen();

    if a < p_anyway {
        state.swap(point, neighbor);
        return;
    }

    let b: f64 = rng.gen();
    
    // Check if either point is in a pattern site
    let mut involved_site: Option<(usize, Point2)> = None;
    for (idx, site) in sites.iter().enumerate() {
        if let Some(site_pos) = site {
            let site_shape = (bmp.dim().0, bmp.dim().1);
            
            // Check if point or neighbor is within the site's area
            let in_site = |p: Point3| -> bool {
                p.2 == 0 && // Only consider points on the first z-layer
                p.0 >= site_pos.0 && p.0 < site_pos.0 + site_shape.0 &&
                p.1 >= site_pos.1 && p.1 < site_pos.1 + site_shape.1
            };
            
            if in_site(point) || in_site(neighbor) {
                involved_site = Some((idx, *site_pos));
                break;
            }
        }
    }
    
    // If no site is involved, use the exchange probability
    if involved_site.is_none() {
        if b < p_exchange {
            state.swap(point, neighbor);
        }
        return;
    }
    
    // A pattern at site s is involved
    let (site_idx, site_pos) = involved_site.unwrap();
    
    // Calculate current goodness
    let current_goodness = goodness(&site_pos, state, bmp);
    
    // Temporarily do the exchange to check if it improves the pattern
    state.swap(point, neighbor);
    let new_goodness = goodness(&site_pos, state, bmp);
    
    if new_goodness > current_goodness {
        // Exchange improves the pattern, keep it
        // Check if the pattern is now good enough (e.g., > 0.98)
        if new_goodness > 0.98 {
            // Delete site from the list
            sites[site_idx] = None;
            
            // Find a new site
            let mut best_site: Option<Point2> = None;
            let mut best_goodness: f32 = 0.0;
            
            for _ in 0..N_TRIALS { // Try 100 random positions XXX maybe deduplicate this logic that's also in init_state??? Would that follow instructions?
                let point = rand_point(2);
                let s: Point2 = (point[0], point[1]);
                
                let g = if collides(s, sites, (bmp.dim().0, bmp.dim().1)) {
                    0.0
                } else {
                    goodness(&s, state, bmp)
                };
                
                if g > best_goodness {
                    best_goodness = g;
                    best_site = Some(s);
                }
            }
            
            sites[site_idx] = best_site;
        }
    } else {
        // Exchange doesn't improve the pattern, revert it
        state.swap(point, neighbor);
    }
}

fn init_state() -> (Array3<bool>, Vec<Option<Point2>>, Array2<bool>) {
    // Initialize the bitmap
    let bmp: Array2<bool> = array![
        [false, false, false, false, false, false, false, true, true, true, true, true, true, true, false, false, false, false, false, false, false],
        [false, false, false, false, false, true, true, false, false, false, true, false, false, false, true, true, false, false, false, false, false],
        [false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false],
        [false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false],
        [false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false],
        [false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false],
        [false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false],
        [true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true],
        [true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true],
        [true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true],
        [true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true],
        [true, false, false, false, false, false, false, false, false, true, true, true, false, false, false, false, false, false, false, false, true],
        [true, false, false, false, false, false, false, false, false, true, true, true, false, false, false, false, false, false, false, false, true],
        [true, false, false, false, false, false, false, false, true, false, true, false, true, false, false, false, false, false, false, false, true],
        [false, true, false, false, false, false, false, true, false, false, true, false, false, true, false, false, false, false, false, true, false],
        [false, true, false, false, false, false, false, true, false, false, true, false, false, true, false, false, false, false, false, true, false],
        [false, false, true, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, true, false, false],
        [false, false, false, true, false, true, false, false, false, false, true, false, false, false, false, true, false, true, false, false, false],
        [false, false, false, false, true, true, false, false, false, false, true, false, false, false, false, true, true, false, false, false, false],
        [false, false, false, false, true, true, true, false, false, false, true, false, false, false, true, true, true, false, false, false, false],
        [false, false, false, false, false, false, false, true, true, true, true, true, true, true, false, false, false, false, false, false, false],
    ];
    assert!(
        bmp.dim().0 == bmp.dim().1,
        "number of arrays == length of first array"
    );

    // Count true values in bmp
    let tot: usize = bmp.iter().map(|&b| b as usize).sum();
    println!("total number of true bits in bmp: {}", tot);
    let r: f64 = tot as f64 / (bmp.dim().0 * bmp.dim().1) as f64;

    // Initialize state with random bits
    let mut state = Array3::<bool>::from_elem((L, L, L), false);
    let mut i = 0;
    println!("L.pow(3) * r: {}", L.pow(3) as f64 * r);
    // flip some bits
    while (i as f64) < L.pow(3) as f64 * r {
        let points = rand_point(3);
        let (x, y, z) = (points[0], points[1], points[2]);

        if !state[[x, y, z]] {
            state[[x, y, z]] = true;
            i += 1;
        }
    }
    println!("attempted to flip bits {} times.", i);

    // Initialize sites
    let mut sites: Vec<Option<Point2>> = Vec::new();

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

fn run_sim(mut state: Array3<bool>, mut sites: Vec<Option<Point2>>, bmp: Array2<bool>) {
    // Initialize visualization with cursive
    let siv = cursive::default();
    let mut siv = siv.into_runner();
    
    // Create Canvas without capturing state
    let canvas = Canvas::new(state.slice(s![.., .., 0]).to_owned())
        .with_draw(|grid: &Array2<bool>, printer: &Printer| {
            for (pos, value) in grid.indexed_iter() {
                let ch = if *value { "█" } else { "." };
                printer.print((pos.1, pos.0), ch);
            }
        })
        .with_required_size(|_, _| Vec2::new(L, L));

    siv.add_layer(NamedView::new("canvas", canvas));
    siv.add_global_callback('q', |s| s.quit());
    siv.refresh();

    let mut step = 0;
    while siv.is_running() {
        step += 1;
        let point = rand_point(3);
        let (x1, y1, z1) = (point[0], point[1], point[2]);
        
        if let Some((x2, y2, z2)) = random_direct_neighbor((x1, y1, z1), &state) {
            let p_anyway = 0.01;
            let p_exchange = 0.8;
            try_exchange(
                p_anyway,
                p_exchange,
                &mut sites,
                (x1, y1, z1),
                (x2, y2, z2),
                &mut state,
                &bmp,
            );
            
            if step % 100_000 == 0 {
                // Update canvas with new state
                if let Some(mut canvas) = siv.find_name::<Canvas<Array2<bool>>>("canvas") {
                    *canvas.state_mut() = state.slice(s![.., .., 0]).to_owned();
                }
                siv.step();
                siv.refresh();
            }
        }
    }
}

#[cfg(test)]
mod tests;

fn main() {
    // Initialize state, sites, and bitmap
    let (state, sites, bmp) = init_state();
    run_sim(state, sites, bmp);
}
