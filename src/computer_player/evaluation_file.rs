use crate::computer_player::Evaluation;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::Path;

impl Evaluation {
    /// Save to a file.  
    /// ファイルへ保存します。  
    pub fn save(&self, file_name: &str) {
        let mut text = String::new();
        // Open the file.
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(Path::new(file_name))
            // TODO error handling.
            .unwrap();

        for state in &self.features_1_to_7 {
            for win_draw in state.iter() {
                for value in win_draw.iter() {
                    text.push_str(&format!("{},", value));
                }
            }
            text.push_str("\r\n");
        }
        for state in &self.features_8_to_13 {
            for win_draw in state.iter() {
                for value in win_draw.iter() {
                    text.push_str(&format!("{},", value));
                }
            }
            text.push_str("\r\n");
        }
        for state in &self.features_14_19_20_25 {
            for win_draw in state.iter() {
                for value in win_draw.iter() {
                    text.push_str(&format!("{},", value));
                }
            }
            text.push_str("\r\n");
        }
        for state in &self.features_15_18_21_24 {
            for win_draw in state.iter() {
                for value in win_draw.iter() {
                    text.push_str(&format!("{},", value));
                }
            }
            text.push_str("\r\n");
        }
        for state in &self.features_16_17_22_23 {
            for win_draw in state.iter() {
                for value in win_draw.iter() {
                    text.push_str(&format!("{},", value));
                }
            }
            text.push_str("\r\n");
        }

        // Write.
        let mut file_buf = BufWriter::new(file);
        // write_all method required to use 'use std::io::Write;'.
        if let Err(why) = file_buf.write_all(text.as_bytes()) {
            panic!("couldn't write log. : {}", why);
        }
    }

    /// Load from a file.  
    /// TODO ファイルから読み込みます。  
    pub fn load(&self) {}
}
