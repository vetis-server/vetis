use caramelo::{expect, matchers::truthy};

use crate::hook::{HookPhase, HookPhases};

#[test]
fn test_hook_phases() {
    let phases = HookPhase::Access | HookPhase::PostContent;
    expect(phases.contains(HookPhase::Access)).to_be(truthy());
}

#[test]
fn test_new_phases() {
    let phases = HookPhases::new(HookPhase::PostContent);
    expect(phases.contains(HookPhase::PostContent)).to_be(truthy());
}

#[test]
fn test_iterator_hook_phases() {
    let phases = HookPhase::Access | HookPhase::PostContent;
    for phase in phases {
        expect(phases.contains(phase)).to_be(truthy());
    }
}

#[test]
fn test_iter_hook_phases() {
    let phases = HookPhase::Access | HookPhase::PostContent;
    for phase in phases.iter() {
        expect(phases.contains(phase)).to_be(truthy());
    }
}
