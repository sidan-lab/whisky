use std::fmt;

#[derive(Debug, Clone)]
pub struct WError {
    traces: Vec<String>,
}

impl WError {
    pub fn new(error_origin: &str, err: &str) -> Self {
        let msg = format!("[Error - {}]: {}", error_origin, err);
        WError { traces: vec![msg] }
    }

    pub fn add_trace(&mut self, trace: String) {
        self.traces.push(trace);
    }
}

// Implement the Display trait for WError
impl fmt::Display for WError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.traces.join("\n"))
    }
}
