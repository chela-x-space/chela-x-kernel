use super::{
    TaskAllowedTransition, TaskNoOpTransition, TaskRejectedTransition, TaskStateSnapshot,
    TaskTransitionDecision, TaskTransitionRejectionReason, TaskTransitionRequest,
};

pub(super) fn allow(request: &TaskTransitionRequest) -> TaskTransitionDecision {
    TaskTransitionDecision::Allowed(TaskAllowedTransition::new(
        request.current_task_state_snapshot().clone(),
        TaskStateSnapshot::new(
            request
                .current_task_state_snapshot()
                .task_instance_reference()
                .clone(),
            request.requested_target_task_state(),
            request
                .current_task_state_snapshot()
                .state_sequence()
                .next(),
        ),
        request.transition_reason_reference().cloned(),
        request.transition_authority_reference().cloned(),
        request.transition_evidence_references().to_vec(),
    ))
}

pub(super) fn reject(
    request: &TaskTransitionRequest,
    reason: TaskTransitionRejectionReason,
) -> TaskTransitionDecision {
    TaskTransitionDecision::Rejected(TaskRejectedTransition::new(
        request.current_task_state_snapshot().clone(),
        request.requested_target_task_state(),
        reason,
    ))
}

pub(super) fn noop(request: &TaskTransitionRequest) -> TaskTransitionDecision {
    TaskTransitionDecision::NoOp(TaskNoOpTransition::new(
        request.current_task_state_snapshot().clone(),
    ))
}
