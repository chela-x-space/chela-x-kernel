use crate::service_capability::ServiceCapabilityReference;
use crate::service_error::{ServiceErrorCode, ServiceResult};
use crate::service_validation::{
    reject_duplicates, validate_namespaced_identifier, validate_version_reference,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceIdentityKind {
    ExternalService,
    ExternalAdapter,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceIdentity {
    service_identifier: String,
    service_identity_kind: ServiceIdentityKind,
    service_namespace: String,
    service_version_reference: String,
    declared_capability_references: Vec<ServiceCapabilityReference>,
    environment_reference: Option<String>,
}

impl ServiceIdentity {
    pub fn new(
        service_identifier: impl Into<String>,
        service_identity_kind: ServiceIdentityKind,
        service_namespace: impl Into<String>,
        service_version_reference: impl Into<String>,
        declared_capability_references: Vec<ServiceCapabilityReference>,
        environment_reference: Option<String>,
    ) -> ServiceResult<Self> {
        let service_identifier = validate_namespaced_identifier(
            service_identifier,
            ServiceErrorCode::InvalidServiceIdentity,
            "service identity references require namespaced logical identifiers",
        )?;
        let service_namespace = validate_namespaced_identifier(
            service_namespace,
            ServiceErrorCode::InvalidServiceIdentity,
            "service namespace references require namespaced logical identifiers",
        )?;
        reject_duplicates(
            &declared_capability_references,
            ServiceErrorCode::CapabilityMismatch,
            "duplicate declared service capability reference",
        )?;
        Ok(Self {
            service_identifier,
            service_identity_kind,
            service_namespace,
            service_version_reference: validate_version_reference(
                service_version_reference,
                ServiceErrorCode::InvalidServiceIdentity,
                "service version references must be namespace-safe and transport-neutral",
            )?,
            declared_capability_references,
            environment_reference: environment_reference
                .map(|value| {
                    validate_namespaced_identifier(
                        value,
                        ServiceErrorCode::InvalidServiceIdentity,
                        "service environment references require namespaced logical identifiers",
                    )
                })
                .transpose()?,
        })
    }

    pub fn service_identifier(&self) -> &str {
        &self.service_identifier
    }

    pub fn service_identity_kind(&self) -> ServiceIdentityKind {
        self.service_identity_kind
    }

    pub fn service_namespace(&self) -> &str {
        &self.service_namespace
    }

    pub fn service_version_reference(&self) -> &str {
        &self.service_version_reference
    }

    pub fn declared_capability_references(&self) -> &[ServiceCapabilityReference] {
        &self.declared_capability_references
    }
}
