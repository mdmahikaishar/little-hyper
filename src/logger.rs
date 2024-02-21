use crate::Request;

/// Logger
/// 
/// 
#[derive(Default)]
pub struct Logger {
    log: bool,
}

impl Logger {
    pub fn new(log: bool) -> Self {
        Self { log }
    }

    pub fn set(&mut self, log: bool) {
        self.log = log;
    }

    /// Request
    /// 
    /// Log the request
    pub fn request(&self, request: &Request) {
        if !self.log {
            return;
        }

        println!(
            "{method:<6} {path}",
            method = request.method,
            path = request.path
        );
    }
}
