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

fn goodness(cords : &Point2, side : &Array2<bool>, bmp : &Array2<bool>) -> f32 {
    if  cords.0 + bmp.dim().0 < side.len() && cords.1 + bmp.dim().1 < side[0].len() {
        return 0.0;
    }
    let window : Vec<&[bool]> = side.into_iter()
        .map(|v| { &v[cords.0..cords.0 + bmp.len()]}).collect();
    let bits = xor(&window, bmp);
    let mut tot = 0;
    for i in bits.into_iter().flatten() {
        if !i {tot += 1};
    }
    tot as f32 / bmp.len().pow(2) as f32
}

fn xor(slice : &[&[bool]], bmp : &Array2<bool>) -> Vec<Vec<bool>> {
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

fn collides(s : Point2, sites : LinkedList<Option<Point2>>) -> bool {
    unimplemented!();
}

fn init_state() -> Array3<bool> {
    unimplemented!();
}

fn main() {
    let BMP : Array2<bool> = array![[false, false, true,  false, false],
                                    [false, false, true,  false, false],
                                    [true,  true,  true,  true,  true],
                                    [false, false, true,  false, false],
                                    [false, false, true,  false, false]];
    assert!(BMP.dim().0 == BMP.dim().1, "number of arrays == length of first array");
    
    let mut tot : usize = 0;
    for b in BMP.iter() {
        tot += *b as usize;
    }
    let r = tot / BMP.len().pow(2); 
    let mut rng = rand::thread_rng();
    let mut state = Array3::<bool>::uninit((L, L, L));
    let mut i = 0;

    // flip some bits
    while i < L.pow(3) * r {
        /*
        let x: f64 = rng.gen();
        let x: usize = (x * L as f64).floor() as usize;
        let y: f64 = rng.gen();
        let y: usize = (y * L as f64).floor() as usize;
        let z: f64 = rng.gen();
        let z: usize = (z * L as f64).floor() as usize;
        */
        let (x, y, z) = rand_point(3).into_iter().collect_tuple().unwrap();
        if state.get((x, y, z)).unwrap() == false {
           state[x][y][z] = true;
           i += 1;
        }
    }
    // What's good?
    let side = state.as_slice();
    const n_sites : usize = 6;
    const n_trials : usize = 100;
    let mut sites: LinkedList<Option<Point2>> = LinkedList::new();
    for i in 0..n_sites {
        let (mut best_site, mut best_goodness) : (Option<Point2>, f64) = (None, 0.0);
        let mut g : f64 = 0.0;
        for j in 0..n_trials {
            let s: Point2 = rand_point(3).into_iter().collect_tuple().unwrap();
            if collides(s, sites) { 
                g = 0.0;
            } else { 
                g = goodness(&s, side, BMP.as_slice().unwrap());
            }
            if g > best_goodness {
                best_goodness = g;
                best_site = Some(s);
            }
            sites.push_back(best_site);
        }
    }
}
