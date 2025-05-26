use rand::prelude::*;
use itertools::{izip, Itertools};
use std::collections::LinkedList;
use ndarray::*;

type Point2 = (usize, usize);
type Point3 = (usize, usize, usize);
const L : usize = 15;
/*
const BMP : [[bool; 5]; 5] =
           [[false, false, true,  false, false],
            [false, false, true,  false, false],
            [true,  true,  true,  true,  true],
            [false, false, true,  false, false],
            [false, false, true,  false, false]];*/

fn goodness(cords: &Point2, side: &ArrayView2<bool>, bmp: &ArrayView2<bool>) -> f32 {
    let (bmp_rows, bmp_cols) = bmp.dim();
    let (side_rows, side_cols) = side.dim();
    
    // Check if the window fits within side dimensions
    if cords.0 + bmp_rows > side_rows || cords.1 + bmp_cols > side_cols {
        return 0.0;
    }
    
    // Extract the window from side using ndarray slicing
    let window = side.slice(s![cords.0..cords.0 + bmp_rows, cords.1..cords.1 + bmp_cols]);
    
    // Compute XOR between window and bmp
    let bits = window.iter().zip(bmp.iter()).map(|(&a, &b)| a != b);
    
    // Count true values (mismatches) and compute goodness
    let tot = bits.filter(|&x| x).count();
    tot as f32 / (bmp_rows * bmp_cols) as f32
}

/*
fn xor(slice: &[&[bool]], bmp: &Array2<bool>) -> Vec<Vec<bool>> {
    let mut ret = vec![vec![false; bmp.len()]; bmp.len()];
    for (ia , ib , iret)  in izip!(slice, bmp, ret.iter_mut()) { // TODO:iter w ndarr
        for (ja, jb, jret) in izip!(ia.iter(), ib, iret.iter_mut()) {
           *jret = match &mut (ja, jb) {
               (true, false) => true,
               (false, true) => true,
               _ => false,
           };
        }
    }
    ret
}
*/

fn rand_point(n: usize) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    (0..n)
        .map(|_| (rng.gen::<f64>() * L as f64).floor() as usize)
        .collect()
}

fn collides(s: Point2, sites: LinkedList<Option<Point2>>) -> bool {
    // TODO: Implement collision detection
    // Consider checking if point s overlaps with any site in sites
    unimplemented!();
}

fn init_state() -> Array3<bool> {
    // TODO: Implement state initialization
    // Consider returning Array3::zeros((L, L, L)) or similar
    unimplemented!();
}

fn main() {
    let bmp: Array2<bool> = array![
        [false, false, true, false, false],
        [false, false, true, false, false],
        [true, true, true, true, true],
        [false, false, true, false, false],
        [false, false, true, false, false]
    ];
    assert!(bmp.dim().0 == bmp.dim().1, "number of arrays == length of first array");
    
    let tot: usize = bmp.iter().map(|&b| b as usize).sum();
    let r = tot / (bmp.dim().0 * bmp.dim().1);
    
    // Initialize state with zeros
    let mut state = Array3::<bool>::from_elem((L, L, L), false);
    let mut i = 0;

    // Flip some bits
    while i < L.pow(3) * r {
        let coords = rand_point(3);
        let (x, y, z) = (coords[0], coords[1], coords[2]);
        if !state[[x, y, z]] {
            state[[x, y, z]] = true;
            i += 1;
        }
    }
    
    // Get 2D view for goodness calculation (assuming we want XY plane at z=0)
    let side = state.slice(s![.., .., 0]);
    
    const N_SITES: usize = 6;
    const N_TRIALS: usize = 100;
    let mut sites: LinkedList<Option<Point2>> = LinkedList::new();
    
    for _ in 0..N_SITES {
        let (mut best_site, mut best_goodness): (Option<Point2>, f32) = (None, 0.0);
        for _ in 0..N_TRIALS {
            let coords = rand_point(2);
            let s: Point2 = (coords[0], coords[1]);
            let g = if collides(s, sites.clone()) {
                0.0
            } else {
                goodness(&s, &side, &bmp.view())
            };
            if g > best_goodness {
                best_goodness = g;
                best_site = Some(s);
            }
        }
        sites.push_back(best_site);
    }
}
