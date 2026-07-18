use super::{
    TaskAssignmentStatus, TaskReadinessBlocked, TaskReadinessBlocker, TaskReadinessDecision,
    TaskReadinessEvidence, TaskReadinessInput, TaskReadinessReady, TaskReadinessRejection,
    TaskReadinessRejectionReason, TaskReadinessRequirement, TaskState,
};

pub struct TaskReadinessControl;

impl TaskReadinessControl {
    pub fn evaluate(input: &TaskReadinessInput) -> TaskReadinessDecision {
        if has_contradictory_requirements(input.task_readiness_requirements()) {
            return TaskReadinessDecision::Rejected(TaskReadinessRejection::new(
                input.task_instance_reference().clone(),
                TaskReadinessRejectionReason::ContradictoryRequirement,
            ));
        }

        let mut blockers = Vec::new();
        if matches!(
            input.task_state(),
            TaskState::Completed | TaskState::Failed | TaskState::Cancelled | TaskState::Archived
        ) {
            blockers.push(TaskReadinessBlocker::TerminalTaskState);
        }

        for requirement in input.task_readiness_requirements() {
            match requirement {
                TaskReadinessRequirement::OwnershipRequired if input.task_ownership().is_none() => {
                    push_blocker(&mut blockers, TaskReadinessBlocker::MissingOwner);
                }
                TaskReadinessRequirement::AssignmentRequired
                    if input.task_assignment().is_none() =>
                {
                    push_blocker(&mut blockers, TaskReadinessBlocker::MissingAssignment);
                }
                TaskReadinessRequirement::AcceptedAssignmentRequired => {
                    let accepted = input.task_assignment().is_some_and(|assignment| {
                        assignment.task_assignment_status() == TaskAssignmentStatus::Accepted
                    });
                    if !accepted {
                        push_blocker(&mut blockers, TaskReadinessBlocker::AssignmentNotAccepted);
                    }
                }
                TaskReadinessRequirement::RequiredInputAvailable
                    if !has_evidence(
                        input.task_readiness_evidence(),
                        TaskReadinessEvidence::RequiredInputAvailable,
                    ) =>
                {
                    push_blocker(&mut blockers, TaskReadinessBlocker::MissingRequiredInput);
                }
                TaskReadinessRequirement::DependenciesComplete
                    if !has_evidence(
                        input.task_readiness_evidence(),
                        TaskReadinessEvidence::DependenciesComplete,
                    ) =>
                {
                    push_blocker(&mut blockers, TaskReadinessBlocker::DependencyIncomplete);
                }
                TaskReadinessRequirement::AuthorizationAllowed => {
                    let allowed = input
                        .authorization_outcome()
                        .is_none_or(|outcome| !outcome.is_denied())
                        && has_evidence(
                            input.task_readiness_evidence(),
                            TaskReadinessEvidence::AuthorizationAllowed,
                        );
                    if !allowed {
                        push_blocker(&mut blockers, TaskReadinessBlocker::AuthorizationDenied);
                    }
                }
                TaskReadinessRequirement::EvidencePrerequisitesAvailable
                    if !has_evidence(
                        input.task_readiness_evidence(),
                        TaskReadinessEvidence::EvidencePrerequisitesAvailable,
                    ) =>
                {
                    push_blocker(&mut blockers, TaskReadinessBlocker::MissingRequiredEvidence);
                }
                TaskReadinessRequirement::LaterAssignmentPermitted
                | TaskReadinessRequirement::OwnershipRequired
                | TaskReadinessRequirement::AssignmentRequired
                | TaskReadinessRequirement::RequiredInputAvailable
                | TaskReadinessRequirement::DependenciesComplete
                | TaskReadinessRequirement::EvidencePrerequisitesAvailable => {}
            }
        }

        if blockers.is_empty() {
            TaskReadinessDecision::Ready(TaskReadinessReady::new(
                input.task_instance_reference().clone(),
                input.task_readiness_evidence().to_vec(),
            ))
        } else {
            TaskReadinessDecision::Blocked(TaskReadinessBlocked::new(
                input.task_instance_reference().clone(),
                blockers,
            ))
        }
    }
}

fn has_contradictory_requirements(requirements: &[TaskReadinessRequirement]) -> bool {
    let later_assignment =
        requirements.contains(&TaskReadinessRequirement::LaterAssignmentPermitted);
    later_assignment
        && requirements.iter().any(|requirement| {
            matches!(
                requirement,
                TaskReadinessRequirement::AssignmentRequired
                    | TaskReadinessRequirement::AcceptedAssignmentRequired
            )
        })
}

fn has_evidence(evidence: &[TaskReadinessEvidence], expected: TaskReadinessEvidence) -> bool {
    evidence.contains(&expected)
}

fn push_blocker(blockers: &mut Vec<TaskReadinessBlocker>, blocker: TaskReadinessBlocker) {
    if !blockers.contains(&blocker) {
        blockers.push(blocker);
    }
}
