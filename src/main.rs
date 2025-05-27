use cursive::*;
use ndarray::*;
use rand::prelude::*;
use std::vec;

type Point2 = (usize, usize);
type Point3 = (usize, usize, usize);
const L: usize = 20;
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
    unimplemented!()
}

fn random_direct_neighbor(point: Point3, state: &Array3<bool>) -> Option<Point3> {
    unimplemented!()
}

fn try_exchange(
    p_anyway: f64,
    p_exchange: f64,
    sites: &Vec<Option<Point2>>,
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

    // TODO: Complete the pattern generating external cause
    /* Pseudocode:
     * Let b be randomly selected from the interval [0, 1]
     * If both cells are in the interior of the box then
     *     If b < probexchange then do the exchange
     *     Else don’t do the exchange
     *     return
     * If none of 〈x1, y1, z1〉 and 〈x2, y2, z2〉 fall in a patch s in Sites
     *     If b < probexchange then do the exchange32
     *     Else don’t do the exchange
     *     return
     * # Continuing, we know a pattern at site s is involved
     * If the pattern at s will be improved by the exchange then
     *     Do the exchange
     *     If the goodness of the pattern at s is now good enough
     *     Delete s from the list Sites
     *     Find a new site s′ to grow a pattern and add s′ to Sites
     * Else don’t do the exchange
     */
}

fn init_state() -> Array3<bool> {
    let mut rng = rand::thread_rng();
    Array3::from_shape_fn((L, L, L), |_| rng.gen_bool(0.5))
}

#[cfg(test)]
mod tests;


fn main() {
    let bmp: Array2<bool> = array![
        [false, false, true, false, false],
        [false, false, true, false, false],
        [true, true, true, true, true],
        [false, false, true, false, false],
        [false, false, true, false, false]
    ];
    assert!(
        bmp.dim().0 == bmp.dim().1,
        "number of arrays == length of first array"
    );

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
    const N_SITES: usize = 6;
    const N_TRIALS: usize = 100;
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
    println!("{:#?}", sites);

    // TODO: initizlize visualization of face with cursive

    // main simulation loop
    let mut step = 0;
    loop {
        step += 1;
        let point = rand_point(3);
        let (x1, y1, z1) = (point[0], point[1], point[2]);
        let (x2, y2, z2) = random_direct_neighbor((x1, y1, z1), &state).unwrap();
        let p_anyway = 0.01;
        let p_exchange = 0.7;
        try_exchange(
            p_anyway,
            p_exchange,
            &sites,
            (x1, y1, z1),
            (x2, y2, z2),
            &mut state,
            &bmp,
        );
        // TODO: update visualization of face with cursive every 10,000 steps
    }
}
