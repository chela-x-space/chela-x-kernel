use kernel_domain::{ExecutionSessionId, RuntimeId, RuntimeStateSnapshot};

use crate::studio::{StudioAuditReference, StudioError, StudioErrorCode, StudioResult};
use crate::studio_validation::reject_duplicates;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudioRuntimeProjection {
    selected_runtime_id: RuntimeId,
    runtime_state_snapshots: Vec<RuntimeStateSnapshot>,
    current_execution_session_ids: Vec<ExecutionSessionId>,
    studio_audit_reference: StudioAuditReference,
}

impl StudioRuntimeProjection {
    pub fn new(
        selected_runtime_id: RuntimeId,
        runtime_state_snapshots: Vec<RuntimeStateSnapshot>,
        current_execution_session_ids: Vec<ExecutionSessionId>,
        studio_audit_reference: StudioAuditReference,
    ) -> StudioResult<Self> {
        if runtime_state_snapshots.is_empty() {
            return Err(StudioError::new(
                StudioErrorCode::ProjectionMismatch,
                "studio runtime projection requires at least one runtime state snapshot",
            )?);
        }
        if runtime_state_snapshots
            .iter()
            .any(|snapshot| snapshot.runtime_id() != &selected_runtime_id)
        {
            return Err(StudioError::new(
                StudioErrorCode::RuntimeProjectionMismatch,
                "studio runtime projection snapshots must preserve the selected runtime identifier",
            )?);
        }
        reject_duplicates(
            &current_execution_session_ids,
            StudioErrorCode::ProjectionMismatch,
            "duplicate execution session identifier in studio runtime projection",
        )?;
        Ok(Self {
            selected_runtime_id,
            runtime_state_snapshots,
            current_execution_session_ids,
            studio_audit_reference,
        })
    }

    pub fn selected_runtime_id(&self) -> &RuntimeId {
        &self.selected_runtime_id
    }
    pub fn runtime_state_snapshots(&self) -> &[RuntimeStateSnapshot] {
        &self.runtime_state_snapshots
    }
    pub fn current_execution_session_ids(&self) -> &[ExecutionSessionId] {
        &self.current_execution_session_ids
    }
    pub fn studio_audit_reference(&self) -> &StudioAuditReference {
        &self.studio_audit_reference
    }
}
