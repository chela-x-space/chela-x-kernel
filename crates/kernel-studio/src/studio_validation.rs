use kernel_domain::{CorrelationId, OwnershipPath};
use kernel_gateway::{GatewayQueryPayload, GatewayRequestEnvelope, GatewayResponseEnvelope};

use crate::studio::{StudioError, StudioErrorCode, StudioResult, StudioViewKind};

pub fn reject_duplicates<T: PartialEq>(
    values: &[T],
    code: StudioErrorCode,
    detail: &'static str,
) -> StudioResult<()> {
    if values
        .iter()
        .enumerate()
        .any(|(index, value)| values[..index].iter().any(|prior| prior == value))
    {
        return Err(StudioError::new(code, detail)?);
    }
    Ok(())
}

pub fn require_exact_scope(
    expected_ownership_path: &OwnershipPath,
    actual_ownership_path: &OwnershipPath,
    detail: &'static str,
) -> StudioResult<()> {
    if expected_ownership_path.enterprise_id() != actual_ownership_path.enterprise_id()
        || expected_ownership_path.workspace_id() != actual_ownership_path.workspace_id()
        || expected_ownership_path.project_id() != actual_ownership_path.project_id()
        || expected_ownership_path.organizational_unit_id()
            != actual_ownership_path.organizational_unit_id()
    {
        return Err(StudioError::new(StudioErrorCode::ScopeMismatch, detail)?);
    }
    Ok(())
}

pub fn require_correlation(
    expected: &CorrelationId,
    actual: &CorrelationId,
    detail: &'static str,
) -> StudioResult<()> {
    if expected != actual {
        return Err(StudioError::new(
            StudioErrorCode::ResponseCorrelationMismatch,
            detail,
        )?);
    }
    Ok(())
}

pub fn require_query_only(
    gateway_request_envelopes: &[GatewayRequestEnvelope],
) -> StudioResult<()> {
    if gateway_request_envelopes.is_empty() {
        return Err(StudioError::new(
            StudioErrorCode::InvalidStudioRequest,
            "studio view requests require at least one gateway query envelope",
        )?);
    }
    if gateway_request_envelopes
        .iter()
        .any(|envelope| !matches!(envelope, GatewayRequestEnvelope::Query { .. }))
    {
        return Err(StudioError::new(
            StudioErrorCode::ViewQueryMismatch,
            "studio view requests require query-only gateway envelopes",
        )?);
    }
    Ok(())
}

pub fn require_query_responses_only(
    gateway_response_envelopes: &[GatewayResponseEnvelope],
) -> StudioResult<()> {
    if gateway_response_envelopes.is_empty() {
        return Err(StudioError::new(
            StudioErrorCode::ProjectionMismatch,
            "studio view responses require at least one gateway query response envelope",
        )?);
    }
    if gateway_response_envelopes
        .iter()
        .any(|envelope| !matches!(envelope, GatewayResponseEnvelope::Query { .. }))
    {
        return Err(StudioError::new(
            StudioErrorCode::ProjectionMismatch,
            "studio view responses require query-only gateway response envelopes",
        )?);
    }
    Ok(())
}

pub fn require_view_query_support(
    studio_view_kind: StudioViewKind,
    gateway_query_payload: &GatewayQueryPayload,
) -> StudioResult<()> {
    let supported = match studio_view_kind {
        StudioViewKind::TopView => {
            matches!(gateway_query_payload, GatewayQueryPayload::GatewayStatus)
        }
        StudioViewKind::DigitalTwin => {
            matches!(
                gateway_query_payload,
                GatewayQueryPayload::RuntimeSnapshot(_)
                    | GatewayQueryPayload::WorkflowState(_)
                    | GatewayQueryPayload::TaskState(_)
                    | GatewayQueryPayload::MemoryRetrieval(_)
                    | GatewayQueryPayload::MemoryQuery(_)
            )
        }
        StudioViewKind::Runtime => {
            matches!(
                gateway_query_payload,
                GatewayQueryPayload::RuntimeSnapshot(_) | GatewayQueryPayload::ExecutionSession(_)
            )
        }
        StudioViewKind::Workflow => {
            matches!(
                gateway_query_payload,
                GatewayQueryPayload::WorkflowState(_) | GatewayQueryPayload::TaskState(_)
            )
        }
        StudioViewKind::Task => {
            matches!(
                gateway_query_payload,
                GatewayQueryPayload::TaskState(_) | GatewayQueryPayload::ExecutionSession(_)
            )
        }
        StudioViewKind::EventTimeline => {
            matches!(gateway_query_payload, GatewayQueryPayload::MemoryQuery(_))
        }
        StudioViewKind::Memory => {
            matches!(
                gateway_query_payload,
                GatewayQueryPayload::MemoryRetrieval(_) | GatewayQueryPayload::MemoryQuery(_)
            )
        }
        StudioViewKind::Audit => {
            matches!(
                gateway_query_payload,
                GatewayQueryPayload::MemoryRetrieval(_)
                    | GatewayQueryPayload::MemoryQuery(_)
                    | GatewayQueryPayload::ExecutionSession(_)
            )
        }
        StudioViewKind::Revenue => {
            matches!(
                gateway_query_payload,
                GatewayQueryPayload::MemoryQuery(_) | GatewayQueryPayload::GatewayStatus
            )
        }
        StudioViewKind::CommandConsole => false,
    };
    if !supported {
        return Err(StudioError::new(
            StudioErrorCode::UnsupportedView,
            "gateway query payload is not supported by the requested Studio view",
        )?);
    }
    Ok(())
}
