use super::{TaskState, TaskTransitionControl, TaskTransitionDecision};

#[test]
fn task_lifecycle_same_state_request_is_noop() {
    let request =
        super::lifecycle_test_support::transition_request(TaskState::Pending, TaskState::Pending);

    match TaskTransitionControl::evaluate(&request) {
        TaskTransitionDecision::NoOp(noop) => {
            assert_eq!(
                noop.current_task_state_snapshot().task_state(),
                TaskState::Pending
            );
            assert_eq!(
                noop.current_task_state_snapshot().state_sequence().value(),
                1
            );
        }
        other => panic!("expected noop transition, got {other:?}"),
    }
}

#[test]
fn task_lifecycle_same_request_produces_same_noop() {
    let request =
        super::lifecycle_test_support::transition_request(TaskState::Pending, TaskState::Pending);

    let first = TaskTransitionControl::evaluate(&request);
    let second = TaskTransitionControl::evaluate(&request);

    assert_eq!(first, second);
}
