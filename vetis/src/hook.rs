use std::ops::{BitAnd, BitOr};

use crate::{
    Request, Response, VetisFutureResult, VetisHook,
    matcher::{Matcher, MatcherConfig},
};
use cel::Context;
use serde::{Deserialize, Serialize};

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize, PartialOrd, Ord)]
/// HookPhase is used to tell on which request phase this code
/// will run.
/// The variants appear in the order they act in a request lifecycle,
/// see description for each enum variant for more details.
pub enum HookPhase {
    /// PreProcess phase happens at very beggining of request lifecycle
    /// here you can do caching
    PreProcess = 1 << 0,
    /// Translate phase happens before any routing, where you can change
    /// request contents to forward it to another location, for example.
    Rewrite = 1 << 1,
    /// Access phase allows you do any authentication and authorization,
    /// Hooks for basic_auth and others will act here.
    Access = 1 << 2,
    /// Content phase happens eventually after routing, at this phase
    /// you can redirect, proxy, serve static content or forward to a
    /// CGI server.
    Content = 1 << 3,
    /// PostContent phase makes possible manage responses output,
    /// Hooks for content compression goes here.
    PostContent = 1 << 4,
}

impl HookPhase {
    const ALL: [HookPhase; 5] = [
        HookPhase::PreProcess,
        HookPhase::Rewrite,
        HookPhase::Access,
        HookPhase::Content,
        HookPhase::PostContent,
    ];
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
/// Hold hook phases flags
pub struct HookPhases(u8);

impl HookPhases {
    /// Create new hook phases
    pub fn new(flags: HookPhase) -> Self {
        Self(flags as u8)
    }

    /// Check if this set contains a variant
    pub fn contains(self, flag: HookPhase) -> bool {
        (self.0 & (flag as u8)) != 0
    }

    /// Allow iterated over hook phases
    pub fn iter(self) -> impl Iterator<Item = HookPhase> {
        HookPhase::ALL
            .into_iter()
            .filter(move |&flag| self.contains(flag))
    }
}

impl BitOr for HookPhase {
    type Output = HookPhases;
    fn bitor(self, rhs: Self) -> HookPhases {
        HookPhases((self as u8) | (rhs as u8))
    }
}

impl BitOr<HookPhase> for HookPhases {
    type Output = HookPhases;
    fn bitor(self, rhs: HookPhase) -> HookPhases {
        HookPhases(self.0 | (rhs as u8))
    }
}

impl BitAnd<HookPhase> for HookPhases {
    type Output = HookPhases;
    fn bitand(self, rhs: HookPhase) -> HookPhases {
        HookPhases(self.0 & (rhs as u8))
    }
}

impl IntoIterator for HookPhases {
    type Item = HookPhase;
    type IntoIter = HookPhasesIter;

    fn into_iter(self) -> Self::IntoIter {
        HookPhasesIter { set: self, index: 0 }
    }
}

/// Allow iterate over hook phases
pub struct HookPhasesIter {
    set: HookPhases,
    index: usize,
}

impl Iterator for HookPhasesIter {
    type Item = HookPhase;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < HookPhase::ALL.len() {
            let flag = HookPhase::ALL[self.index];
            self.index += 1;
            if self
                .set
                .contains(flag)
            {
                return Some(flag);
            }
        }
        None
    }
}

/// Trait for handling different types of paths in the server
pub trait Hook {
    /// Check if there's a match within this uri
    ///
    /// # Return
    ///
    /// * `bool` - True if there's a match
    fn has_matches(&self, context: &Context) -> bool;
    /// Returns phases on which hook will run
    ///
    /// # Returns
    ///
    /// * `&[HookPhase]` - Hook phases
    fn phases(&self) -> HookPhases;
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
