use crate::look_and_model::Position;

/// Evaluation.
/// 評価値。
pub struct Evaluation {}
impl Evaluation {
    /// Returns the probability of a move.  
    /// Weight is a per 840.
    /// The least common multiple of 1 to 8 is 840.
    /// 指し手の確率を返します。  
    /// 840分率です。
    /// 1 から 8 の最小公倍数は 840。
    ///
    /// [a, b, c, d, e, f, g, resign]
    pub fn ways_weight(pos: &Position) -> [u16; 8] {
        // 25の特徴の状態を調べます。
        let features25 = Evaluation::get_25_features(pos);

        // マスの特徴量を求めます。
        // 7つの指し手のマスを調べます。
        let way_values = [
            Evaluation::get_value_by_sq(pos, pos.fallen_sq_or_none('a')),
            Evaluation::get_value_by_sq(pos, pos.fallen_sq_or_none('b')),
            Evaluation::get_value_by_sq(pos, pos.fallen_sq_or_none('c')),
            Evaluation::get_value_by_sq(pos, pos.fallen_sq_or_none('d')),
            Evaluation::get_value_by_sq(pos, pos.fallen_sq_or_none('e')),
            Evaluation::get_value_by_sq(pos, pos.fallen_sq_or_none('f')),
            Evaluation::get_value_by_sq(pos, pos.fallen_sq_or_none('g')),
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

    pub fn get_value_by_sq(pos: &Position, sq: Option<usize>) -> u16 {
        let mut sum = 0;
        for feature in &Evaluation::get_features_by_sq(pos, sq) {
            sum += Evaluation::get_value_by_feature(pos, *feature);
        }

        105 // TODO sum
    }

    pub fn get_value_by_feature(pos: &Position, feature: Option<u8>) -> u16 {
        0
    }

    pub fn get_features_by_sq(pos: &Position, sq: Option<usize>) -> [Option<u8>; 4] {
        [None, None, None, None]
    }

    pub fn get_25_features(pos: &Position) -> [u16; 25] {
        [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]
    }
}
