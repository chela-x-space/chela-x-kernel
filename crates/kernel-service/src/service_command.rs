use kernel_application::ApplicationCommandIntent;

use crate::service::ServiceIntentKind;
use crate::service_capability::ServiceCapabilityReference;
use crate::service_context::ServiceRequestContext;
use crate::service_error::ServiceResult;
use crate::service_validation::require_application_request_context;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceCommandIntent {
    service_request_context: ServiceRequestContext,
    required_capability: ServiceCapabilityReference,
    application_command_intent: ApplicationCommandIntent,
}

impl ServiceCommandIntent {
    pub fn new(
        service_request_context: ServiceRequestContext,
        required_capability: ServiceCapabilityReference,
        application_command_intent: ApplicationCommandIntent,
    ) -> ServiceResult<Self> {
        service_request_context
            .service_capability_declaration()
            .require_capability(&required_capability, ServiceIntentKind::Command)?;
        require_application_request_context(
            service_request_context.application_request_context(),
            application_command_intent.application_request_context(),
            crate::ServiceErrorCode::ApplicationRequestMismatch,
            "service command intent must preserve the original application request context",
        )?;
        Ok(Self {
            service_request_context,
            required_capability,
            application_command_intent,
        })
    }

    pub fn service_request_context(&self) -> &ServiceRequestContext {
        &self.service_request_context
    }

    pub fn application_command_intent(&self) -> &ApplicationCommandIntent {
        &self.application_command_intent
    }
}
