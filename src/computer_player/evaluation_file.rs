use crate::computer_player::Evaluation;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::Path;

impl Evaluation {
    /// Save to a file.  
    /// TODO ファイルへ保存します。  
    pub fn save(&self, file_name: &str) {
        let mut text = String::new();
        // Open the file.
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(Path::new(file_name))
            // TODO error handling.
            .unwrap();

        for feat in &self.features_1_to_7 {
            for win_draw in feat.iter() {
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
    /// TODO ファイルへ保存します。  
    pub fn load(&self) {}
}
