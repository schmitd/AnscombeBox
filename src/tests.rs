use super::*;
use site::SiteManager;


#[test]
fn test_xor() {
    let slice: Array2<bool> = array![[true, false, true], [false, true, false], [true, false, true]];
    let bmp: Array2<bool> = array![[true, false, true], [false, false, false], [true, true, false]];
    let result: Array2<bool> = array![[false, false, false], [false, true, false], [false, true, true]];
    assert_eq!(xor(&slice, &bmp), result);
}
#[test]
fn test_goodness() {
    let bmp: Array2<bool> = array![
        [false, false, true, false, false],
        [false, false, true, false, false],
        [true, true, true, true, true],
        [false, false, true, false, false],
        [false, false, true, false, false]
    ];
    let side: Array3<bool> = Array3::from_elem((5, 5, 1), true);
    let cords: Point2 = (0, 0);
    let goodness = goodness(&cords, &side, &bmp);
    assert_eq!(
        goodness, 0.36,
        "Goodness should be 0.36 for the given bitmap and side"
    );
}
#[test]
fn test_collides_true() {
    let mut sites = SiteManager::new();
    sites.add_site((2, 1));
    let site_shape: (usize, usize) = (3, 3);
    let s: Point2 = (0, 0);
    let bmp: Array2<bool> = Array2::from_elem(site_shape, false);
    assert_eq!(sites.collides_with_sites(s, site_shape, &bmp), true);
}
#[test]
fn test_collides_false() {
    let mut sites = SiteManager::new();
    sites.add_site((3, 1));
    let site_shape: (usize, usize) = (3, 3);
    let s: Point2 = (0, 0);
    let bmp: Array2<bool> = Array2::from_elem(site_shape, false);
    assert_eq!(sites.collides_with_sites(s, site_shape, &bmp), false);
}