#[typetag::serde(tag = "dest")]
/// A trait which describe log configuration
pub trait TelemetryConfig {
    /// Allow clone any type which implements
    /// LogConfig
    ///
    /// # Returns
    ///
    /// - `Box<dyn PathConfig>` - Clone of boxed log config
    fn boxed_clone(&self) -> Box<dyn TelemetryConfig>;
}

impl Clone for Box<dyn TelemetryConfig> {
    fn clone(&self) -> Self {
        self.boxed_clone()
    }
}
