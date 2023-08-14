pub struct Logger {

}

impl Logger {
    pub fn init() -> crate::prelude::Result<Self> {
        Ok(Logger {})
    }

    pub fn stop(self) {
        // Consume self to drop
    }
}

impl Drop for Logger {
    fn drop(&mut self) {
        // TODO: Shutdown anything that is running e.g. Traces for Open Telemetry
    }
}