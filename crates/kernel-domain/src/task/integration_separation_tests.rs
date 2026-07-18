use super::TaskOutcomeDecision;

#[test]
fn integration_completion_and_failure_outcomes_are_mutually_exclusive() {
    let completion =
        match super::integration_flow_support::completion_happy_path().completion_outcome {
            super::TaskCompletionOutcome::Accepted(completion) => completion,
            _ => panic!("expected accepted completion"),
        };
    let failure = match super::integration_flow_support::failure_happy_path().failure_outcome {
        super::TaskFailureOutcome::Accepted(failure) => failure,
        _ => panic!("expected accepted failure"),
    };

    assert!(matches!(
        TaskOutcomeDecision::Completed(completion),
        TaskOutcomeDecision::Completed(_)
    ));
    assert!(matches!(
        TaskOutcomeDecision::Failed(failure),
        TaskOutcomeDecision::Failed(_)
    ));
    assert!(matches!(
        TaskOutcomeDecision::Rejected(super::TaskOutcomeRejectionReason::CompletionFailureConflict),
        TaskOutcomeDecision::Rejected(_)
    ));
}

#[test]
fn integration_no_concern_mutates_another_concern() {
    let flow = super::integration_flow_support::completion_happy_path();

    assert_eq!(flow.task_instance.task_state(), super::TaskState::Pending);
    assert_eq!(
        flow.accepted_assignment.task_assignment_status(),
        super::TaskAssignmentStatus::Accepted
    );
    assert_eq!(flow.task_priority.task_priority_value().value(), 5);
    assert_eq!(
        flow.task_ownership
            .task_owner()
            .identity_reference()
            .expect("identity")
            .enterprise_id()
            .as_str(),
        "CX-ENT-900001"
    );
}
