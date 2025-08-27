use image::{DynamicImage, GenericImageView, Pixel};
use ndarray::{array, Array2};
use std::fs;
use std::path::Path;

/// Load a bitmap from a BMP file and convert it to a boolean array
/// where true represents non-transparent/non-white pixels
pub fn load_bitmap_from_bmp<P: AsRef<Path>>(
    path: P,
) -> Result<Array2<bool>, Box<dyn std::error::Error>> {
    let img = image::open(path)?;
    bitmap_from_image(&img)
}

/// Convert an image to a boolean array
/// Pixels are considered "true" if they are not white (RGB > 240) or transparent
pub fn bitmap_from_image(img: &DynamicImage) -> Result<Array2<bool>, Box<dyn std::error::Error>> {
    let (width, height) = img.dimensions();

    // Create a boolean array with the same dimensions as the image
    let mut bitmap = Array2::from_elem((height as usize, width as usize), false);

    for (x, y, pixel) in img.pixels() {
        let rgba = pixel.to_rgba();
        let (r, g, b, a) = (rgba[0], rgba[1], rgba[2], rgba[3]);

        // Consider pixel as "true" if it's not white and not transparent
        let is_white = r > 240 && g > 240 && b > 240;
        let is_transparent = a < 128;

        if !is_white && !is_transparent {
            bitmap[[y as usize, x as usize]] = true;
        }
    }

    Ok(bitmap)
}

/// Load multiple bitmaps from a directory
pub fn load_bitmaps_from_directory<P: AsRef<Path>>(
    dir_path: P,
) -> Result<Vec<Array2<bool>>, Box<dyn std::error::Error>> {
    let mut bitmaps = Vec::new();

    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

        if let Some(extension) = path.extension() {
            if extension == "bmp" || extension == "png" || extension == "jpg" || extension == "jpeg"
            {
                match load_bitmap_from_bmp(&path) {
                    Ok(bitmap) => bitmaps.push(bitmap),
                    Err(e) => eprintln!("Failed to load bitmap from {:?}: {}", path, e),
                }
            }
        }
    }

    Ok(bitmaps)
}

/// Create a simple test bitmap for debugging
pub fn create_test_bitmap() -> Array2<bool> {
    array![
        [false, true, false],
        [true, true, true],
        [false, true, false],
    ]
}

/// Save a boolean array as a BMP file for debugging/visualization
pub fn save_bitmap_as_bmp<P: AsRef<Path>>(
    bitmap: &Array2<bool>,
    path: P,
) -> Result<(), Box<dyn std::error::Error>> {
    let (height, width) = bitmap.dim();
    let mut img = image::RgbaImage::new(width as u32, height as u32);

    for ((y, x), &value) in bitmap.indexed_iter() {
        let pixel = if value {
            image::Rgba([0, 0, 0, 255]) // Black for true
        } else {
            image::Rgba([255, 255, 255, 255]) // White for false
        };
        img.put_pixel(x.try_into().unwrap(), y.try_into().unwrap(), pixel);
    }

    img.save(path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_test_bitmap() {
        let bitmap = create_test_bitmap();
        assert_eq!(bitmap.dim(), (3, 3));
        assert!(bitmap[[1, 1]]); // Center should be true
        assert!(!bitmap[[0, 0]]); // Corner should be false
    }

    #[test]
    fn test_bitmap_from_image() {
        // Create a simple test image
        let mut img = image::RgbaImage::new(3, 3);
        img.put_pixel(1, 1, image::Rgba([0, 0, 0, 255])); // Black pixel in center

        let bitmap = bitmap_from_image(&DynamicImage::ImageRgba8(img)).unwrap();
        assert_eq!(bitmap.dim(), (3, 3));
        assert!(bitmap[[1, 1]]); // Center should be true
        assert!(!bitmap[[0, 0]]); // Corner should be false
    }
}
