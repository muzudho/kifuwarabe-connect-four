//! The thinking department of a computer.  
//! See 'Search' struct in 'look_and_model' for details.  
//! コンピューターの思考部です。  
//! 詳しくは 'look_and_model' の 'Search' 構造体 を見てください。  
use crate::log::LogExt;
use crate::{
    computer_player::{Evaluation, Search},
    EvaluationWay, GameResult, Position, SearchDirection, SearchInfo, SQUARES_NUM,
};
use casual_logger::{Level, Log};
use rand::Rng;
use std::time::Instant;

/// Search.  
/// 探索部。  
impl Default for Search {
    fn default() -> Self {
        Search {
            start_pieces_num: 0,
            nodes: 0,
            stopwatch: Instant::now(),
        }
    }
}

/// Search.  
/// 探索部。  
impl Search {
    /// This is the place to put the stone.  
    /// 石を置く場所です。  
    ///
    /// # Arguments
    ///
    /// * `pos` - Position.  
    ///             局面。  
    ///
    /// # Returns
    ///
    /// * `Option<u8>` - Address of square.  
    ///                     マスの番地。  
    /// * `GameResult` - Evaluation.  
    ///                     評価値。  
    pub fn go(
        &mut self,
        pos: &mut Position,
        evaluation: &Evaluation,
    ) -> (Option<char>, GameResult) {
        let (win_file, win_result) = self.first_node(pos, &EvaluationWay::Win, evaluation);
        match win_result {
            GameResult::Win => {
                return (win_file, win_result);
            }
            _ => {}
        }

        let (draw_file, draw_result) = self.first_node(pos, &EvaluationWay::Draw, evaluation);
        match draw_result {
            GameResult::Draw => {
                return (draw_file, draw_result);
            }
            _ => {}
        }

        let number = rand::thread_rng().gen_range(0, 2);
        if number == 0 {
            (win_file, win_result)
        } else {
            (draw_file, draw_result)
        }
    }

    /// The state node of the search tree. Commonly called search.  
    /// 検索ツリーの状態ノード。一般に 'search' と呼ばれます。  
    ///
    /// * `pos` - Position.  
    ///             局面。  
    ///
    /// # Returns
    ///
    /// * `Option<u8>` - Address of square.  
    ///                     マスの番地。  
    /// * `GameResult` - Evaluation.  
    ///                     評価値。  
    fn first_node(
        &mut self,
        pos: &mut Position,
        way: &EvaluationWay,
        evaluation: &Evaluation,
    ) -> (Option<char>, GameResult) {
        let mut best_file = None;
        let mut best_result = GameResult::Lose;

        // Full width search.
        // 全幅探索。
        for file in &['a', 'b', 'c', 'd', 'e', 'f', 'g'] {
            let mut search_info = SearchInfo::new(&way, &[1, 1, 1, 1, 1, 1, 1]);
            // I only look at the empty square.
            // 空きマスだけを見ます。
            if !pos.is_file_fill(*file) {
                let mut info_backwarding = None;
                let (forward_cut_off, info_leaf_child) =
                    self.node_exit_to_child_side(pos, *file, &mut search_info);

                if let None = forward_cut_off {
                    // If you move forward, it's your opponent's turn.
                    // 前向きに探索したら、次は対戦相手の番です。
                    let (_opponent_sq, opponent_game_result) = self.node(pos, way, evaluation);
                    // I'm back.
                    // 戻ってきました。
                    info_backwarding = Some(opponent_game_result);
                }
                let (best_file_child, best_result_child) = &self.node_enter_from_child_side(
                    pos,
                    *file,
                    &mut best_file,
                    &mut best_result,
                    forward_cut_off,
                    info_leaf_child,
                    info_backwarding,
                    &mut search_info,
                );
                best_file = *best_file_child;
                best_result = *best_result_child;
            }
        }

        // End of turn.
        // 手番の終わり。
        (best_file, best_result)
    }

