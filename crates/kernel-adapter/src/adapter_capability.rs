use crate::adapter::{
    AdapterApiVersion, AdapterIntentKind, ADAPTER_COMMAND_CAPABILITY, ADAPTER_QUERY_CAPABILITY,
};
use crate::adapter_error::{AdapterError, AdapterErrorCode, AdapterResult};
use crate::adapter_validation::{reject_duplicates, validate_namespaced_identifier};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterCapabilityReference(String);

impl AdapterCapabilityReference {
    pub fn new(value: impl Into<String>) -> AdapterResult<Self> {
        Ok(Self(validate_namespaced_identifier(
            value,
            AdapterErrorCode::CapabilityMismatch,
            "adapter capability references require namespaced identifiers",
        )?))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterCapabilityDeclaration {
    adapter_api_version: AdapterApiVersion,
    admitted_capabilities: Vec<AdapterCapabilityReference>,
    supports_commands: bool,
    supports_queries: bool,
}

impl AdapterCapabilityDeclaration {
    pub fn new(
        adapter_api_version: AdapterApiVersion,
        admitted_capabilities: Vec<AdapterCapabilityReference>,
        supports_commands: bool,
        supports_queries: bool,
    ) -> AdapterResult<Self> {
        if admitted_capabilities.is_empty() {
            return Err(AdapterError::new(
                AdapterErrorCode::CapabilityMismatch,
                "adapter capability declarations require at least one admitted capability",
            )?);
        }
        reject_duplicates(
            &admitted_capabilities,
            AdapterErrorCode::CapabilityMismatch,
            "duplicate adapter capability reference",
        )?;
        if supports_commands
            && !admitted_capabilities
                .iter()
                .any(|capability| capability.as_str() == ADAPTER_COMMAND_CAPABILITY)
        {
            return Err(AdapterError::new(
                AdapterErrorCode::CapabilityMismatch,
                "command admission requires the canonical adapter command capability",
            )?);
        }
        if supports_queries
            && !admitted_capabilities
                .iter()
                .any(|capability| capability.as_str() == ADAPTER_QUERY_CAPABILITY)
        {
            return Err(AdapterError::new(
                AdapterErrorCode::CapabilityMismatch,
                "query admission requires the canonical adapter query capability",
            )?);
        }
        Ok(Self {
            adapter_api_version,
            admitted_capabilities,
            supports_commands,
            supports_queries,
        })
    }

    pub fn adapter_api_version(&self) -> &AdapterApiVersion {
        &self.adapter_api_version
    }

    pub fn admitted_capabilities(&self) -> &[AdapterCapabilityReference] {
        &self.admitted_capabilities
    }

    pub fn supports_commands(&self) -> bool {
        self.supports_commands
    }

    pub fn supports_queries(&self) -> bool {
        self.supports_queries
    }

    pub fn admits(&self, capability_reference: &AdapterCapabilityReference) -> bool {
        self.admitted_capabilities
            .iter()
            .any(|capability| capability == capability_reference)
    }

    pub fn require_capability(
        &self,
        capability_reference: &AdapterCapabilityReference,
        intent_kind: AdapterIntentKind,
    ) -> AdapterResult<()> {
        if !self.admits(capability_reference) {
            return Err(AdapterError::new(
                AdapterErrorCode::CapabilityMismatch,
                "adapter capability declaration does not admit the requested capability",
            )?);
        }
        match intent_kind {
            AdapterIntentKind::Command
                if capability_reference.as_str() != ADAPTER_COMMAND_CAPABILITY =>
            {
                return Err(AdapterError::new(
                    AdapterErrorCode::CapabilityMismatch,
                    "adapter command intent requires the canonical adapter command capability",
                )?);
            }
            AdapterIntentKind::Query
                if capability_reference.as_str() != ADAPTER_QUERY_CAPABILITY =>
            {
                return Err(AdapterError::new(
                    AdapterErrorCode::CapabilityMismatch,
                    "adapter query intent requires the canonical adapter query capability",
                )?);
            }
            AdapterIntentKind::Command if !self.supports_commands => {
                return Err(AdapterError::new(
                    AdapterErrorCode::CapabilityMismatch,
                    "adapter capability declaration does not admit command intent",
                )?);
            }
            AdapterIntentKind::Query if !self.supports_queries => {
                return Err(AdapterError::new(
                    AdapterErrorCode::CapabilityMismatch,
                    "adapter capability declaration does not admit query intent",
                )?);
            }
            _ => {}
        }
        Ok(())
    }
}
