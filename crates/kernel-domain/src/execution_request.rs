use crate::authorization::AuthorizationDecisionReference;
use crate::errors::{DomainError, DomainResult};
use crate::request::TimeReference;
use crate::{
    ExecutionSessionId, TaskInstanceReference, TaskReadinessDecision, TaskState, TaskStateSnapshot,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionRequest {
    execution_session_id: ExecutionSessionId,
    task_instance_reference: TaskInstanceReference,
    task_state_snapshot: TaskStateSnapshot,
    task_readiness_decision: TaskReadinessDecision,
    authorization_decision_reference: AuthorizationDecisionReference,
    requested_at: TimeReference,
}

impl ExecutionRequest {
    pub fn new(
        execution_session_id: ExecutionSessionId,
        task_instance_reference: TaskInstanceReference,
        task_state_snapshot: TaskStateSnapshot,
        task_readiness_decision: TaskReadinessDecision,
        authorization_decision_reference: AuthorizationDecisionReference,
        requested_at: TimeReference,
    ) -> DomainResult<Self> {
        if task_state_snapshot.task_instance_reference() != &task_instance_reference {
            return Err(DomainError::InvalidExecution(
                "execution request task state snapshot must match the requested task instance",
            ));
        }
        if task_state_snapshot.task_state() != TaskState::InProgress {
            return Err(DomainError::InvalidExecution(
                "execution request requires an InProgress task state snapshot",
            ));
        }
        match &task_readiness_decision {
            TaskReadinessDecision::Ready(ready)
                if ready.task_instance_reference() == &task_instance_reference => {}
            TaskReadinessDecision::Ready(_) => {
                return Err(DomainError::InvalidExecution(
                    "execution request readiness decision must match the requested task instance",
                ));
            }
            TaskReadinessDecision::Blocked(_) | TaskReadinessDecision::Rejected(_) => {
                return Err(DomainError::InvalidExecution(
                    "execution request requires a ready task readiness decision",
                ));
            }
        }
        if authorization_decision_reference.outcome().is_denied() {
            return Err(DomainError::InvalidExecution(
                "execution request requires an allowed authorization decision",
            ));
        }
        Ok(Self {
            execution_session_id,
            task_instance_reference,
            task_state_snapshot,
            task_readiness_decision,
            authorization_decision_reference,
            requested_at,
        })
    }

    pub fn execution_session_id(&self) -> &ExecutionSessionId {
        &self.execution_session_id
    }
    pub fn task_instance_reference(&self) -> &TaskInstanceReference {
        &self.task_instance_reference
    }
    pub fn task_state_snapshot(&self) -> &TaskStateSnapshot {
        &self.task_state_snapshot
    }
    pub fn task_readiness_decision(&self) -> &TaskReadinessDecision {
        &self.task_readiness_decision
    }
    pub fn authorization_decision_reference(&self) -> &AuthorizationDecisionReference {
        &self.authorization_decision_reference
    }
    pub fn requested_at(&self) -> &TimeReference {
        &self.requested_at
    }
}
