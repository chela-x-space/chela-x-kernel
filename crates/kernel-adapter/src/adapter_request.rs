use crate::adapter_command::AdapterCommandIntent;
use crate::adapter_query::AdapterQueryIntent;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdapterRequestEnvelope {
    Command(Box<AdapterCommandIntent>),
    Query(Box<AdapterQueryIntent>),
}

impl AdapterRequestEnvelope {
    pub fn command(adapter_command_intent: AdapterCommandIntent) -> Self {
        Self::Command(Box::new(adapter_command_intent))
    }

    pub fn query(adapter_query_intent: AdapterQueryIntent) -> Self {
        Self::Query(Box::new(adapter_query_intent))
    }

    pub fn adapter_request_id(&self) -> &crate::AdapterRequestId {
        match self {
            Self::Command(intent) => intent.adapter_request_context().adapter_request_id(),
            Self::Query(intent) => intent.adapter_request_context().adapter_request_id(),
        }
    }

    pub fn adapter_identity(&self) -> &crate::AdapterIdentity {
        match self {
            Self::Command(intent) => intent.adapter_request_context().adapter_identity(),
            Self::Query(intent) => intent.adapter_request_context().adapter_identity(),
        }
    }

    pub fn service_request_context(&self) -> &kernel_service::ServiceRequestContext {
        match self {
            Self::Command(intent) => intent.adapter_request_context().service_request_context(),
            Self::Query(intent) => intent.adapter_request_context().service_request_context(),
        }
    }
}
