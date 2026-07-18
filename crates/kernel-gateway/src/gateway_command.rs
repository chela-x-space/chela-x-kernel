use kernel_domain::{
    ExecutionOutcome, ExecutionRequest, MemoryCaptureDecision, MemoryCaptureRequest,
    TaskTransitionDecision, TaskTransitionRequest, WorkflowTransitionControlRequest,
    WorkflowTransitionDecision,
};

use crate::gateway_contract::GatewayOperationKind;
use crate::gateway_error::{GatewayError, GatewayErrorCode, GatewayResult};
use crate::gateway_request::GatewayRequestContext;
use crate::gateway_validation::require_exact_scope;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GatewayCommandPayload {
    WorkflowTransition(Box<WorkflowTransitionControlRequest>),
    TaskTransition(Box<TaskTransitionRequest>),
    Execution(Box<ExecutionRequest>),
    MemoryCapture(Box<MemoryCaptureRequest>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayCommandRequest {
    gateway_command_payload: GatewayCommandPayload,
}

impl GatewayCommandRequest {
    pub fn new(gateway_command_payload: GatewayCommandPayload) -> Self {
        Self {
            gateway_command_payload,
        }
    }

    pub(crate) fn validate_against_context(
        &self,
        gateway_request_context: &GatewayRequestContext,
    ) -> GatewayResult<()> {
        if gateway_request_context
            .gateway_operation_reference()
            .operation_kind()
            != GatewayOperationKind::Command
        {
            return Err(GatewayError::new(
                GatewayErrorCode::InvalidCommand,
                "gateway command request requires a command operation reference",
            )?);
        }
        match &self.gateway_command_payload {
            GatewayCommandPayload::WorkflowTransition(workflow_transition_request) => {
                if gateway_request_context
                    .gateway_operation_reference()
                    .resource_segment()
                    != "workflow"
                {
                    return Err(GatewayError::new(
                        GatewayErrorCode::InvalidCommand,
                        "gateway command operation must target workflow requests",
                    )?);
                }
                require_exact_scope(
                    gateway_request_context.ownership_path(),
                    workflow_transition_request
                        .current_workflow_state_snapshot()
                        .ownership_path(),
                    "workflow transition ownership scope must match the gateway request context",
                )?;
            }
            GatewayCommandPayload::TaskTransition(task_transition_request) => {
                if gateway_request_context
                    .gateway_operation_reference()
                    .resource_segment()
                    != "task"
                {
                    return Err(GatewayError::new(
                        GatewayErrorCode::InvalidCommand,
                        "gateway command operation must target task requests",
                    )?);
                }
                if gateway_request_context
                    .authorization_request_record()
                    .target()
                    .resource_identifier()
                    != task_transition_request
                        .current_task_state_snapshot()
                        .task_instance_reference()
                        .task_instance_id()
                        .as_str()
                {
                    return Err(GatewayError::new(
                        GatewayErrorCode::ScopeMismatch,
                        "task transition resource identifier must match the gateway authorization target",
                    )?);
                }
            }
            GatewayCommandPayload::Execution(execution_request) => {
                if gateway_request_context
                    .gateway_operation_reference()
                    .resource_segment()
                    != "execution"
                {
                    return Err(GatewayError::new(
                        GatewayErrorCode::InvalidCommand,
                        "gateway command operation must target execution requests",
                    )?);
                }
                if gateway_request_context
                    .authorization_request_record()
                    .target()
                    .resource_identifier()
                    != execution_request.execution_session_id().as_str()
                {
                    return Err(GatewayError::new(
                        GatewayErrorCode::ScopeMismatch,
                        "execution resource identifier must match the gateway authorization target",
                    )?);
                }
            }
            GatewayCommandPayload::MemoryCapture(memory_capture_request) => {
                if gateway_request_context
                    .gateway_operation_reference()
                    .resource_segment()
                    != "memory"
                {
                    return Err(GatewayError::new(
                        GatewayErrorCode::InvalidCommand,
                        "gateway command operation must target memory requests",
                    )?);
                }
                require_exact_scope(
                    gateway_request_context.ownership_path(),
                    memory_capture_request.memory_record().ownership_path(),
                    "memory capture ownership scope must match the gateway request context",
                )?;
            }
        }
        Ok(())
    }

    pub fn gateway_command_payload(&self) -> &GatewayCommandPayload {
        &self.gateway_command_payload
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GatewayCommandResponse {
    WorkflowTransition(Box<WorkflowTransitionDecision>),
    TaskTransition(Box<TaskTransitionDecision>),
    ExecutionOutcome(Box<ExecutionOutcome>),
    MemoryCapture(Box<MemoryCaptureDecision>),
    Rejected(Box<crate::GatewayError>),
}
