use kernel_domain::{
    MemoryProjection, OwnershipPath, RuntimeStateSnapshot, TaskStateSnapshot, TimeReference,
    WorkflowStateSnapshot,
};

use crate::studio::{StudioAuditReference, StudioErrorCode, StudioResult};
use crate::studio_validation::{reject_duplicates, require_exact_scope};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudioDigitalTwinProjection {
    ownership_path: OwnershipPath,
    runtime_state_snapshots: Vec<RuntimeStateSnapshot>,
    workflow_state_snapshots: Vec<WorkflowStateSnapshot>,
    task_state_snapshots: Vec<TaskStateSnapshot>,
    memory_projections: Vec<MemoryProjection>,
    captured_at: TimeReference,
    studio_audit_reference: StudioAuditReference,
}

impl StudioDigitalTwinProjection {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ownership_path: OwnershipPath,
        runtime_state_snapshots: Vec<RuntimeStateSnapshot>,
        workflow_state_snapshots: Vec<WorkflowStateSnapshot>,
        task_state_snapshots: Vec<TaskStateSnapshot>,
        memory_projections: Vec<MemoryProjection>,
        captured_at: TimeReference,
        studio_audit_reference: StudioAuditReference,
    ) -> StudioResult<Self> {
        reject_duplicates(
            &workflow_state_snapshots,
            StudioErrorCode::ProjectionMismatch,
            "duplicate workflow snapshot in studio digital twin projection",
        )?;
        reject_duplicates(
            &task_state_snapshots,
            StudioErrorCode::ProjectionMismatch,
            "duplicate task snapshot in studio digital twin projection",
        )?;
        for workflow_state_snapshot in &workflow_state_snapshots {
            require_exact_scope(
                &ownership_path,
                workflow_state_snapshot.ownership_path(),
                "digital twin workflow snapshot must match the requested studio ownership scope",
            )?;
        }
        Ok(Self {
            ownership_path,
            runtime_state_snapshots,
            workflow_state_snapshots,
            task_state_snapshots,
            memory_projections,
            captured_at,
            studio_audit_reference,
        })
    }

    pub fn ownership_path(&self) -> &OwnershipPath {
        &self.ownership_path
    }
    pub fn runtime_state_snapshots(&self) -> &[RuntimeStateSnapshot] {
        &self.runtime_state_snapshots
    }
    pub fn workflow_state_snapshots(&self) -> &[WorkflowStateSnapshot] {
        &self.workflow_state_snapshots
    }
    pub fn task_state_snapshots(&self) -> &[TaskStateSnapshot] {
        &self.task_state_snapshots
    }
    pub fn memory_projections(&self) -> &[MemoryProjection] {
        &self.memory_projections
    }
    pub fn captured_at(&self) -> &TimeReference {
        &self.captured_at
    }
    pub fn studio_audit_reference(&self) -> &StudioAuditReference {
        &self.studio_audit_reference
    }
}
