use kernel_domain::{
    AuditEvidenceId, AuthorizationSubject, CorrelationId, EnglishNamespace, EventTraceReference,
    OwnershipPath, TimeReference,
};
use kernel_gateway::{GatewayAuthenticationContext, GatewayAuthorizationBinding};
use kernel_studio::{StudioAuditReference, StudioSelectionContext};

use crate::application::ApplicationApiVersion;
use crate::application_capability::ApplicationCapabilityDeclaration;
use crate::application_error::{ApplicationError, ApplicationErrorCode, ApplicationResult};
use crate::application_identity::ApplicationIdentity;
use crate::application_session::ApplicationSessionReference;
use crate::application_validation::{reject_duplicates, require_correlation, require_exact_scope};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationRequestId(EnglishNamespace);

impl ApplicationRequestId {
    pub fn new(value: impl Into<String>) -> ApplicationResult<Self> {
        let value = EnglishNamespace::new("application_request_id", value)
            .map_err(ApplicationError::from_domain_rejection)?;
        if !value.as_str().contains('.') {
            return Err(ApplicationError::new(
                ApplicationErrorCode::ApplicationRequestIdentityMismatch,
                "application request identifiers require namespaced logical identifiers",
            )?);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationAuditReference {
    application_trace_reference: EventTraceReference,
    causation_reference: Option<EventTraceReference>,
    correlation_id: CorrelationId,
    audit_evidence_ids: Vec<AuditEvidenceId>,
    studio_audit_reference: Option<StudioAuditReference>,
}

impl ApplicationAuditReference {
    pub fn new(
        application_trace_reference: EventTraceReference,
        causation_reference: Option<EventTraceReference>,
        correlation_id: CorrelationId,
        audit_evidence_ids: Vec<AuditEvidenceId>,
        studio_audit_reference: Option<StudioAuditReference>,
    ) -> ApplicationResult<Self> {
        if audit_evidence_ids.is_empty() {
            return Err(ApplicationError::new(
                ApplicationErrorCode::AuditEvidenceMismatch,
                "application audit references require at least one evidence identifier",
            )?);
        }
        reject_duplicates(
            &audit_evidence_ids,
            ApplicationErrorCode::AuditEvidenceMismatch,
            "duplicate application audit evidence identifier",
        )?;
        if let Some(studio_audit_reference) = studio_audit_reference.as_ref() {
            if studio_audit_reference.correlation_id() != Some(&correlation_id) {
                return Err(ApplicationError::new(
                    ApplicationErrorCode::AuditEvidenceMismatch,
                    "application audit correlation must match the preserved Studio audit correlation reference",
                )?);
            }
        }
        Ok(Self {
            application_trace_reference,
            causation_reference,
            correlation_id,
            audit_evidence_ids,
            studio_audit_reference,
        })
    }

    pub fn correlation_id(&self) -> &CorrelationId {
        &self.correlation_id
    }

    pub fn causation_reference(&self) -> Option<&EventTraceReference> {
        self.causation_reference.as_ref()
    }

    pub fn audit_evidence_ids(&self) -> &[AuditEvidenceId] {
        &self.audit_evidence_ids
    }

    pub fn studio_audit_reference(&self) -> Option<&StudioAuditReference> {
        self.studio_audit_reference.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationRequestContext {
    application_api_version: ApplicationApiVersion,
    application_request_id: ApplicationRequestId,
    application_identity: ApplicationIdentity,
    application_capability_declaration: ApplicationCapabilityDeclaration,
    gateway_authentication_context: GatewayAuthenticationContext,
    gateway_authorization_binding: GatewayAuthorizationBinding,
    studio_selection_context: StudioSelectionContext,
    application_session_reference: Option<ApplicationSessionReference>,
    correlation_id: CorrelationId,
    causation_reference: Option<EventTraceReference>,
    requested_at: TimeReference,
    application_audit_reference: ApplicationAuditReference,
}

impl ApplicationRequestContext {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        application_api_version: ApplicationApiVersion,
        application_request_id: ApplicationRequestId,
        application_identity: ApplicationIdentity,
        application_capability_declaration: ApplicationCapabilityDeclaration,
        gateway_authentication_context: GatewayAuthenticationContext,
        gateway_authorization_binding: GatewayAuthorizationBinding,
        studio_selection_context: StudioSelectionContext,
        application_session_reference: Option<ApplicationSessionReference>,
        correlation_id: CorrelationId,
        causation_reference: Option<EventTraceReference>,
        requested_at: TimeReference,
        application_audit_reference: ApplicationAuditReference,
    ) -> ApplicationResult<Self> {
        gateway_authorization_binding
            .require_allow()
            .map_err(ApplicationError::from_gateway_rejection)?;
        if &AuthorizationSubject::Principal(
            gateway_authentication_context
                .authenticated_principal()
                .clone(),
        ) != gateway_authorization_binding
            .authorization_request_record()
            .requester()
        {
            return Err(ApplicationError::new(
                ApplicationErrorCode::AuthorizationEvidenceMismatch,
                "application request authentication evidence must match the preserved gateway authorization requester",
            )?);
        }
        require_exact_scope(
            studio_selection_context.ownership_path(),
            gateway_authorization_binding
                .authorization_request_record()
                .target()
                .scope()
                .ownership_path(),
            ApplicationErrorCode::ScopeMismatch,
            "application request scope must match the preserved gateway authorization scope",
        )?;
        if application_capability_declaration.application_api_version() != &application_api_version
        {
            return Err(ApplicationError::new(
                ApplicationErrorCode::CapabilityMismatch,
                "application capability declarations must preserve the application API version",
            )?);
        }
        for declared_capability in application_identity.declared_capability_references() {
            if !application_capability_declaration.admits(declared_capability) {
                return Err(ApplicationError::new(
                    ApplicationErrorCode::CapabilityMismatch,
                    "application capability declarations must admit every capability declared by the application identity",
                )?);
            }
        }
        require_correlation(
            &correlation_id,
            application_audit_reference.correlation_id(),
            ApplicationErrorCode::AuditEvidenceMismatch,
            "application request audit correlation must match the request correlation reference",
        )?;
        if let Some(application_session_reference) = application_session_reference.as_ref() {
            if application_session_reference.application_identifier()
                != application_identity.application_identifier()
            {
                return Err(ApplicationError::new(
                    ApplicationErrorCode::SessionApplicationMismatch,
                    "application session references must preserve the application identity",
                )?);
            }
            require_exact_scope(
                studio_selection_context.ownership_path(),
                application_session_reference.ownership_path(),
                ApplicationErrorCode::SessionScopeMismatch,
                "application session scope must match the application request scope",
            )?;
            require_correlation(
                &correlation_id,
                application_session_reference.correlation_id(),
                ApplicationErrorCode::SessionCorrelationMismatch,
                "application session correlation must match the application request correlation reference",
            )?;
        }
        Ok(Self {
            application_api_version,
            application_request_id,
            application_identity,
            application_capability_declaration,
            gateway_authentication_context,
            gateway_authorization_binding,
            studio_selection_context,
            application_session_reference,
            correlation_id,
            causation_reference,
            requested_at,
            application_audit_reference,
        })
    }

    pub fn application_request_id(&self) -> &ApplicationRequestId {
        &self.application_request_id
    }

    pub fn application_api_version(&self) -> &ApplicationApiVersion {
        &self.application_api_version
    }

    pub fn application_identity(&self) -> &ApplicationIdentity {
        &self.application_identity
    }

    pub fn application_capability_declaration(&self) -> &ApplicationCapabilityDeclaration {
        &self.application_capability_declaration
    }

    pub fn gateway_authentication_context(&self) -> &GatewayAuthenticationContext {
        &self.gateway_authentication_context
    }

    pub fn gateway_authorization_binding(&self) -> &GatewayAuthorizationBinding {
        &self.gateway_authorization_binding
    }

    pub fn studio_selection_context(&self) -> &StudioSelectionContext {
        &self.studio_selection_context
    }

    pub fn application_session_reference(&self) -> Option<&ApplicationSessionReference> {
        self.application_session_reference.as_ref()
    }

    pub fn correlation_id(&self) -> &CorrelationId {
        &self.correlation_id
    }

    pub fn causation_reference(&self) -> Option<&EventTraceReference> {
        self.causation_reference.as_ref()
    }

    pub fn requested_at(&self) -> &TimeReference {
        &self.requested_at
    }

    pub fn application_audit_reference(&self) -> &ApplicationAuditReference {
        &self.application_audit_reference
    }

    pub fn ownership_path(&self) -> &OwnershipPath {
        self.studio_selection_context.ownership_path()
    }
}
