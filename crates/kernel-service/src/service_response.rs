use kernel_application::ApplicationResponseEnvelope;

use crate::service::ServiceResponseKind;
use crate::service_context::ServiceRequestContext;
use crate::service_error::{ServiceErrorCode, ServiceResult};
use crate::service_identity::ServiceIdentity;
use crate::service_validation::{
    require_application_response_matches_request, validate_namespaced_identifier,
    validate_non_empty_text,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceResponseStatusReference(String);

impl ServiceResponseStatusReference {
    pub fn new(value: impl Into<String>) -> ServiceResult<Self> {
        Ok(Self(validate_namespaced_identifier(
            value,
            ServiceErrorCode::ResponseRequestMismatch,
            "service response status references require namespaced logical identifiers",
        )?))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceResponseEnvelope {
    service_request_id: crate::ServiceRequestId,
    service_identity: ServiceIdentity,
    service_response_kind: ServiceResponseKind,
    service_response_status_reference: ServiceResponseStatusReference,
    application_response_envelope: ApplicationResponseEnvelope,
    responded_at: String,
}

impl ServiceResponseEnvelope {
    pub fn new(
        service_request_context: &ServiceRequestContext,
        application_response_envelope: ApplicationResponseEnvelope,
        service_response_status_reference: ServiceResponseStatusReference,
        responded_at: impl Into<String>,
    ) -> ServiceResult<Self> {
        require_application_response_matches_request(
            service_request_context.application_request_context(),
            &application_response_envelope,
            ServiceErrorCode::ResponseRequestMismatch,
            "service responses must preserve the original application request identity and correlation",
        )?;
        Ok(Self {
            service_request_id: service_request_context.service_request_id().clone(),
            service_identity: service_request_context.service_identity().clone(),
            service_response_kind: application_response_envelope
                .application_response_kind()
                .into(),
            service_response_status_reference,
            application_response_envelope,
            responded_at: validate_non_empty_text(
                responded_at,
                ServiceErrorCode::ResponseRequestMismatch,
                "service responses require a caller-supplied response time reference",
            )?,
        })
    }

    pub fn service_request_id(&self) -> &crate::ServiceRequestId {
        &self.service_request_id
    }

    pub fn service_identity(&self) -> &ServiceIdentity {
        &self.service_identity
    }

    pub fn service_response_kind(&self) -> ServiceResponseKind {
        self.service_response_kind
    }

    pub fn application_response_envelope(&self) -> &ApplicationResponseEnvelope {
        &self.application_response_envelope
    }
}
