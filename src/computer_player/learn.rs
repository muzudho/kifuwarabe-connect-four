use crate::computer_player::FEATURE_V_H_B_S_LEN;
use crate::FILE_LEN;
use crate::{
    computer_player::{Bestmove, Learning, Search},
    log::LogExt,
    Engine, ResultChannel, SearchInfo, EVALUATION_FILE_NAME,
};
use casual_logger::Log;

impl Default for Learning {
    fn default() -> Self {
        Learning {}
    }
}
impl Learning {
    pub fn learn(&mut self, engine: &mut Engine) {
        engine.enter("go");

        engine.evaluation.save(EVALUATION_FILE_NAME);
    }

    /// uh...  
    /// うーん……。  
    pub fn uh(&mut self, engine: &mut Engine) {
        let mut search = Search::default();
        search.start_pieces_num = engine.pos.pieces_num;

        let mut search_info = SearchInfo::default();
        let files_way = [
            {
                let mut bestmove = Bestmove::default();
                search.node_exit(
                    &mut engine.pos,
                    &engine.evaluation,
                    &ResultChannel::Win,
                    'a',
                    &mut search_info,
                    &mut bestmove,
                );
                bestmove
            },
            {
                let mut bestmove = Bestmove::default();
                search.node_exit(
                    &mut engine.pos,
                    &engine.evaluation,
                    &ResultChannel::Win,
                    'b',
                    &mut search_info,
                    &mut bestmove,
                );
                bestmove
            },
            {
                let mut bestmove = Bestmove::default();
                search.node_exit(
                    &mut engine.pos,
                    &engine.evaluation,
                    &ResultChannel::Win,
                    'c',
                    &mut search_info,
                    &mut bestmove,
                );
                bestmove
            },
            {
                let mut bestmove = Bestmove::default();
                search.node_exit(
                    &mut engine.pos,
                    &engine.evaluation,
                    &ResultChannel::Win,
                    'd',
                    &mut search_info,
                    &mut bestmove,
                );
                bestmove
            },
            {
                let mut bestmove = Bestmove::default();
                search.node_exit(
                    &mut engine.pos,
                    &engine.evaluation,
                    &ResultChannel::Win,
                    'e',
                    &mut search_info,
                    &mut bestmove,
                );
                bestmove
            },
            {
                let mut bestmove = Bestmove::default();
                search.node_exit(
                    &mut engine.pos,
                    &engine.evaluation,
                    &ResultChannel::Win,
                    'f',
                    &mut search_info,
                    &mut bestmove,
                );
                bestmove
            },
            {
                let mut bestmove = Bestmove::default();
                search.node_exit(
                    &mut engine.pos,
                    &engine.evaluation,
                    &ResultChannel::Win,
                    'g',
                    &mut search_info,
                    &mut bestmove,
                );
                bestmove
            },
        ];

        let mut tensor = engine
            .evaluation
            .ways_weight(&engine.pos, &ResultChannel::Win);
        let old_tensor = tensor.clone();
        let give_values = [
            Learning::give(&mut tensor, 0),
            Learning::give(&mut tensor, 1),
            Learning::give(&mut tensor, 2),
            Learning::give(&mut tensor, 3),
            Learning::give(&mut tensor, 4),
            Learning::give(&mut tensor, 5),
            Learning::give(&mut tensor, 6),
        ];
        let take_values = [0, 0, 0, 0, 0, 0, 0];

        let mut text = String::new();
        text.push_str(&format!(
            "\
File Vert Hori Baro Sini Total Best File   Result Learn Give Take
---- ---- ---- ---- ---- -----      ------ ------       ---- ----
"
        ));
        let file = 0;
        text.push_str(&format!(
            "   a {0: >4} {1: >4} {2: >4} {3: >4} {4: >5}      {5: <6} {6: <6}       {7: >4} {8: >4}
",
            old_tensor[file][0],
            old_tensor[file][1],
            old_tensor[file][2],
            old_tensor[file][3],
            old_tensor[file][0] + old_tensor[file][1] + old_tensor[file][2] + old_tensor[file][3],
            if let Some(file) = files_way[file].file {
                file.to_string()
            } else {
                "resign".to_string()
            },
            &format!("{:?}", files_way[file].result),
            give_values[file],
            take_values[file]
        ));
        let file = 1;
        text.push_str(&format!(
            "   b {0: >4} {1: >4} {2: >4} {3: >4} {4: >5}      {5: <6} {6: <6}       {7: >4} {8: >4}
",
            old_tensor[file][0],
            old_tensor[file][1],
            old_tensor[file][2],
            old_tensor[file][3],
            old_tensor[file][0] + old_tensor[file][1] + old_tensor[file][2] + old_tensor[file][3],
            if let Some(file) = files_way[file].file {
                file.to_string()
            } else {
                "resign".to_string()
            },
            &format!("{:?}", files_way[file].result),
            give_values[file],
            take_values[file]
        ));
        let file = 2;
        text.push_str(&format!(
            "   c {0: >4} {1: >4} {2: >4} {3: >4} {4: >5}      {5: <6} {6: <6}       {7: >4} {8: >4}
",
            old_tensor[file][0],
            old_tensor[file][1],
            old_tensor[file][2],
            old_tensor[file][3],
            old_tensor[file][0] + old_tensor[file][1] + old_tensor[file][2] + old_tensor[file][3],
            if let Some(file) = files_way[file].file {
                file.to_string()
            } else {
                "resign".to_string()
            },
            &format!("{:?}", files_way[file].result),
            give_values[file],
            take_values[file]
        ));
        let file = 3;
        text.push_str(&format!(
            "   d {0: >4} {1: >4} {2: >4} {3: >4} {4: >5}      {5: <6} {6: <6}       {7: >4} {8: >4}
",
            old_tensor[file][0],
            old_tensor[file][1],
            old_tensor[file][2],
            old_tensor[file][3],
            old_tensor[file][0] + old_tensor[file][1] + old_tensor[file][2] + old_tensor[file][3],
            if let Some(file) = files_way[file].file {
                file.to_string()
            } else {
                "resign".to_string()
            },
            &format!("{:?}", files_way[file].result),
            give_values[file],
            take_values[file]
        ));
        let file = 4;
        text.push_str(&format!(
            "   e {0: >4} {1: >4} {2: >4} {3: >4} {4: >5}      {5: <6} {6: <6}       {7: >4} {8: >4}
",
            old_tensor[file][0],
            old_tensor[file][1],
            old_tensor[file][2],
            old_tensor[file][3],
            old_tensor[file][0] + old_tensor[file][1] + old_tensor[file][2] + old_tensor[file][3],
            if let Some(file) = files_way[file].file {
                file.to_string()
            } else {
                "resign".to_string()
            },
            &format!("{:?}", files_way[file].result),
            give_values[file],
            take_values[file]
        ));
        let file = 5;
        text.push_str(&format!(
            "   f {0: >4} {1: >4} {2: >4} {3: >4} {4: >5}      {5: <6} {6: <6}       {7: >4} {8: >4}
",
            old_tensor[file][0],
            old_tensor[file][1],
            old_tensor[file][2],
            old_tensor[file][3],
            old_tensor[file][0] + old_tensor[file][1] + old_tensor[file][2] + old_tensor[file][3],
            if let Some(file) = files_way[file].file {
                file.to_string()
            } else {
                "resign".to_string()
            },
            &format!("{:?}", files_way[file].result),
            give_values[file],
            take_values[file]
        ));
        let file = 6;
        text.push_str(&format!(
            "   g {0: >4} {1: >4} {2: >4} {3: >4} {4: >5}      {5: <6} {6: <6}       {7: >4} {8: >4}
",
            old_tensor[file][0],
            old_tensor[file][1],
            old_tensor[file][2],
            old_tensor[file][3],
            old_tensor[file][0] + old_tensor[file][1] + old_tensor[file][2] + old_tensor[file][3],
            if let Some(file) = files_way[file].file {
                file.to_string()
            } else {
                "resign".to_string()
            },
            &format!("{:?}", files_way[file].result),
            give_values[file],
            take_values[file]
        ));
        Log::print_info(&text);
    }

    fn give(tensor: &mut [[u8; FEATURE_V_H_B_S_LEN]; FILE_LEN], file: usize) -> u16 {
        let mut give = 0;
        if 0 < tensor[file][0] {
            tensor[file][0] -= 1;
            give += 1;
        }
        if 0 < tensor[file][1] {
            tensor[file][0] -= 1;
            give += 1;
        }
        if 0 < tensor[file][2] {
            tensor[file][0] -= 1;
            give += 1;
        }
        if 0 < tensor[file][3] {
            tensor[file][0] -= 1;
            give += 1;
        }
        give
    }
}
