use uuid::Uuid;

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
    fn orientation(p1: &Point, p2: &Point) -> i32 {
        let dy = p2.row as i32 - p1.row as i32;
        let dx = p2.col as i32 - p1.col as i32;

        if dy > 0 {
            // q is above p
            if dx < 0 {
                1 // q is above and to the left of p
            } else if dx > 0 {
                2 // q is above and to the right of p
            } else {
                3 // q is above and in the same column as p
            }
        } else {
            0 // q is not above p
        }
    }

    // fn is_point_between(p: &Point, q: &Point, r: &Point) -> bool {
    //     q.col < p.col.max(r.col)
    //         && q.col > p.col.min(r.col)
    //         && q.row < p.row.max(r.row)
    //         && q.row > p.row.min(r.row)
    // }

    pub fn intersects(&self, other: &Connection) -> bool {
        let o1 = Self::orientation(&self.p1, &self.p2);
        let o2 = Self::orientation(&other.p1, &other.p2);
        println!("O1: {}", o1);
        println!("O2: {}", o2);

        // Check for general perpendicular intersection
        if (o1 == 1 && o2 == 2) || (o1 == 2 && o2 == 1) {
            if (self.p1.row == other.p2.row) && (self.p1.col.abs_diff(other.p2.col) == 1) {
                true
            } else {
                false
            }
        } else {
            // No perpendicular intersection in the middle of the lines
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orientation_intersects() {
        let p1 = Point { row: 1, col: 1 };
        let p2 = Point { row: 2, col: 2 };
        let result = Connection::orientation(&p1, &p2);
        assert_eq!(2, result, "point is not below");

        let p1 = Point { row: 1, col: 1 };
        let p2 = Point { row: 2, col: 0 };
        let result = Connection::orientation(&p1, &p2);
        assert_eq!(1, result, "point is not to the left");

        let p1 = Point { row: 1, col: 1 };
        let p2 = Point { row: 2, col: 1 };
        let result = Connection::orientation(&p1, &p2);
        assert_eq!(3, result, "point is above");

        let p1 = Point { row: 1, col: 1 };
        let p2 = Point { row: 2, col: 1 };
        let result = Connection::orientation(&p2, &p1);
        assert_eq!(0, result, "point is not below");
    }
    #[test]
    fn test_connection_intersects() {
        let connection1 = Connection {
            p1: Point { row: 1, col: 1 },
            p2: Point { row: 2, col: 2 },
        };

        let connection2 = Connection {
            p1: Point { row: 1, col: 2 },
            p2: Point { row: 2, col: 1 },
        };

        // Test for a general intersection
        assert!(
            !connection2.intersects(&connection1),
            "Connections do not intersect 1"
        );

        let mut connection1 = Connection {
            p1: Point { row: 2, col: 2 },
            p2: Point { row: 3, col: 1 },
        };

        let mut connection2 = Connection {
            p1: Point { row: 2, col: 1 },
            p2: Point { row: 3, col: 2 },
        };

        // Test for a general intersection
        assert!(
            !connection2.intersects(&connection1),
            "Connections do not intersect 2"
        );

        connection1 = Connection {
            p1: Point { row: 2, col: 2 },
            p2: Point { row: 3, col: 2 },
        };

        connection2 = Connection {
            p1: Point { row: 2, col: 1 },
            p2: Point { row: 3, col: 1 },
        };

        assert!(
            !connection2.intersects(&connection1),
            "Connections do not intersect vertically"
        );

        connection1 = Connection {
            p1: Point { row: 2, col: 2 },
            p2: Point { row: 3, col: 2 },
        };

        connection2 = Connection {
            p1: Point { row: 2, col: 1 },
            p2: Point { row: 3, col: 2 },
        };

        // Test for a general intersection
        assert!(
            !connection2.intersects(&connection1),
            "Connections do not intersect on vertice"
        );
    }
}
