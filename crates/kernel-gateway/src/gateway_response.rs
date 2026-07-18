use kernel_domain::{CorrelationId, TimeReference};

use crate::gateway::GatewayAuditReference;
use crate::gateway_command::{GatewayCommandPayload, GatewayCommandResponse};
use crate::gateway_contract::GatewayApiVersion;
use crate::gateway_error::{GatewayError, GatewayErrorCode, GatewayResult};
use crate::gateway_query::{GatewayQueryPayload, GatewayQueryResponse};
use crate::gateway_request::GatewayRequestEnvelope;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GatewayResponseEnvelope {
    Command {
        gateway_api_version: GatewayApiVersion,
        gateway_operation_reference: crate::GatewayOperationReference,
        correlation_id: CorrelationId,
        gateway_command_response: Box<GatewayCommandResponse>,
        gateway_audit_reference: Box<GatewayAuditReference>,
        responded_at: TimeReference,
    },
    Query {
        gateway_api_version: GatewayApiVersion,
        gateway_operation_reference: crate::GatewayOperationReference,
        correlation_id: CorrelationId,
        gateway_query_response: Box<GatewayQueryResponse>,
        gateway_audit_reference: Box<GatewayAuditReference>,
        responded_at: TimeReference,
    },
}

impl GatewayResponseEnvelope {
    pub fn new(
        gateway_request_envelope: &GatewayRequestEnvelope,
        correlation_id: CorrelationId,
        gateway_response: GatewayEnvelopeResponse,
        gateway_audit_reference: GatewayAuditReference,
        responded_at: TimeReference,
    ) -> GatewayResult<Self> {
        if gateway_request_envelope
            .gateway_request_context()
            .correlation_id()
            != &correlation_id
        {
            return Err(GatewayError::new(
                GatewayErrorCode::InvalidRequest,
                "gateway response correlation must match the original request correlation",
            )?);
        }
        if gateway_audit_reference.correlation_id() != Some(&correlation_id) {
            return Err(GatewayError::new(
                GatewayErrorCode::InternalContractViolation,
                "gateway response audit correlation must match the gateway response correlation",
            )?);
        }
        match (gateway_request_envelope, gateway_response) {
            (
                GatewayRequestEnvelope::Command {
                    gateway_request_context,
                    gateway_command_request,
                },
                GatewayEnvelopeResponse::Command(gateway_command_response),
            ) => {
                validate_command_response(gateway_command_request, &gateway_command_response)?;
                Ok(Self::Command {
                    gateway_api_version: gateway_request_context.gateway_api_version().clone(),
                    gateway_operation_reference: gateway_request_context
                        .gateway_operation_reference()
                        .clone(),
                    correlation_id,
                    gateway_command_response,
                    gateway_audit_reference: Box::new(gateway_audit_reference),
                    responded_at,
                })
            }
            (
                GatewayRequestEnvelope::Query {
                    gateway_request_context,
                    gateway_query_request,
                },
                GatewayEnvelopeResponse::Query(gateway_query_response),
            ) => {
                validate_query_response(gateway_query_request, &gateway_query_response)?;
                Ok(Self::Query {
                    gateway_api_version: gateway_request_context.gateway_api_version().clone(),
                    gateway_operation_reference: gateway_request_context
                        .gateway_operation_reference()
                        .clone(),
                    correlation_id,
                    gateway_query_response,
                    gateway_audit_reference: Box::new(gateway_audit_reference),
                    responded_at,
                })
            }
            (GatewayRequestEnvelope::Command { .. }, GatewayEnvelopeResponse::Query(_)) => {
                Err(GatewayError::new(
                    GatewayErrorCode::InvalidCommand,
                    "gateway command request cannot be paired with a query response",
                )?)
            }
            (GatewayRequestEnvelope::Query { .. }, GatewayEnvelopeResponse::Command(_)) => {
                Err(GatewayError::new(
                    GatewayErrorCode::InvalidQuery,
                    "gateway query request cannot be paired with a command response",
                )?)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GatewayEnvelopeResponse {
    Command(Box<GatewayCommandResponse>),
    Query(Box<GatewayQueryResponse>),
}

fn validate_command_response(
    gateway_command_request: &crate::GatewayCommandRequest,
    gateway_command_response: &GatewayCommandResponse,
) -> GatewayResult<()> {
    match (
        gateway_command_request.gateway_command_payload(),
        gateway_command_response,
    ) {
        (
            GatewayCommandPayload::WorkflowTransition(_),
            GatewayCommandResponse::WorkflowTransition(_),
        )
        | (GatewayCommandPayload::TaskTransition(_), GatewayCommandResponse::TaskTransition(_))
        | (GatewayCommandPayload::Execution(_), GatewayCommandResponse::ExecutionOutcome(_))
        | (GatewayCommandPayload::MemoryCapture(_), GatewayCommandResponse::MemoryCapture(_))
        | (_, GatewayCommandResponse::Rejected(_)) => Ok(()),
        _ => Err(GatewayError::new(
            GatewayErrorCode::InvalidCommand,
            "gateway command response must preserve the original command intent",
        )?),
    }
}

fn validate_query_response(
    gateway_query_request: &crate::GatewayQueryRequest,
    gateway_query_response: &GatewayQueryResponse,
) -> GatewayResult<()> {
    match (
        gateway_query_request.gateway_query_payload(),
        gateway_query_response,
    ) {
        (
            GatewayQueryPayload::RuntimeSnapshot(_),
            GatewayQueryResponse::RuntimeStateSnapshots(_),
        )
        | (
            GatewayQueryPayload::WorkflowState(_),
            GatewayQueryResponse::WorkflowStateSnapshots(_),
        )
        | (GatewayQueryPayload::TaskState(_), GatewayQueryResponse::TaskStateSnapshots(_))
        | (GatewayQueryPayload::ExecutionSession(_), GatewayQueryResponse::ExecutionSessions(_))
        | (GatewayQueryPayload::GatewayStatus, GatewayQueryResponse::StatusSnapshot(_))
        | (_, GatewayQueryResponse::Rejected(_)) => Ok(()),
        (
            GatewayQueryPayload::MemoryRetrieval(memory_retrieval_request),
            GatewayQueryResponse::MemoryRetrieval(memory_retrieval_result),
        ) if memory_retrieval_request.as_ref()
            == memory_retrieval_result.memory_retrieval_request() =>
        {
            Ok(())
        }
        (
            GatewayQueryPayload::MemoryQuery(memory_query),
            GatewayQueryResponse::MemoryQuery(memory_query_result),
        ) if memory_query == memory_query_result.memory_query() => Ok(()),
        _ => Err(GatewayError::new(
            GatewayErrorCode::InvalidQuery,
            "gateway query response must preserve the original query intent",
        )?),
    }
}
