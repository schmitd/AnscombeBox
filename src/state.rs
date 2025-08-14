use ndarray::*;
use rand::prelude::*;

pub type Point2 = (usize, usize);
pub type Point3 = (usize, usize, usize);

pub const GRID_SIZE: usize = 60;
pub const N_SITES: usize = 6;
pub const N_TRIALS: usize = 100;
pub const DISPLAY_UPDATE_INTERVAL: usize = 11_000; // if this is too low, cursive becomes the bottleneck
pub const PATTERN_COMPLETION_THRESHOLD: f32 = 0.98;
pub const PROBABILITY_ANYWAY: f64 = 0.01;
pub const PROBABILITY_EXCHANGE: f64 = 0.8;

pub struct GameState {
    pub state: Array3<bool>,
    pub sites: Vec<Option<Point2>>,
    pub bmp: Array2<bool>,
    pub player: Point2,
    step_count: usize,
}

impl GameState {
    pub fn new(state: Array3<bool>, sites: Vec<Option<Point2>>, bmp: Array2<bool>) -> Self {
        Self {
            state,
            sites,
            bmp,
            player: (0, 0),
            step_count: 0,
        }
    }

    // Method to handle player movement with bounds checking
    pub fn move_player(&mut self, direction: char) {
        match direction {
            'w' => {
                if self.player.0 > 0 {
                    self.player.0 -= 1;
                }
            }

            'a' => {
                if self.player.1 > 0 {
                    self.player.1 -= 1;
                }
            }
            's' => {
                if self.player.0 < GRID_SIZE - 1 {
                    self.player.0 += 1;
                }
            }

            'd' => {
                if self.player.1 < GRID_SIZE - 1 {
                    self.player.1 += 1;
                }
            }
            _ => {}
        }
    }

    // Method to force a new site at player position
    pub fn force_site(&mut self) {
        self.sites.push(Some(self.player));
    }

    // Perform one simulation step
    pub fn step(&mut self) {
        self.step_count += 1;

        let point = self.generate_random_point_3d();
        if let Some(neighbor) = self.find_random_neighbor(point) {
            self.try_exchange(point, neighbor);
        }
    }

    // Check if it's time to update the display
    pub fn should_update_display(&self) -> bool {
        self.step_count % DISPLAY_UPDATE_INTERVAL == 0
    }

    // Get the current 2D slice for rendering
    pub fn get_render_slice(&self) -> Array2<bool> {
        self.state.slice(s![.., .., 0]).to_owned()
    }

    // Get render data with player position highlighted
    pub fn get_render_data_with_player(&self) -> (Array2<bool>, Point2) {
        (self.get_render_slice(), self.player)
    }

    // Get current step count
    #[allow(dead_code)]
    pub fn get_step_count(&self) -> usize {
        self.step_count
    }

    // Generate a random 3D point within the grid
    fn generate_random_point_3d(&self) -> Point3 {
        let mut rng = rand::thread_rng();
        (
            rng.gen_range(0..GRID_SIZE),
            rng.gen_range(0..GRID_SIZE),
            rng.gen_range(0..GRID_SIZE),
        )
    }

    // Find a random valid neighbor of a point
    fn find_random_neighbor(&self, point: Point3) -> Option<Point3> {
        let (x, y, z) = point;
        let mut rng = rand::thread_rng();

        let directions = [
            (1, 0, 0),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ];

        // Filter valid directions (within bounds)
        let valid_neighbors: Vec<_> = directions
            .iter()
            .filter_map(|&(dx, dy, dz)| {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                let nz = z as isize + dz;

                if nx >= 0
                    && nx < GRID_SIZE as isize
                    && ny >= 0
                    && ny < GRID_SIZE as isize
                    && nz >= 0
                    && nz < GRID_SIZE as isize
                {
                    Some((nx as usize, ny as usize, nz as usize))
                } else {
                    None
                }
            })
            .collect();

        if valid_neighbors.is_empty() {
            None
        } else {
            let idx = rng.gen_range(0..valid_neighbors.len());
            Some(valid_neighbors[idx])
        }
    }

    // Main exchange logic
    fn try_exchange(&mut self, point: Point3, neighbor: Point3) {
        let mut rng = rand::thread_rng();

        // Random exchange with small probability
        if rng.gen::<f64>() < PROBABILITY_ANYWAY {
            self.state.swap(point, neighbor);
            return;
        }

        // Check if either point is in a pattern site
        if let Some((site_idx, site_pos)) = self.find_involved_site(point, neighbor) {
            self.handle_pattern_exchange(point, neighbor, site_idx, site_pos);
        } else {
            // No pattern involved, use normal exchange probability
            if rng.gen::<f64>() < PROBABILITY_EXCHANGE {
                self.state.swap(point, neighbor);
            }
        }
    }

