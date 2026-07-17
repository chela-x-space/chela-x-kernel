use super::assignment_validation::validate_assignment_eligibility;
use super::{
    subject::TaskAssignee, TaskAssignment, TaskAssignmentChange, TaskAssignmentDecision,
    TaskAssignmentNoOp, TaskAssignmentRejection, TaskAssignmentRejectionReason,
    TaskAssignmentRequest, TaskAssignmentStatus,
};

pub struct TaskAssignmentControl;

impl TaskAssignmentControl {
    pub fn evaluate(request: &TaskAssignmentRequest) -> TaskAssignmentDecision {
        if request.requested_assignee().is_none()
            && request.current_assignment().task_assignment_status()
                == TaskAssignmentStatus::Unassigned
        {
            return TaskAssignmentDecision::NoOp(TaskAssignmentNoOp::new(
                request.current_assignment().clone(),
            ));
        }
        if request.task_assignment_authority().is_none() {
            return reject(request, TaskAssignmentRejectionReason::MissingAuthority);
        }

        match request.requested_assignee() {
            Some(task_assignee) => assign(request, task_assignee),
            None => unassign(request),
        }
    }
}

fn assign(request: &TaskAssignmentRequest, task_assignee: &TaskAssignee) -> TaskAssignmentDecision {
    if let Err(reason) =
        validate_assignment_eligibility(task_assignee, request.authorization_outcome())
    {
        return reject(request, reason);
    }
    if request.current_assignment().task_assignee() == Some(task_assignee)
        && matches!(
            request.current_assignment().task_assignment_status(),
            TaskAssignmentStatus::Assigned | TaskAssignmentStatus::Accepted
        )
    {
        return TaskAssignmentDecision::NoOp(TaskAssignmentNoOp::new(
            request.current_assignment().clone(),
        ));
    }

    let current_assignment = TaskAssignment::new(
        request
            .current_assignment()
            .task_instance_reference()
            .clone(),
        Some(task_assignee.clone()),
        TaskAssignmentStatus::Assigned,
        request.task_assignment_authority().cloned(),
        request.task_assignment_reason().cloned(),
    )
    .expect("validated assignment request constructs deterministic snapshot");

    TaskAssignmentDecision::Updated(TaskAssignmentChange::new(
        request.current_assignment().clone(),
        current_assignment,
    ))
}

fn unassign(request: &TaskAssignmentRequest) -> TaskAssignmentDecision {
    let Some(reason) = request.task_assignment_reason().cloned() else {
        return reject(request, TaskAssignmentRejectionReason::MissingReason);
    };

    let current_assignment = TaskAssignment::new(
        request
            .current_assignment()
            .task_instance_reference()
            .clone(),
        None,
        TaskAssignmentStatus::Released,
        request.task_assignment_authority().cloned(),
        Some(reason),
    )
    .expect("validated unassignment request constructs deterministic snapshot");

    TaskAssignmentDecision::Updated(TaskAssignmentChange::new(
        request.current_assignment().clone(),
        current_assignment,
    ))
}

fn reject(
    request: &TaskAssignmentRequest,
    reason: TaskAssignmentRejectionReason,
) -> TaskAssignmentDecision {
    TaskAssignmentDecision::Rejected(TaskAssignmentRejection::new(
        request.current_assignment().clone(),
        reason,
    ))
}
