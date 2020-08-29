use crate::computer_player::Evaluation;
use crate::computer_player::{
    N3POW4, N3POW5, N3POW6, N3POW7, NOUGHT_AND_CROSS_LEN, WIN_AND_DRAW_LEN,
};
use crate::log::LogExt;
use crate::look_and_model::EvaluationWay;
use crate::look_and_model::Position;
use crate::{Piece, FILE_LEN};
use casual_logger::Log;

/// Initial value of evaluation.
/// 評価値の初期値。
///
/// 0 <= x < 255/8
pub const INIT_VAL: u8 = 25;

impl Default for Evaluation {
    fn default() -> Self {
        Evaluation {
            features_1_to_7: [[[[INIT_VAL; NOUGHT_AND_CROSS_LEN]; WIN_AND_DRAW_LEN]; N3POW6]; 7],
            features_8_to_13: [[[[INIT_VAL; NOUGHT_AND_CROSS_LEN]; WIN_AND_DRAW_LEN]; N3POW7]; 6],
            features_14_19_20_25: [[[[INIT_VAL; NOUGHT_AND_CROSS_LEN]; WIN_AND_DRAW_LEN]; N3POW4];
                4],
            features_15_18_21_24: [[[[INIT_VAL; NOUGHT_AND_CROSS_LEN]; WIN_AND_DRAW_LEN]; N3POW5];
                4],
            features_16_17_22_23: [[[[INIT_VAL; NOUGHT_AND_CROSS_LEN]; WIN_AND_DRAW_LEN]; N3POW6];
                4],
        }
    }
}
impl Evaluation {
    /// [a, b, c, d, e, f, g]
    pub fn ways_weight(&self, pos: &Position, way: &EvaluationWay) -> [u8; FILE_LEN] {
        // マスの特徴量を求めます。
        // 7つの指し手のマスを調べます。
        let win_way_values = [
            self.get_value_by_sq(pos, pos.fallen_sq_or_none('a'), way),
            self.get_value_by_sq(pos, pos.fallen_sq_or_none('b'), way),
            self.get_value_by_sq(pos, pos.fallen_sq_or_none('c'), way),
            self.get_value_by_sq(pos, pos.fallen_sq_or_none('d'), way),
            self.get_value_by_sq(pos, pos.fallen_sq_or_none('e'), way),
            self.get_value_by_sq(pos, pos.fallen_sq_or_none('f'), way),
            self.get_value_by_sq(pos, pos.fallen_sq_or_none('g'), way),
        ];

        [
            win_way_values[0],
            win_way_values[1],
            win_way_values[2],
            win_way_values[3],
            win_way_values[4],
            win_way_values[5],
            win_way_values[6],
        ]
    }

    fn get_value_by_sq(&self, pos: &Position, sq: Option<usize>, way: &EvaluationWay) -> u8 {
        let mut sum = 0;
        for feature in &self.get_elemental_features_by_sq(sq) {
            sum += self.get_value_by_feature(pos, *feature, way);
        }

        sum
    }

