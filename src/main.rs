use rand::prelude::*;
use std::{collections::LinkedList, vec};
use ndarray::*;

type Point2 = (usize, usize);
// type Point3 = (usize, usize, usize);
const L : usize = 15;
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
    let window = side.slice(s![cords.0..cords.0 + bmp.dim().0, cords.1..cords.1 + bmp.dim().1, 0]);
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

fn rand_point(n : usize) ->  Vec<usize> {
    let mut ret : Vec<usize> = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..n {
        let r: f64 = rng.gen();
        let r: usize = (r * L as f64).floor() as usize;
        ret.push(r);
    }
    ret
}

fn collides(s: Point2, sites: &Vec<Option<Point2>>, site_shape: (usize, usize)) -> bool {
    unimplemented!("Window collision detection isn't implemented");
}

fn init_state() -> Array3<bool> {
    let mut rng = rand::thread_rng();
    Array3::from_shape_fn((L, L, L), |_| rng.gen_bool(0.5))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_goodness() {
        let bmp: Array2<bool> = array![
            [false, false, true,  false, false],
            [false, false, true,  false, false],
            [true,  true,  true,  true,  true],
            [false, false, true,  false, false],
            [false, false, true,  false, false]
        ];

        let side: Array3<bool> = Array3::from_elem((5, 5, 1), true);
        let cords: Point2 = (0, 0);

        let goodness = goodness(&cords, &side, &bmp);
        assert_eq!(goodness, 0.36, "Goodness should be 0.36 for the given bitmap and side");
    }

    #[test]
    fn test_collides_true() {
        let mut sites: Vec<Option<Point2>> = Vec::new();
        sites.push(Some((2, 1)));
        let site_shape: (usize, usize) = (3, 3);
        let s: Point2 = (0, 0);
        assert_eq!(collides(s, &sites, site_shape), true);
    }

    #[test]
    fn test_collides_false() {
        let mut sites: Vec<Option<Point2>> = Vec::new();
        sites.push(Some((3, 1)));
        let site_shape: (usize, usize) = (3, 3);
        let s: Point2 = (0, 0);
        assert_eq!(collides(s, &sites, site_shape), false);
    }
}

fn main() {
    let bmp: Array2<bool> = array![
        [false, false, true,  false, false],
        [false, false, true,  false, false],
        [true,  true,  true,  true,  true],
        [false, false, true,  false, false],
        [false, false, true,  false, false]
    ];
    assert!(bmp.dim().0 == bmp.dim().1, "number of arrays == length of first array");
    
    // Count true values in bmp
    let tot: usize = bmp.iter().filter(|&&b| b).count();
    let r = tot / (bmp.dim().0 * bmp.dim().1);
    
    //let rng = rand::thread_rng(); Unused?
    let mut state = Array3::<bool>::from_elem((L, L, L), false);
    let mut i = 0;

    // flip some bits
    while i < L.pow(3) * r {
        let points = rand_point(3);
        let (x, y, z) = (points[0], points[1], points[2]);
        
        if !state[[x, y, z]] {
           state[[x, y, z]] = true;
           i += 1;
        }
    }
    
    // What's good?
    const N_SITES : usize = 6;
    const N_TRIALS : usize = 100;
    let mut sites: Vec<Option<Point2>> = Vec::new();
    
    for _ in 0..N_SITES {
        let (mut best_site, mut best_goodness) : (Option<Point2>, f32) = (None, 0.0);
        
        for _ in 0..N_TRIALS {
            let points = rand_point(2);
            let s: Point2 = (points[0], points[1]);
            
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
    println!("{:#?}", sites);
}
