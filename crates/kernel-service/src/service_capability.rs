use crate::service::{
    ServiceApiVersion, ServiceIntentKind, SERVICE_COMMAND_CAPABILITY, SERVICE_QUERY_CAPABILITY,
};
use crate::service_error::{ServiceError, ServiceErrorCode, ServiceResult};
use crate::service_validation::{reject_duplicates, validate_namespaced_identifier};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceCapabilityReference(String);

impl ServiceCapabilityReference {
    pub fn new(value: impl Into<String>) -> ServiceResult<Self> {
        Ok(Self(validate_namespaced_identifier(
            value,
            ServiceErrorCode::CapabilityMismatch,
            "service capability references require namespaced identifiers",
        )?))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceCapabilityDeclaration {
    service_api_version: ServiceApiVersion,
    admitted_capabilities: Vec<ServiceCapabilityReference>,
    supports_commands: bool,
    supports_queries: bool,
}

impl ServiceCapabilityDeclaration {
    pub fn new(
        service_api_version: ServiceApiVersion,
        admitted_capabilities: Vec<ServiceCapabilityReference>,
        supports_commands: bool,
        supports_queries: bool,
    ) -> ServiceResult<Self> {
        if admitted_capabilities.is_empty() {
            return Err(ServiceError::new(
                ServiceErrorCode::CapabilityMismatch,
                "service capability declarations require at least one admitted capability",
            )?);
        }
        reject_duplicates(
            &admitted_capabilities,
            ServiceErrorCode::CapabilityMismatch,
            "duplicate service capability reference",
        )?;
        if supports_commands
            && !admitted_capabilities
                .iter()
                .any(|capability| capability.as_str() == SERVICE_COMMAND_CAPABILITY)
        {
            return Err(ServiceError::new(
                ServiceErrorCode::CapabilityMismatch,
                "command admission requires the canonical service command capability",
            )?);
        }
        if supports_queries
            && !admitted_capabilities
                .iter()
                .any(|capability| capability.as_str() == SERVICE_QUERY_CAPABILITY)
        {
            return Err(ServiceError::new(
                ServiceErrorCode::CapabilityMismatch,
                "query admission requires the canonical service query capability",
            )?);
        }
        Ok(Self {
            service_api_version,
            admitted_capabilities,
            supports_commands,
            supports_queries,
        })
    }

    pub fn service_api_version(&self) -> &ServiceApiVersion {
        &self.service_api_version
    }

    pub fn admitted_capabilities(&self) -> &[ServiceCapabilityReference] {
        &self.admitted_capabilities
    }

    pub fn supports_commands(&self) -> bool {
        self.supports_commands
    }

    pub fn supports_queries(&self) -> bool {
        self.supports_queries
    }

    pub fn admits(&self, capability_reference: &ServiceCapabilityReference) -> bool {
        self.admitted_capabilities
            .iter()
            .any(|capability| capability == capability_reference)
    }

    pub fn require_capability(
        &self,
        capability_reference: &ServiceCapabilityReference,
        intent_kind: ServiceIntentKind,
    ) -> ServiceResult<()> {
        if !self.admits(capability_reference) {
            return Err(ServiceError::new(
                ServiceErrorCode::CapabilityMismatch,
                "service capability declaration does not admit the requested capability",
            )?);
        }
        match intent_kind {
            ServiceIntentKind::Command
                if capability_reference.as_str() != SERVICE_COMMAND_CAPABILITY =>
            {
                return Err(ServiceError::new(
                    ServiceErrorCode::CapabilityMismatch,
                    "service command intent requires the canonical service command capability",
                )?);
            }
            ServiceIntentKind::Query
                if capability_reference.as_str() != SERVICE_QUERY_CAPABILITY =>
            {
                return Err(ServiceError::new(
                    ServiceErrorCode::CapabilityMismatch,
                    "service query intent requires the canonical service query capability",
                )?);
            }
            ServiceIntentKind::Command if !self.supports_commands => {
                return Err(ServiceError::new(
                    ServiceErrorCode::CapabilityMismatch,
                    "service capability declaration does not admit command intent",
                )?);
            }
            ServiceIntentKind::Query if !self.supports_queries => {
                return Err(ServiceError::new(
                    ServiceErrorCode::CapabilityMismatch,
                    "service capability declaration does not admit query intent",
                )?);
            }
            _ => {}
        }
        Ok(())
    }
}
