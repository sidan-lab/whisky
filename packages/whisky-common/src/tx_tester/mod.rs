use crate::{Output, TxBuilderBody, TxIn, WError};

mod extra_signatories;
mod inputs;
mod mints;
mod outputs;
mod validity_range;

#[derive(Clone)]
pub struct TxTester {
    pub tx_body: TxBuilderBody,
    pub inputs_evaluating: Vec<TxIn>,
    pub outputs_evaluating: Vec<Output>,
    pub traces: Option<WError>,
}

impl TxTester {
    pub fn new(tx_body: &TxBuilderBody) -> Self {
        TxTester {
            tx_body: tx_body.clone(),
            inputs_evaluating: vec![],
            outputs_evaluating: vec![],
            traces: None,
        }
    }

    pub fn add_trace(&mut self, func_name: &str, message: &str) {
        if let Some(existing_trace) = &mut self.traces {
            let msg = format!("[Error - {}]: {}", func_name, message);
            existing_trace.add_trace(&msg);
        } else {
            self.traces = Some(WError::new(func_name, message));
        }
    }

    pub fn success(&self) -> bool {
        self.traces.is_none()
    }

    pub fn errors(&self) -> String {
        if let Some(traces) = &self.traces {
            format!("{:?}", traces)
        } else {
            "No errors".to_string()
        }
    }
}
