use std::sync::Arc;

use cel::{Context, Env};

use crate::VetisResult;

/// Trait for handling different types of matchers in the server.
/// Matcher is entirely based on CEL for its evaluation, let's take
/// a CEL context with functions and variables as argment.
pub trait Matcher {
    /// Compile matcher for later use
    fn compile(&mut self, engine: Arc<Env>) -> VetisResult<()>;
    /// Test if a request matches with this Matcher logic
    fn is_match(&self, context: &Context) -> bool;
}

#[typetag::serde(tag = "type")]
/// A trait which matcher configuration
pub trait MatcherConfig {
    /// Allow clone any type which implements
    /// MatcherConfig
    ///
    /// # Returns
    ///
    /// * `Box<dyn MatcherConfig>` - Clone of boxed matcher config
    fn boxed_clone(&self) -> Box<dyn MatcherConfig>;
    /// Returns a boxed matcher out of it
    ///
    /// # Returns
    ///
    /// * `Box<dyn Matcher>` - A boxed path
    fn boxed_matcher(&self) -> Box<dyn Matcher>;
}

impl Clone for Box<dyn MatcherConfig> {
    fn clone(&self) -> Self {
        self.boxed_clone()
    }
}
