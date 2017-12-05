pub struct Board {
    values: Vec<Vec<String>>,
}

impl Board {
    // Static methods
    pub fn init(num_rows: i32, num_cols: i32) -> Board {
        return Board { values: vec![vec![" ".to_string(); num_cols as usize]; num_rows as usize] };
    }

    // Instance methods
    pub fn values_at_positions(&self, positions: Vec<Vec<i32>>) -> Vec<String> {
        return positions.iter().map(|point| self.values[point[1 as usize] as usize][point[0 as usize] as usize].clone()).collect();
    }

    pub fn has_piece_at_positions(&self, positions: Vec<Vec<i32>>) -> bool {
        return self.values_at_positions(positions).iter().any(|val| val != " ")
    }

    pub fn num_of_rows(&self) -> i32 {
        return self.values.len() as i32;
    }

    pub fn num_of_cols(&self) -> i32 {
        if let Some(first_row) = self.values.first() {
            return first_row.len() as i32;
        }

        return 0;
    }

    pub fn values(&self) -> &Vec<Vec<String>> {
        return &self.values;
    }
}