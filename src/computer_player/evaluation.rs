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
    pub fn ways_weight() -> [u16; 8] {
        [105, 105, 105, 105, 105, 105, 105, 105]
    }
}
