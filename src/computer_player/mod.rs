pub mod evaluation_file;
pub mod evaluation_model;
mod learn;
pub mod search;

/// Nought and cross.
/// 先後。
pub const NOUGHT_AND_CROSS_LEN: usize = 2;

/// Win and draw.
/// 勝ちと負け。
pub const WIN_AND_DRAW_LEN: usize = 2;

/// 3^4
pub const N3POW4: usize = 81;

/// 3^5
pub const N3POW5: usize = 243;

/// 3^6
pub const N3POW6: usize = 729;

/// 3^7
pub const N3POW7: usize = 2187;

/// Evaluation.
/// 評価値。
pub struct Evaluation {
    // Win and draw value.
    // 勝ち評価値と、引き分け評価値。
    pub features_1_to_7: [[[[u8; NOUGHT_AND_CROSS_LEN]; WIN_AND_DRAW_LEN]; N3POW6]; 7],
    pub features_8_to_13: [[[[u8; NOUGHT_AND_CROSS_LEN]; WIN_AND_DRAW_LEN]; N3POW7]; 6],
    pub features_14_19_20_25: [[[[u8; NOUGHT_AND_CROSS_LEN]; WIN_AND_DRAW_LEN]; N3POW4]; 4],
    pub features_15_18_21_24: [[[[u8; NOUGHT_AND_CROSS_LEN]; WIN_AND_DRAW_LEN]; N3POW5]; 4],
    pub features_16_17_22_23: [[[[u8; NOUGHT_AND_CROSS_LEN]; WIN_AND_DRAW_LEN]; N3POW6]; 4],
}

pub struct Learning {
    evaluation: Evaluation,
}
