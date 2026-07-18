use crate::execution_test_support::{accepted_failure, execution_session, recovery_eligibility};
use crate::{
    ExecutionOutcome, ExecutionRetryEligibilityDecision, ExecutionRetryIneligibilityReason,
};

#[test]
fn execution_retry_is_eligible_for_failed_session_with_policy_and_recovery_k8_006() {
    let outcome = ExecutionOutcome::failed(
        execution_session(),
        accepted_failure(),
        crate::request::TimeReference::new("2026-07-18T00:33:00Z").expect("time"),
    )
    .expect("failed");
    let decision =
        ExecutionRetryEligibilityDecision::evaluate(&outcome, Some(&recovery_eligibility()));
    match decision {
        ExecutionRetryEligibilityDecision::Eligible { .. } => {}
        ExecutionRetryEligibilityDecision::Ineligible(reason) => {
            panic!("expected eligible retry, got {reason:?}")
        }
    }
}

#[test]
fn execution_retry_is_ineligible_when_outcome_is_not_failed_k8_006() {
    let outcome = ExecutionOutcome::terminated(
        execution_session(),
        crate::ExecutionTermination::Cancelled,
        crate::execution_test_support::transition_reason(),
        crate::request::TimeReference::new("2026-07-18T00:34:00Z").expect("time"),
    )
    .expect("terminated");
    let decision =
        ExecutionRetryEligibilityDecision::evaluate(&outcome, Some(&recovery_eligibility()));
    assert_eq!(
        decision,
        ExecutionRetryEligibilityDecision::Ineligible(ExecutionRetryIneligibilityReason::NotFailed)
    );
}

#[test]
fn execution_retry_is_ineligible_without_recovery_eligibility_k8_006() {
    let outcome = ExecutionOutcome::failed(
        execution_session(),
        accepted_failure(),
        crate::request::TimeReference::new("2026-07-18T00:35:00Z").expect("time"),
    )
    .expect("failed");
    let decision = ExecutionRetryEligibilityDecision::evaluate(&outcome, None);
    assert_eq!(
        decision,
        ExecutionRetryEligibilityDecision::Ineligible(
            ExecutionRetryIneligibilityReason::RuntimeRecoveryUnavailable
        )
    );
}
