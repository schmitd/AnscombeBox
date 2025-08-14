use crate::state::{Point2, PATTERN_COMPLETION_THRESHOLD};
use ndarray::*;

/// Represents a site where a pattern can be formed
pub struct Site {
    /// Position of the site in the grid
    pub position: Point2,
    /// Custom bitmap for this site (if None, uses default)
    pub custom_bitmap: Option<Array2<bool>>,
    /// Whether this site is active (being used for pattern matching)
    pub is_active: bool,
}

impl Site {
    /// Create a new site with default bitmap
    pub fn new(position: Point2) -> Self {
        Self {
            position,
            custom_bitmap: None,
            is_active: true,
        }
    }

    /// Create a new site with a custom bitmap
    pub fn with_custom_bitmap(position: Point2, bitmap: Array2<bool>) -> Self {
        Self {
            position,
            custom_bitmap: Some(bitmap),
            is_active: true,
        }
    }

    /// Get the bitmap for this site (custom or default)
    pub fn get_bitmap<'a>(&'a self, default_bitmap: &'a Array2<bool>) -> &'a Array2<bool> {
        match &self.custom_bitmap {
            Some(custom) => custom,
            None => default_bitmap,
        }
    }

    /// Check if the pattern at this site is complete (takes goodness as parameter)
    pub fn is_complete(&self, goodness: f32) -> bool {
        goodness > PATTERN_COMPLETION_THRESHOLD
    }

    /// Deactivate this site (pattern is complete)
    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    /// Reactivate this site (for finding new patterns)
    pub fn reactivate(&mut self) {
        self.is_active = true;
    }

    /// Move the site to a new position
    pub fn move_to(&mut self, new_position: Point2) {
        self.position = new_position;
    }

    /// Get the dimensions of this site's bitmap
    pub fn get_dimensions<'a>(&'a self, default_bitmap: &'a Array2<bool>) -> (usize, usize) {
        let bitmap = self.get_bitmap(default_bitmap);
        (bitmap.dim().0, bitmap.dim().1)
    }
}

/// Collection of sites with helper methods
pub struct SiteManager {
    sites: Vec<Site>,
}

impl SiteManager {
    /// Create a new site manager
    pub fn new() -> Self {
        Self { sites: Vec::new() }
    }

    /// Add a new site with default bitmap
    pub fn add_site(&mut self, position: Point2) {
        self.sites.push(Site::new(position));
    }

    /// Add a new site with custom bitmap
    pub fn add_custom_site(&mut self, position: Point2, bitmap: Array2<bool>) {
        self.sites.push(Site::with_custom_bitmap(position, bitmap));
    }

    /// Get all active sites
    pub fn get_active_sites(&self) -> Vec<&Site> {
        self.sites.iter().filter(|site| site.is_active).collect()
    }

    /// Get all active sites as mutable references
    pub fn get_active_sites_mut(&mut self) -> Vec<&mut Site> {
        self.sites
            .iter_mut()
            .filter(|site| site.is_active)
            .collect()
    }

    /// Get all sites (active and inactive)
    pub fn get_all_sites(&self) -> &[Site] {
        &self.sites
    }

    /// Get all sites as mutable references
    pub fn get_all_sites_mut(&mut self) -> &mut [Site] {
        &mut self.sites
    }

    /// Find a site at a specific position
    pub fn find_site_at(&self, position: Point2) -> Option<&Site> {
        self.sites.iter().find(|site| site.position == position)
    }

    /// Find a site at a specific position (mutable)
    pub fn find_site_at_mut(&mut self, position: Point2) -> Option<&mut Site> {
        self.sites.iter_mut().find(|site| site.position == position)
    }

    /// Remove a site at a specific position
    pub fn remove_site_at(&mut self, position: Point2) -> Option<Site> {
        if let Some(index) = self.sites.iter().position(|site| site.position == position) {
            Some(self.sites.remove(index))
        } else {
            None
        }
    }

    /// Check if a position collides with any existing site
    pub fn collides_with_sites(
        &self,
        position: Point2,
        _site_shape: (usize, usize),
        default_bitmap: &Array2<bool>,
    ) -> bool {
        for site in &self.sites {
            if !site.is_active {
                continue;
            }

            let site_shape = site.get_dimensions(default_bitmap);

            // Check if the two rectangles overlap
            let pos_end = (position.0 + site_shape.0, position.1 + site_shape.1);
            let site_end = (
                site.position.0 + site_shape.0,
                site.position.1 + site_shape.1,
            );

            if position.0 < site_end.0
                && pos_end.0 > site.position.0
                && position.1 < site_end.1
                && pos_end.1 > site.position.1
            {
                return true;
            }
        }
        false
    }

    /// Get the number of active sites
    pub fn active_count(&self) -> usize {
        self.sites.iter().filter(|site| site.is_active).count()
    }

    /// Get the total number of sites
    pub fn total_count(&self) -> usize {
        self.sites.len()
    }

    /// Clear all sites
    pub fn clear(&mut self) {
        self.sites.clear();
    }
}
