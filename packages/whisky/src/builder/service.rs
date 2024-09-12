use super::{Evaluator, TxBuilder};

impl TxBuilder {
    pub fn set_evaluator(&mut self, evaluator: Box<dyn Evaluator>) -> &mut Self {
        self.evaluator = Some(evaluator);
        self
    }
}