    // Find if either point is within a pattern site
    fn find_involved_site(&self, point: Point3, neighbor: Point3) -> Option<(usize, Point2)> {
        for (idx, site) in self.sites.iter().enumerate() {
            if let Some(site_pos) = site {
                let site_shape = (self.bmp.dim().0, self.bmp.dim().1);

                if self.is_point_in_site(point, *site_pos, site_shape)
                    || self.is_point_in_site(neighbor, *site_pos, site_shape)
                {
                    return Some((idx, *site_pos));
                }
            }
        }
        None
    }

    // Check if a point is within a site's area (only considers z=0 layer)
    fn is_point_in_site(
        &self,
        point: Point3,
        site_pos: Point2,
        site_shape: (usize, usize),
    ) -> bool {
        point.2 == 0 && // Only consider points on the first z-layer
        point.0 >= site_pos.0 && point.0 < site_pos.0 + site_shape.0 &&
        point.1 >= site_pos.1 && point.1 < site_pos.1 + site_shape.1
    }

    // Handle exchange when a pattern site is involved
    fn handle_pattern_exchange(
        &mut self,
        point: Point3,
        neighbor: Point3,
        site_idx: usize,
        site_pos: Point2,
    ) {
        let current_goodness = self.calculate_pattern_goodness(&site_pos);

        // Temporarily perform the exchange
        self.state.swap(point, neighbor);
        let new_goodness = self.calculate_pattern_goodness(&site_pos);

        if new_goodness > current_goodness {
            // Exchange improves the pattern, keep it
            if new_goodness > PATTERN_COMPLETION_THRESHOLD {
                // Pattern is complete, find a new site
                self.sites[site_idx] = None;
                self.sites[site_idx] = self.find_new_site();
            }
        } else {
            // Exchange doesn't improve the pattern, revert it
            self.state.swap(point, neighbor);
        }
    }

    // Calculate how well a pattern matches at a given position
    fn calculate_pattern_goodness(&self, position: &Point2) -> f32 {
        // Check if the bitmap would fit within the slice at the given coordinates
        if position.0 + self.bmp.dim().0 > GRID_SIZE || position.1 + self.bmp.dim().1 > GRID_SIZE {
            return 0.0;
        }

        // Extract the 2D slice from the 3D array (z=0 layer)
        let window = self.state.slice(s![
            position.0..position.0 + self.bmp.dim().0,
            position.1..position.1 + self.bmp.dim().1,
            0
        ]);

        // Count matching bits (XOR gives true for mismatches)
        let mut matches = 0;
        let total_bits = self.bmp.dim().0 * self.bmp.dim().1;

        for ((i, j), &bmp_value) in self.bmp.indexed_iter() {
            if window[[i, j]] == bmp_value {
                matches += 1;
            }
        }

        matches as f32 / total_bits as f32
    }

    // Find a new site location
    fn find_new_site(&self) -> Option<Point2> {
        let mut rng = rand::thread_rng();
        let mut best_site: Option<Point2> = None;
        let mut best_goodness = 0.0;

        for _ in 0..N_TRIALS {
            let position = (rng.gen_range(0..GRID_SIZE), rng.gen_range(0..GRID_SIZE));

            if !self.site_collides_with_existing(position) {
                let goodness = self.calculate_pattern_goodness(&position);
                if goodness > best_goodness {
                    best_goodness = goodness;
                    best_site = Some(position);
                }
            }
        }

        best_site
    }

    // Check if a potential site collides with existing sites
    fn site_collides_with_existing(&self, position: Point2) -> bool {
        let site_shape = (self.bmp.dim().0, self.bmp.dim().1);

        for existing_site in self.sites.iter().flatten() {
            // Check if the two rectangles overlap
            let pos_end = (position.0 + site_shape.0, position.1 + site_shape.1);
            let existing_end = (
                existing_site.0 + site_shape.0,
                existing_site.1 + site_shape.1,
            );

            if position.0 < existing_end.0
                && pos_end.0 > existing_site.0
                && position.1 < existing_end.1
                && pos_end.1 > existing_site.1
            {
                return true;
            }
        }
        false
    }
}
