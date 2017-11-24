#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd)]
pub struct Location(pub i32, pub i32);

impl Location {
    pub fn distance(&self, other: &Location) -> usize {
        (((self.0 - other.0).pow(2) + (self.1 - other.1).pow(2)) as f64).sqrt() as usize
    }

    pub fn neighbours(self, impassable : Vec<(Location, usize)>) -> Vec<(Location, usize)> {
        let mut nearby = vec![Location(self.0 + 1, self.1), Location(self.0 - 1, self.1),
                              Location(self.0, self.1 + 1), Location(self.0, self.1 - 1),
                              Location(self.0 + 1, self.1 + 1), Location(self.0 - 1, self.1 - 1),
                              Location(self.0 + 1, self.1 - 1), Location(self.0 - 1, self.1 + 1)];
        nearby.retain(|potential_location| {
            let mut keep = true;
            for location in impassable.iter() {
                if *potential_location == location.0 {
                    keep = false;
                }
            }
            keep
        });
        nearby.into_iter().map(|p| (p, 1)).collect()
    }
}