    fn get_value_by_feature(&self, pos: &Position, feature: Option<u8>, way: &EvaluationWay) -> u8 {
        if let Some(feature) = feature {
            let state = self.get_state_by_feature(pos, feature) as usize;
            match feature {
                1 => self.features_1_to_7[0][state][*way as usize][pos.turn as usize],
                2 => self.features_1_to_7[1][state][*way as usize][pos.turn as usize],
                3 => self.features_1_to_7[2][state][*way as usize][pos.turn as usize],
                4 => self.features_1_to_7[3][state][*way as usize][pos.turn as usize],
                5 => self.features_1_to_7[4][state][*way as usize][pos.turn as usize],
                6 => self.features_1_to_7[5][state][*way as usize][pos.turn as usize],
                7 => self.features_1_to_7[6][state][*way as usize][pos.turn as usize],
                8 => self.features_8_to_13[0][state][*way as usize][pos.turn as usize],
                9 => self.features_8_to_13[1][state][*way as usize][pos.turn as usize],
                10 => self.features_8_to_13[2][state][*way as usize][pos.turn as usize],
                11 => self.features_8_to_13[3][state][*way as usize][pos.turn as usize],
                12 => self.features_8_to_13[4][state][*way as usize][pos.turn as usize],
                13 => self.features_8_to_13[5][state][*way as usize][pos.turn as usize],
                14 => self.features_14_19_20_25[0][state][*way as usize][pos.turn as usize],
                15 => self.features_15_18_21_24[0][state][*way as usize][pos.turn as usize],
                16 => self.features_16_17_22_23[0][state][*way as usize][pos.turn as usize],
                17 => self.features_16_17_22_23[1][state][*way as usize][pos.turn as usize],
                18 => self.features_15_18_21_24[1][state][*way as usize][pos.turn as usize],
                19 => self.features_14_19_20_25[1][state][*way as usize][pos.turn as usize],
                20 => self.features_14_19_20_25[2][state][*way as usize][pos.turn as usize],
                21 => self.features_15_18_21_24[2][state][*way as usize][pos.turn as usize],
                22 => self.features_16_17_22_23[2][state][*way as usize][pos.turn as usize],
                23 => self.features_16_17_22_23[3][state][*way as usize][pos.turn as usize],
                24 => self.features_15_18_21_24[3][state][*way as usize][pos.turn as usize],
                25 => self.features_14_19_20_25[3][state][*way as usize][pos.turn as usize],
                _ => panic!(Log::print_fatal(&format!(
                    "(Err.123)  Invalid feature. / {}",
                    feature
                ))),
            }
        } else {
            0
        }
    }

    fn get_state_by_feature(&self, pos: &Position, feature: u8) -> u16 {
        match feature {
            1 => self.get_feature_state_by_figures(pos, vec![0, 7, 14, 21, 28, 35]),
            2 => self.get_feature_state_by_figures(pos, vec![1, 8, 15, 22, 29, 36]),
            3 => self.get_feature_state_by_figures(pos, vec![2, 9, 16, 23, 30, 37]),
            4 => self.get_feature_state_by_figures(pos, vec![3, 10, 17, 24, 31, 38]),
            5 => self.get_feature_state_by_figures(pos, vec![4, 11, 18, 25, 32, 39]),
            6 => self.get_feature_state_by_figures(pos, vec![5, 12, 19, 26, 33, 40]),
            7 => self.get_feature_state_by_figures(pos, vec![6, 13, 20, 27, 34, 41]),
            8 => self.get_feature_state_by_figures(pos, vec![35, 36, 37, 38, 39, 40, 41]),
            9 => self.get_feature_state_by_figures(pos, vec![28, 29, 30, 31, 32, 33, 34]),
            10 => self.get_feature_state_by_figures(pos, vec![21, 22, 23, 24, 25, 26, 27]),
            11 => self.get_feature_state_by_figures(pos, vec![14, 15, 16, 17, 18, 19, 20]),
            12 => self.get_feature_state_by_figures(pos, vec![7, 8, 9, 10, 11, 12, 13]),
            13 => self.get_feature_state_by_figures(pos, vec![0, 1, 2, 3, 4, 5, 6]),
            14 => self.get_feature_state_by_figures(pos, vec![21, 15, 9, 3]),
            15 => self.get_feature_state_by_figures(pos, vec![28, 22, 16, 10, 4]),
            16 => self.get_feature_state_by_figures(pos, vec![35, 29, 23, 27, 11, 5]),
            17 => self.get_feature_state_by_figures(pos, vec![36, 30, 24, 18, 12, 6]),
            18 => self.get_feature_state_by_figures(pos, vec![37, 31, 25, 19, 13]),
            19 => self.get_feature_state_by_figures(pos, vec![38, 32, 26, 20]),
            20 => self.get_feature_state_by_figures(pos, vec![14, 22, 30, 38]),
            21 => self.get_feature_state_by_figures(pos, vec![7, 15, 23, 31, 39]),
            22 => self.get_feature_state_by_figures(pos, vec![0, 8, 16, 24, 32, 40]),
            23 => self.get_feature_state_by_figures(pos, vec![1, 9, 17, 25, 33, 41]),
            24 => self.get_feature_state_by_figures(pos, vec![2, 10, 18, 26, 34]),
            25 => self.get_feature_state_by_figures(pos, vec![3, 11, 19, 27]),
            _ => panic!(Log::print_fatal(&format!(
                "(Err.160)  Invalid feature. / {}",
                feature
            ))),
        }
    }