    /// The state node of the search tree. Commonly called search.  
    /// 検索ツリーの状態ノード。一般に 'search' と呼ばれます。  
    ///
    /// * `pos` - Position.  
    ///             局面。  
    ///
    /// # Returns
    ///
    /// * `Option<u8>` - Address of square.  
    ///                     マスの番地。  
    /// * `GameResult` - Evaluation.  
    ///                     評価値。  
    fn node(
        &mut self,
        pos: &mut Position,
        way: &EvaluationWay,
        evaluation: &Evaluation,
    ) -> (Option<char>, GameResult) {
        let mut best_win_file = None;
        let mut best_win_result = GameResult::Lose;

        // Select one at random.
        // ランダムに１つ選びます。
        if let (Some(file), mut search_info) = self.choose_file(pos, way, evaluation) {
            // I only look at the empty square.
            // 空きマスだけを見ます。
            if !pos.is_file_fill(file) {
                let mut info_backwarding = None;
                let (forward_cut_off, info_leaf_child) =
                    self.node_exit_to_child_side(pos, file, &mut search_info);

                if let None = forward_cut_off {
                    // If you move forward, it's your opponent's turn.
                    // 前向きに探索したら、次は対戦相手の番です。
                    let (_opponent_sq, opponent_game_result) = self.node(pos, way, evaluation);
                    // I'm back.
                    // 戻ってきました。
                    info_backwarding = Some(opponent_game_result);
                }
                let (best_file_child, best_result_child) = &self.node_enter_from_child_side(
                    pos,
                    file,
                    &mut best_win_file,
                    &mut best_win_result,
                    forward_cut_off,
                    info_leaf_child,
                    info_backwarding,
                    &mut search_info,
                );
                best_win_file = *best_file_child;
                best_win_result = *best_result_child;
            }
        }

        // End of turn.
        // 手番の終わり。
        (best_win_file, best_win_result)
    }

    fn node_exit_to_child_side(
        &mut self,
        pos: &mut Position,
        file: char,
        search_info: &mut SearchInfo,
    ) -> (Option<ForwardCutOff>, bool) {
        let mut info_leaf = false;
        // Let's put a stone for now.
        // とりあえず石を置きましょう。
        pos.do_move(file);
        self.nodes += 1;

        // Find out why you are not doing a forward search.
        // If not, I will search.
        // 前向き検索を行わない理由を調べてください。
        // 無ければ探索します。
        let forward_cut_off = if pos.is_opponent_win() {
            // The opponent wins.
            // 対戦相手の勝ち。
            if Log::enabled(Level::Info) && pos.info_enabled {
                search_info.result = Some(GameResult::Win);
                search_info.comment = Some("Resign.".to_string());
            }
            Some(ForwardCutOff::OpponentWin)
        } else if SQUARES_NUM <= pos.pieces_num {
            // Draw if there is no place to put.
            // 置く場所が無ければ引き分け。
            if Log::enabled(Level::Info) && pos.info_enabled {
                info_leaf = true;
                search_info.result = Some(GameResult::Draw);
                search_info.comment = Some("It is ok.".to_string());
            }
            Some(ForwardCutOff::Draw)
        } else {
            if Log::enabled(Level::Info) && pos.info_enabled {
                search_info.comment = Some("Search.".to_string());
            }
            None
        };

        // (1) Outputs information for forward search.
        // (一) 前向き探索の情報を出力します。
        if pos.info_enabled {
            search_info.nps = self.nps();
            search_info.nodes = self.nodes;
            search_info.pv = pos.pv.to_string();
            search_info.search_direction = SearchDirection::Forward;
            search_info.leaf = info_leaf;
            search_info.pieces_num = None;
            search_info.turn = pos.turn;
            Log::print_info(&search_info.to_string());
        }

        return (forward_cut_off, info_leaf);
    }

