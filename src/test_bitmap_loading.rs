#[cfg(test)]
mod tests {
    use crate::bitmap_loader::{load_bitmap_from_bmp, load_bitmaps_from_directory};

    #[test]
    fn test_load_main_bitmap() {
        let result = load_bitmap_from_bmp("main_bitmap.bmp");
        assert!(result.is_ok(), "Should be able to load main_bitmap.bmp");

        let bitmap = result.unwrap();
        assert_eq!(bitmap.dim(), (20, 20), "Main bitmap should be 20x20");
    }

    #[test]
    fn test_load_player_bitmap() {
        let result = load_bitmap_from_bmp("player_bitmap.bmp");
        assert!(result.is_ok(), "Should be able to load player_bitmap.bmp");

        let bitmap = result.unwrap();
        assert_eq!(bitmap.dim(), (3, 3), "Player bitmap should be 3x3");
    }

    #[test]
    fn test_load_bitmaps_from_directory() {
        let result = load_bitmaps_from_directory("bitmaps");
        assert!(
            result.is_ok(),
            "Should be able to load bitmaps from directory"
        );

        let bitmaps = result.unwrap();
        assert!(
            !bitmaps.is_empty(),
            "Should have loaded some bitmaps from directory"
        );

        // Check that we have the expected bitmaps
        let expected_count = 3; // square, diagonal, hollow_square
        assert_eq!(
            bitmaps.len(),
            expected_count,
            "Should have loaded {} bitmaps",
            expected_count
        );
    }

    #[test]
    fn test_bitmap_content() {
        let bitmap = load_bitmap_from_bmp("player_bitmap.bmp").unwrap();

        // The player bitmap should be a cross pattern
        assert!(bitmap[[1, 1]], "Center should be true");
        assert!(bitmap[[0, 1]], "Left center should be true");
        assert!(bitmap[[2, 1]], "Right center should be true");
        assert!(bitmap[[1, 0]], "Top center should be true");
        assert!(bitmap[[1, 2]], "Bottom center should be true");

        // Corners should be false
        assert!(!bitmap[[0, 0]], "Top-left corner should be false");
        assert!(!bitmap[[2, 0]], "Top-right corner should be false");
        assert!(!bitmap[[0, 2]], "Bottom-left corner should be false");
        assert!(!bitmap[[2, 2]], "Bottom-right corner should be false");
    }
}
