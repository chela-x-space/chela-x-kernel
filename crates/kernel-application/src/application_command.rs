use kernel_gateway::GatewayRequestEnvelope;
use kernel_studio::StudioCommandRequest;

use crate::application::ApplicationIntentKind;
use crate::application_capability::ApplicationCapabilityReference;
use crate::application_context::ApplicationRequestContext;
use crate::application_error::{ApplicationError, ApplicationErrorCode, ApplicationResult};
use crate::application_validation::{require_correlation, require_exact_scope};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationCommandIntent {
    application_request_context: ApplicationRequestContext,
    required_capability: ApplicationCapabilityReference,
    studio_command_request: StudioCommandRequest,
}

impl ApplicationCommandIntent {
    pub fn new(
        application_request_context: ApplicationRequestContext,
        required_capability: ApplicationCapabilityReference,
        studio_command_request: StudioCommandRequest,
    ) -> ApplicationResult<Self> {
        application_request_context
            .application_capability_declaration()
            .require_capability(&required_capability, ApplicationIntentKind::Command)?;
        require_correlation(
            application_request_context.correlation_id(),
            studio_command_request.correlation_id(),
            ApplicationErrorCode::CommandQueryMismatch,
            "application command intent must preserve the Studio command correlation reference",
        )?;
        if application_request_context.studio_selection_context()
            != studio_command_request.studio_selection_context()
        {
            return Err(ApplicationError::new(
                ApplicationErrorCode::ViewRequestMismatch,
                "application command intent must preserve the Studio selection context",
            )?);
        }
        let GatewayRequestEnvelope::Command {
            gateway_request_context,
            ..
        } = studio_command_request.gateway_request_envelope()
        else {
            return Err(ApplicationError::new(
                ApplicationErrorCode::CommandQueryMismatch,
                "application command intent requires a Studio command request backed by a gateway command envelope",
            )?);
        };
        if gateway_request_context.gateway_authentication_context()
            != application_request_context.gateway_authentication_context()
        {
            return Err(ApplicationError::new(
                ApplicationErrorCode::AuthorizationEvidenceMismatch,
                "application command intent must preserve the gateway authentication evidence",
            )?);
        }
        if gateway_request_context.gateway_authorization_binding()
            != application_request_context.gateway_authorization_binding()
        {
            return Err(ApplicationError::new(
                ApplicationErrorCode::AuthorizationEvidenceMismatch,
                "application command intent must preserve the gateway authorization evidence",
            )?);
        }
        require_exact_scope(
            application_request_context.ownership_path(),
            gateway_request_context.ownership_path(),
            ApplicationErrorCode::ScopeMismatch,
            "application command intent scope must match the preserved gateway command scope",
        )?;
        require_correlation(
            application_request_context.correlation_id(),
            gateway_request_context.correlation_id(),
            ApplicationErrorCode::CommandQueryMismatch,
            "application command intent correlation must match the preserved gateway command correlation",
        )?;
        Ok(Self {
            application_request_context,
            required_capability,
            studio_command_request,
        })
    }

    pub fn application_request_context(&self) -> &ApplicationRequestContext {
        &self.application_request_context
    }

    pub fn studio_command_request(&self) -> &StudioCommandRequest {
        &self.studio_command_request
    }
}
