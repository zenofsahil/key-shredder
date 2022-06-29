use std::convert::AsRef;
use std::ops::Range;
use eframe::egui::TextBuffer;

#[derive(Default)]
pub struct ReservedString(pub String, pub usize);

impl AsRef<str> for ReservedString {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl TextBuffer for ReservedString {
    fn is_mutable(&self) -> bool {
        true
    }

    fn insert_text(&mut self, text: &str, char_index: usize) -> usize {
        if char_index > self.1 {
            0
        } else {
            // Get the byte index from the character index
            let byte_idx = self.byte_index_from_char_index(char_index);

            // Then insert the string
            self.0.insert_str(byte_idx, text);

            text.chars().count()
        }
    }

    fn delete_char_range(&mut self, char_range: Range<usize>) {
        assert!(char_range.start <= char_range.end);

        // Get both byte indices
        let byte_start = self.byte_index_from_char_index(char_range.start);
        let byte_end = self.byte_index_from_char_index(char_range.end);

        // Then drain all characters within this range
        self.0.drain(byte_start..byte_end);
    }

    fn clear(&mut self) {
        self.0.clear();
    }
}
