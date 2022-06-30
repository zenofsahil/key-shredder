use std::convert::AsRef;
use std::ops::Range;
use eframe::egui::TextBuffer;

/// The ReservedString will act as the buffer for the Key Shredder app text
/// widget. The buffer used is usually a String but any type that implements
/// the `eframe::egui::TextBuffer` trait can be used. 
///
/// The first value is the actual String type that will act as the underlying 
/// buffer and the second value is the length of the hint/prompt that the user
/// is supposed to follow along to. This value acts as the limit to how many
/// characters can be added to the underlying buffer as we don't want the user
/// to be able to continue typing and adding to the text widget buffer once the 
/// prompt has been completed. 
#[derive(Default, Debug)]
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
