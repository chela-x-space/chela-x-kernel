use crate::adapter::AdapterKind;
use crate::adapter_capability::AdapterCapabilityReference;
use crate::adapter_error::{AdapterErrorCode, AdapterResult};
use crate::adapter_validation::{
    reject_duplicates, validate_namespaced_identifier, validate_version_reference,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdapterIdentityKind {
    ExternalSystem,
    ExternalAdapter,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterIdentity {
    adapter_identifier: String,
    adapter_identity_kind: AdapterIdentityKind,
    adapter_kind: AdapterKind,
    adapter_namespace: String,
    adapter_version_reference: String,
    declared_capability_references: Vec<AdapterCapabilityReference>,
    environment_reference: Option<String>,
}

impl AdapterIdentity {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        adapter_identifier: impl Into<String>,
        adapter_identity_kind: AdapterIdentityKind,
        adapter_kind: AdapterKind,
        adapter_namespace: impl Into<String>,
        adapter_version_reference: impl Into<String>,
        declared_capability_references: Vec<AdapterCapabilityReference>,
        environment_reference: Option<String>,
    ) -> AdapterResult<Self> {
        let adapter_identifier = validate_namespaced_identifier(
            adapter_identifier,
            AdapterErrorCode::InvalidAdapterIdentity,
            "adapter identity references require namespaced logical identifiers",
        )?;
        let adapter_namespace = validate_namespaced_identifier(
            adapter_namespace,
            AdapterErrorCode::InvalidAdapterIdentity,
            "adapter namespace references require namespaced logical identifiers",
        )?;
        reject_duplicates(
            &declared_capability_references,
            AdapterErrorCode::CapabilityMismatch,
            "duplicate declared adapter capability reference",
        )?;
        Ok(Self {
            adapter_identifier,
            adapter_identity_kind,
            adapter_kind,
            adapter_namespace,
            adapter_version_reference: validate_version_reference(
                adapter_version_reference,
                AdapterErrorCode::InvalidAdapterIdentity,
                "adapter version references must be namespace-safe and transport-neutral",
            )?,
            declared_capability_references,
            environment_reference: environment_reference
                .map(|value| {
                    validate_namespaced_identifier(
                        value,
                        AdapterErrorCode::InvalidAdapterIdentity,
                        "adapter environment references require namespaced logical identifiers",
                    )
                })
                .transpose()?,
        })
    }

    pub fn adapter_identifier(&self) -> &str {
        &self.adapter_identifier
    }

    pub fn adapter_identity_kind(&self) -> AdapterIdentityKind {
        self.adapter_identity_kind
    }

    pub fn adapter_kind(&self) -> AdapterKind {
        self.adapter_kind
    }

    pub fn adapter_namespace(&self) -> &str {
        &self.adapter_namespace
    }

    pub fn adapter_version_reference(&self) -> &str {
        &self.adapter_version_reference
    }

    pub fn declared_capability_references(&self) -> &[AdapterCapabilityReference] {
        &self.declared_capability_references
    }
}
