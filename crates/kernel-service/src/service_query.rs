use kernel_application::ApplicationQueryIntent;

use crate::service::ServiceIntentKind;
use crate::service_capability::ServiceCapabilityReference;
use crate::service_context::ServiceRequestContext;
use crate::service_error::ServiceResult;
use crate::service_validation::require_application_request_context;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceQueryIntent {
    service_request_context: ServiceRequestContext,
    required_capability: ServiceCapabilityReference,
    application_query_intent: ApplicationQueryIntent,
}

impl ServiceQueryIntent {
    pub fn new(
        service_request_context: ServiceRequestContext,
        required_capability: ServiceCapabilityReference,
        application_query_intent: ApplicationQueryIntent,
    ) -> ServiceResult<Self> {
        service_request_context
            .service_capability_declaration()
            .require_capability(&required_capability, ServiceIntentKind::Query)?;
        require_application_request_context(
            service_request_context.application_request_context(),
            application_query_intent.application_request_context(),
            crate::ServiceErrorCode::ApplicationRequestMismatch,
            "service query intent must preserve the original application request context",
        )?;
        Ok(Self {
            service_request_context,
            required_capability,
            application_query_intent,
        })
    }

    pub fn service_request_context(&self) -> &ServiceRequestContext {
        &self.service_request_context
    }

    pub fn application_query_intent(&self) -> &ApplicationQueryIntent {
        &self.application_query_intent
    }
}