    fn node_enter_from_child_side(
        &mut self,
        pos: &mut Position,
        file: char,
        best_file: &mut Option<char>,
        best_result: &mut GameResult,
        forward_cut_off: Option<ForwardCutOff>,
        info_leaf: bool,
        info_backwarding: Option<GameResult>,
        search_info: &mut SearchInfo,
    ) -> (Option<char>, GameResult) {
        let mut backward_cut_off = None;
        // (2) Remove the placed stone.
        // (二) 置いた石は取り除きます。
        pos.undo_move();

        if let Some(opponent_game_result) = info_backwarding {
            match opponent_game_result {
                GameResult::Lose => {
                    // I beat the opponent.
                    // 相手を負かしました。

                    // The search ends.
                    // 探索を終了します。
                    backward_cut_off = Some(BackwardCutOff::YouWin);
                }
                GameResult::Draw => {
                    // If neither is wrong, draw.
                    // お互いがミスしなければ引き分け。

                    match best_result {
                        GameResult::Lose => {
                            // If it gets better, change it to this. Generally called 'Update alpha evaluation'.
                            // 良くなるならこの手に変えます。一般的には 'α評価値の更新' と呼びます。
                            *best_file = Some(file);
                            *best_result = GameResult::Draw;
                        }
                        _ => {}
                    }
                    // I will continue.
                    // まだ続けます。
                }
                GameResult::Win => {
                    // Don't choose to lose.
                    // 自分が負ける手は選びません。

                    // I will continue.
                    // まだ続けます。
                }
            }
        }

        // (3) Outputs backward search information.
        // (三) 後ろ向き探索の情報を出力します。
        if Log::enabled(Level::Info) && pos.info_enabled {
            if let Some(opponent_game_result) = info_backwarding {
                match opponent_game_result {
                    GameResult::Lose => {
                        // I beat the opponent.
                        // 相手を負かしました。
                        search_info.result = Some(GameResult::Win);
                        search_info.comment = Some("Hooray!".to_string());
                    }
                    GameResult::Draw => {
                        // If neither is wrong, draw.
                        // お互いがミスしなければ引き分け。
                        search_info.result = Some(GameResult::Draw);
                        search_info.comment = Some("Fmmm.".to_string());
                    }
                    GameResult::Win => {
                        // Don't choose to lose.
                        // 自分が負ける手は選びません。
                        search_info.result = Some(GameResult::Lose);
                        search_info.comment = Some("Damn!".to_string());
                    }
                }
            }
            search_info.nps = self.nps();
            search_info.nodes = self.nodes;
            search_info.pv = pos.pv.to_string();
            search_info.search_direction = SearchDirection::Backward;
            search_info.leaf = info_leaf;
            search_info.pieces_num = Some(pos.pieces_num);
            search_info.turn = pos.turn;
            Log::print_info(&search_info.to_string());
        }

        // (4) Depending on the condition, the sibling node search is skipped.
        // (四) 条件によっては、兄弟ノードの検索がスキップされます。
        if let Some(forward_cut_off) = forward_cut_off {
            match forward_cut_off {
                ForwardCutOff::OpponentWin => {
                    return (Some(file), GameResult::Win);
                }
                ForwardCutOff::Draw => {
                    return (Some(file), GameResult::Draw);
                }
            }
        } else if let Some(backward_cut_off) = backward_cut_off {
            match backward_cut_off {
                BackwardCutOff::YouWin => {
                    return (Some(file), GameResult::Win);
                }
            }
        }

        return (*best_file, *best_result);
    }

    /// Select one file at random.
    /// TODO 重みを付けて、ランダムに列を１つ選びます。
    fn choose_file(
        &mut self,
        pos: &Position,
        way: &EvaluationWay,
        evaluation: &Evaluation,
    ) -> (Option<char>, SearchInfo) {
        let w = evaluation.ways_weight(pos, way);
        let mut search_info = SearchInfo::new(way, &w);
        // Upper bound.
        let a_up: u16 = w[0] as u16;
        let b_up = a_up + w[1] as u16;
        let c_up = b_up + w[2] as u16;
        let d_up = c_up + w[3] as u16;
        let e_up = d_up + w[4] as u16;
        let f_up = e_up + w[5] as u16;
        let total = f_up + w[6] as u16;
        if total == 0 {
            if pos.info_enabled {
                search_info.chosen_file = None;
            }
            return (None, search_info);
        }

        let number = rand::thread_rng().gen_range(0, total);
        let file = if number < a_up {
            'a'
        } else if number < b_up {
            'b'
        } else if number < c_up {
            'c'
        } else if number < d_up {
            'd'
        } else if number < e_up {
            'e'
        } else if number < f_up {
            'f'
        } else {
            'g'
        };

        if pos.info_enabled {
            search_info.chosen_file = Some(file);
        }

        (Some(file), search_info)
    }
}

/// The reason for ending the forward search.  
/// 前向き探索を終了した理由。  
#[derive(Clone, Copy)]
enum ForwardCutOff {
    /// End with a opponent win.  
    /// 相手の勝ちにつき、終了。  
    OpponentWin,
    /// End with a draw.  
    /// 引き分けにつき、終了。  
    Draw,
}

/// The reason for ending the backward search.  
/// 後ろ向き探索を終了した理由。  
enum BackwardCutOff {
    /// End with a you win.  
    /// あなたの勝ちにつき、終了。  
    YouWin,
}
