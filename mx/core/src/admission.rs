//! SMTP-time admission decision (ADR 0001).
//!
//! The milter never lets a message through unscanned. It has exactly four
//! possible answers at end-of-message, and this module decides between them.

use std::time::Duration;

/// Outcome of the inline scan, as seen by the milter at end-of-message.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanOutcome {
    /// All analyzers finished and the policy allows delivery.
    Clean,
    /// The policy decided the message must not be delivered as is.
    Violation,
    /// No verdict within the inline time budget, or an analyzer failed.
    NoVerdict,
}

/// Health of the analysis pipeline at the moment of the decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PipelineState {
    /// How long analyzers have been continuously unavailable; `None` if healthy.
    pub analyzers_down_for: Option<Duration>,
    /// Messages currently in the hold queue waiting for a verdict.
    pub hold_backlog: u64,
}

/// Operator-configurable limits (`outage.*` in the configuration).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OutageLimits {
    pub tempfail_after: Duration,
    pub backlog_threshold: u64,
}

impl Default for OutageLimits {
    fn default() -> Self {
        Self {
            tempfail_after: Duration::from_secs(300),
            backlog_threshold: 1000,
        }
    }
}

/// What the milter tells Postfix.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Admission {
    /// `250`, deliver normally.
    Accept,
    /// `250`, but place the message in Postfix's hold queue for the
    /// quarantine workflow. The message stays in the queue until it is
    /// durably stored.
    HoldForQuarantine,
    /// `250`, hold the message until the asynchronous scan finishes.
    HoldForScan,
    /// `451 4.7.1`, ask the sender to retry later.
    TempFail,
}

/// Decide the SMTP answer for one message.
///
/// A message is accepted for delivery only on a positive `Clean` verdict.
/// Without a verdict it is held, unless the pipeline is degraded beyond the
/// configured limits, in which case the sender is asked to retry so the hold
/// queue cannot grow without bound.
#[must_use]
pub fn decide(outcome: ScanOutcome, state: PipelineState, limits: OutageLimits) -> Admission {
    match outcome {
        ScanOutcome::Clean => Admission::Accept,
        ScanOutcome::Violation => Admission::HoldForQuarantine,
        ScanOutcome::NoVerdict => {
            let down_too_long = state
                .analyzers_down_for
                .is_some_and(|d| d >= limits.tempfail_after);
            let backlog_full = state.hold_backlog >= limits.backlog_threshold;
            if down_too_long || backlog_full {
                Admission::TempFail
            } else {
                Admission::HoldForScan
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const HEALTHY: PipelineState = PipelineState {
        analyzers_down_for: None,
        hold_backlog: 0,
    };

    fn limits() -> OutageLimits {
        OutageLimits::default()
    }

    #[test]
    fn clean_is_accepted() {
        assert_eq!(
            decide(ScanOutcome::Clean, HEALTHY, limits()),
            Admission::Accept
        );
    }

    #[test]
    fn violation_is_held_for_quarantine_even_when_degraded() {
        let degraded = PipelineState {
            analyzers_down_for: Some(Duration::from_secs(3600)),
            hold_backlog: u64::MAX,
        };
        assert_eq!(
            decide(ScanOutcome::Violation, degraded, limits()),
            Admission::HoldForQuarantine
        );
    }

    #[test]
    fn no_verdict_is_held_while_within_limits() {
        let state = PipelineState {
            analyzers_down_for: Some(Duration::from_secs(299)),
            hold_backlog: 999,
        };
        assert_eq!(
            decide(ScanOutcome::NoVerdict, state, limits()),
            Admission::HoldForScan
        );
    }

    #[test]
    fn no_verdict_tempfails_after_long_outage() {
        let state = PipelineState {
            analyzers_down_for: Some(Duration::from_secs(300)),
            hold_backlog: 0,
        };
        assert_eq!(
            decide(ScanOutcome::NoVerdict, state, limits()),
            Admission::TempFail
        );
    }

    #[test]
    fn no_verdict_tempfails_when_backlog_is_full() {
        let state = PipelineState {
            analyzers_down_for: None,
            hold_backlog: 1000,
        };
        assert_eq!(
            decide(ScanOutcome::NoVerdict, state, limits()),
            Admission::TempFail
        );
    }

    /// The central invariant: nothing but a positive clean verdict is ever
    /// delivered, whatever the pipeline state.
    #[test]
    fn only_clean_is_ever_accepted() {
        let states = [
            HEALTHY,
            PipelineState {
                analyzers_down_for: Some(Duration::ZERO),
                hold_backlog: 0,
            },
            PipelineState {
                analyzers_down_for: Some(Duration::from_secs(10_000)),
                hold_backlog: 10_000,
            },
        ];
        for state in states {
            for outcome in [ScanOutcome::Violation, ScanOutcome::NoVerdict] {
                assert_ne!(decide(outcome, state, limits()), Admission::Accept);
            }
        }
    }
}
