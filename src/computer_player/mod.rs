pub mod evaluation_file;
pub mod evaluation_model;
pub mod search;

use evaluation_model::{N3POW4, N3POW5, N3POW6, N3POW7, WIN_AND_DRAW_LEN};

/// Evaluation.
/// 評価値。
pub struct Evaluation {
    // Win and draw value.
    // 勝ち評価値と、引き分け評価値。
    pub features_1_to_7: [[[u8; WIN_AND_DRAW_LEN]; N3POW6]; 7],
    pub features_8_to_13: [[[u8; WIN_AND_DRAW_LEN]; N3POW7]; 6],
    pub features_14_19_20_25: [[[u8; WIN_AND_DRAW_LEN]; N3POW4]; 4],
    pub features_15_18_21_24: [[[u8; WIN_AND_DRAW_LEN]; N3POW5]; 4],
    pub features_16_17_22_23: [[[u8; WIN_AND_DRAW_LEN]; N3POW6]; 4],
}
