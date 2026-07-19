use kernel_application::ApplicationRequestContext;

use crate::service::ServiceApiVersion;
use crate::service_capability::ServiceCapabilityDeclaration;
use crate::service_error::{ServiceError, ServiceErrorCode, ServiceResult};
use crate::service_identity::ServiceIdentity;
use crate::service_validation::{validate_namespaced_identifier, validate_non_empty_text};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceRequestId(String);

impl ServiceRequestId {
    pub fn new(value: impl Into<String>) -> ServiceResult<Self> {
        Ok(Self(validate_namespaced_identifier(
            value,
            ServiceErrorCode::ServiceRequestIdentityMismatch,
            "service request identifiers require namespaced logical identifiers",
        )?))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceRequestContext {
    service_api_version: ServiceApiVersion,
    service_request_id: ServiceRequestId,
    service_identity: ServiceIdentity,
    service_capability_declaration: ServiceCapabilityDeclaration,
    application_request_context: ApplicationRequestContext,
    requested_at: String,
}

impl ServiceRequestContext {
    pub fn new(
        service_api_version: ServiceApiVersion,
        service_request_id: ServiceRequestId,
        service_identity: ServiceIdentity,
        service_capability_declaration: ServiceCapabilityDeclaration,
        application_request_context: ApplicationRequestContext,
        requested_at: impl Into<String>,
    ) -> ServiceResult<Self> {
        if service_capability_declaration.service_api_version() != &service_api_version {
            return Err(ServiceError::new(
                ServiceErrorCode::CapabilityMismatch,
                "service capability declarations must preserve the service API version",
            )?);
        }
        for declared_capability in service_identity.declared_capability_references() {
            if !service_capability_declaration.admits(declared_capability) {
                return Err(ServiceError::new(
                    ServiceErrorCode::CapabilityMismatch,
                    "service capability declarations must admit every capability declared by the service identity",
                )?);
            }
        }
        Ok(Self {
            service_api_version,
            service_request_id,
            service_identity,
            service_capability_declaration,
            application_request_context,
            requested_at: validate_non_empty_text(
                requested_at,
                ServiceErrorCode::InvalidServiceRequest,
                "service request contexts require a caller-supplied request time reference",
            )?,
        })
    }

    pub fn service_api_version(&self) -> &ServiceApiVersion {
        &self.service_api_version
    }

    pub fn service_request_id(&self) -> &ServiceRequestId {
        &self.service_request_id
    }

    pub fn service_identity(&self) -> &ServiceIdentity {
        &self.service_identity
    }

    pub fn service_capability_declaration(&self) -> &ServiceCapabilityDeclaration {
        &self.service_capability_declaration
    }

    pub fn application_request_context(&self) -> &ApplicationRequestContext {
        &self.application_request_context
    }
}
