use kernel_service::{ServiceApiVersion, ServiceStatusSnapshot, ServiceValidationStatus};

use crate::adapter::{AdapterApiVersion, AdapterIntentKind};
use crate::adapter_capability::AdapterCapabilityDeclaration;
use crate::adapter_error::{AdapterError, AdapterErrorCode, AdapterResult};
use crate::adapter_identity::AdapterIdentity;
use crate::adapter_validation::{
    reject_duplicates, validate_namespaced_identifier, validate_non_empty_text,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdapterValidationStatus {
    Validated,
    ValidationPending,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterCompatibilityReference(String);

impl AdapterCompatibilityReference {
    pub fn new(value: impl Into<String>) -> AdapterResult<Self> {
        Ok(Self(validate_namespaced_identifier(
            value,
            AdapterErrorCode::ServiceCompatibilityMismatch,
            "adapter compatibility references require namespaced logical identifiers",
        )?))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterStatusSnapshot {
    adapter_api_version: AdapterApiVersion,
    adapter_identity: AdapterIdentity,
    adapter_capability_declaration: AdapterCapabilityDeclaration,
    supported_intent_kinds: Vec<AdapterIntentKind>,
    compatibility_references: Vec<AdapterCompatibilityReference>,
    service_api_version: ServiceApiVersion,
    service_status_snapshot: ServiceStatusSnapshot,
    adapter_validation_status: AdapterValidationStatus,
    generated_at: String,
}

impl AdapterStatusSnapshot {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        adapter_api_version: AdapterApiVersion,
        adapter_identity: AdapterIdentity,
        adapter_capability_declaration: AdapterCapabilityDeclaration,
        supported_intent_kinds: Vec<AdapterIntentKind>,
        compatibility_references: Vec<AdapterCompatibilityReference>,
        service_api_version: ServiceApiVersion,
        service_status_snapshot: ServiceStatusSnapshot,
        adapter_validation_status: AdapterValidationStatus,
        generated_at: impl Into<String>,
    ) -> AdapterResult<Self> {
        if supported_intent_kinds.is_empty() {
            return Err(AdapterError::new(
                AdapterErrorCode::InternalContractViolation,
                "adapter status snapshots require at least one supported intent kind",
            )?);
        }
        if compatibility_references.is_empty() {
            return Err(AdapterError::new(
                AdapterErrorCode::ServiceCompatibilityMismatch,
                "adapter status snapshots require at least one compatibility reference",
            )?);
        }
        reject_duplicates(
            &supported_intent_kinds,
            AdapterErrorCode::InternalContractViolation,
            "duplicate supported adapter intent kind in status snapshot",
        )?;
        reject_duplicates(
            &compatibility_references,
            AdapterErrorCode::ServiceCompatibilityMismatch,
            "duplicate compatibility reference in adapter status snapshot",
        )?;
        if adapter_capability_declaration.adapter_api_version() != &adapter_api_version {
            return Err(AdapterError::new(
                AdapterErrorCode::CapabilityMismatch,
                "adapter status snapshots must preserve the adapter API version in capability declarations",
            )?);
        }
        if supported_intent_kinds.contains(&AdapterIntentKind::Command)
            && !adapter_capability_declaration.supports_commands()
        {
            return Err(AdapterError::new(
                AdapterErrorCode::CapabilityMismatch,
                "command intent support requires command capability admission",
            )?);
        }
        if supported_intent_kinds.contains(&AdapterIntentKind::Query)
            && !adapter_capability_declaration.supports_queries()
        {
            return Err(AdapterError::new(
                AdapterErrorCode::CapabilityMismatch,
                "query intent support requires query capability admission",
            )?);
        }
        if adapter_validation_status == AdapterValidationStatus::Validated
            && service_status_snapshot.service_validation_status()
                != ServiceValidationStatus::Validated
        {
            return Err(AdapterError::new(
                AdapterErrorCode::ServiceCompatibilityMismatch,
                "validated adapter status snapshots require validated service status evidence",
            )?);
        }
        Ok(Self {
            adapter_api_version,
            adapter_identity,
            adapter_capability_declaration,
            supported_intent_kinds,
            compatibility_references,
            service_api_version,
            service_status_snapshot,
            adapter_validation_status,
            generated_at: validate_non_empty_text(
                generated_at,
                AdapterErrorCode::InternalContractViolation,
                "adapter status snapshots require a caller-supplied generation time reference",
            )?,
        })
    }

    pub fn adapter_validation_status(&self) -> AdapterValidationStatus {
        self.adapter_validation_status
    }
}
