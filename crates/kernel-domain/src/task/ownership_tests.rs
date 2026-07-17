use crate::identifier::{AgentId, EnglishNamespace, EnterpriseId, HumanId, StableVersion};
use crate::identity::{AgentIdentity, HumanIdentity, IdentityReference};
use crate::lifecycle::{AgentLifecycle, HumanLifecycle};
use crate::ownership::{OwnershipScope, OwnershipSubject};
use crate::state::TransitionAuthorityReference;

use super::{TaskInstanceId, TaskInstanceReference, TaskOwner, TaskOwnership};

fn task_instance_reference() -> TaskInstanceReference {
    TaskInstanceReference::new(TaskInstanceId::new("task.instance.ownership").expect("instance"))
}

#[test]
fn task_ownership_valid_human_owner_binding_is_preserved() {
    let owner = TaskOwner::from_identity(IdentityReference::Human(HumanIdentity::new(
        HumanId::new("CX-EMP-700001").expect("human"),
        EnterpriseId::new("CX-ENT-700001").expect("enterprise"),
        HumanLifecycle::Active,
    )));
    let ownership = TaskOwnership::new(
        task_instance_reference(),
        owner.clone(),
        OwnershipScope::Enterprise,
        TransitionAuthorityReference::new("authority.owner").expect("authority"),
    );

    assert_eq!(ownership.task_owner(), &owner);
}

#[test]
fn task_ownership_owner_identity_reference_is_preserved() {
    let identity = IdentityReference::Agent(
        AgentIdentity::new(
            AgentId::new("CX-AGT-700001").expect("agent"),
            EnglishNamespace::new("agent_namespace", "agent.reviewer").expect("namespace"),
            StableVersion::new("agent_version", "1.0.0").expect("version"),
            EnterpriseId::new("CX-ENT-700001").expect("enterprise"),
            AgentLifecycle::Active,
        )
        .expect("identity"),
    );
    let owner = TaskOwner::from_identity(identity.clone());
    let ownership = TaskOwnership::new(
        task_instance_reference(),
        owner,
        OwnershipScope::Enterprise,
        TransitionAuthorityReference::new("authority.owner").expect("authority"),
    );

    assert_eq!(
        ownership
            .task_owner()
            .identity_reference()
            .expect("identity"),
        &identity
    );
}

#[test]
fn task_ownership_subject_binding_is_preserved() {
    let subject =
        OwnershipSubject::Project(crate::ProjectId::new("CX-PROJ-700001").expect("project"));
    let owner = TaskOwner::from_ownership_subject(subject.clone());
    let ownership = TaskOwnership::new(
        task_instance_reference(),
        owner,
        OwnershipScope::Project,
        TransitionAuthorityReference::new("authority.owner").expect("authority"),
    );

    assert_eq!(
        ownership.task_owner().ownership_subject().expect("subject"),
        &subject
    );
}

#[test]
fn task_ownership_same_input_produces_equal_snapshot() {
    let owner = TaskOwner::from_identity(IdentityReference::Human(HumanIdentity::new(
        HumanId::new("CX-EMP-700001").expect("human"),
        EnterpriseId::new("CX-ENT-700001").expect("enterprise"),
        HumanLifecycle::Active,
    )));

    let left = TaskOwnership::new(
        task_instance_reference(),
        owner.clone(),
        OwnershipScope::Enterprise,
        TransitionAuthorityReference::new("authority.owner").expect("authority"),
    );
    let right = TaskOwnership::new(
        task_instance_reference(),
        owner,
        OwnershipScope::Enterprise,
        TransitionAuthorityReference::new("authority.owner").expect("authority"),
    );

    assert_eq!(left, right);
}

#[test]
fn task_ownership_binding_does_not_change_task_lifecycle() {
    let task_instance = crate::task::instance_tests::minimal_task_instance_for_shared_tests();
    let lifecycle_before = task_instance.task_state();

    let _ownership = TaskOwnership::new(
        TaskInstanceReference::new(task_instance.task_instance_id().clone()),
        TaskOwner::from_identity(IdentityReference::Human(HumanIdentity::new(
            HumanId::new("CX-EMP-700001").expect("human"),
            EnterpriseId::new("CX-ENT-700001").expect("enterprise"),
            HumanLifecycle::Active,
        ))),
        OwnershipScope::Enterprise,
        TransitionAuthorityReference::new("authority.owner").expect("authority"),
    );

    assert_eq!(task_instance.task_state(), lifecycle_before);
}
