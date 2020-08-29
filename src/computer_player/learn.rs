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
        let a_way = {
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
        };
        let b_way = {
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
        };
        let c_way = {
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
        };
        let d_way = {
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
        };
        let e_way = {
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
        };
        let f_way = {
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
        };
        let g_way = {
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
        };

        let tensor = engine
            .evaluation
            .ways_weight(&engine.pos, &ResultChannel::Win);
        let mut text = String::new();
        text.push_str(&format!(
            "File Vert Hori Baro Sini Total Best File   Result
"
        ));
        text.push_str(&format!(
            "---- ---- ---- ---- ---- -----      ------ ------
"
        ));
        let file = 0;
        text.push_str(&format!(
            "   a {0: >4} {1: >4} {2: >4} {3: >4} {4: >5}      {5: >6} {6: >6}
",
            tensor[file][0],
            tensor[file][1],
            tensor[file][2],
            tensor[file][3],
            tensor[file][0] + tensor[file][1] + tensor[file][2] + tensor[file][3],
            if let Some(file) = a_way.file {
                file.to_string()
            } else {
                "resign".to_string()
            },
            a_way.result
        ));
        let file = 1;
        text.push_str(&format!(
            "   b {0: >4} {1: >4} {2: >4} {3: >4} {4: >5}      {5: >6} {6: >6}
",
            tensor[file][0],
            tensor[file][1],
            tensor[file][2],
            tensor[file][3],
            tensor[file][0] + tensor[file][1] + tensor[file][2] + tensor[file][3],
            if let Some(file) = b_way.file {
                file.to_string()
            } else {
                "resign".to_string()
            },
            a_way.result
        ));
        let file = 2;
        text.push_str(&format!(
            "   c {0: >4} {1: >4} {2: >4} {3: >4} {4: >5}      {5: >6} {6: >6}
",
            tensor[file][0],
            tensor[file][1],
            tensor[file][2],
            tensor[file][3],
            tensor[file][0] + tensor[file][1] + tensor[file][2] + tensor[file][3],
            if let Some(file) = c_way.file {
                file.to_string()
            } else {
                "resign".to_string()
            },
            a_way.result
        ));
        let file = 3;
        text.push_str(&format!(
            "   d {0: >4} {1: >4} {2: >4} {3: >4} {4: >5}      {5: >6} {6: >6}
",
            tensor[file][0],
            tensor[file][1],
            tensor[file][2],
            tensor[file][3],
            tensor[file][0] + tensor[file][1] + tensor[file][2] + tensor[file][3],
            if let Some(file) = d_way.file {
                file.to_string()
            } else {
                "resign".to_string()
            },
            a_way.result
        ));
        let file = 4;
        text.push_str(&format!(
            "   e {0: >4} {1: >4} {2: >4} {3: >4} {4: >5}      {5: >6} {6: >6}
",
            tensor[file][0],
            tensor[file][1],
            tensor[file][2],
            tensor[file][3],
            tensor[file][0] + tensor[file][1] + tensor[file][2] + tensor[file][3],
            if let Some(file) = e_way.file {
                file.to_string()
            } else {
                "resign".to_string()
            },
            a_way.result
        ));
        let file = 5;
        text.push_str(&format!(
            "   f {0: >4} {1: >4} {2: >4} {3: >4} {4: >5}      {5: >6} {6: >6}
",
            tensor[file][0],
            tensor[file][1],
            tensor[file][2],
            tensor[file][3],
            tensor[file][0] + tensor[file][1] + tensor[file][2] + tensor[file][3],
            if let Some(file) = f_way.file {
                file.to_string()
            } else {
                "resign".to_string()
            },
            a_way.result
        ));
        let file = 6;
        text.push_str(&format!(
            "   g {0: >4} {1: >4} {2: >4} {3: >4} {4: >5}      {5: >6} {6: >6}
",
            tensor[file][0],
            tensor[file][1],
            tensor[file][2],
            tensor[file][3],
            tensor[file][0] + tensor[file][1] + tensor[file][2] + tensor[file][3],
            if let Some(file) = g_way.file {
                file.to_string()
            } else {
                "resign".to_string()
            },
            a_way.result
        ));
        Log::print_info(&text);
    }
}
