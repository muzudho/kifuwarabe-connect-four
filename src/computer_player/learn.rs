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

        let old_tensor = engine.evaluation.ways_weight(&engine.pos, &result_channel);
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
        let give_values;
        let mut take1_values;
        let gives_total;
        if 0 < obtainer_count {
            give_values = [
                {
                    let file_ch = 'a';
                    let old =
                        engine
                            .evaluation
                            .get_values_by_file(&engine.pos, file_ch, &result_channel);
                    let gives = engine.evaluation.give_value_by_file(
                        &engine.pos,
                        file_ch,
                        &result_channel,
                        4,
                    );
                    let new_ =
                        engine
                            .evaluation
                            .get_values_by_file(&engine.pos, file_ch, &result_channel);
                    Log::print_info(&format!(
                        "{} old=|{}|{}|{}|{}| gives={} new=|{}|{}|{}|{}|",
                        file_ch,
                        old[0],
                        old[1],
                        old[2],
                        old[3],
                        gives,
                        new_[0],
                        new_[1],
                        new_[2],
                        new_[3],
                    ));
                    gives
                },
                {
                    let file_ch = 'b';
                    let old =
                        engine
                            .evaluation
                            .get_values_by_file(&engine.pos, file_ch, &result_channel);
                    let gives = engine.evaluation.give_value_by_file(
                        &engine.pos,
                        file_ch,
                        &result_channel,
                        4,
                    );
                    let new_ =
                        engine
                            .evaluation
                            .get_values_by_file(&engine.pos, file_ch, &result_channel);
                    Log::print_info(&format!(
                        "{} old=|{}|{}|{}|{}| gives={} new=|{}|{}|{}|{}|",
                        file_ch,
                        old[0],
                        old[1],
                        old[2],
                        old[3],
                        gives,
                        new_[0],
                        new_[1],
                        new_[2],
                        new_[3],
                    ));
                    gives
                },
                {
                    let file_ch = 'c';
                    let old =
                        engine
                            .evaluation
                            .get_values_by_file(&engine.pos, file_ch, &result_channel);
                    let gives = engine.evaluation.give_value_by_file(
                        &engine.pos,
                        file_ch,
                        &result_channel,
                        4,
                    );
                    let new_ =
                        engine
                            .evaluation
                            .get_values_by_file(&engine.pos, file_ch, &result_channel);
                    Log::print_info(&format!(
                        "{} old=|{}|{}|{}|{}| gives={} new=|{}|{}|{}|{}|",
                        file_ch,
                        old[0],
                        old[1],
                        old[2],
                        old[3],
                        gives,
                        new_[0],
                        new_[1],
                        new_[2],
                        new_[3],
                    ));
                    gives
                },
                {
                    let file_ch = 'd';
                    let old =
                        engine
                            .evaluation
                            .get_values_by_file(&engine.pos, file_ch, &result_channel);
                    let gives = engine.evaluation.give_value_by_file(
                        &engine.pos,
                        file_ch,
                        &result_channel,
                        4,
                    );
                    let new_ =
                        engine
                            .evaluation
                            .get_values_by_file(&engine.pos, file_ch, &result_channel);
                    Log::print_info(&format!(
                        "{} old=|{}|{}|{}|{}| gives={} new=|{}|{}|{}|{}|",
                        file_ch,
                        old[0],
                        old[1],
                        old[2],
                        old[3],
                        gives,
                        new_[0],
                        new_[1],
                        new_[2],
                        new_[3],
                    ));
                    gives
                },
                {
                    let file_ch = 'e';
                    let old =
                        engine
                            .evaluation
                            .get_values_by_file(&engine.pos, file_ch, &result_channel);
                    let gives = engine.evaluation.give_value_by_file(
                        &engine.pos,
                        file_ch,
                        &result_channel,
                        4,
                    );
                    let new_ =
                        engine
                            .evaluation
                            .get_values_by_file(&engine.pos, file_ch, &result_channel);
                    Log::print_info(&format!(
                        "{} old=|{}|{}|{}|{}| gives={} new=|{}|{}|{}|{}|",
                        file_ch,
                        old[0],
                        old[1],
                        old[2],
                        old[3],
                        gives,
                        new_[0],
                        new_[1],
                        new_[2],
                        new_[3],
                    ));
                    gives
                },
                {
                    let file_ch = 'f';
                    let old =
                        engine
                            .evaluation
                            .get_values_by_file(&engine.pos, file_ch, &result_channel);
                    let gives = engine.evaluation.give_value_by_file(
                        &engine.pos,
                        file_ch,
                        &result_channel,
                        4,
                    );
                    let new_ =
                        engine
                            .evaluation
                            .get_values_by_file(&engine.pos, file_ch, &result_channel);
                    Log::print_info(&format!(
                        "{} old=|{}|{}|{}|{}| gives={} new=|{}|{}|{}|{}|",
                        file_ch,
                        old[0],
                        old[1],
                        old[2],
                        old[3],
                        gives,
                        new_[0],
                        new_[1],
                        new_[2],
                        new_[3],
                    ));
                    gives
                },
                {
                    let file_ch = 'g';
                    let old =
                        engine
                            .evaluation
                            .get_values_by_file(&engine.pos, file_ch, &result_channel);
                    let gives = engine.evaluation.give_value_by_file(
                        &engine.pos,
                        file_ch,
                        &result_channel,
                        4,
                    );
                    let new_ =
                        engine
                            .evaluation
                            .get_values_by_file(&engine.pos, file_ch, &result_channel);
                    Log::print_info(&format!(
                        "{} old=|{}|{}|{}|{}| gives={} new=|{}|{}|{}|{}|",
                        file_ch,
                        old[0],
                        old[1],
                        old[2],
                        old[3],
                        gives,
                        new_[0],
                        new_[1],
                        new_[2],
                        new_[3],
                    ));
                    gives
                },
            ];
            gives_total = {
                let mut sum = 0;
                for file in 0..FILE_LEN {
                    sum += give_values[file];
                }
                sum
            };
            let obtain_point = gives_total / obtainer_count;
            let rest_point = gives_total % obtainer_count;
            take1_values = [
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
                    take1_values
                        [files[rand::thread_rng().gen_range(0, obtainer_count) as usize]] += 1;
                }
            }
        } else {
            give_values = [0, 0, 0, 0, 0, 0, 0];
            take1_values = [0, 0, 0, 0, 0, 0, 0];
            gives_total = 0;
        }

        let rest_values = [
            {
                engine.evaluation.set_values_by_file(
                    &engine.pos,
                    'a',
                    &result_channel,
                    take1_values[0],
                )
            },
            {
                engine.evaluation.set_values_by_file(
                    &engine.pos,
                    'b',
                    &result_channel,
                    take1_values[1],
                )
            },
            {
                engine.evaluation.set_values_by_file(
                    &engine.pos,
                    'c',
                    &result_channel,
                    take1_values[2],
                )
            },
            {
                engine.evaluation.set_values_by_file(
                    &engine.pos,
                    'd',
                    &result_channel,
                    take1_values[3],
                )
            },
            {
                engine.evaluation.set_values_by_file(
                    &engine.pos,
                    'e',
                    &result_channel,
                    take1_values[4],
                )
            },
            {
                engine.evaluation.set_values_by_file(
                    &engine.pos,
                    'f',
                    &result_channel,
                    take1_values[5],
                )
            },
            {
                engine.evaluation.set_values_by_file(
                    &engine.pos,
                    'g',
                    &result_channel,
                    take1_values[6],
                )
            },
        ];
        // Refund.
        // 還付。
        let refund_total = {
            let mut sum = 0;
            for val in &rest_values {
                sum += val;
            }
            sum
        };
        let mut refund1_values;
        {
            let refund_point = refund_total / co_obtainer_count;
            let refund_rest_point = refund_total % co_obtainer_count;
            refund1_values = [
                {
                    if !obtainer[0] {
                        refund_point
                    } else {
                        0
                    }
                },
                {
                    if !obtainer[1] {
                        refund_point
                    } else {
                        0
                    }
                },
                {
                    if !obtainer[2] {
                        refund_point
                    } else {
                        0
                    }
                },
                {
                    if !obtainer[3] {
                        refund_point
                    } else {
                        0
                    }
                },
                {
                    if !obtainer[4] {
                        refund_point
                    } else {
                        0
                    }
                },
                {
                    if !obtainer[5] {
                        refund_point
                    } else {
                        0
                    }
                },
                {
                    if !obtainer[6] {
                        refund_point
                    } else {
                        0
                    }
                },
            ];
            {
                let mut refunder_files = Vec::<usize>::new();
                for file in 0..FILE_LEN {
                    if !obtainer[file] {
                        refunder_files.push(file);
                    }
                }
                for _i in 0..refund_rest_point {
                    refund1_values[refunder_files
                        [rand::thread_rng().gen_range(0, co_obtainer_count) as usize]] += 1;
                }
            }
        }
        let lost_values = [
            {
                engine.evaluation.set_values_by_file(
                    &engine.pos,
                    'a',
                    &result_channel,
                    refund1_values[0],
                )
            },
            {
                engine.evaluation.set_values_by_file(
                    &engine.pos,
                    'b',
                    &result_channel,
                    refund1_values[1],
                )
            },
            {
                engine.evaluation.set_values_by_file(
                    &engine.pos,
                    'c',
                    &result_channel,
                    refund1_values[2],
                )
            },
            {
                engine.evaluation.set_values_by_file(
                    &engine.pos,
                    'd',
                    &result_channel,
                    refund1_values[3],
                )
            },
            {
                engine.evaluation.set_values_by_file(
                    &engine.pos,
                    'e',
                    &result_channel,
                    refund1_values[4],
                )
            },
            {
                engine.evaluation.set_values_by_file(
                    &engine.pos,
                    'f',
                    &result_channel,
                    refund1_values[5],
                )
            },
            {
                engine.evaluation.set_values_by_file(
                    &engine.pos,
                    'g',
                    &result_channel,
                    refund1_values[6],
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
            "Result channel={:?} Gives total={}
",
            result_channel, gives_total
        ));
        text.push_str(&format!(
            "\
File Vert Hori Baro Sini Total Best File   Result Learn Give Take Rest Refund Next Vert Hori Baro Sini Total
---- ---- ---- ---- ---- -----      ------ ------       ---- ---- ---- ------      ---- ---- ---- ---- -----
"
        ));
        text.push_str(&self.file_line_str(
            0,
            'a',
            &old_tensor,
            &engine.evaluation.ways_weight(&engine.pos, &result_channel),
            &files_way,
            &give_values,
            &take1_values,
            &rest_values,
            &refund1_values,
        ));
        text.push_str(&self.file_line_str(
            1,
            'b',
            &old_tensor,
            &engine.evaluation.ways_weight(&engine.pos, &result_channel),
            &files_way,
            &give_values,
            &take1_values,
            &rest_values,
            &refund1_values,
        ));
        text.push_str(&self.file_line_str(
            2,
            'c',
            &old_tensor,
            &engine.evaluation.ways_weight(&engine.pos, &result_channel),
            &files_way,
            &give_values,
            &take1_values,
            &rest_values,
            &refund1_values,
        ));
        text.push_str(&self.file_line_str(
            3,
            'd',
            &old_tensor,
            &engine.evaluation.ways_weight(&engine.pos, &result_channel),
            &files_way,
            &give_values,
            &take1_values,
            &rest_values,
            &refund1_values,
        ));
        text.push_str(&self.file_line_str(
            4,
            'e',
            &old_tensor,
            &engine.evaluation.ways_weight(&engine.pos, &result_channel),
            &files_way,
            &give_values,
            &take1_values,
            &rest_values,
            &refund1_values,
        ));
        text.push_str(&self.file_line_str(
            5,
            'f',
            &old_tensor,
            &engine.evaluation.ways_weight(&engine.pos, &result_channel),
            &files_way,
            &give_values,
            &take1_values,
            &rest_values,
            &refund1_values,
        ));
        text.push_str(&self.file_line_str(
            6,
            'g',
            &old_tensor,
            &engine.evaluation.ways_weight(&engine.pos, &result_channel),
            &files_way,
            &give_values,
            &take1_values,
            &rest_values,
            &refund1_values,
        ));
        Log::print_info(&text);
    }

    fn file_line_str(
        &self,
        file: usize,
        file_ch: char,
        old_tensor: &[[u8; FEATURE_V_H_B_S_LEN]; FILE_LEN],
        new_tensor: &[[u8; FEATURE_V_H_B_S_LEN]; FILE_LEN],
        files_way: &[Bestmove; FILE_LEN],
        give_values: &[u16],
        take1_values: &[u16],
        rest_values: &[u16],
        refund_values: &[u16],
    ) -> String {
        format!(
            "   {0} {1: >4} {2: >4} {3: >4} {4: >4} {5: >5}      {6: <6} {7: <6}       {8: >4} {9: >4} {10: >4} {11: >6}      {12: >4} {13: >4} {14: >4} {15: >4} {16: >5}
",
            file_ch,
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
            take1_values[file],
            rest_values[file],
            refund_values[file],
            new_tensor[file][0],
            new_tensor[file][1],
            new_tensor[file][2],
            new_tensor[file][3],
            new_tensor[file][0] + new_tensor[file][1] + new_tensor[file][2] + new_tensor[file][3],
        )
    }
}
