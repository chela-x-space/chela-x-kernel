use crate::authorization::AuthorizationDecisionOutcome;
use crate::identifier::{EnterpriseId, HumanId};
use crate::identity::{HumanIdentity, IdentityReference};
use crate::lifecycle::HumanLifecycle;
use crate::ownership::OwnershipScope;
use crate::state::{
    StateSequence, TransitionAuthorityReference, TransitionEvidenceReference,
    TransitionReasonReference,
};

use super::{
    TaskAssignee, TaskAssignment, TaskAssignmentControl, TaskAssignmentDecision,
    TaskAssignmentReason, TaskAssignmentRequest, TaskAssignmentStatus, TaskDependency,
    TaskDependencyControl, TaskDependencyCoordinationDecision, TaskDependencyCoordinationRequest,
    TaskDependencyFact, TaskDependencyGraphReference, TaskDependencyId, TaskDependencyReference,
    TaskDependencyRequirement, TaskDependencySet, TaskDependencySource, TaskDependencyTarget,
    TaskDependencyType, TaskEvidenceReference, TaskLifecycleGuards, TaskOwner, TaskOwnership,
    TaskPriority, TaskPriorityClass, TaskPriorityValue, TaskReadinessControl,
    TaskReadinessDecision, TaskReadinessEvidence, TaskReadinessInput, TaskReadinessRequirement,
    TaskState, TaskStateSnapshot, TaskTransitionDecision, TaskTransitionRequest,
};

pub(super) fn task_instance_reference() -> super::TaskInstanceReference {
    super::outcome_test_support::task_instance_reference()
}

pub(super) fn pending_snapshot() -> TaskStateSnapshot {
    snapshot(TaskState::Pending, 1)
}

pub(super) fn snapshot(task_state: TaskState, sequence: u64) -> TaskStateSnapshot {
    TaskStateSnapshot::new(
        task_instance_reference(),
        task_state,
        StateSequence::new(sequence).expect("sequence"),
    )
}

pub(super) fn predecessor_reference() -> super::TaskInstanceReference {
    super::TaskInstanceReference::new(
        super::TaskInstanceId::new("task.instance.predecessor.900001").expect("instance"),
    )
}

pub(super) fn predecessor_snapshot(task_state: TaskState, sequence: u64) -> TaskStateSnapshot {
    TaskStateSnapshot::new(
        predecessor_reference(),
        task_state,
        StateSequence::new(sequence).expect("sequence"),
    )
}

pub(super) fn ownership() -> TaskOwnership {
    TaskOwnership::new(
        task_instance_reference(),
        TaskOwner::from_identity(IdentityReference::Human(HumanIdentity::new(
            HumanId::new("CX-EMP-900001").expect("human"),
            EnterpriseId::new("CX-ENT-900001").expect("enterprise"),
            HumanLifecycle::Active,
        ))),
        OwnershipScope::Enterprise,
        TransitionAuthorityReference::new("authority.owner").expect("authority"),
    )
}

pub(super) fn accepted_assignment() -> TaskAssignment {
    TaskAssignment::new(
        task_instance_reference(),
        Some(active_assignee()),
        TaskAssignmentStatus::Accepted,
        Some(TransitionAuthorityReference::new("authority.assign").expect("authority")),
        None,
    )
    .expect("accepted assignment")
}

pub(super) fn assigned_decision() -> TaskAssignmentDecision {
    TaskAssignmentControl::evaluate(&TaskAssignmentRequest::new(
        TaskAssignment::new(
            task_instance_reference(),
            None,
            TaskAssignmentStatus::Unassigned,
            None,
            None,
        )
        .expect("unassigned"),
        Some(active_assignee()),
        Some(TransitionAuthorityReference::new("authority.assign").expect("authority")),
        Some(TaskAssignmentReason::new("assign").expect("reason")),
        Some(AuthorizationDecisionOutcome::Allow),
    ))
}

pub(super) fn priority() -> TaskPriority {
    TaskPriority::new(
        task_instance_reference(),
        TaskPriorityClass::Explicit,
        TaskPriorityValue::new(5).expect("priority"),
    )
}

pub(super) fn satisfied_dependency_decision() -> TaskDependencyCoordinationDecision {
    dependency_decision(vec![TaskDependencyFact::new(
        predecessor_snapshot(TaskState::Completed, 1),
        vec![TaskEvidenceReference::new(
            super::TaskEvidenceId::new("task.evidence.dep").expect("evidence id"),
        )],
        vec![super::outcome_test_support::output_contract_primary()],
    )
    .expect("fact")])
}

pub(super) fn unsatisfied_dependency_decision() -> TaskDependencyCoordinationDecision {
    dependency_decision(vec![TaskDependencyFact::new(
        predecessor_snapshot(TaskState::Pending, 1),
        Vec::new(),
        Vec::new(),
    )
    .expect("fact")])
}

pub(super) fn unresolved_dependency_decision() -> TaskDependencyCoordinationDecision {
    dependency_decision(Vec::new())
}

