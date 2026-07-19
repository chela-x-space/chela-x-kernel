use kernel_studio::{StudioViewReference, StudioViewRequest};

use crate::application::ApplicationIntentKind;
use crate::application_capability::ApplicationCapabilityReference;
use crate::application_context::ApplicationRequestContext;
use crate::application_error::{ApplicationError, ApplicationErrorCode, ApplicationResult};
use crate::application_navigation::ApplicationViewIntent;
use crate::application_validation::{require_correlation, require_exact_scope};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationQueryIntent {
    application_request_context: ApplicationRequestContext,
    required_capability: ApplicationCapabilityReference,
    application_view_intent: ApplicationViewIntent,
    studio_view_request: StudioViewRequest,
}

impl ApplicationQueryIntent {
    pub fn new(
        application_request_context: ApplicationRequestContext,
        required_capability: ApplicationCapabilityReference,
        application_view_intent: ApplicationViewIntent,
        studio_view_request: StudioViewRequest,
    ) -> ApplicationResult<Self> {
        application_request_context
            .application_capability_declaration()
            .require_capability(&required_capability, ApplicationIntentKind::Query)?;
        if !application_request_context
            .application_capability_declaration()
            .supports_view(application_view_intent.studio_view_reference())
        {
            return Err(ApplicationError::new(
                ApplicationErrorCode::CapabilityMismatch,
                "application query intent requires a declared Studio view capability",
            )?);
        }
        require_correlation(
            application_request_context.correlation_id(),
            studio_view_request.correlation_id(),
            ApplicationErrorCode::CommandQueryMismatch,
            "application query intent must preserve the Studio view correlation reference",
        )?;
        if application_request_context.studio_selection_context()
            != application_view_intent.studio_selection_context()
            || application_request_context.studio_selection_context()
                != studio_view_request.studio_selection_context()
        {
            return Err(ApplicationError::new(
                ApplicationErrorCode::ViewRequestMismatch,
                "application query intent must preserve the Studio selection context",
            )?);
        }
        let requested_view_reference: &StudioViewReference =
            application_view_intent.studio_view_reference();
        if studio_view_request.studio_view_reference() != requested_view_reference {
            return Err(ApplicationError::new(
                ApplicationErrorCode::ViewRequestMismatch,
                "application query intent must preserve the Studio view identity",
            )?);
        }
        for gateway_request_envelope in studio_view_request.gateway_request_envelopes() {
            let gateway_request_context = gateway_request_envelope.gateway_request_context();
            if gateway_request_context.gateway_authentication_context()
                != application_request_context.gateway_authentication_context()
            {
                return Err(ApplicationError::new(
                    ApplicationErrorCode::AuthorizationEvidenceMismatch,
                    "application query intent must preserve the gateway authentication evidence",
                )?);
            }
            if gateway_request_context.gateway_authorization_binding()
                != application_request_context.gateway_authorization_binding()
            {
                return Err(ApplicationError::new(
                    ApplicationErrorCode::AuthorizationEvidenceMismatch,
                    "application query intent must preserve the gateway authorization evidence",
                )?);
            }
            require_exact_scope(
                application_request_context.ownership_path(),
                gateway_request_context.ownership_path(),
                ApplicationErrorCode::ScopeMismatch,
                "application query intent scope must match the preserved gateway query scope",
            )?;
            require_correlation(
                application_request_context.correlation_id(),
                gateway_request_context.correlation_id(),
                ApplicationErrorCode::CommandQueryMismatch,
                "application query intent correlation must match the preserved gateway query correlation",
            )?;
        }
        Ok(Self {
            application_request_context,
            required_capability,
            application_view_intent,
            studio_view_request,
        })
    }

    pub fn application_request_context(&self) -> &ApplicationRequestContext {
        &self.application_request_context
    }

    pub fn studio_view_request(&self) -> &StudioViewRequest {
        &self.studio_view_request
    }

    pub fn application_view_intent(&self) -> &ApplicationViewIntent {
        &self.application_view_intent
    }
}