    /// Elemental features of the square.
    /// そのマスの成分特徴。
    fn get_elemental_features_by_sq(&self, sq: Option<usize>) -> [Option<u8>; 4] {
        if let Some(sq) = sq {
            match sq {
                0 => [Some(1), Some(13), None, Some(22)],
                1 => [Some(2), Some(13), None, Some(23)],
                2 => [Some(3), Some(13), None, Some(24)],
                3 => [Some(4), Some(13), Some(14), Some(25)],
                4 => [Some(5), Some(13), Some(15), None],
                5 => [Some(6), Some(13), Some(16), None],
                6 => [Some(7), Some(13), Some(17), None],
                7 => [Some(1), Some(12), None, Some(21)],
                8 => [Some(2), Some(12), None, Some(22)],
                9 => [Some(3), Some(12), Some(14), Some(23)],
                10 => [Some(4), Some(12), Some(15), Some(24)],
                11 => [Some(5), Some(12), Some(16), Some(25)],
                12 => [Some(6), Some(12), Some(17), None],
                13 => [Some(7), Some(12), Some(18), None],
                14 => [Some(1), Some(11), None, Some(20)],
                15 => [Some(2), Some(11), Some(14), Some(21)],
                16 => [Some(3), Some(11), Some(15), Some(22)],
                17 => [Some(4), Some(11), Some(16), Some(23)],
                18 => [Some(5), Some(11), Some(17), Some(24)],
                19 => [Some(6), Some(11), Some(18), Some(25)],
                20 => [Some(7), Some(11), Some(19), None],
                21 => [Some(1), Some(10), Some(14), None],
                22 => [Some(2), Some(10), Some(15), Some(20)],
                23 => [Some(3), Some(10), Some(16), Some(21)],
                24 => [Some(4), Some(10), Some(17), Some(22)],
                25 => [Some(5), Some(10), Some(18), Some(23)],
                26 => [Some(6), Some(10), Some(19), Some(24)],
                27 => [Some(7), Some(10), None, Some(25)],
                28 => [Some(1), Some(9), Some(15), None],
                29 => [Some(2), Some(9), Some(16), None],
                30 => [Some(3), Some(9), Some(17), Some(20)],
                31 => [Some(4), Some(9), Some(18), Some(21)],
                32 => [Some(5), Some(9), Some(19), Some(22)],
                33 => [Some(6), Some(9), None, Some(23)],
                34 => [Some(7), Some(9), None, Some(24)],
                35 => [Some(1), Some(8), Some(16), None],
                36 => [Some(2), Some(8), Some(17), None],
                37 => [Some(3), Some(8), Some(18), None],
                38 => [Some(4), Some(8), Some(19), Some(20)],
                39 => [Some(5), Some(8), None, Some(21)],
                40 => [Some(6), Some(8), None, Some(22)],
                41 => [Some(7), Some(8), None, Some(23)],
                _ => panic!(Log::print_fatal(&format!(
                    "(Err.113)  Invalid square. / {}",
                    sq
                ))),
            }
        } else {
            [None, None, None, None]
        }
    }

    fn get_feature_state_by_figures(&self, pos: &Position, figures: Vec<u8>) -> u16 {
        let mut sum = 0;
        for figure in figures {
            sum *= 3;
            sum += if let Some(piece) = pos.board[figure as usize] {
                match piece {
                    Piece::Nought => 1,
                    Piece::Cross => 2,
                }
            } else {
                0
            };
        }
        sum
    }
}
