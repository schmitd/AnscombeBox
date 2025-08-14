struct GameState {
    // Encapsulate all mutable game components
    state: Array3<bool>,
    sites: Vec<Option<Point2>>,
    bmp: Array2<bool>,
    player: Point2,
}

impl GameState {
    // Method to handle player movement
    fn move_player(&mut self, direction: char) {
        match direction {
            'w' => {
                self.player.1 += 1;
            },
            'a' => {
                self.player.0 -= 1;
            },
            's' => {
                self.player.1 -= 1;
            },
            'd' => {
                self.player.0 += 1;
            },
            _ => {} // Ignore other keys
        }
    }
}
