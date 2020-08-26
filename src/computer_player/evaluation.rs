use crate::log::LogExt;
use crate::look_and_model::Position;
use casual_logger::Log;

/// Evaluation.
/// 評価値。
pub struct Evaluation {}
impl Default for Evaluation {
    fn default() -> Self {
        Evaluation {}
    }
}
impl Evaluation {
    /// Returns the probability of a move.  
    /// Weight is a per 840.
    /// The least common multiple of 1 to 8 is 840.
    /// 指し手の確率を返します。  
    /// 840分率です。
    /// 1 から 8 の最小公倍数は 840。
    ///
    /// [a, b, c, d, e, f, g, resign]
    pub fn ways_weight(&self, pos: &Position) -> [u16; 8] {
        // 25の特徴の状態を調べます。
        let features25 = self.get_25_features(pos);

        // マスの特徴量を求めます。
        // 7つの指し手のマスを調べます。
        let way_values = [
            self.get_value_by_sq(pos, pos.fallen_sq_or_none('a')),
            self.get_value_by_sq(pos, pos.fallen_sq_or_none('b')),
            self.get_value_by_sq(pos, pos.fallen_sq_or_none('c')),
            self.get_value_by_sq(pos, pos.fallen_sq_or_none('d')),
            self.get_value_by_sq(pos, pos.fallen_sq_or_none('e')),
            self.get_value_by_sq(pos, pos.fallen_sq_or_none('f')),
            self.get_value_by_sq(pos, pos.fallen_sq_or_none('g')),
        ];

        // 投了の評価値を求めます。
        let resign_value = 840
            - way_values[0]
            - way_values[1]
            - way_values[2]
            - way_values[3]
            - way_values[4]
            - way_values[5]
            - way_values[6];
        [
            way_values[0],
            way_values[1],
            way_values[2],
            way_values[3],
            way_values[4],
            way_values[5],
            way_values[6],
            resign_value,
        ]
    }

    pub fn get_value_by_sq(&self, pos: &Position, sq: Option<usize>) -> u16 {
        let mut sum = 0;
        for feature in &self.get_elemental_features_by_sq(sq) {
            sum += self.get_value_by_feature(pos, *feature);
        }

        105 // TODO sum
    }

    pub fn get_value_by_feature(&self, pos: &Position, feature: Option<u8>) -> u16 {
        if let Some(feature) = feature {
            let state = self.get_state_by_feature(feature);
            0 // TODO
        } else {
            0
        }
    }

    pub fn get_state_by_feature(&self, feature: u8) -> u16 {
        0 // TODO
    }

    /// Elemental features of the square.
    /// そのマスの成分特徴。
    pub fn get_elemental_features_by_sq(&self, sq: Option<usize>) -> [Option<u8>; 4] {
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

    pub fn get_25_features(&self, pos: &Position) -> [u16; 25] {
        // TODO
        [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]
    }
}
