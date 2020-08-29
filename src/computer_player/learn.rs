use crate::computer_player::{Learning, Search};
use crate::Engine;

impl Default for Learning {
    fn default() -> Self {
        Learning {}
    }
}
impl Learning {
    pub fn learn(&mut self, engine: &mut Engine) {
        /*
        self.search.evaluation.load("evaluation.csv");
        self.search.start_pieces_num = engine.pos.pieces_num;

        self.search.evaluation.save("evaluation.csv");
        */
    }
}
