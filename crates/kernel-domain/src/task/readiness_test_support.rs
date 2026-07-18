use crate::authorization::AuthorizationDecisionOutcome;
use crate::identifier::{EnterpriseId, HumanId};
use crate::identity::{HumanIdentity, IdentityReference};
use crate::lifecycle::HumanLifecycle;
use crate::ownership::OwnershipScope;
use crate::state::TransitionAuthorityReference;

use super::{
    TaskAssignment, TaskAssignmentStatus, TaskInstanceReference, TaskOwnership,
    TaskReadinessEvidence, TaskReadinessInput, TaskReadinessRequirement, TaskState,
};

pub(super) fn task_instance_reference() -> TaskInstanceReference {
    let instance = crate::task::instance_tests::minimal_task_instance_for_shared_tests();
    TaskInstanceReference::new(instance.task_instance_id().clone())
}

pub(super) fn accepted_assignment() -> TaskAssignment {
    TaskAssignment::new(
        task_instance_reference(),
        Some(super::TaskAssignee::from_identity(
            IdentityReference::Human(HumanIdentity::new(
                HumanId::new("CX-EMP-700102").expect("human"),
                EnterpriseId::new("CX-ENT-700001").expect("enterprise"),
                HumanLifecycle::Active,
            )),
        )),
        TaskAssignmentStatus::Accepted,
        Some(TransitionAuthorityReference::new("authority.assign").expect("authority")),
        None,
    )
    .expect("assignment")
}

pub(super) fn ownership() -> TaskOwnership {
    TaskOwnership::new(
        task_instance_reference(),
        super::TaskOwner::from_identity(IdentityReference::Human(HumanIdentity::new(
            HumanId::new("CX-EMP-700101").expect("human"),
            EnterpriseId::new("CX-ENT-700001").expect("enterprise"),
            HumanLifecycle::Active,
        ))),
        OwnershipScope::Enterprise,
        TransitionAuthorityReference::new("authority.owner").expect("authority"),
    )
}

pub(super) fn ready_input() -> TaskReadinessInput {
    TaskReadinessInput::new(
        task_instance_reference(),
        TaskState::Pending,
        None,
        Some(ownership()),
        Some(accepted_assignment()),
        vec![
            TaskReadinessRequirement::OwnershipRequired,
            TaskReadinessRequirement::AcceptedAssignmentRequired,
            TaskReadinessRequirement::RequiredInputAvailable,
            TaskReadinessRequirement::DependenciesComplete,
            TaskReadinessRequirement::AuthorizationAllowed,
            TaskReadinessRequirement::EvidencePrerequisitesAvailable,
        ],
        vec![
            TaskReadinessEvidence::RequiredInputAvailable,
            TaskReadinessEvidence::DependenciesComplete,
            TaskReadinessEvidence::AuthorizationAllowed,
            TaskReadinessEvidence::EvidencePrerequisitesAvailable,
        ],
        Some(AuthorizationDecisionOutcome::Allow),
    )
}
