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
                2 // Counter Clockwise
            }
        }

        // fn on_segment(p: &Point, q: &Point, r: &Point) -> bool {
        //     q.col <= p.col.max(r.col)
        //         && q.col >= p.col.min(r.col)
        //         && q.row <= p.row.max(r.row)
        //         && q.row >= p.row.min(r.row)
        // }

        let o1 = orientation(&self.p1, &self.p2, &other.p1);
        let o2 = orientation(&self.p1, &self.p2, &other.p2);
        let o3 = orientation(&other.p1, &other.p2, &self.p1);
        let o4 = orientation(&other.p1, &other.p2, &self.p2);

        println!("\n01: {:?}", o1);
        println!("02: {:?}", o2);
        println!("03: {:?}", o3);
        println!("04: {:?}\n", o4);

        if o1 != o2 && o3 != o4 {
            return true;
        }
        // if o1 == 0 && on_segment(&self.p1, &other.p1, &self.p2) {
        //     return true;
        // }
        // if o2 == 0 && on_segment(&self.p1, &other.p2, &self.p2) {
        //     return true;
        // }
        // if o3 == 0 && on_segment(&other.p1, &self.p1, &other.p2) {
        //     return true;
        // }
        // if o4 == 0 && on_segment(&other.p1, &self.p2, &other.p2) {
        //     return true;
        // }

        false

        //
        // if o1 == 0 && on_segment(&self.p1, &other.p1, &self.p2) {
        //     return true;
        // } else if o2 == 0 && on_segment(&self.p1, &other.p2, &self.p2) {
        //     return true;
        // } else if o3 == 0 && on_segment(&other.p1, &self.p1, &other.p2) {
        //     return true;
        // } else if o4 == 0 && on_segment(&other.p1, &self.p2, &other.p2) {
        //     return true;
        // } else {
        //     false
        // }
    }
}
