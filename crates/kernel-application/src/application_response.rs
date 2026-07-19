use kernel_domain::{CorrelationId, EnglishNamespace, OwnershipPath, TimeReference};
use kernel_studio::{StudioCommandResponse, StudioResponseEnvelope, StudioViewResponse};

use crate::application::{ApplicationRequestEnvelope, ApplicationResponseKind};
use crate::application_context::{ApplicationAuditReference, ApplicationRequestId};
use crate::application_error::{ApplicationError, ApplicationErrorCode, ApplicationResult};
use crate::application_identity::ApplicationIdentity;
use crate::application_validation::require_correlation;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationResponseStatusReference(EnglishNamespace);

impl ApplicationResponseStatusReference {
    pub fn new(value: impl Into<String>) -> ApplicationResult<Self> {
        Ok(Self(
            EnglishNamespace::new("application_response_status_reference", value)
                .map_err(ApplicationError::from_domain_rejection)?,
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApplicationResponsePayload {
    View(Box<StudioViewResponse>),
    Command(Box<StudioCommandResponse>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationResponseEnvelope {
    application_request_id: ApplicationRequestId,
    application_identity: ApplicationIdentity,
    application_response_kind: ApplicationResponseKind,
    application_response_status_reference: ApplicationResponseStatusReference,
    correlation_id: CorrelationId,
    ownership_path: OwnershipPath,
    studio_response_envelope: StudioResponseEnvelope,
    application_audit_reference: ApplicationAuditReference,
    responded_at: TimeReference,
}

impl ApplicationResponseEnvelope {
    pub fn new(
        application_request_envelope: &ApplicationRequestEnvelope,
        application_response_payload: ApplicationResponsePayload,
        application_response_status_reference: ApplicationResponseStatusReference,
        application_audit_reference: ApplicationAuditReference,
        responded_at: TimeReference,
    ) -> ApplicationResult<Self> {
        require_correlation(
            application_request_envelope.correlation_id(),
            application_audit_reference.correlation_id(),
            ApplicationErrorCode::ResponseCorrelationMismatch,
            "application response audit correlation must match the application request correlation reference",
        )?;
        let correlation_id = application_request_envelope.correlation_id().clone();
        let studio_response_envelope = match (
            application_request_envelope,
            application_response_payload,
        ) {
            (
                ApplicationRequestEnvelope::Command(command_intent),
                ApplicationResponsePayload::Command(studio_command_response),
            ) => {
                require_correlation(
                    &correlation_id,
                    studio_command_response.correlation_id(),
                    ApplicationErrorCode::ResponseCorrelationMismatch,
                    "application command responses must preserve the Studio command correlation reference",
                )?;
                let _ = command_intent;
                StudioResponseEnvelope::command(*studio_command_response)
            }
            (
                ApplicationRequestEnvelope::Query(_),
                ApplicationResponsePayload::View(studio_view_response),
            ) => StudioResponseEnvelope::view(*studio_view_response),
            (ApplicationRequestEnvelope::Command(_), ApplicationResponsePayload::View(_))
            | (ApplicationRequestEnvelope::Query(_), ApplicationResponsePayload::Command(_)) => {
                return Err(ApplicationError::new(
                    ApplicationErrorCode::ResponseRequestMismatch,
                    "application response payloads must preserve the original command or query intent",
                )?);
            }
        };
        let application_response_kind = match studio_response_envelope {
            StudioResponseEnvelope::View(_) => ApplicationResponseKind::View,
            StudioResponseEnvelope::Command(_) => ApplicationResponseKind::Command,
        };
        Ok(Self {
            application_request_id: application_request_envelope
                .application_request_id()
                .clone(),
            application_identity: application_request_envelope.application_identity().clone(),
            application_response_kind,
            application_response_status_reference,
            correlation_id,
            ownership_path: application_request_envelope.ownership_path().clone(),
            studio_response_envelope,
            application_audit_reference,
            responded_at,
        })
    }

    pub fn application_request_id(&self) -> &ApplicationRequestId {
        &self.application_request_id
    }

    pub fn application_identity(&self) -> &ApplicationIdentity {
        &self.application_identity
    }

    pub fn application_response_kind(&self) -> ApplicationResponseKind {
        self.application_response_kind
    }

    pub fn correlation_id(&self) -> &CorrelationId {
        &self.correlation_id
    }

    pub fn studio_response_envelope(&self) -> &StudioResponseEnvelope {
        &self.studio_response_envelope
    }
}
