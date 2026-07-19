use kernel_service::ServiceCommandIntent;

use crate::adapter::AdapterIntentKind;
use crate::adapter_capability::AdapterCapabilityReference;
use crate::adapter_context::AdapterRequestContext;
use crate::adapter_error::{AdapterErrorCode, AdapterResult};
use crate::adapter_validation::require_service_request_context;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterCommandIntent {
    adapter_request_context: AdapterRequestContext,
    required_capability: AdapterCapabilityReference,
    service_command_intent: ServiceCommandIntent,
}

impl AdapterCommandIntent {
    pub fn new(
        adapter_request_context: AdapterRequestContext,
        required_capability: AdapterCapabilityReference,
        service_command_intent: ServiceCommandIntent,
    ) -> AdapterResult<Self> {
        adapter_request_context
            .adapter_capability_declaration()
            .require_capability(&required_capability, AdapterIntentKind::Command)?;
        require_service_request_context(
            adapter_request_context.service_request_context(),
            service_command_intent.service_request_context(),
            AdapterErrorCode::AdapterRequestIdentityMismatch,
            "adapter command intent must preserve the original service request context",
        )?;
        Ok(Self {
            adapter_request_context,
            required_capability,
            service_command_intent,
        })
    }

    pub fn adapter_request_context(&self) -> &AdapterRequestContext {
        &self.adapter_request_context
    }

    pub fn service_command_intent(&self) -> &ServiceCommandIntent {
        &self.service_command_intent
    }
}
