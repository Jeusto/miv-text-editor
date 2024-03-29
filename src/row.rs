use crate::SearchDirection;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Default)]
pub struct Row {
    string: String,
    len: usize,
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        Self {
            string: String::from(slice),
            len: slice.graphemes(true).count(),
        }
    }
}

impl Row {
    pub fn get_display_graphemes(&self, start: usize, end: usize) -> String {
        let mut result = String::new();

        for (index, grapheme) in self.string[..].graphemes(true).enumerate() {
            if index >= start && index < end {
                result.push_str(grapheme);
            }
        }
        result
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.string.as_bytes()
    }

    pub fn is_empty(&self) -> bool {
        self.string.is_empty()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn insert(&mut self, at: usize, c: char) {
        // End of line, just append
        if at >= self.len {
            self.string.push(c);
            self.len += 1;
        } else {
            // Not end of line, divide and append in the correct position in the middle
            let mut result: String = String::new();
            let mut length = 0;
            for (index, grapheme) in self.string[..].graphemes(true).enumerate() {
                length += 1;

                if index == at {
                    length += 1;
                    result.push(c);
                }

                result.push_str(grapheme);
            }

            self.len = length;
            self.string = result;
        }
    }

    pub fn delete(&mut self, at: usize) {
        // End of line, do nothing
        if at > self.len {
            return;
        }
        let mut result: String = String::new();
        let mut length = 0;
        for (index, grapheme) in self.string[..].graphemes(true).enumerate() {
            if index != at {
                length += 1;
                result.push_str(grapheme);
            }
        }
        self.len = length;
        self.string = result;
    }

    pub fn append(&mut self, new: &Self) {
        self.string = format!("{}{}", self.string, new.string);
        self.len += new.len;
    }

    pub fn split(&mut self, at: usize) -> Self {
        let mut row: String = String::new();
        let mut length = 0;
        let mut splitted_row: String = String::new();
        let mut splitted_length = 0;

        for (index, grapheme) in self.string[..].graphemes(true).enumerate() {
            if index < at {
                length += 1;
                row.push_str(grapheme);
            } else {
                splitted_length += 1;
                splitted_row.push_str(grapheme);
            }
        }

        self.string = row;
        self.len = length;

        Self {
            string: splitted_row,
            len: splitted_length,
        }
    }

    pub fn find(&self, query: &str, at: usize, direction: SearchDirection) -> Option<usize> {
        if at > self.len || query.is_empty() {
            return None;
        }

        // If forward search, search from current pos to end of line
        // Else search from start of line to current pos
        let start = if direction == SearchDirection::Forward {
            at
        } else {
            0
        };
        let end = if direction == SearchDirection::Forward {
            self.len
        } else {
            at
        };

        let substring: String = self.string[..]
            .graphemes(true)
            .skip(start)
            .take(end - start)
            .collect();

        let matching_byte_index = if direction == SearchDirection::Forward {
            substring.find(query)
        } else {
            substring.rfind(query)
        };

        if let Some(matching_byte_index) = matching_byte_index {
            for (grapheme_index, (byte_index, _)) in
                substring[..].grapheme_indices(true).enumerate()
            {
                if matching_byte_index == byte_index {
                    return Some(start + grapheme_index);
                }
            }
        }
        None
    }

    fn highlight_match(&mut self, word: Option<&str>) {
        // if let Some(word) = word {
        //     if word.is_empty() {
        //         return;
        //     }
        //     let mut index = 0;
        //     while let Some(search_match) = self.find(word, index, SearchDirection::Forward) {
        //         if let Some(next_index) = search_match.checked_add(word[..].graphemes(true).count())
        //         {
        //             #[allow(clippy::indexing_slicing)]
        //             for i in index.saturating_add(search_match)..next_index {
        //                 self.highlighting[i] = highlighting::Type::Match;
        //             }
        //             index = next_index;
        //         } else {
        //             break;
        //         }
        //     }
        // }
    }
}
