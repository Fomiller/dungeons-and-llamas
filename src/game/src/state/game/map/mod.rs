pub mod color;
pub mod connection;
pub mod encounter;

use ansi_term::Color;
use color::Rgb;
use rand::seq::SliceRandom;
use rand::Rng;
use std::usize;
use uuid::Uuid;

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
pub enum GameMapQueryResult<'a, Encounter> {
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
        self.values.push(value);
        self.row_indices.push(row);
        self.col_indices.push(col);
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
    }

    pub fn get_row_values(&self, row: usize) -> Vec<&Encounter> {
        let mut row_values = Vec::new();

        for (i, &r) in self.row_indices.iter().enumerate() {
            if r == row {
                row_values.push(&self.values[i]);
            }
        }

        row_values
    }

    fn get_value(&self, row: usize, col: usize) -> GameMapQueryResult<Encounter> {
        if row >= self.rows || col >= self.cols {
            return GameMapQueryResult::OutOfBounds;
        }

        for (i, (&r, &c)) in self
            .row_indices
            .iter()
            .zip(self.col_indices.iter())
            .enumerate()
        {
            if r == row && c == col {
                return GameMapQueryResult::Value(&self.values[i]);
            }
        }

        GameMapQueryResult::NotPresent
    }

    pub fn create_connection(row: usize, col: usize) -> (usize, usize) {
        let min_conn = if col == 0 { col } else { col - 1 };

        let conn_col = rand::thread_rng().gen_range((min_conn)..=(col + 1));

        let conn_row = row + 1;

        (conn_row, conn_col)
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
        Ok(())
    }

    pub fn get_neighbors(&self, row: usize, col: usize) -> Vec<GameMapQueryResult<Encounter>> {
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
                continue; // Skip to the next target point if this one is out of bounds
            }

            // Convert back to usize since we know the point is in bounds
            let new_row = new_row as usize;
            let new_col = new_col as usize;

            // Get the value at the new position
            match self.get_value(new_row, new_col) {
                GameMapQueryResult::Value(_) => {
                    // If the value exists, add this point to valid_points
                    valid_points.push(Point {
                        row: new_row,
                        col: new_col,
                    });
                }
                GameMapQueryResult::NotPresent => {
                    valid_points.push(Point {
                        row: new_row,
                        col: new_col,
                    });
                }
                GameMapQueryResult::OutOfBounds => continue, // Skip out-of-bounds points
            }
        }

        valid_points // Return all valid points found
    }

    pub fn generate(width: usize, height: usize, paths: usize) -> anyhow::Result<Self> {
        let result;
        loop {
            let mut game_map = Self::new(height, width);

            // create paths
            let mut start_positions = Vec::new();

            // create start positions
            for _ in 0..paths {
                let start = rand::thread_rng().gen_range(0..width);
                start_positions.push(start)
            }

            // ensure that there are at least 2 different starting positions
            if start_positions.len() > 1 && start_positions[0] == start_positions[1] {
                start_positions[1] = rand::thread_rng().gen_range(start_positions[0]..=width)
            }

            start_positions.sort();
            for (path_index, col) in start_positions.iter().enumerate() {
                // logic
                // store the last encounters point
                let mut p0: Option<Point> = None;
                let min_brightness = Some(0);
                let color = Rgb::generate_random_rgb(min_brightness);

                for row in 0..height {
                    // if first row
                    if let None = p0 {
                        let p1 = Point { row, col: *col };

                        let mut value = Encounter {
                            color,
                            connected: false,
                            encounter_type: EncounterType::None,
                            location: p1,
                            parent: None,
                            starting_room: true,
                            symbol: (path_index + 1).to_string(),
                            visited: false,
                            id: Uuid::new_v4(),
                        };

                        // if a path has already started from this point change the symbol
                        match game_map.get_value(p1.row, p1.col) {
                            GameMapQueryResult::Value(_) => {
                                value.symbol = "@".to_string();
                                game_map.set_value(p1.row, p1.col, Some(value));
                            }
                            _ => {
                                game_map.set_value(p1.row, p1.col, Some(value));
                            }
                        };

                        // set the previous point value
                        p0 = Some(p1);
                    } else {
                        let mut valid_points =
                            game_map.find_all_valid_points(p0.expect("Could not unwrap p0"));

                        let mut rng = rand::thread_rng();

                        valid_points.shuffle(&mut rng);

                        for p1 in valid_points {
                            let mut encounter = Encounter {
                                color,
                                connected: false,
                                encounter_type: EncounterType::None,
                                location: p1,
                                parent: p0,
                                starting_room: false,
                                symbol: (path_index + 1).to_string(),
                                visited: false,
                                id: Uuid::new_v4(),
                            };
                            match game_map.add_connection(p0.expect("Could not unwrap p0"), p1) {
                                Ok(_) => match game_map.get_value(p1.row, p1.col) {
                                    GameMapQueryResult::Value(_) => {
                                        encounter.symbol = "@".to_string();
                                        game_map.set_value(p1.row, p1.col, Some(encounter));
                                        p0 = Some(p1);
                                        break;
                                    }
                                    _ => {
                                        game_map.set_value(p1.row, p1.col, Some(encounter));
                                        p0 = Some(p1);
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
                // reset the starting point for starting a new path
                p0 = None
            }

            // if creating game map fails start loop again,
            // otherwise exit and return result
            match game_map.try_create_map() {
                Ok(_) => {
                    result = game_map;
                    break;
                }
                Err(e) => eprintln!("Invalid map, trying again "),
            }
        }

        Ok(result)
    }

    pub fn try_create_map(&mut self) -> anyhow::Result<()> {
        match self.try_assign_encounters() {
            Ok(_) => return Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn try_assign_encounters_in_row(
        &mut self,
        row: usize,
        encounter: Option<EncounterType>,
    ) -> anyhow::Result<()> {
        for (i, (&r, col)) in self
            .row_indices
            .iter()
            .zip(self.col_indices.iter())
            .enumerate()
        {
            if r == row {
                if let Some(encounter) = encounter {
                    self.values[i].encounter_type = encounter
                } else {
                    let child = &self.values[i];
                    let parent = child.parent.expect("No parent found on Encounter");

                    loop {
                        let encounter_type = self
                            .randomize_encounter(None)
                            .expect("Could not create random encounters");

                        // Elite and Rest Sites can’t be assigned below the 6th Floor.
                        if Self::assignment_rule_1(row, encounter_type) {
                            println!("Assignment rule 1 broken");
                            continue;
                        }

                        // Elite, Merchant and Rest Site cannot be consecutive.
                        // (eg. you cant have 2 Rest Sites connected with a Path)
                        if self.assignment_rule_2(parent, &child) {
                            println!("Assignment rule 2 broken");
                        }

                        // A Room that that has 2 or more Paths going out must
                        // have all destinations be unique. 2 destinations
                        // originating from the same Room cannot share the same Location.
                        if self.assignment_rule_3(parent, &encounter_type) {
                            println!("Assignment rule 3 broken");
                        };

                        self.values[i].encounter_type = encounter_type;
                        break;
                    }
                }
            }
        }

        Ok(())
    }

    // evaluates to true if rule is broken
    pub fn assignment_rule_1(row: usize, encounter_type: EncounterType) -> bool {
        let exclude = vec![
            EncounterType::Elite,
            EncounterType::Rest,
            EncounterType::Merchant,
        ];
        if row < 6 {
            exclude.contains(&encounter_type)
        } else {
            false
        }
    }

    // evaluates to true if rule is broken
    pub fn assignment_rule_2(&self, parent: Point, child: &Encounter) -> bool {
        let exclude = vec![EncounterType::Elite, EncounterType::Rest];

        if let GameMapQueryResult::Value(parent) = self.get_value(parent.row, parent.col) {
            if exclude.contains(&parent.encounter_type) && exclude.contains(&child.encounter_type) {
                parent.encounter_type == child.encounter_type
            } else {
                false // if they are not in the same list it does not matter return true
            }
        } else {
            true
        }

        // if both parent and child are in the exclude list they cannot be equal
    }

    // evaluates to true if rule is broken
    pub fn assignment_rule_3(&self, parent: Point, encounter_type: &EncounterType) -> bool {
        // find all connections that share the same parent
        let children: Vec<&Connection> =
            self.connections.iter().filter(|c| c.p1 == parent).collect();

        // create a list of the encounter types for the connections that share the same parent
        let mut exclude: Vec<EncounterType> = children
            .iter()
            .filter_map(|c| {
                let query = self.get_value(c.p2.row, c.p2.col);
                if let GameMapQueryResult::Value(encounter) = query {
                    Some(encounter.encounter_type)
                } else {
                    None
                }
            })
            .collect();

        exclude.sort();
        exclude.dedup();

        if exclude.contains(encounter_type) {
            true
        } else {
            false
        }
    }

    pub fn try_assign_encounters(&mut self) -> anyhow::Result<()> {
        for row in 0..self.rows {
            match row {
                // start must be a monster
                0 => self.try_assign_encounters_in_row(row, Some(EncounterType::Monster))?,

                // 9th round must be treasure
                r if r == (self.rows / 2) => {
                    self.try_assign_encounters_in_row(row, Some(EncounterType::Treasure))?
                }

                // last round must be a rest
                r if r == (self.rows - 1) => {
                    self.try_assign_encounters_in_row(row, Some(EncounterType::Rest))?
                }

                // all other rounds are random
                _ => self.try_assign_encounters_in_row(row, None)?,
            }
        }
        Ok(())
    }

    pub fn randomize_encounter(
        &self,
        exclude: Option<Vec<EncounterType>>,
    ) -> Option<EncounterType> {
        let mut encounter = None;
        let attempts = exclude.as_ref().map_or(1, |vec| vec.len());

        let percentage = rand::thread_rng().gen::<f64>();

        for _ in 0..attempts {
            let result = match percentage {
                x if x < 0.00 => EncounterType::Treasure,
                x if x < 0.05 => EncounterType::Merchant,
                x if x < 0.12 => EncounterType::Rest,
                x if x < 0.16 => EncounterType::Elite,
                x if x < 0.22 => EncounterType::Event,
                x if x < 1.00 => EncounterType::Monster,
                _ => EncounterType::None,
            };

            if exclude.as_ref().map_or(true, |ex| !ex.contains(&result)) {
                encounter = Some(result);
                break;
            }
        }

        encounter
    }

    fn print(&self) {
        for row in (0..self.rows).rev() {
            for col in 0..self.cols {
                match self.get_value(row, col) {
                    GameMapQueryResult::Value(e) => {
                        let mut color = e.color;

                        let connections: Vec<&Connection> = self
                            .connections
                            .iter()
                            .filter(|c| c.p2 == e.location)
                            .collect();

                        if connections.len() > 1 {
                            for conn in connections.iter() {
                                if let GameMapQueryResult::Value(value) =
                                    self.get_value(conn.p2.row, conn.p2.col)
                                {
                                    color = Rgb::new(255, 0, 0)
                                }
                            }
                        }

                        print!(
                            " {} ",
                            Color::RGB(color.r, color.g, color.b)
                                .paint(e.encounter_type.to_string())
                        );
                    }
                    GameMapQueryResult::NotPresent => print!(" * "),
                    GameMapQueryResult::OutOfBounds => print!(" * "),
                }
            }
            println!("\n                ")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::GameMap;

    #[test]
    fn test_map_generation() -> anyhow::Result<()> {
        let height = 16;
        let width = 8;
        let paths = 3;
        let map = GameMap::generate(width, height, paths)?;

        map.print();

        assert_eq!(1, 2);
        Ok(())
    }
}
