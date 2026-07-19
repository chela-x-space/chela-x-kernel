use kernel_application::{ApplicationApiVersion, ApplicationValidationStatus};

use crate::service::{ServiceApiVersion, ServiceIntentKind};
use crate::service_capability::ServiceCapabilityDeclaration;
use crate::service_error::{ServiceError, ServiceErrorCode, ServiceResult};
use crate::service_identity::ServiceIdentity;
use crate::service_validation::{
    reject_duplicates, validate_namespaced_identifier, validate_non_empty_text,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceValidationStatus {
    Validated,
    ValidationPending,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceDependencyCompatibilityReference(String);

impl ServiceDependencyCompatibilityReference {
    pub fn new(value: impl Into<String>) -> ServiceResult<Self> {
        Ok(Self(validate_namespaced_identifier(
            value,
            ServiceErrorCode::CompatibilityMismatch,
            "service dependency compatibility references require namespaced logical identifiers",
        )?))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceStatusSnapshot {
    service_api_version: ServiceApiVersion,
    service_identity: ServiceIdentity,
    service_capability_declaration: ServiceCapabilityDeclaration,
    supported_intent_kinds: Vec<ServiceIntentKind>,
    dependency_compatibility_references: Vec<ServiceDependencyCompatibilityReference>,
    application_api_version: ApplicationApiVersion,
    application_validation_status: ApplicationValidationStatus,
    service_validation_status: ServiceValidationStatus,
    generated_at: String,
}

impl ServiceStatusSnapshot {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        service_api_version: ServiceApiVersion,
        service_identity: ServiceIdentity,
        service_capability_declaration: ServiceCapabilityDeclaration,
        supported_intent_kinds: Vec<ServiceIntentKind>,
        dependency_compatibility_references: Vec<ServiceDependencyCompatibilityReference>,
        application_api_version: ApplicationApiVersion,
        application_validation_status: ApplicationValidationStatus,
        service_validation_status: ServiceValidationStatus,
        generated_at: impl Into<String>,
    ) -> ServiceResult<Self> {
        if supported_intent_kinds.is_empty() {
            return Err(ServiceError::new(
                ServiceErrorCode::InternalContractViolation,
                "service status snapshots require at least one supported intent kind",
            )?);
        }
        if dependency_compatibility_references.is_empty() {
            return Err(ServiceError::new(
                ServiceErrorCode::CompatibilityMismatch,
                "service status snapshots require at least one dependency compatibility reference",
            )?);
        }
        reject_duplicates(
            &supported_intent_kinds,
            ServiceErrorCode::InternalContractViolation,
            "duplicate supported service intent kind in status snapshot",
        )?;
        reject_duplicates(
            &dependency_compatibility_references,
            ServiceErrorCode::CompatibilityMismatch,
            "duplicate dependency compatibility reference in service status snapshot",
        )?;
        if service_capability_declaration.service_api_version() != &service_api_version {
            return Err(ServiceError::new(
                ServiceErrorCode::CompatibilityMismatch,
                "service status snapshots must preserve the service API version in capability declarations",
            )?);
        }
        if supported_intent_kinds.contains(&ServiceIntentKind::Command)
            && !service_capability_declaration.supports_commands()
        {
            return Err(ServiceError::new(
                ServiceErrorCode::CapabilityMismatch,
                "command intent support requires command capability admission",
            )?);
        }
        if supported_intent_kinds.contains(&ServiceIntentKind::Query)
            && !service_capability_declaration.supports_queries()
        {
            return Err(ServiceError::new(
                ServiceErrorCode::CapabilityMismatch,
                "query intent support requires query capability admission",
            )?);
        }
        Ok(Self {
            service_api_version,
            service_identity,
            service_capability_declaration,
            supported_intent_kinds,
            dependency_compatibility_references,
            application_api_version,
            application_validation_status,
            service_validation_status,
            generated_at: validate_non_empty_text(
                generated_at,
                ServiceErrorCode::InternalContractViolation,
                "service status snapshots require a caller-supplied generation time reference",
            )?,
        })
    }

    pub fn service_validation_status(&self) -> ServiceValidationStatus {
        self.service_validation_status
    }
}
