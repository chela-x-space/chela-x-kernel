use kernel_domain::{
    ExecutionSession, ExecutionSessionId, MemoryProjection, MemoryQuery, MemoryQueryResult,
    MemoryRetrievalRequest, MemoryRetrievalResult, RuntimeId, RuntimeStateSnapshot,
    TaskInstanceReference, TaskStateSnapshot, WorkflowId, WorkflowStateSnapshot,
};

use crate::gateway::GatewayStatusSnapshot;
use crate::gateway_contract::GatewayOperationKind;
use crate::gateway_error::{GatewayError, GatewayErrorCode, GatewayResult};
use crate::gateway_request::GatewayRequestContext;
use crate::gateway_validation::require_exact_scope;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GatewayQueryPayload {
    RuntimeSnapshot(RuntimeId),
    WorkflowState(WorkflowId),
    TaskState(TaskInstanceReference),
    ExecutionSession(ExecutionSessionId),
    MemoryRetrieval(Box<MemoryRetrievalRequest>),
    MemoryQuery(MemoryQuery),
    GatewayStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayQueryRequest {
    gateway_query_payload: GatewayQueryPayload,
}

impl GatewayQueryRequest {
    pub fn new(gateway_query_payload: GatewayQueryPayload) -> Self {
        Self {
            gateway_query_payload,
        }
    }

    pub(crate) fn validate_against_context(
        &self,
        gateway_request_context: &GatewayRequestContext,
    ) -> GatewayResult<()> {
        let operation_kind = gateway_request_context
            .gateway_operation_reference()
            .operation_kind();
        if !matches!(
            operation_kind,
            GatewayOperationKind::Query | GatewayOperationKind::Status
        ) {
            return Err(GatewayError::new(
                GatewayErrorCode::InvalidQuery,
                "gateway query request requires a query or status operation reference",
            )?);
        }
        match &self.gateway_query_payload {
            GatewayQueryPayload::RuntimeSnapshot(runtime_id) => {
                if gateway_request_context
                    .gateway_operation_reference()
                    .resource_segment()
                    != "runtime"
                    || operation_kind != GatewayOperationKind::Query
                {
                    return Err(GatewayError::new(
                        GatewayErrorCode::InvalidQuery,
                        "runtime snapshot query requires a runtime query operation reference",
                    )?);
                }
                if gateway_request_context
                    .authorization_request_record()
                    .target()
                    .resource_identifier()
                    != runtime_id.as_str()
                {
                    return Err(GatewayError::new(
                        GatewayErrorCode::ScopeMismatch,
                        "runtime query resource identifier must match the gateway authorization target",
                    )?);
                }
            }
            GatewayQueryPayload::WorkflowState(workflow_id) => {
                if gateway_request_context
                    .gateway_operation_reference()
                    .resource_segment()
                    != "workflow"
                    || operation_kind != GatewayOperationKind::Query
                {
                    return Err(GatewayError::new(
                        GatewayErrorCode::InvalidQuery,
                        "workflow state query requires a workflow query operation reference",
                    )?);
                }
                if gateway_request_context
                    .authorization_request_record()
                    .target()
                    .resource_identifier()
                    != workflow_id.as_str()
                {
                    return Err(GatewayError::new(
                        GatewayErrorCode::ScopeMismatch,
                        "workflow query resource identifier must match the gateway authorization target",
                    )?);
                }
            }
            GatewayQueryPayload::TaskState(task_instance_reference) => {
                if gateway_request_context
                    .gateway_operation_reference()
                    .resource_segment()
                    != "task"
                    || operation_kind != GatewayOperationKind::Query
                {
                    return Err(GatewayError::new(
                        GatewayErrorCode::InvalidQuery,
                        "task state query requires a task query operation reference",
                    )?);
                }
                if gateway_request_context
                    .authorization_request_record()
                    .target()
                    .resource_identifier()
                    != task_instance_reference.task_instance_id().as_str()
                {
                    return Err(GatewayError::new(
                        GatewayErrorCode::ScopeMismatch,
                        "task query resource identifier must match the gateway authorization target",
                    )?);
                }
            }
            GatewayQueryPayload::ExecutionSession(execution_session_id) => {
                if gateway_request_context
                    .gateway_operation_reference()
                    .resource_segment()
                    != "execution"
                    || operation_kind != GatewayOperationKind::Query
                {
                    return Err(GatewayError::new(
                        GatewayErrorCode::InvalidQuery,
                        "execution session query requires an execution query operation reference",
                    )?);
                }
                if gateway_request_context
                    .authorization_request_record()
                    .target()
                    .resource_identifier()
                    != execution_session_id.as_str()
                {
                    return Err(GatewayError::new(
                        GatewayErrorCode::ScopeMismatch,
                        "execution query resource identifier must match the gateway authorization target",
                    )?);
                }
            }
            GatewayQueryPayload::MemoryRetrieval(memory_retrieval_request) => {
                if gateway_request_context
                    .gateway_operation_reference()
                    .resource_segment()
                    != "memory"
                    || operation_kind != GatewayOperationKind::Query
                {
                    return Err(GatewayError::new(
                        GatewayErrorCode::InvalidQuery,
                        "memory retrieval requires a memory query operation reference",
                    )?);
                }
                require_exact_scope(
                    gateway_request_context.ownership_path(),
                    memory_retrieval_request.ownership_path(),
                    "memory retrieval ownership scope must match the gateway request context",
                )?;
                if gateway_request_context.authorization_decision_reference()
                    != memory_retrieval_request.authorization_decision_reference()
                {
                    return Err(GatewayError::new(
                        GatewayErrorCode::AuthorizationDenied,
                        "memory retrieval authorization decision must match the gateway request context",
                    )?);
                }
            }
            GatewayQueryPayload::MemoryQuery(_) => {
                if gateway_request_context
                    .gateway_operation_reference()
                    .resource_segment()
                    != "memory"
                    || operation_kind != GatewayOperationKind::Query
                {
                    return Err(GatewayError::new(
                        GatewayErrorCode::InvalidQuery,
                        "memory query requires a memory query operation reference",
                    )?);
                }
            }
            GatewayQueryPayload::GatewayStatus => {
                if gateway_request_context
                    .gateway_operation_reference()
                    .resource_segment()
                    != "gateway"
                    || operation_kind != GatewayOperationKind::Status
                {
                    return Err(GatewayError::new(
                        GatewayErrorCode::InvalidQuery,
                        "gateway status query requires a gateway status operation reference",
                    )?);
                }
            }
        }
        Ok(())
    }

    pub fn gateway_query_payload(&self) -> &GatewayQueryPayload {
        &self.gateway_query_payload
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GatewayQueryResponse {
    RuntimeStateSnapshots(Vec<RuntimeStateSnapshot>),
    WorkflowStateSnapshots(Vec<WorkflowStateSnapshot>),
    TaskStateSnapshots(Vec<TaskStateSnapshot>),
    ExecutionSessions(Vec<ExecutionSession>),
    MemoryRetrieval(Box<MemoryRetrievalResult>),
    MemoryQuery(Box<MemoryQueryResult>),
    MemoryProjections(Vec<MemoryProjection>),
    StatusSnapshot(GatewayStatusSnapshot),
    Rejected(crate::GatewayError),
}
