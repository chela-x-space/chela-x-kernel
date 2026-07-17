use crate::identifier::{EnterpriseId, HumanId};
use crate::identity::{HumanIdentity, IdentityReference};
use crate::lifecycle::HumanLifecycle;
use crate::state::TransitionAuthorityReference;

use super::{
    TaskAssignee, TaskAssignment, TaskAssignmentStatus, TaskInstanceId, TaskInstanceReference,
};

fn task_instance_reference() -> TaskInstanceReference {
    TaskInstanceReference::new(TaskInstanceId::new("task.instance.assignment").expect("instance"))
}

fn human_assignee(lifecycle: HumanLifecycle) -> TaskAssignee {
    TaskAssignee::from_identity(IdentityReference::Human(HumanIdentity::new(
        HumanId::new("CX-EMP-700002").expect("human"),
        EnterpriseId::new("CX-ENT-700001").expect("enterprise"),
        lifecycle,
    )))
}

fn unassigned_assignment() -> TaskAssignment {
    TaskAssignment::new(
        task_instance_reference(),
        None,
        TaskAssignmentStatus::Unassigned,
        None,
        None,
    )
    .expect("unassigned")
}

#[test]
fn task_assignment_valid_assignment_preserves_assignee() {
    let assignee = human_assignee(HumanLifecycle::Active);
    let assignment = TaskAssignment::new(
        task_instance_reference(),
        Some(assignee.clone()),
        TaskAssignmentStatus::Assigned,
        Some(TransitionAuthorityReference::new("authority.assign").expect("authority")),
        None,
    )
    .expect("assignment");

    assert_eq!(assignment.task_assignee(), Some(&assignee));
}

#[test]
fn task_assignment_is_distinct_from_ownership() {
    let assignment = unassigned_assignment();
    assert_eq!(
        assignment.task_assignment_status(),
        TaskAssignmentStatus::Unassigned
    );
}
