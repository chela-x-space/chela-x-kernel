use super::{TaskState, TaskTransitionDecision, TaskTransitionRejectionReason};

#[test]
fn integration_state_sequence_increments_once_per_allowed_transition() {
    let flow = super::integration_flow_support::completion_happy_path();

    assert_eq!(
        super::integration_test_support::allowed_snapshot(&flow.start_decision)
            .state_sequence()
            .value(),
        2
    );
    assert_eq!(
        super::integration_test_support::allowed_snapshot(&flow.completed_decision)
            .state_sequence()
            .value(),
        3
    );
    assert_eq!(
        super::integration_test_support::allowed_snapshot(&flow.archived_decision)
            .state_sequence()
            .value(),
        4
    );
}

#[test]
fn integration_rejected_and_noop_sequence_are_preserved() {
    let blocked = super::integration_test_support::ready_readiness_decision(
        Some(super::integration_test_support::ownership()),
        Some(super::integration_test_support::accepted_assignment()),
        Some(super::integration_test_support::priority()),
        Vec::new(),
    );
    let rejected =
        super::TaskTransitionControl::evaluate(&super::integration_test_support::start_request(
            super::integration_test_support::pending_snapshot(),
            blocked,
            super::integration_test_support::accepted_assignment(),
            false,
        ));
    let noop = super::TaskTransitionControl::evaluate(
        &super::TaskTransitionRequest::new(
            super::integration_test_support::pending_snapshot(),
            TaskState::Pending,
            None,
            None,
            Vec::new(),
            None,
            None,
            super::TaskLifecycleGuards::default(),
        )
        .expect("request"),
    );

    assert!(matches!(
        rejected,
        TaskTransitionDecision::Rejected(transition)
            if transition.reason() == TaskTransitionRejectionReason::ReadinessNotSatisfied
                && transition.current_task_state_snapshot().state_sequence().value() == 1
    ));
    assert!(matches!(
        noop,
        TaskTransitionDecision::NoOp(transition)
            if transition.current_task_state_snapshot().state_sequence().value() == 1
    ));
}
