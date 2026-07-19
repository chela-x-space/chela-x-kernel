use kernel_domain::{EnglishNamespace, StableVersion};

use crate::application_capability::ApplicationCapabilityReference;
use crate::application_error::{ApplicationError, ApplicationErrorCode, ApplicationResult};
use crate::application_validation::reject_duplicates;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplicationIdentityKind {
    ExternalApplication,
    ExternalAdapter,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationIdentity {
    application_identifier: EnglishNamespace,
    application_identity_kind: ApplicationIdentityKind,
    application_namespace: EnglishNamespace,
    application_version_reference: StableVersion,
    declared_capability_references: Vec<ApplicationCapabilityReference>,
    environment_reference: Option<EnglishNamespace>,
}

impl ApplicationIdentity {
    pub fn new(
        application_identifier: impl Into<String>,
        application_identity_kind: ApplicationIdentityKind,
        application_namespace: impl Into<String>,
        application_version_reference: impl Into<String>,
        declared_capability_references: Vec<ApplicationCapabilityReference>,
        environment_reference: Option<String>,
    ) -> ApplicationResult<Self> {
        let application_identifier =
            EnglishNamespace::new("application_identifier", application_identifier)
                .map_err(ApplicationError::from_domain_rejection)?;
        let application_namespace =
            EnglishNamespace::new("application_namespace", application_namespace)
                .map_err(ApplicationError::from_domain_rejection)?;
        if !application_identifier.as_str().contains('.')
            || !application_namespace.as_str().contains('.')
        {
            return Err(ApplicationError::new(
                ApplicationErrorCode::InvalidApplicationIdentity,
                "application identity references require namespaced logical identifiers",
            )?);
        }
        reject_duplicates(
            &declared_capability_references,
            ApplicationErrorCode::CapabilityMismatch,
            "duplicate declared application capability reference",
        )?;
        Ok(Self {
            application_identifier,
            application_identity_kind,
            application_namespace,
            application_version_reference: StableVersion::new(
                "application_version_reference",
                application_version_reference,
            )
            .map_err(ApplicationError::from_domain_rejection)?,
            declared_capability_references,
            environment_reference: environment_reference
                .map(|value| {
                    EnglishNamespace::new("application_environment_reference", value)
                        .map_err(ApplicationError::from_domain_rejection)
                })
                .transpose()?,
        })
    }

    pub fn application_identifier(&self) -> &EnglishNamespace {
        &self.application_identifier
    }

    pub fn application_identity_kind(&self) -> ApplicationIdentityKind {
        self.application_identity_kind
    }

    pub fn application_namespace(&self) -> &EnglishNamespace {
        &self.application_namespace
    }

    pub fn application_version_reference(&self) -> &StableVersion {
        &self.application_version_reference
    }

    pub fn declared_capability_references(&self) -> &[ApplicationCapabilityReference] {
        &self.declared_capability_references
    }

    pub fn environment_reference(&self) -> Option<&EnglishNamespace> {
        self.environment_reference.as_ref()
    }
}
