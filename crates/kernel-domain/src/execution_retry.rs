use crate::runtime::RecoveryEligibility;
use crate::{
    AgentRecoveryReference, ExecutionOutcome, ExecutionSessionId, TaskFailurePolicyReference,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionRetryIneligibilityReason {
    NotFailed,
    FailurePolicyMissing,
    RuntimeRecoveryUnavailable,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecutionRetryEligibilityDecision {
    Eligible {
        execution_session_id: ExecutionSessionId,
        task_failure_policy_reference: TaskFailurePolicyReference,
        recovery_reference: AgentRecoveryReference,
    },
    Ineligible(ExecutionRetryIneligibilityReason),
}

impl ExecutionRetryEligibilityDecision {
    pub fn evaluate(
        execution_outcome: &ExecutionOutcome,
        recovery_eligibility: Option<&RecoveryEligibility>,
    ) -> Self {
        let ExecutionOutcome::Failed {
            execution_session,
            task_failure,
            ..
        } = execution_outcome
        else {
            return Self::Ineligible(ExecutionRetryIneligibilityReason::NotFailed);
        };
        let Some(task_failure_policy_reference) =
            task_failure.task_failure_policy_reference().cloned()
        else {
            return Self::Ineligible(ExecutionRetryIneligibilityReason::FailurePolicyMissing);
        };
        let Some(recovery_eligibility) = recovery_eligibility else {
            return Self::Ineligible(ExecutionRetryIneligibilityReason::RuntimeRecoveryUnavailable);
        };
        let Some(recovery_reference) = recovery_eligibility.recovery_reference().cloned() else {
            return Self::Ineligible(ExecutionRetryIneligibilityReason::RuntimeRecoveryUnavailable);
        };
        if !recovery_eligibility.eligible() {
            return Self::Ineligible(ExecutionRetryIneligibilityReason::RuntimeRecoveryUnavailable);
        }
        Self::Eligible {
            execution_session_id: execution_session.execution_session_id().clone(),
            task_failure_policy_reference,
            recovery_reference,
        }
    }
}
