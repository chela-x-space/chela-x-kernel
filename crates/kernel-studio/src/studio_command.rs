use kernel_domain::{
    CorrelationId, ExecutionSessionId, MemoryRecordReference, TaskInstanceReference, TimeReference,
    WorkflowId,
};
use kernel_gateway::{GatewayCommandPayload, GatewayCommandResponse, GatewayRequestEnvelope};

use crate::studio::{
    StudioApiVersion, StudioAuditReference, StudioError, StudioErrorCode, StudioResult,
    StudioSelectionContext, StudioViewKind, StudioViewReference,
};
use crate::studio_validation::{require_correlation, require_exact_scope};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudioCommandRequest {
    studio_api_version: StudioApiVersion,
    studio_view_reference: StudioViewReference,
    studio_selection_context: StudioSelectionContext,
    correlation_id: CorrelationId,
    requested_at: TimeReference,
    gateway_request_envelope: Box<GatewayRequestEnvelope>,
    studio_audit_reference: StudioAuditReference,
}

impl StudioCommandRequest {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        studio_api_version: StudioApiVersion,
        studio_view_reference: StudioViewReference,
        studio_selection_context: StudioSelectionContext,
        correlation_id: CorrelationId,
        requested_at: TimeReference,
        gateway_request_envelope: GatewayRequestEnvelope,
        studio_audit_reference: StudioAuditReference,
    ) -> StudioResult<Self> {
        if studio_view_reference.view_kind() != StudioViewKind::CommandConsole {
            return Err(StudioError::new(
                StudioErrorCode::CommandOperationMismatch,
                "studio command requests require the command console view",
            )?);
        }
        let GatewayRequestEnvelope::Command {
            gateway_request_context,
            gateway_command_request,
        } = &gateway_request_envelope
        else {
            return Err(StudioError::new(
                StudioErrorCode::CommandOperationMismatch,
                "studio command requests require a gateway command envelope",
            )?);
        };
        require_correlation(
            &correlation_id,
            gateway_request_context.correlation_id(),
            "studio command correlation must match the gateway command correlation reference",
        )?;
        require_exact_scope(
            studio_selection_context.ownership_path(),
            gateway_request_context.ownership_path(),
            "studio command scope must match the gateway command ownership path",
        )?;
        if studio_audit_reference.correlation_id() != Some(&correlation_id) {
            return Err(StudioError::new(
                StudioErrorCode::AuditReferenceMismatch,
                "studio command audit correlation must match the studio command correlation reference",
            )?);
        }
        match gateway_command_request.gateway_command_payload() {
            GatewayCommandPayload::WorkflowTransition(workflow_transition_request) => {
                let selected_workflow_id: &WorkflowId = studio_selection_context
                    .selected_workflow_id()
                    .ok_or_else(|| {
                        StudioError::new(
                            StudioErrorCode::InvalidSelection,
                            "workflow studio commands require a selected workflow identifier",
                        )
                        .expect("studio error")
                    })?;
                if workflow_transition_request
                    .current_workflow_state_snapshot()
                    .workflow_id()
                    != selected_workflow_id
                {
                    return Err(StudioError::new(
                        StudioErrorCode::InvalidSelection,
                        "workflow studio command selection must match the gateway workflow transition target",
                    )?);
                }
            }
            GatewayCommandPayload::TaskTransition(task_transition_request) => {
                let selected_task_instance_reference: &TaskInstanceReference =
                    studio_selection_context
                        .selected_task_instance_reference()
                        .ok_or_else(|| {
                            StudioError::new(
                                StudioErrorCode::InvalidSelection,
                                "task studio commands require a selected task instance reference",
                            )
                            .expect("studio error")
                        })?;
                if task_transition_request
                    .current_task_state_snapshot()
                    .task_instance_reference()
                    != selected_task_instance_reference
                {
                    return Err(StudioError::new(
                        StudioErrorCode::InvalidSelection,
                        "task studio command selection must match the gateway task transition target",
                    )?);
                }
            }
            GatewayCommandPayload::Execution(execution_request) => {
                let selected_execution_session_id: &ExecutionSessionId = studio_selection_context
                    .selected_execution_session_id()
                    .ok_or_else(|| {
                        StudioError::new(
                            StudioErrorCode::InvalidSelection,
                            "execution studio commands require a selected execution session identifier",
                        )
                        .expect("studio error")
                    })?;
                if execution_request.execution_session_id() != selected_execution_session_id {
                    return Err(StudioError::new(
                        StudioErrorCode::InvalidSelection,
                        "execution studio command selection must match the gateway execution target",
                    )?);
                }
            }
            GatewayCommandPayload::MemoryCapture(memory_capture_request) => {
                let selected_memory_record_reference: &MemoryRecordReference =
                    studio_selection_context
                        .selected_memory_record_reference()
                        .ok_or_else(|| {
                            StudioError::new(
                                StudioErrorCode::InvalidSelection,
                                "memory studio commands require a selected memory record reference",
                            )
                            .expect("studio error")
                        })?;
                if memory_capture_request
                    .memory_record()
                    .memory_record_reference()
                    != selected_memory_record_reference
                {
                    return Err(StudioError::new(
                        StudioErrorCode::InvalidSelection,
                        "memory studio command selection must match the gateway memory capture target",
                    )?);
                }
            }
        }
        Ok(Self {
            studio_api_version,
            studio_view_reference,
            studio_selection_context,
            correlation_id,
            requested_at,
            gateway_request_envelope: Box::new(gateway_request_envelope),
            studio_audit_reference,
        })
    }

    pub fn correlation_id(&self) -> &CorrelationId {
        &self.correlation_id
    }
    pub fn gateway_request_envelope(&self) -> &GatewayRequestEnvelope {
        &self.gateway_request_envelope
    }
    pub fn studio_selection_context(&self) -> &StudioSelectionContext {
        &self.studio_selection_context
    }
    pub fn studio_view_reference(&self) -> &StudioViewReference {
        &self.studio_view_reference
    }
    pub fn studio_audit_reference(&self) -> &StudioAuditReference {
        &self.studio_audit_reference
    }
    pub fn studio_api_version(&self) -> &StudioApiVersion {
        &self.studio_api_version
    }
    pub fn requested_at(&self) -> &TimeReference {
        &self.requested_at
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudioCommandResponse {
    studio_view_reference: StudioViewReference,
    correlation_id: CorrelationId,
    gateway_command_response: GatewayCommandResponse,
    studio_audit_reference: StudioAuditReference,
    responded_at: TimeReference,
}

impl StudioCommandResponse {
    pub fn new(
        studio_command_request: &StudioCommandRequest,
        correlation_id: CorrelationId,
        gateway_command_response: GatewayCommandResponse,
        studio_audit_reference: StudioAuditReference,
        responded_at: TimeReference,
    ) -> StudioResult<Self> {
        require_correlation(
            studio_command_request.correlation_id(),
            &correlation_id,
            "studio command response correlation must match the original studio command correlation reference",
        )?;
        if studio_audit_reference.correlation_id() != Some(&correlation_id) {
            return Err(StudioError::new(
                StudioErrorCode::AuditReferenceMismatch,
                "studio command response audit correlation must match the studio response correlation reference",
            )?);
        }
        Ok(Self {
            studio_view_reference: studio_command_request.studio_view_reference().clone(),
            correlation_id,
            gateway_command_response,
            studio_audit_reference,
            responded_at,
        })
    }

    pub fn correlation_id(&self) -> &CorrelationId {
        &self.correlation_id
    }
}
