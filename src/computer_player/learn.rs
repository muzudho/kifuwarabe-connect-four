use crate::computer_player::Learning;
use crate::{Engine, EVALUATION_FILE_NAME};

impl Default for Learning {
    fn default() -> Self {
        Learning {}
    }
}
impl Learning {
    pub fn learn(&mut self, engine: &mut Engine) {
        engine.enter("go");

        engine.evaluation.save(EVALUATION_FILE_NAME);
    }
}
