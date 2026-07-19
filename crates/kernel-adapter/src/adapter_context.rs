use kernel_service::ServiceRequestContext;

use crate::adapter::AdapterApiVersion;
use crate::adapter_capability::AdapterCapabilityDeclaration;
use crate::adapter_error::{AdapterError, AdapterErrorCode, AdapterResult};
use crate::adapter_identity::AdapterIdentity;
use crate::adapter_validation::{validate_namespaced_identifier, validate_non_empty_text};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterRequestId(String);

impl AdapterRequestId {
    pub fn new(value: impl Into<String>) -> AdapterResult<Self> {
        Ok(Self(validate_namespaced_identifier(
            value,
            AdapterErrorCode::AdapterRequestIdentityMismatch,
            "adapter request identifiers require namespaced logical identifiers",
        )?))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterRequestContext {
    adapter_api_version: AdapterApiVersion,
    adapter_request_id: AdapterRequestId,
    adapter_identity: AdapterIdentity,
    adapter_capability_declaration: AdapterCapabilityDeclaration,
    service_request_context: ServiceRequestContext,
    requested_at: String,
}

impl AdapterRequestContext {
    pub fn new(
        adapter_api_version: AdapterApiVersion,
        adapter_request_id: AdapterRequestId,
        adapter_identity: AdapterIdentity,
        adapter_capability_declaration: AdapterCapabilityDeclaration,
        service_request_context: ServiceRequestContext,
        requested_at: impl Into<String>,
    ) -> AdapterResult<Self> {
        if adapter_capability_declaration.adapter_api_version() != &adapter_api_version {
            return Err(AdapterError::new(
                AdapterErrorCode::CapabilityMismatch,
                "adapter capability declarations must preserve the adapter API version",
            )?);
        }
        for declared_capability in adapter_identity.declared_capability_references() {
            if !adapter_capability_declaration.admits(declared_capability) {
                return Err(AdapterError::new(
                    AdapterErrorCode::CapabilityMismatch,
                    "adapter capability declarations must admit every capability declared by the adapter identity",
                )?);
            }
        }
        Ok(Self {
            adapter_api_version,
            adapter_request_id,
            adapter_identity,
            adapter_capability_declaration,
            service_request_context,
            requested_at: validate_non_empty_text(
                requested_at,
                AdapterErrorCode::InvalidAdapterRequest,
                "adapter request contexts require a caller-supplied request time reference",
            )?,
        })
    }

    pub fn adapter_api_version(&self) -> &AdapterApiVersion {
        &self.adapter_api_version
    }

    pub fn adapter_request_id(&self) -> &AdapterRequestId {
        &self.adapter_request_id
    }

    pub fn adapter_identity(&self) -> &AdapterIdentity {
        &self.adapter_identity
    }

    pub fn adapter_capability_declaration(&self) -> &AdapterCapabilityDeclaration {
        &self.adapter_capability_declaration
    }

    pub fn service_request_context(&self) -> &ServiceRequestContext {
        &self.service_request_context
    }
}
