pub mod connection;
pub mod encounter;

use rand::seq::SliceRandom;

use std::usize;

use rand::Rng;

use self::connection::{Connection, Point};
use self::encounter::{Encounter, EncounterType};

#[derive(Debug, Clone)]
pub struct GameMap {
    rows: usize,
    cols: usize,
    values: Vec<Encounter>,
    row_indices: Vec<usize>,
    col_indices: Vec<usize>,
    connections: Vec<Connection>,
}

#[derive(Debug)]
pub enum MatrixQueryResult<'a, Encounter> {
    Value(&'a Encounter),
    NotPresent,
    OutOfBounds,
}

impl GameMap {
    fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            values: Vec::new(),
            row_indices: Vec::new(),
            col_indices: Vec::new(),
            connections: Vec::new(),
        }
    }

    fn add_value(&mut self, row: usize, col: usize, value: Encounter) {
        println!("Adding new value ({},{})", row, col);
        self.values.push(value);
        self.row_indices.push(row);
        self.col_indices.push(col);
    }

    fn add_connection(&mut self, p1: Point, p2: Point) -> anyhow::Result<()> {
        let new_connection = Connection { p1, p2 };

        for existing_connection in &self.connections {
            if new_connection.intersects(existing_connection) {
                return Err(anyhow::anyhow!(format!(
                    "New connection would intersect with an existing connection (existing: {:?}) (new: {:?})",
                    existing_connection, new_connection
                )));
            }
        }

        self.connections.push(new_connection);
        println!("NEW CONNECTION CREATED: {:?}", new_connection);
        Ok(())
    }

    fn get_value(&self, row: usize, col: usize) -> MatrixQueryResult<Encounter> {
        if row >= self.rows || col >= self.cols {
            return MatrixQueryResult::OutOfBounds;
        }

        for (i, (&r, &c)) in self
            .row_indices
            .iter()
            .zip(self.col_indices.iter())
            .enumerate()
        {
            if r == row && c == col {
                return MatrixQueryResult::Value(&self.values[i]);
            }
        }

        MatrixQueryResult::NotPresent
    }

    pub fn get_neighbors(&self, row: usize, col: usize) -> Vec<MatrixQueryResult<Encounter>> {
        let mut neighbors = Vec::new();

        let directions = [
            (row.wrapping_sub(1), col),
            (row + 1, col),
            (row, col.wrapping_sub(1)),
            (row, col + 1),
        ];

        for (r, c) in directions.iter() {
            neighbors.push(self.get_value(*r, *c));
        }

        neighbors
    }
    pub fn find_all_valid_points(&self, start: Point) -> Vec<Point> {
        println!("Start: {:?}", start);
        // Relative coordinates for B, C, and D with respect to A
        let relative_targets = vec![
            (1, -1), // B
            (1, 0),  // C
            (1, 1),  // D
        ];

        let mut valid_points = Vec::new(); // To store all valid points

        for (row_offset, col_offset) in relative_targets.iter() {
            // Calculate new row and column based on A's coordinates
            let new_row = (start.row as isize + row_offset) as isize;
            let new_col = (start.col as isize + col_offset) as isize;

            // Out-of-bounds check (new_row or new_col cannot be negative or exceed matrix dimensions)
            if new_row < 0
                || new_row >= self.rows as isize
                || new_col < 0
                || new_col >= self.cols as isize
            {
                println!("new row was out of bounds, skipping");
                continue; // Skip to the next target point if this one is out of bounds
            }

            // Convert back to usize since we know the point is in bounds
            let new_row = new_row as usize;
            let new_col = new_col as usize;

            // Get the value at the new position
            match self.get_value(new_row, new_col) {
                MatrixQueryResult::Value(_) => {
                    // If the value exists, add this point to valid_points
                    valid_points.push(Point {
                        row: new_row,
                        col: new_col,
                    });
                }
                MatrixQueryResult::NotPresent => {
                    valid_points.push(Point {
                        row: new_row,
                        col: new_col,
                    });
                }
                MatrixQueryResult::OutOfBounds => continue, // Skip out-of-bounds points
            }
        }

        valid_points // Return all valid points found
    }

    fn set_value(&mut self, row: usize, col: usize, mut value: Option<Encounter>) {
        for i in 0..self.values.len() {
            if self.row_indices[i] == row && self.col_indices[i] == col {
                if let Some(value) = value.take() {
                    self.values[i] = value;
                } else {
                    self.values.remove(i);
                    self.row_indices.remove(i);
                    self.col_indices.remove(i);
                }
            }
        }
        if let Some(value) = value {
            self.add_value(row, col, value)
        }
        print!("\n\n\n");
        self.print();
        println!("Connections {:?}", self.connections);
        print!("\n\n\n");
    }

    fn print(&self) {
        for row in (0..self.rows).rev() {
            for col in (0..self.cols) {
                match self.get_value(row, col) {
                    MatrixQueryResult::Value(e) => print!(" {} ", e.marker),
                    MatrixQueryResult::NotPresent => print!(" * "),
                    MatrixQueryResult::OutOfBounds => print!(" * "),
                }
            }
            println!("\n                ")
        }
    }

    pub fn generate(width: usize, height: usize, paths: usize) -> Self {
        let mut game_map = Self::new(height, width);
        // create paths
        let mut start_positions = vec![];

        // create start positions
        for _ in 0..paths {
            let start = rand::thread_rng().gen_range(0..width);
            start_positions.push(start)
        }

        // ensure that there are at least 2 different starting positions
        if start_positions[0] == start_positions[1] {
            println!("UPDATING START POSITION");
            start_positions[1] = rand::thread_rng().gen_range(start_positions[0]..=width)
        }

        start_positions.sort();
        for (path_index, col) in start_positions.iter().enumerate() {
            // logic
            println!("\n\n-------NEW PATH-------\n\n");

            // store the last encounters point
            let mut p0: Option<Point> = None;

            for row in 0..height {
                // let conn = Self::create_connection(row, *col);
                // let conn = game_map.find_closest_valid_point(row, *col);
                // let current = (row, *col);

                // let conn = Self::inspect_connect(current, conn, &game_map)
                //     .expect("LAST ROW HANDLE THIS!!");

                // USE AN IF LET HERE ON P0
                if let None = p0 {
                    let value = Encounter {
                        encounter_type: EncounterType::Monster,
                        visited: false,
                        starting_room: true,
                        connected: false,
                        marker: (path_index + 1).to_string(),
                    };
                    game_map.set_value(row, *col, Some(value));
                    let p1 = Point { row, col: *col };
                    p0 = Some(p1);
                } else {
                    let mut value = Encounter {
                        encounter_type: EncounterType::Monster,
                        visited: false,
                        starting_room: false,
                        connected: false,
                        marker: (path_index + 1).to_string(),
                    };

                    let mut valid_points = game_map.find_all_valid_points(p0.unwrap());
                    println!("VALID POINTS: {:?}", valid_points);

                    let mut rng = rand::thread_rng();

                    valid_points.shuffle(&mut rng);

                    for p2 in valid_points {
                        match game_map.add_connection(p0.unwrap(), p2) {
                            Ok(_) => match game_map.get_value(p2.row, p2.col) {
                                MatrixQueryResult::Value(_) => {
                                    value.marker = "@".to_string();
                                    game_map.set_value(p2.row, p2.col, Some(value));
                                    p0 = Some(p2);
                                    break;
                                }
                                _ => {
                                    game_map.set_value(p2.row, p2.col, Some(value));
                                    p0 = Some(p2);
                                    break;
                                }
                            },
                            Err(e) => {
                                eprintln!("Failed to add connection: {}, trying agin.", e);
                            }
                        }
                    }
                }
            }
            p0 = None
        }

        // for (i, start) in start_positions.iter().enumerate() {
        //     if i == 0 {
        //         let mut path = Path::new();
        //         let connection = Self::create_connection(*start, 7);
        //         path.push(connection);
        //
        //         for _ in 0..height {
        //             let connection = Self::create_connection(*path.peek().unwrap(), 7);
        //             path.push(connection);
        //         }
        //
        //         println!("---------");
        //         for value in path.iter() {
        //             println!("{}", value);
        //         }
        //     } else {
        //         let mut path = Path::new();
        //         let connection = Self::create_connection(*start, 7);
        //         path.push(connection);
        //
        //         for i in 0..height {
        //             // this will be the index of the next encounter
        //             let connection = Self::create_connection(*path.peek().unwrap(), 7);
        //             let round = rounds[i].clone();
        //
        //             // next Encounter position
        //             let _next = round[connection];
        //             // encounter to the left of the next encounter
        //             let left = round[connection];
        //             // encounter to the right of the next encounter
        //             let right = round[connection];
        //             // current encounter index
        //             let current = path.peek().unwrap();
        //
        //             // need to check the current round left and right as well.
        //             if connection < *current {
        //                 if let Some(encounter) = right {
        //                     if encounter.connected {
        //                         path.push(connection + 1)
        //                     } else {
        //                         path.push(connection)
        //                     }
        //                 }
        //             } else if connection > *current {
        //                 if let Some(encounter) = left {
        //                     if encounter.connected {
        //                         path.push(connection - 1)
        //                     } else {
        //                         path.push(connection)
        //                     }
        //                 }
        //             } else {
        //                 path.push(connection)
        //             }
        //             path.push(connection);
        //         }
        //
        //         println!("---------");
        //         for value in path.iter() {
        //             println!("{}", value);
        //         }
        //     }
        // }

        //
        // let boss = Encounter {
        //     encounter_type: EncounterType::Boss,
        //     connection: None,
        //     visited: false,
        //     starting_room: false,
        //     connected: false,
        // };

        // rounds.push(vec![Some(boss)]);
        game_map
    }

    // pub fn inspect_connect(
    //     current: (usize, usize),
    //     conn: (usize, usize),
    //     game_map: &GameMap,
    // ) -> Option<(usize, usize)> {
    //     println!("CURRENT: {:?}", current);
    //     println!("WANTED: {:?}", conn);
    //     let is_left = conn.1 < current.1;
    //     let is_right = conn.1 > current.1;
    //
    //     let left = if current.1 > 0 {
    //         Some((current.0, current.1 - 1))
    //     } else {
    //         None
    //     };
    //
    //     let right = if current.1 < game_map.cols {
    //         Some((current.0, current.1 + 1))
    //     } else {
    //         None
    //     };
    //
    //     let up = if current.0 + 1 <= game_map.rows {
    //         Some((current.0 + 1, current.1))
    //     } else {
    //         None
    //     };
    //
    //     let left_up = if left.is_some() {
    //         Some((current.0 + 1, current.1 - 1))
    //     } else {
    //         None
    //     };
    //
    //     let right_up = if right.is_some() {
    //         Some((current.0 + 1, current.1 + 1))
    //     } else {
    //         None
    //     };
    //
    //     let left_node = if let Some(left) = left {
    //         game_map.get_value(left.0, left.1)
    //     } else {
    //         None
    //     };
    //
    //     let right_node = if let Some(right) = right {
    //         game_map.get_value(right.0, right.1)
    //     } else {
    //         None
    //     };
    //
    //     let up_node = if let Some(up) = up {
    //         game_map.get_value(up.0, up.1)
    //     } else {
    //         None
    //     };
    //
    //     let left_up_node = if let Some(left_up) = left_up {
    //         game_map.get_value(left_up.0, left_up.1)
    //     } else {
    //         None
    //     };
    //
    //     let right_up_node = if let Some(right_up) = right_up {
    //         game_map.get_value(right_up.0, right_up.1)
    //     } else {
    //         None
    //     };
    //
    //     if is_left && left.is_none() && up.is_some() {
    //         Some((conn.0, current.1))
    //     } else if is_right && right.is_none() && up.is_some() {
    //         Some((conn.0, current.1 - 1))
    //     } else if is_left && left.is_some() && up.is_some() {
    //         if let Some(left_node) = left_node {
    //             let left_node_conns = &left_node.connections;
    //
    //             if let Some(up_node) = up_node {
    //                 let up_node_conns = &up_node.connections;
    //
    //                 if left_node_conns.contains(&conn) && up_node_conns.contains(&conn) {
    //                     return Some((conn.0, current.1));
    //                 } else {
    //                     println!("BING");
    //                     return Some(conn);
    //                 }
    //             } else {
    //                 println!("BANG");
    //                 return Some(conn);
    //             }
    //         } else {
    //             println!("BOOM");
    //             Some(conn)
    //         }
    //     } else if is_right && right.is_some() && up.is_some() {
    //         if let Some(right_node) = right_node {
    //             let right_node_conns = &right_node.connections;
    //
    //             if let Some(up_node) = up_node {
    //                 let up_node_conns = &up_node.connections;
    //
    //                 if right_node_conns.contains(&conn) && up_node_conns.contains(&conn) {
    //                     return Some((conn.0, current.1));
    //                 } else {
    //                     println!("BING2");
    //                     return Some(conn);
    //                 }
    //             } else {
    //                 println!("BANG2");
    //                 return Some(conn);
    //             }
    //         } else {
    //             println!("BOOM2");
    //             Some(conn)
    //         }
    //     } else {
    //         if up.is_some() {
    //             println!("POW");
    //             Some(conn)
    //         } else {
    //             None
    //         }
    //     }
    // }
    pub fn create_connection(row: usize, col: usize) -> (usize, usize) {
        println!("ROW: {}", row);
        println!("COL: {}", col);

        let min_range = 0;
        let max_range = row;

        let min_conn = if col == 0 { col } else { col - 1 };
        println!("MIN: {}", min_conn);

        let mut connection_col = rand::thread_rng().gen_range((min_conn)..=(col + 1));
        println!("CC: {}", connection_col);
        let connection_row = row + 1;

        // if connection_col < min_range {
        //     println!("CC: {}", connection_col);
        //     connection_col = min_range
        // } else if connection_col > max_range {
        //     connection_col = max_range
        // };
        (connection_row, connection_col)
    }

    pub fn randomize_encounter() -> EncounterType {
        match rand::thread_rng().gen::<f64>() {
            x if x < 0.00 => EncounterType::Treasure,
            x if x < 0.05 => EncounterType::Merchant,
            x if x < 0.12 => EncounterType::Rest,
            x if x < 0.16 => EncounterType::Elite,
            x if x < 0.22 => EncounterType::Event,
            x if x < 1.00 => EncounterType::Monster,
            _ => EncounterType::Boss,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Connection, Point};
    use super::{Encounter, EncounterType, GameMap, MatrixQueryResult};
    #[test]
    fn test_map_generation() {
        let height = 6;
        let width = 6;
        let mut map = GameMap::generate(width, height, 5);
        map.print();

        assert_eq!(1, 2)
    }
}
