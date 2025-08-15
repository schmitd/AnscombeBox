use ndarray::*;

type Point2 = (usize, usize);

pub struct GameState {
    // Encapsulate all mutable game components
    pub state: Array3<bool>,
    pub sites: Vec<Option<Point2>>,
    pub bmp: Array2<bool>,
    pub player: Point2,
}

impl GameState {
    // Method to handle player movement
    pub fn move_player(&mut self, direction: char) {
        let grid_size = self.state.dim().0; // Assuming square grid
        
        match direction {
            'w' => {
                if self.player.1 < grid_size - 1 {
                    self.player.1 += 1;
                }
            },
            'a' => {
                if self.player.0 > 0 {
                    self.player.0 -= 1;
                }
            },
            's' => {
                if self.player.1 > 0 {
                    self.player.1 -= 1;
                }
            },
            'd' => {
                if self.player.0 < grid_size - 1 {
                    self.player.0 += 1;
                }
            },
            _ => {} // Ignore other keys
        }
    }

    // Method to force a new site at player position
    pub fn force_site(&mut self) {
        self.sites.push(Some(self.player));
    }
}
