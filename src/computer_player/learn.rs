use crate::computer_player::{Evaluation, Learning};
use crate::Engine;

impl Default for Learning {
    fn default() -> Self {
        Learning {
            evaluation: Evaluation::default(),
        }
    }
}
impl Learning {
    pub fn learn(&mut self, engine: &mut Engine) {
        self.evaluation.load("evaluation.csv");

        self.evaluation.save("evaluation.csv");
    }
}
