use kernel_service::ServiceQueryIntent;

use crate::adapter::AdapterIntentKind;
use crate::adapter_capability::AdapterCapabilityReference;
use crate::adapter_context::AdapterRequestContext;
use crate::adapter_error::{AdapterErrorCode, AdapterResult};
use crate::adapter_validation::require_service_request_context;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterQueryIntent {
    adapter_request_context: AdapterRequestContext,
    required_capability: AdapterCapabilityReference,
    service_query_intent: ServiceQueryIntent,
}

impl AdapterQueryIntent {
    pub fn new(
        adapter_request_context: AdapterRequestContext,
        required_capability: AdapterCapabilityReference,
        service_query_intent: ServiceQueryIntent,
    ) -> AdapterResult<Self> {
        adapter_request_context
            .adapter_capability_declaration()
            .require_capability(&required_capability, AdapterIntentKind::Query)?;
        require_service_request_context(
            adapter_request_context.service_request_context(),
            service_query_intent.service_request_context(),
            AdapterErrorCode::AdapterRequestIdentityMismatch,
            "adapter query intent must preserve the original service request context",
        )?;
        Ok(Self {
            adapter_request_context,
            required_capability,
            service_query_intent,
        })
    }

    pub fn adapter_request_context(&self) -> &AdapterRequestContext {
        &self.adapter_request_context
    }

    pub fn service_query_intent(&self) -> &ServiceQueryIntent {
        &self.service_query_intent
    }
}
