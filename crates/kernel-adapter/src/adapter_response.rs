use kernel_service::{ServiceResponseEnvelope, ServiceResponseKind};

use crate::adapter::AdapterResponseKind;
use crate::adapter_error::{AdapterError, AdapterErrorCode, AdapterResult};
use crate::adapter_identity::AdapterIdentity;
use crate::adapter_request::AdapterRequestEnvelope;
use crate::adapter_validation::{
    require_correlation_continuity, validate_namespaced_identifier, validate_non_empty_text,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterResponseStatusReference(String);

impl AdapterResponseStatusReference {
    pub fn new(value: impl Into<String>) -> AdapterResult<Self> {
        Ok(Self(validate_namespaced_identifier(
            value,
            AdapterErrorCode::ResponseRequestMismatch,
            "adapter response status references require namespaced logical identifiers",
        )?))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterResponseEnvelope {
    adapter_request_id: crate::AdapterRequestId,
    adapter_identity: AdapterIdentity,
    adapter_response_kind: AdapterResponseKind,
    adapter_response_status_reference: AdapterResponseStatusReference,
    service_response_envelope: ServiceResponseEnvelope,
    responded_at: String,
}

impl AdapterResponseEnvelope {
    pub fn new(
        adapter_request_envelope: &AdapterRequestEnvelope,
        service_response_envelope: ServiceResponseEnvelope,
        adapter_response_status_reference: AdapterResponseStatusReference,
        responded_at: impl Into<String>,
    ) -> AdapterResult<Self> {
        let service_request_context = adapter_request_envelope.service_request_context();
        if service_response_envelope.service_request_id()
            != service_request_context.service_request_id()
            || service_response_envelope.service_identity()
                != service_request_context.service_identity()
        {
            return Err(AdapterError::new(
                AdapterErrorCode::ResponseRequestMismatch,
                "adapter responses must preserve the original service request identity and service identity",
            )?);
        }
        require_correlation_continuity(
            service_request_context,
            &service_response_envelope,
            AdapterErrorCode::CorrelationMismatch,
            "adapter responses must preserve the original service correlation reference",
        )?;
        match (
            adapter_request_envelope,
            service_response_envelope.service_response_kind(),
        ) {
            (AdapterRequestEnvelope::Command(_), ServiceResponseKind::Command)
            | (AdapterRequestEnvelope::Query(_), ServiceResponseKind::View) => {}
            (AdapterRequestEnvelope::Command(_), ServiceResponseKind::View)
            | (AdapterRequestEnvelope::Query(_), ServiceResponseKind::Command)
            | (_, ServiceResponseKind::Error) => {
                return Err(AdapterError::new(
                    AdapterErrorCode::ResponseRequestMismatch,
                    "adapter response kinds must preserve the original command or query intent",
                )?);
            }
        }
        Ok(Self {
            adapter_request_id: adapter_request_envelope.adapter_request_id().clone(),
            adapter_identity: adapter_request_envelope.adapter_identity().clone(),
            adapter_response_kind: service_response_envelope.service_response_kind().into(),
            adapter_response_status_reference,
            service_response_envelope,
            responded_at: validate_non_empty_text(
                responded_at,
                AdapterErrorCode::ResponseRequestMismatch,
                "adapter responses require a caller-supplied response time reference",
            )?,
        })
    }

    pub fn adapter_response_kind(&self) -> AdapterResponseKind {
        self.adapter_response_kind
    }

    pub fn service_response_envelope(&self) -> &ServiceResponseEnvelope {
        &self.service_response_envelope
    }
}
