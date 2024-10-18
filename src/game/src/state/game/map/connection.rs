#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Connection {
    pub p1: Point,
    pub p2: Point,
}

impl Connection {
    pub fn intersects(&self, other: &Connection) -> bool {
        fn orientation(p: &Point, q: &Point, r: &Point) -> i32 {
            let val = (q.col as i32 - p.col as i32) * (r.row as i32 - q.row as i32)
                - (q.row as i32 - p.row as i32) * (r.col as i32 - q.col as i32);
            if val == 0 {
                0 // Collinear
            } else if val > 0 {
                1 // Clockwise
            } else {
                2 // Counterclockwise
            }
        }

        fn is_point_between(p: &Point, q: &Point, r: &Point) -> bool {
            q.col < p.col.max(r.col)
                && q.col > p.col.min(r.col)
                && q.row < p.row.max(r.row)
                && q.row > p.row.min(r.row)
        }

        let o1 = orientation(&self.p1, &self.p2, &other.p1);
        let o2 = orientation(&self.p1, &self.p2, &other.p2);
        let o3 = orientation(&other.p1, &other.p2, &self.p1);
        let o4 = orientation(&other.p1, &other.p2, &self.p2);

        // Check for general perpendicular intersection
        if o1 != o2 && o3 != o4 {
            // Ensure the intersection occurs in the middle of the segments, not at their vertices
            if is_point_between(&self.p1, &other.p1, &self.p2)
                && is_point_between(&other.p1, &self.p1, &other.p2)
            {
                return true;
            }
        }

        // No perpendicular intersection in the middle of the lines
        false
    }
}