pub(super) fn ready_readiness_decision(
    task_ownership: Option<TaskOwnership>,
    task_assignment: Option<TaskAssignment>,
    task_priority: Option<TaskPriority>,
    task_readiness_evidence: Vec<TaskReadinessEvidence>,
) -> TaskReadinessDecision {
    TaskReadinessControl::evaluate(&TaskReadinessInput::new(
        task_instance_reference(),
        TaskState::Pending,
        task_priority,
        task_ownership,
        task_assignment,
        vec![
            TaskReadinessRequirement::OwnershipRequired,
            TaskReadinessRequirement::AcceptedAssignmentRequired,
            TaskReadinessRequirement::RequiredInputAvailable,
            TaskReadinessRequirement::DependenciesComplete,
            TaskReadinessRequirement::AuthorizationAllowed,
            TaskReadinessRequirement::EvidencePrerequisitesAvailable,
        ],
        task_readiness_evidence,
        Some(AuthorizationDecisionOutcome::Allow),
    ))
}

pub(super) fn start_request(
    current_task_state_snapshot: TaskStateSnapshot,
    task_readiness_decision: TaskReadinessDecision,
    task_assignment: TaskAssignment,
    dependencies_satisfied: bool,
) -> TaskTransitionRequest {
    TaskTransitionRequest::new(
        current_task_state_snapshot.clone(),
        TaskState::InProgress,
        Some(TransitionReasonReference::new("start.reason").expect("reason")),
        Some(TransitionAuthorityReference::new("start.authority").expect("authority")),
        vec![TransitionEvidenceReference::new("start.evidence").expect("evidence")],
        Some(task_readiness_decision),
        Some(task_assignment),
        TaskLifecycleGuards::new(
            Some(current_task_state_snapshot.state_sequence()),
            true,
            true,
            dependencies_satisfied,
            false,
            false,
            false,
            false,
            None,
            None,
        ),
    )
    .expect("start request")
}

pub(super) fn complete_request(
    current_task_state_snapshot: TaskStateSnapshot,
) -> TaskTransitionRequest {
    TaskTransitionRequest::new(
        current_task_state_snapshot.clone(),
        TaskState::Completed,
        Some(TransitionReasonReference::new("complete.reason").expect("reason")),
        Some(TransitionAuthorityReference::new("complete.authority").expect("authority")),
        vec![TransitionEvidenceReference::new("complete.evidence").expect("evidence")],
        None,
        Some(accepted_assignment()),
        TaskLifecycleGuards::new(
            Some(current_task_state_snapshot.state_sequence()),
            false,
            true,
            true,
            true,
            true,
            true,
            false,
            None,
            None,
        ),
    )
    .expect("complete request")
}

pub(super) fn failure_request(
    current_task_state_snapshot: TaskStateSnapshot,
) -> TaskTransitionRequest {
    TaskTransitionRequest::new(
        current_task_state_snapshot.clone(),
        TaskState::Failed,
        Some(TransitionReasonReference::new("failure.reason").expect("reason")),
        Some(TransitionAuthorityReference::new("failure.authority").expect("authority")),
        vec![TransitionEvidenceReference::new("failure.evidence").expect("evidence")],
        None,
        Some(accepted_assignment()),
        TaskLifecycleGuards::new(
            Some(current_task_state_snapshot.state_sequence()),
            false,
            true,
            true,
            false,
            false,
            false,
            true,
            Some(super::TaskFailureCode::new("task.failure.timeout").expect("code")),
            Some(
                super::TaskFailureCategory::new("task.failure_category.execution")
                    .expect("category"),
            ),
        ),
    )
    .expect("failure request")
}

pub(super) fn archive_request(
    current_task_state_snapshot: TaskStateSnapshot,
) -> TaskTransitionRequest {
    TaskTransitionRequest::new(
        current_task_state_snapshot.clone(),
        TaskState::Archived,
        None,
        Some(TransitionAuthorityReference::new("archive.authority").expect("authority")),
        vec![TransitionEvidenceReference::new("archive.evidence").expect("evidence")],
        None,
        None,
        TaskLifecycleGuards::new(
            Some(current_task_state_snapshot.state_sequence()),
            false,
            true,
            true,
            false,
            false,
            false,
            false,
            None,
            None,
        ),
    )
    .expect("archive request")
}

pub(super) fn allowed_snapshot(decision: &TaskTransitionDecision) -> TaskStateSnapshot {
    match decision {
        TaskTransitionDecision::Allowed(transition) => {
            transition.current_task_state_snapshot().clone()
        }
        _ => panic!("expected allowed transition"),
    }
}

fn active_assignee() -> TaskAssignee {
    TaskAssignee::from_identity(IdentityReference::Human(HumanIdentity::new(
        HumanId::new("CX-EMP-900002").expect("human"),
        EnterpriseId::new("CX-ENT-900001").expect("enterprise"),
        HumanLifecycle::Active,
    )))
}

fn dependency_decision(
    task_dependency_facts: Vec<TaskDependencyFact>,
) -> TaskDependencyCoordinationDecision {
    let task_dependency = TaskDependency::new(
        TaskDependencyReference::new(
            TaskDependencyId::new("task.dependency.900001").expect("dependency"),
        ),
        TaskDependencySource::new(predecessor_reference()),
        TaskDependencyTarget::new(task_instance_reference()),
        TaskDependencyType::Success,
        TaskDependencyRequirement::SuccessfulCompletion,
    )
    .expect("dependency");

    TaskDependencyControl::evaluate(
        &TaskDependencyCoordinationRequest::new(
            TaskDependencySet::new(
                TaskDependencyGraphReference::new("task.dependency.graph.900001")
                    .expect("graph reference"),
                vec![task_dependency],
            ),
            task_dependency_facts,
        )
        .expect("coordination request"),
    )
}
