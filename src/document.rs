use crate::Position;
use crate::Row;
use std::fs;

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    pub file_name: Option<String>,
}

impl Document {
    pub fn open(file_name: &str) -> Result<Self, std::io::Error> {
        let file_contents = fs::read_to_string(file_name)?;
        let mut rows = Vec::new();

        for line in file_contents.lines() {
            rows.push(Row::from(line));
        }

        Ok(Self {
            rows,
            file_name: Some(file_name.to_string()),
        })
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    fn insert_newline(&mut self, at: &Position) {
        if at.y > self.len() {
            return;
        }

        // Cursor at the end of the line, just insert a new empty row
        if at.y == self.len() {
            self.rows.push(Row::default());
        }

        // Otherwise split the line at the cursor position and insert a new row containing the right half
        let new_row = self.rows.get_mut(at.y).unwrap().split(at.x);
        self.rows.insert(at.y + 1, new_row);
    }

    pub fn insert(&mut self, at: &Position, c: char) {
        if (c == '\n') {
            self.insert_newline(at);
            return;
        }

        if at.y == self.len() {
            let mut new_row = Row::default();
            new_row.insert(0, c);
            self.rows.push(new_row);
        } else if at.y < self.len() {
            let row = self.rows.get_mut(at.y).unwrap();
            row.insert(at.x, c);
        }
    }

    pub fn delete(&mut self, at: &Position) {
        let len = self.len();

        // Last line, nothing to delete
        if at.y >= len {
            return;
        }

        // If we're at the end of a line and there's a line after, append them together
        if at.x == self.rows.get_mut(at.y).unwrap().len() && at.y < len - 1 {
            let next_row = self.rows.remove(at.y + 1);
            let row = self.rows.get_mut(at.y).unwrap();
            row.append(&next_row);
        } else {
            // Otherwise, just delete the single character
            let row = self.rows.get_mut(at.y).unwrap();
            row.delete(at.x);
        }
    }
}
