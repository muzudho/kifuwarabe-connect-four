use crate::computer_player::FEATURE_V_H_B_S_LEN;
use crate::FILE_LEN;
use crate::{
    computer_player::{Bestmove, Learning, Search},
    log::LogExt,
    Engine, GameResult, ResultChannel, SearchInfo, EVALUATION_FILE_NAME,
};
use casual_logger::Log;
use rand::Rng;

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
        let old_info_enabled = engine.pos.info_enabled;
        engine.pos.info_enabled = false;

        self.uh_by_result_channel(engine, ResultChannel::Win);
        self.uh_by_result_channel(engine, ResultChannel::Draw);

        engine.pos.info_enabled = old_info_enabled;
        engine.evaluation.save(EVALUATION_FILE_NAME);
    }

    /// uh...  
    /// うーん……。  
    pub fn uh_by_result_channel(&mut self, engine: &mut Engine, result_channel: ResultChannel) {
        let mut search = Search::default();
        search.start_pieces_num = engine.pos.pieces_num;

        let mut search_info = SearchInfo::default();
        let files_way = [
            {
                let mut bestmove = Bestmove::default();
                search.node_exit(
                    &mut engine.pos,
                    &engine.evaluation,
                    &result_channel,
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
                    &result_channel,
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
                    &result_channel,
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
                    &result_channel,
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
                    &result_channel,
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
                    &result_channel,
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
                    &result_channel,
                    'g',
                    &mut search_info,
                    &mut bestmove,
                );
                bestmove
            },
        ];

        let mut tensor = engine.evaluation.ways_weight(&engine.pos, &result_channel);
        let old_tensor = tensor.clone();
        // The number of files for which points can be obtained.
        // 点数を得られる列数。
        let mut obtainer = [false; FILE_LEN];
        let mut obtainer_count = 0;
        for file in 0..FILE_LEN {
            match result_channel {
                ResultChannel::Win => match files_way[file].result {
                    GameResult::Win => {
                        obtainer[file] = true;
                        obtainer_count += 1;
                    }
                    _ => {}
                },
                ResultChannel::Draw => match files_way[file].result {
                    GameResult::Draw => {
                        obtainer[file] = true;
                        obtainer_count += 1;
                    }
                    _ => {}
                },
            }
        }
        let co_obtainer_count = FILE_LEN as u16 - obtainer_count;
        // It can move the evaluation value.
        // 評価値が移動できます。
        let mut give_values = [0, 0, 0, 0, 0, 0, 0];
        let mut take_values = [0, 0, 0, 0, 0, 0, 0];
        if 0 < obtainer_count {
            give_values = [
                Learning::give(&mut tensor, 0),
                Learning::give(&mut tensor, 1),
                Learning::give(&mut tensor, 2),
                Learning::give(&mut tensor, 3),
                Learning::give(&mut tensor, 4),
                Learning::give(&mut tensor, 5),
                Learning::give(&mut tensor, 6),
            ];
            let gives_total = {
                let mut sum = 0;
                for file in 0..FILE_LEN {
                    sum += give_values[file];
                }
                sum
            };
            let obtain_point = gives_total / obtainer_count;
            let rest_point = gives_total % obtainer_count;
            take_values = [
                {
                    if obtainer[0] {
                        obtain_point
                    } else {
                        0
                    }
                },
                {
                    if obtainer[1] {
                        obtain_point
                    } else {
                        0
                    }
                },
                {
                    if obtainer[2] {
                        obtain_point
                    } else {
                        0
                    }
                },
                {
                    if obtainer[3] {
                        obtain_point
                    } else {
                        0
                    }
                },
                {
                    if obtainer[4] {
                        obtain_point
                    } else {
                        0
                    }
                },
                {
                    if obtainer[5] {
                        obtain_point
                    } else {
                        0
                    }
                },
                {
                    if obtainer[6] {
                        obtain_point
                    } else {
                        0
                    }
                },
            ];
            {
                let mut files = Vec::<usize>::new();
                for file in 0..FILE_LEN {
                    if obtainer[file] {
                        files.push(file);
                    }
                }
                for _i in 0..rest_point {
                    take_values[files[rand::thread_rng().gen_range(0, obtainer_count) as usize]] +=
                        1;
                }
            }
        }

        let rest_values = [
            {
                engine.evaluation.set_values_by_file(
                    &engine.pos,
                    'a',
                    &result_channel,
                    take_values[0],
                )
            },
            {
                engine.evaluation.set_values_by_file(
                    &engine.pos,
                    'b',
                    &result_channel,
                    take_values[1],
                )
            },
            {
                engine.evaluation.set_values_by_file(
                    &engine.pos,
                    'c',
                    &result_channel,
                    take_values[2],
                )
            },
            {
                engine.evaluation.set_values_by_file(
                    &engine.pos,
                    'd',
                    &result_channel,
                    take_values[3],
                )
            },
            {
                engine.evaluation.set_values_by_file(
                    &engine.pos,
                    'e',
                    &result_channel,
                    take_values[4],
                )
            },
            {
                engine.evaluation.set_values_by_file(
                    &engine.pos,
                    'f',
                    &result_channel,
                    take_values[5],
                )
            },
            {
                engine.evaluation.set_values_by_file(
                    &engine.pos,
                    'g',
                    &result_channel,
                    take_values[6],
                )
            },
        ];
        // Refund.
        // 還付。
        let rest_value = {
            let mut sum = 0;
            for val in &rest_values {
                sum += val;
            }
            sum
        };
        {
            give_values = [
                Learning::give(&mut tensor, 0),
                Learning::give(&mut tensor, 1),
                Learning::give(&mut tensor, 2),
                Learning::give(&mut tensor, 3),
                Learning::give(&mut tensor, 4),
                Learning::give(&mut tensor, 5),
                Learning::give(&mut tensor, 6),
            ];
            let gives_total = {
                let mut sum = 0;
                for file in 0..FILE_LEN {
                    sum += give_values[file];
                }
                sum
            };
            let obtain_point = gives_total / co_obtainer_count;
            let rest_point = gives_total % co_obtainer_count;
            take_values = [
                {
                    if !obtainer[0] {
                        obtain_point
                    } else {
                        0
                    }
                },
                {
                    if !obtainer[1] {
                        obtain_point
                    } else {
                        0
                    }
                },
                {
                    if !obtainer[2] {
                        obtain_point
                    } else {
                        0
                    }
                },
                {
                    if !obtainer[3] {
                        obtain_point
                    } else {
                        0
                    }
                },
                {
                    if !obtainer[4] {
                        obtain_point
                    } else {
                        0
                    }
                },
                {
                    if !obtainer[5] {
                        obtain_point
                    } else {
                        0
                    }
                },
                {
                    if !obtainer[6] {
                        obtain_point
                    } else {
                        0
                    }
                },
            ];
            {
                let mut files = Vec::<usize>::new();
                for file in 0..FILE_LEN {
                    if !obtainer[file] {
                        files.push(file);
                    }
                }
                for _i in 0..rest_point {
                    take_values
                        [files[rand::thread_rng().gen_range(0, co_obtainer_count) as usize]] += 1;
                }
            }
        }
        let lost_values = [
            {
                engine.evaluation.set_values_by_file(
                    &engine.pos,
                    'a',
                    &result_channel,
                    take_values[0],
                )
            },
            {
                engine.evaluation.set_values_by_file(
                    &engine.pos,
                    'b',
                    &result_channel,
                    take_values[1],
                )
            },
            {
                engine.evaluation.set_values_by_file(
                    &engine.pos,
                    'c',
                    &result_channel,
                    take_values[2],
                )
            },
            {
                engine.evaluation.set_values_by_file(
                    &engine.pos,
                    'd',
                    &result_channel,
                    take_values[3],
                )
            },
            {
                engine.evaluation.set_values_by_file(
                    &engine.pos,
                    'e',
                    &result_channel,
                    take_values[4],
                )
            },
            {
                engine.evaluation.set_values_by_file(
                    &engine.pos,
                    'f',
                    &result_channel,
                    take_values[5],
                )
            },
            {
                engine.evaluation.set_values_by_file(
                    &engine.pos,
                    'g',
                    &result_channel,
                    take_values[6],
                )
            },
        ];
        let lost_value = {
            let mut sum = 0;
            for val in &lost_values {
                sum += val;
            }
            sum
        };
        if 0 < lost_value {
            panic!(Log::print_fatal(&format!(
                "(Err.459) Learn fail. lost_value={}",
                lost_value
            )))
        }

        let mut text = String::new();
        text.push_str(&format!(
            "Result channel: {:?}
",
            result_channel
        ));
        text.push_str(&format!(
            "\
File Vert Hori Baro Sini Total Best File   Result Learn Give Take Next
---- ---- ---- ---- ---- -----      ------ ------       ---- ---- -----
"
        ));
        let file = 0;
        text.push_str(&format!(
            "   a {0: >4} {1: >4} {2: >4} {3: >4} {4: >5}      {5: <6} {6: <6}       {7: >4} {8: >4} {9: >5}
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
            take_values[file],
            tensor[file][0] + tensor[file][1] + tensor[file][2] + tensor[file][3],
        ));
        let file = 1;
        text.push_str(&format!(
            "   b {0: >4} {1: >4} {2: >4} {3: >4} {4: >5}      {5: <6} {6: <6}       {7: >4} {8: >4} {9: >5}
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
            take_values[file],
            tensor[file][0] + tensor[file][1] + tensor[file][2] + tensor[file][3],
        ));
        let file = 2;
        text.push_str(&format!(
            "   c {0: >4} {1: >4} {2: >4} {3: >4} {4: >5}      {5: <6} {6: <6}       {7: >4} {8: >4} {9: >5}
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
            take_values[file],
            tensor[file][0] + tensor[file][1] + tensor[file][2] + tensor[file][3],
        ));
        let file = 3;
        text.push_str(&format!(
            "   d {0: >4} {1: >4} {2: >4} {3: >4} {4: >5}      {5: <6} {6: <6}       {7: >4} {8: >4} {9: >5}
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
            take_values[file],
            tensor[file][0] + tensor[file][1] + tensor[file][2] + tensor[file][3],
        ));
        let file = 4;
        text.push_str(&format!(
            "   e {0: >4} {1: >4} {2: >4} {3: >4} {4: >5}      {5: <6} {6: <6}       {7: >4} {8: >4} {9: >5}
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
            take_values[file],
            tensor[file][0] + tensor[file][1] + tensor[file][2] + tensor[file][3],
        ));
        let file = 5;
        text.push_str(&format!(
            "   f {0: >4} {1: >4} {2: >4} {3: >4} {4: >5}      {5: <6} {6: <6}       {7: >4} {8: >4} {9: >5}
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
            take_values[file],
            tensor[file][0] + tensor[file][1] + tensor[file][2] + tensor[file][3],
        ));
        let file = 6;
        text.push_str(&format!(
            "   g {0: >4} {1: >4} {2: >4} {3: >4} {4: >5}      {5: <6} {6: <6}       {7: >4} {8: >4} {9: >5}
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
            take_values[file],
            tensor[file][0] + tensor[file][1] + tensor[file][2] + tensor[file][3],
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
