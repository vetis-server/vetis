use crate::{
    Request, Response, VetisFutureResult, VetisHook,
    matcher::{Matcher, MatcherConfig},
};
use cel::Context;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize, PartialOrd, Ord)]
/// HookPhase is used to tell on which request phase this code
/// will run.
/// The variants appear in the order they act in a request lifecycle,
/// see description for each enum variant for more details.
pub enum HookPhase {
    /// PreProcess phase happens at very beggining of request lifecycle
    /// here you can do caching
    PreProcess,
    /// Translate phase happens before any routing, where you can change
    /// request contents to forward it to another location, for example.
    Rewrite,
    /// Access phase allows you do any authentication and authorization,
    /// Hooks for basic_auth and others will act here.
    Access,
    /// Content phase happens eventually after routing, at this phase
    /// you can redirect, proxy, serve static content or forward to a
    /// CGI server.
    Content,
    /// PostContent phase makes possible manage responses output,
    /// Hooks for content compression goes here.
    PostContent,
}

/// Trait for handling different types of hooks for a host
pub trait Hook {
    /// Setup a hook
    ///
    /// # Return
    ///
    /// * `VetisFutureResult<'a, ()>` - A result of hook setup
    fn setup<'a>(&'a mut self) -> VetisFutureResult<'a, ()>;

    /// Check if there's a match within this uri
    ///
    /// # Parameters
    ///
    /// * `context` - A expression language context for request evaluation.
    ///
    /// # Return
    ///
    /// * `bool` - True if there's a match
    fn has_matches(&self, context: &Context) -> bool;
    /// Returns phase on which hook will run
    ///
    /// # Returns
    ///
    /// * `HookPhase` - Hook phases
    fn phase(&self) -> HookPhase;
    /// Returns a mutable hook
    ///
    /// # Returns
    ///
    /// * `&str` - The URI of path
    fn matcher_mut(&mut self) -> Option<&mut Box<dyn Matcher>>;
    /// Handle a Hook
    ///
    /// # Argumetns
    ///
    /// * `request` - Incomding request
    /// * `response` - Optional procuded response by previous hook
    ///
    /// # Returns
    ///
    /// * `VetisFutureResult<'a, (Request, Option<Response>)>` - A tuple with request and optional response
    fn handle<'a>(
        &'a self,
        request: Request,
        response: Option<Response>,
    ) -> VetisFutureResult<'a, (Request, Option<Response>)>;
}

#[typetag::serde(tag = "name")]
/// A trait which describe Hook configuration
pub trait HookConfig {
    /// Returns a matcher
    ///
    /// # Returns
    ///
    /// * `Option<&String>` - Matcher for this hook
    fn matcher(&self) -> Option<&Box<dyn MatcherConfig>>;
    /// Allow clone any type which implements
    /// HookConfig
    ///
    /// # Returns
    ///
    /// * `Box<dyn HookConfig>` - Clone of boxed Hook config
    fn boxed_clone(&self) -> Box<dyn HookConfig>;
    /// Returns a boxed Hook out of it
    ///
    /// # Returns
    ///
    /// * `Box<dyn Hook>` - A boxed path
    fn boxed_hook<'a>(&'a self) -> Box<dyn VetisHook>;
}

impl Clone for Box<dyn HookConfig> {
    fn clone(&self) -> Self {
        self.boxed_clone()
    }
}
