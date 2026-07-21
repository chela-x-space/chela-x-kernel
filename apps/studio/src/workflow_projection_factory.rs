use std::fmt::Debug;

use kernel_domain::{
    EnterpriseId, ExecutionSessionId, OwnershipPath, ProjectId, StableVersion, StateSequence,
    TaskInstanceId, TaskInstanceReference, WorkflowFailureCode, WorkflowId, WorkflowState,
    WorkflowStateSnapshot, WorkflowStepReference, WorkspaceId,
};
use kernel_studio::StudioWorkflowProjection;

use crate::projection_factory::host_local_studio_audit_reference;

fn checked<T, E: Debug>(result: Result<T, E>, context: &str) -> Result<T, String> {
    result.map_err(|error| format!("{context}: {error:?}"))
}

fn ownership_path() -> Result<OwnershipPath, String> {
    checked(
        OwnershipPath::new(
            checked(EnterpriseId::new("CX-ENT-000001"), "invalid enterprise id")?,
            Some(checked(
                WorkspaceId::new("CX-WS-000001"),
                "invalid workspace id",
            )?),
            Some(checked(
                ProjectId::new("CX-PROJ-000001"),
                "invalid project id",
            )?),
            None,
        ),
        "invalid ownership path",
    )
}

fn workflow_state_snapshot() -> Result<WorkflowStateSnapshot, String> {
    let sequence =
        StateSequence::new(2).ok_or_else(|| "invalid workflow state sequence".to_owned())?;

    Ok(WorkflowStateSnapshot::new(
        checked(WorkflowId::new("CX-WF-000001"), "invalid workflow id")?,
        ownership_path()?,
        checked(
            StableVersion::new("workflow_version", "1.0.0"),
            "invalid workflow definition version",
        )?,
        WorkflowState::Running,
        sequence,
    ))
}

fn workflow_step_reference(value: &str) -> Result<WorkflowStepReference, String> {
    checked(
        WorkflowStepReference::new(value),
        "invalid workflow step reference",
    )
}

fn task_instance_reference() -> Result<TaskInstanceReference, String> {
    Ok(TaskInstanceReference::new(checked(
        TaskInstanceId::new("CX-TASK-000001"),
        "invalid task instance id",
    )?))
}

pub fn host_local_workflow_projection() -> Result<StudioWorkflowProjection, String> {
    checked(
        StudioWorkflowProjection::new(
            workflow_state_snapshot()?,
            Some(workflow_step_reference("step.current")?),
            vec![workflow_step_reference("step.done")?],
            vec![workflow_step_reference("step.blocked")?],
            vec![task_instance_reference()?],
            vec![checked(
                ExecutionSessionId::new("execution.session-0001"),
                "invalid execution session id",
            )?],
            Some(WorkflowFailureCode::InvalidTransition),
            host_local_studio_audit_reference()?,
        ),
        "invalid studio workflow projection",
    )
}

#[cfg(test)]
mod tests {
    use super::host_local_workflow_projection;
    use kernel_domain::WorkflowFailureCode;

    #[test]
    fn builds_host_local_workflow_projection() {
        let projection = host_local_workflow_projection().expect("projection");

        assert_eq!(
            projection.workflow_state_snapshot().workflow_id().as_str(),
            "CX-WF-000001"
        );

        assert_eq!(
            projection
                .current_step_reference()
                .expect("current step")
                .as_str(),
            "step.current"
        );

        assert_eq!(
            projection.completed_step_references()[0].as_str(),
            "step.done"
        );

        assert_eq!(
            projection.blocked_step_references()[0].as_str(),
            "step.blocked"
        );

        assert_eq!(
            projection.task_instance_references()[0]
                .task_instance_id()
                .as_str(),
            "CX-TASK-000001"
        );

        assert_eq!(
            projection.execution_session_ids()[0].as_str(),
            "execution.session-0001"
        );

        assert_eq!(
            projection.workflow_failure_code(),
            Some(&WorkflowFailureCode::InvalidTransition)
        );
    }
}
