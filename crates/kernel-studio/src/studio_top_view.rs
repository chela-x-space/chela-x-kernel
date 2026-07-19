use kernel_domain::{
    AgentId, ExecutionSessionId, OwnershipPath, RuntimeId, TaskInstanceReference, WorkflowId,
};

use crate::studio::{StudioAuditReference, StudioError, StudioErrorCode, StudioResult};
use crate::studio_validation::reject_duplicates;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StudioAttentionState {
    Nominal,
    NeedsAttention,
    Critical,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudioTopViewProjection {
    ownership_path: OwnershipPath,
    agent_ids: Vec<AgentId>,
    runtime_ids: Vec<RuntimeId>,
    workflow_ids: Vec<WorkflowId>,
    task_instance_references: Vec<TaskInstanceReference>,
    execution_session_ids: Vec<ExecutionSessionId>,
    attention_state: StudioAttentionState,
    studio_audit_reference: StudioAuditReference,
}

impl StudioTopViewProjection {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ownership_path: OwnershipPath,
        agent_ids: Vec<AgentId>,
        runtime_ids: Vec<RuntimeId>,
        workflow_ids: Vec<WorkflowId>,
        task_instance_references: Vec<TaskInstanceReference>,
        execution_session_ids: Vec<ExecutionSessionId>,
        attention_state: StudioAttentionState,
        studio_audit_reference: StudioAuditReference,
    ) -> StudioResult<Self> {
        if agent_ids.is_empty()
            && runtime_ids.is_empty()
            && workflow_ids.is_empty()
            && task_instance_references.is_empty()
            && execution_session_ids.is_empty()
        {
            return Err(StudioError::new(
                StudioErrorCode::ProjectionMismatch,
                "studio top view requires at least one governed subject reference",
            )?);
        }
        reject_duplicates(
            &agent_ids,
            StudioErrorCode::ProjectionMismatch,
            "duplicate agent identifier in studio top view",
        )?;
        reject_duplicates(
            &runtime_ids,
            StudioErrorCode::ProjectionMismatch,
            "duplicate runtime identifier in studio top view",
        )?;
        reject_duplicates(
            &workflow_ids,
            StudioErrorCode::ProjectionMismatch,
            "duplicate workflow identifier in studio top view",
        )?;
        reject_duplicates(
            &task_instance_references,
            StudioErrorCode::ProjectionMismatch,
            "duplicate task instance reference in studio top view",
        )?;
        reject_duplicates(
            &execution_session_ids,
            StudioErrorCode::ProjectionMismatch,
            "duplicate execution session identifier in studio top view",
        )?;
        Ok(Self {
            ownership_path,
            agent_ids,
            runtime_ids,
            workflow_ids,
            task_instance_references,
            execution_session_ids,
            attention_state,
            studio_audit_reference,
        })
    }

    pub fn ownership_path(&self) -> &OwnershipPath {
        &self.ownership_path
    }

    pub fn agent_ids(&self) -> &[AgentId] {
        &self.agent_ids
    }

    pub fn runtime_ids(&self) -> &[RuntimeId] {
        &self.runtime_ids
    }

    pub fn workflow_ids(&self) -> &[WorkflowId] {
        &self.workflow_ids
    }

    pub fn task_instance_references(&self) -> &[TaskInstanceReference] {
        &self.task_instance_references
    }

    pub fn execution_session_ids(&self) -> &[ExecutionSessionId] {
        &self.execution_session_ids
    }

    pub fn attention_state(&self) -> StudioAttentionState {
        self.attention_state
    }

    pub fn studio_audit_reference(&self) -> &StudioAuditReference {
        &self.studio_audit_reference
    }
}
