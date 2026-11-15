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
    /// Content phase happens eventually after routing, at this phase
    /// you can redirect, proxy, serve static content or forward to a
    /// CGI server.
    Content,
    /// Access phase allows you do any authentication and authorization,
    /// Hooks for basic_auth and others will act here.
    Access,
    /// PostContent phase makes possible manage responses output,
    /// Hooks for content compression goes here.
    PostContent,
}

/// Trait for handling different types of paths in the server
pub trait Hook {
    /// Check if there's a match within this uri
    ///
    /// # Return
    ///
    /// * `bool` - True if there's a match
    fn has_matches(&self, context: &Context) -> bool;
    /// Returns the URI of path
    ///
    /// # Returns
    ///
    /// * `&str` - The URI of path
    fn phase(&self) -> &HookPhase;
    /// Returns a mutable hook
    ///
    /// # Returns
    ///
    /// * `&str` - The URI of path
    fn matcher_mut(&mut self) -> Option<&mut Box<dyn Matcher>>;
    /// Return as preprocess Hook
    ///
    /// # Returns
    ///
    /// * `Option<&dyn PreProcessHook>` - A preprocess Hook
    fn handle<'a>(
        &'a self,
        request: Request,
        response: Option<Response>,
    ) -> VetisFutureResult<'a, (Request, Option<Response>)>;
}

impl Ord for Box<dyn VetisHook> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.phase()
            .cmp(other.phase())
    }
}

impl PartialOrd for Box<dyn VetisHook> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.phase()
            .partial_cmp(other.phase())
    }
}

impl Eq for Box<dyn VetisHook> {}

impl PartialEq for Box<dyn VetisHook> {
    fn eq(&self, other: &Self) -> bool {
        self.phase()
            .eq(other.phase())
    }
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
    fn boxed_hook(&self) -> Box<dyn VetisHook>;
}

impl Clone for Box<dyn HookConfig> {
    fn clone(&self) -> Self {
        self.boxed_clone()
    }
}
