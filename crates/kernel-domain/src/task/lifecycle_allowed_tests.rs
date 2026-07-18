use super::{TaskState, TaskTransitionControl, TaskTransitionDecision};

#[test]
fn task_lifecycle_snapshot_preserves_reference_state_and_sequence() {
    let snapshot = super::lifecycle_test_support::task_state_snapshot(TaskState::Pending);

    assert_eq!(snapshot.task_state(), TaskState::Pending);
    assert_eq!(snapshot.state_sequence().value(), 1);
}

#[test]
fn task_lifecycle_pending_to_in_progress_is_allowed() {
    assert_allowed_transition(TaskState::Pending, TaskState::InProgress);
}

#[test]
fn task_lifecycle_pending_to_cancelled_is_allowed() {
    assert_allowed_transition(TaskState::Pending, TaskState::Cancelled);
}

#[test]
fn task_lifecycle_in_progress_to_completed_is_allowed() {
    assert_allowed_transition(TaskState::InProgress, TaskState::Completed);
}

#[test]
fn task_lifecycle_in_progress_to_failed_is_allowed() {
    assert_allowed_transition(TaskState::InProgress, TaskState::Failed);
}

#[test]
fn task_lifecycle_in_progress_to_cancelled_is_allowed() {
    assert_allowed_transition(TaskState::InProgress, TaskState::Cancelled);
}

#[test]
fn task_lifecycle_completed_to_archived_is_allowed() {
    assert_allowed_transition(TaskState::Completed, TaskState::Archived);
}

#[test]
fn task_lifecycle_failed_to_archived_is_allowed() {
    assert_allowed_transition(TaskState::Failed, TaskState::Archived);
}

#[test]
fn task_lifecycle_cancelled_to_archived_is_allowed() {
    assert_allowed_transition(TaskState::Cancelled, TaskState::Archived);
}

fn assert_allowed_transition(current: TaskState, target: TaskState) {
    let request = super::lifecycle_test_support::transition_request(current, target);
    let decision = TaskTransitionControl::evaluate(&request);

    match decision {
        TaskTransitionDecision::Allowed(allowed) => {
            assert_eq!(allowed.previous_task_state_snapshot().task_state(), current);
            assert_eq!(allowed.current_task_state_snapshot().task_state(), target);
            assert_eq!(
                allowed
                    .current_task_state_snapshot()
                    .state_sequence()
                    .value(),
                2
            );
            assert_eq!(
                allowed
                    .transition_reason_reference()
                    .expect("reason")
                    .as_str(),
                "transition.reason"
            );
            assert_eq!(
                allowed
                    .transition_authority_reference()
                    .expect("authority")
                    .as_str(),
                "transition.authority"
            );
            assert_eq!(allowed.transition_evidence_references().len(), 1);
        }
        other => panic!("expected allowed transition, got {other:?}"),
    }
}
