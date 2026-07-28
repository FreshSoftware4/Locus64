#![forbid(unsafe_code)]

use l64_certification::{BurdenCounts, CertificationVerdict};
use l64_native::{DnaError, StateSymbol};
use l64_observation::{AuthorityObservation, ObservationError, observe_dna};
use l64_transport::{BundleError, decode_bundle};
use std::fmt::{self, Write as _};

pub const CHANGE_VERSION: u16 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContactChange {
    Unchanged,
    Changed,
    Added,
    Removed,
}

impl ContactChange {
    fn label(self) -> &'static str {
        match self {
            Self::Unchanged => "UNCHANGED",
            Self::Changed => "CHANGED",
            Self::Added => "ADDED",
            Self::Removed => "REMOVED",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SectionChanges {
    pub nodes: bool,
    pub ports: bool,
    pub contexts: bool,
    pub routes: bool,
}

impl SectionChanges {
    fn between(before: StateSymbol, after: StateSymbol) -> Self {
        let [nodes, ports, contexts, routes] = before.changed_sections(after);
        Self {
            nodes,
            ports,
            contexts,
            routes,
        }
    }

    pub fn any(self) -> bool {
        self.nodes || self.ports || self.contexts || self.routes
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CountMovement {
    pub before: usize,
    pub after: usize,
}

impl CountMovement {
    pub fn delta(self) -> i128 {
        self.after as i128 - self.before as i128
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextChange {
    pub context: usize,
    pub contact: ContactChange,
    pub before_verdict: Option<CertificationVerdict>,
    pub after_verdict: Option<CertificationVerdict>,
    pub before_burdens: Option<BurdenCounts>,
    pub after_burdens: Option<BurdenCounts>,
    pub closed: CountMovement,
    pub open: CountMovement,
    pub invalid: CountMovement,
    pub replay_steps: CountMovement,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorityChange {
    pub exact_equal: bool,
    pub before_symbol: StateSymbol,
    pub after_symbol: StateSymbol,
    pub sections: SectionChanges,
    pub nodes: CountMovement,
    pub contexts: CountMovement,
    pub journal: CountMovement,
    pub before_verdict: CertificationVerdict,
    pub after_verdict: CertificationVerdict,
    pub context_changes: Vec<ContextChange>,
}

impl AuthorityChange {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(out, "L64 NATIVE AUTHORITY CHANGE v{CHANGE_VERSION}");
        self.render_into(&mut out, "");
        out
    }

    pub fn render_into(&self, out: &mut String, prefix: &str) {
        render_authority(out, self, prefix);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BundleMemberChange {
    pub index: usize,
    pub contact: ContactChange,
    pub authority: Option<AuthorityChange>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BundleChange {
    pub before_members: usize,
    pub after_members: usize,
    pub members: Vec<BundleMemberChange>,
}

impl BundleChange {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(out, "L64 NATIVE BUNDLE CHANGE v{CHANGE_VERSION}");
        let _ = writeln!(out, "transport=canonical_l64b");
        let _ = writeln!(out, "ordering=transport_sequence");
        let _ = writeln!(out, "before_members={}", self.before_members);
        let _ = writeln!(out, "after_members={}", self.after_members);
        let _ = writeln!(out, "composite_authority=none");
        let _ = writeln!(out, "composite_change_verdict=none");
        for member in &self.members {
            let prefix = format!("member.{}.", member.index);
            let _ = writeln!(out, "{prefix}contact={}", member.contact.label());
            if let Some(authority) = &member.authority {
                render_authority(&mut out, authority, &prefix);
            }
        }
        out
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangeError {
    Dna(DnaError),
    Observation(ObservationError),
    Bundle(BundleError),
}

impl fmt::Display for ChangeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Dna(error) => write!(f, "DNA comparison failed: {error}"),
            Self::Observation(error) => write!(f, "authority comparison failed: {error}"),
            Self::Bundle(error) => write!(f, "bundle comparison failed: {error}"),
        }
    }
}

impl std::error::Error for ChangeError {}

pub fn compare_dna(before: &[u8], after: &[u8]) -> Result<AuthorityChange, ChangeError> {
    let before_observation = observe_dna(before).map_err(ChangeError::Observation)?;
    let after_observation = observe_dna(after).map_err(ChangeError::Observation)?;
    Ok(compare_observations(
        before == after,
        &before_observation,
        &after_observation,
    ))
}

pub fn compare_bundle(before: &[u8], after: &[u8]) -> Result<BundleChange, ChangeError> {
    let before_bundle = decode_bundle(before).map_err(ChangeError::Bundle)?;
    let after_bundle = decode_bundle(after).map_err(ChangeError::Bundle)?;
    let width = before_bundle.len().max(after_bundle.len());
    let mut members = Vec::with_capacity(width);
    for index in 0..width {
        let before_member = before_bundle.members().get(index);
        let after_member = after_bundle.members().get(index);
        let (contact, authority) = match (before_member, after_member) {
            (Some(before_member), Some(after_member)) => {
                let before_observation =
                    observe_dna(before_member.dna()).map_err(ChangeError::Observation)?;
                let after_observation =
                    observe_dna(after_member.dna()).map_err(ChangeError::Observation)?;
                let authority = compare_observations(
                    before_member.dna() == after_member.dna(),
                    &before_observation,
                    &after_observation,
                );
                let contact = if authority.exact_equal {
                    ContactChange::Unchanged
                } else {
                    ContactChange::Changed
                };
                (contact, Some(authority))
            }
            (None, Some(_)) => (ContactChange::Added, None),
            (Some(_), None) => (ContactChange::Removed, None),
            (None, None) => unreachable!(),
        };
        members.push(BundleMemberChange {
            index,
            contact,
            authority,
        });
    }
    Ok(BundleChange {
        before_members: before_bundle.len(),
        after_members: after_bundle.len(),
        members,
    })
}

fn compare_observations(
    exact_equal: bool,
    before: &AuthorityObservation,
    after: &AuthorityObservation,
) -> AuthorityChange {
    let width = before.contexts.len().max(after.contexts.len());
    let mut context_changes = Vec::with_capacity(width);
    for context in 0..width {
        let before_context = before.contexts.get(context);
        let after_context = after.contexts.get(context);
        let contact = match (before_context, after_context) {
            (Some(left), Some(right)) if left == right => ContactChange::Unchanged,
            (Some(_), Some(_)) => ContactChange::Changed,
            (None, Some(_)) => ContactChange::Added,
            (Some(_), None) => ContactChange::Removed,
            (None, None) => unreachable!(),
        };
        context_changes.push(ContextChange {
            context,
            contact,
            before_verdict: before_context.map(|item| item.certification.verdict),
            after_verdict: after_context.map(|item| item.certification.verdict),
            before_burdens: before_context.map(|item| item.certification.burdens),
            after_burdens: after_context.map(|item| item.certification.burdens),
            closed: movement(
                before_context.map_or(0, |item| item.report.closed),
                after_context.map_or(0, |item| item.report.closed),
            ),
            open: movement(
                before_context.map_or(0, |item| item.report.open),
                after_context.map_or(0, |item| item.report.open),
            ),
            invalid: movement(
                before_context.map_or(0, |item| item.report.invalid),
                after_context.map_or(0, |item| item.report.invalid),
            ),
            replay_steps: movement(
                before_context.map_or(0, |item| item.replay.steps.len()),
                after_context.map_or(0, |item| item.replay.steps.len()),
            ),
        });
    }
    let before_certification = &before.certification;
    let after_certification = &after.certification;
    AuthorityChange {
        exact_equal,
        before_symbol: before_certification.symbol,
        after_symbol: after_certification.symbol,
        sections: SectionChanges::between(before_certification.symbol, after_certification.symbol),
        nodes: movement(
            before_certification.node_count,
            after_certification.node_count,
        ),
        contexts: movement(
            before_certification.context_count,
            after_certification.context_count,
        ),
        journal: movement(
            before_certification.journal_len,
            after_certification.journal_len,
        ),
        before_verdict: before_certification.verdict,
        after_verdict: after_certification.verdict,
        context_changes,
    }
}

fn movement(before: usize, after: usize) -> CountMovement {
    CountMovement { before, after }
}

fn render_authority(out: &mut String, change: &AuthorityChange, prefix: &str) {
    let _ = writeln!(out, "{prefix}scope=exact_native_authority_change");
    let _ = writeln!(out, "{prefix}before_authority=exact_canonical_l64d");
    let _ = writeln!(out, "{prefix}after_authority=exact_canonical_l64d");
    let _ = writeln!(out, "{prefix}exact_equal={}", change.exact_equal);
    let _ = writeln!(out, "{prefix}symbol_authority=non_authoritative");
    let _ = writeln!(out, "{prefix}before_symbol={}", change.before_symbol);
    let _ = writeln!(out, "{prefix}after_symbol={}", change.after_symbol);
    let _ = writeln!(
        out,
        "{prefix}section.nodes.changed={}",
        change.sections.nodes
    );
    let _ = writeln!(
        out,
        "{prefix}section.ports.changed={}",
        change.sections.ports
    );
    let _ = writeln!(
        out,
        "{prefix}section.contexts.changed={}",
        change.sections.contexts
    );
    let _ = writeln!(
        out,
        "{prefix}section.routes.changed={}",
        change.sections.routes
    );
    render_movement(out, prefix, "nodes", change.nodes);
    render_movement(out, prefix, "contexts", change.contexts);
    render_movement(out, prefix, "journal", change.journal);
    let _ = writeln!(out, "{prefix}before_verdict={}", change.before_verdict);
    let _ = writeln!(out, "{prefix}after_verdict={}", change.after_verdict);
    for context in &change.context_changes {
        let context_prefix = format!("{prefix}context.{}.", context.context);
        let _ = writeln!(out, "{context_prefix}contact={}", context.contact.label());
        let _ = writeln!(
            out,
            "{context_prefix}before_verdict={}",
            optional_verdict(context.before_verdict)
        );
        let _ = writeln!(
            out,
            "{context_prefix}after_verdict={}",
            optional_verdict(context.after_verdict)
        );
        render_movement(out, &context_prefix, "closed", context.closed);
        render_movement(out, &context_prefix, "open", context.open);
        render_movement(out, &context_prefix, "invalid", context.invalid);
        render_movement(out, &context_prefix, "replay_steps", context.replay_steps);
    }
}

fn render_movement(out: &mut String, prefix: &str, label: &str, value: CountMovement) {
    let _ = writeln!(out, "{prefix}{label}.before={}", value.before);
    let _ = writeln!(out, "{prefix}{label}.after={}", value.after);
    let _ = writeln!(out, "{prefix}{label}.delta={:+}", value.delta());
}

fn optional_verdict(value: Option<CertificationVerdict>) -> String {
    value
        .map(|verdict| verdict.to_string())
        .unwrap_or_else(|| "NONE".to_string())
}
