use std::fmt;

#[derive(Clone)]
pub struct WError {
    traces: Vec<String>,
}

impl WError {
    pub fn new(error_origin: &str, err: &str) -> Self {
        let msg = format!("[Error - {}]: {}", error_origin, err);
        WError { traces: vec![msg] }
    }

    pub fn add_trace(&mut self, trace: &str) {
        self.traces.push(trace.to_string());
    }

    pub fn from_opt(error_origin: &'static str, err: &'static str) -> impl FnOnce() -> WError {
        move || WError::new(error_origin, err)
    }

    pub fn from_err<F>(error_origin: &'static str) -> impl FnOnce(F) -> WError
    where
        F: std::fmt::Debug + 'static,
    {
        move |err| {
            if let Some(werror) = (&err as &dyn std::any::Any).downcast_ref::<WError>() {
                let mut werror = werror.clone();
                werror.add_trace(error_origin);
                werror
            } else {
                WError::new(error_origin, &format!("{:?}", err))
            }
        }
    }

    pub fn add_err_trace(error_origin: &'static str) -> impl FnOnce(WError) -> WError {
        move |mut werror| {
            werror.add_trace(error_origin);
            werror
        }
    }
}

// Implement the Display trait for WError
impl fmt::Display for WError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.traces.join("\n"))
    }
}

// Implement the Debug trait for WError
impl fmt::Debug for WError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.traces.join("\n"))
    }
}
