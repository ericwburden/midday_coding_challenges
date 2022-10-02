/// Represents a point in 2D space
#[derive(Debug, Eq, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
}

/// Represents a point at the top corner of a building, either the top-left
/// corner or the top-right corner
#[derive(Debug, Eq, PartialEq)]
pub enum TopCorner {
    Left(Point),
    Right(Point),
}

/// Needed for sorting `TopCorner`s
impl std::cmp::PartialOrd for TopCorner {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Needed for sorting `TopCorner`s. In general, points are considered from left
/// to right. There are three edge cases that are addressed in comments below, but
/// they consist of having two corners at the same x value.
impl std::cmp::Ord for TopCorner {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use TopCorner::*;
        use std::cmp::Ordering::*;
        match (self, other) {
            // For a Left and Right corner at the same `x` position, we always
            // consider the Left corner first to avoid "closing" the one
            // building before "opening" the next.
            (Left(a), Right(b)) => if a.x == b.x { Less } else { a.x.cmp(&b.x) },
            (Right(a), Left(b)) => if a.x == b.x { Greater } else { a.x.cmp(&b.x) },
            // For two Left corners at the same `x` position, consider the corner
            // with the largest `y` value first.
            (Left(a), Left(b)) => if a.x == b.x { b.y.cmp(&a.y) } else { a.x.cmp(&b.x) },
            // For two Right corners at the same `x` position, consider the corner
            // with the smallest `y` value first.
            (Right(a), Right(b)) => if a.x == b.x { a.y.cmp(&b.y) } else { a.x.cmp(&b.x) },
        }
    }
}
 
/// Used to keep track of the heights of "open" buildings. An "open" building is
/// one whose Left corner has been considered but their Right corner has not yet
/// been examined.
#[derive(Debug)]
pub struct HeightMap(std::collections::BTreeMap<i32, u32>);

impl Default for HeightMap {
    fn default() -> Self {
        let mut inner = std::collections::BTreeMap::new();
        inner.insert(0, 1);
        HeightMap(inner)
    }
}

impl HeightMap {
    /// Add a height to the `HeightMap`. There may be more than one building of the
    /// same height being examined at one time, so the keys of the inner BTreeMap
    /// indicate "open" building heights and the values represent the number of
    /// buildings with that height currently "open".
    pub fn push(&mut self, height: i32) {
        self.0.entry(height).and_modify(|h| *h += 1).or_insert(1);
    }

    /// Remove a height from the `HeightMap`. If, after removing the height indicated,
    /// there are no more "open" buildings at that height, remove the indicated height
    /// from the HeightMap. Otherwise, just decrement the number of "open" bulidings
    /// at that height.
    pub fn remove(&mut self, height: i32) {
        match self.0.get_mut(&height) {
            Some(v) => if *v <= 1 { self.0.remove(&height); } else { *v -= 1; },
            None => panic!("Height {} not in HeightMap", height),
        }
    }

    /// Return the current maximum height of an "open" building.
    pub fn max(&self) -> Option<i32>{
        self.0.keys().max().map(|x| *x)
    }
}

/// Utility function to convert an entry for a 'building' in the input into points for
/// its two top corners.
pub fn building_to_corners(building: &Vec<i32>) -> [TopCorner; 2] {
    let (left, right, height) = (building[0], building[1], building[2]);
    [TopCorner::Left(Point{ x: left, y: height}),
     TopCorner::Right(Point{ x: right, y: height })]
}

pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // Create an output vector of the maximum possible size.
    let mut skyline = Vec::with_capacity(buildings.len() * 2);

    // Create a `HeightMap` with the ground initialized (height of zero)
    let mut heights = HeightMap::default();

    // Convert the input representing buildings into a vector of corners. Sorting
    // this vector puts the corner points in the order they should be considered.
    let mut corners = buildings.iter().flat_map(building_to_corners).collect::<Vec<_>>();
    corners.sort_unstable();

    // For each corner...
    for corner in corners {
        match corner {
            // ...if the corner is a Left corner, check to see if the corner's height
            // is higher than the highest building currently being examined. If so,
            // add that corner to the skyline. 
            TopCorner::Left(l) => {
                if l.y > heights.max().unwrap() {
                    skyline.push(vec![l.x, l.y]);
                }

                // Whether a new point is added to the skyline or not, the current 
                // building is "open", meaning its height should be considered by
                // other corners until the right corner of this building is passed.
                heights.push(l.y);
            },

            // ...if the corner is a right corner, "close" this current building by
            // removing its height from the HeightMap. If this building is taller than
            // any other building currently "open", add a point to the skyline with 
            // this corner's `x` value and the `y` value of the next tallest building
            // (or the ground if there are no "open" buildings).
            TopCorner::Right(r) => {
                heights.remove(r.y);
                let new_max = heights.max().unwrap();
                if r.y > new_max {
                    skyline.push(vec![r.x, new_max]);
                }
            },
        }
    }

    skyline
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_one() {
        let buildings = vec![
            vec![ 2, 9,10],
            vec![ 3, 7,15],
            vec![ 5,12,12],
            vec![15,20,10],
            vec![19,24, 8]
        ];
        let result = vec![
            vec![ 2,10],
            vec![ 3,15],
            vec![ 7,12],
            vec![12, 0],
            vec![15,10],
            vec![20, 8],
            vec![24, 0]
        ];
        assert_eq!(get_skyline(buildings), result);
    }

    #[test]
    fn test_cast_two() {
        let buildings = vec![vec![0,2,3],vec![2,5,3]];
        let result    = vec![vec![0,3],vec![5,0]];
        assert_eq!(get_skyline(buildings), result);
    }

    #[test]
    fn test_case_three() {
        let buildings = vec![vec![2,9,10], vec![9,12,15]];
        let result = vec![vec![2,10], vec![9,15], vec![12,0]];
        assert_eq!(get_skyline(buildings), result);
    }
}
