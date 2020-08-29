use crate::computer_player::{Evaluation, Learning, Search};
use crate::Engine;

impl Default for Learning {
    fn default() -> Self {
        Learning {
            search: Search::default(),
        }
    }
}
impl Learning {
    pub fn learn(&mut self, engine: &mut Engine) {
        self.search.evaluation.load("evaluation.csv");

        self.search.evaluation.save("evaluation.csv");
    }
}
