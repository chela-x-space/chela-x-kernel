use kernel_domain::EnglishNamespace;
use kernel_studio::StudioViewReference;

use crate::application::{
    ApplicationApiVersion, ApplicationIntentKind, APPLICATION_COMMAND_CAPABILITY,
    APPLICATION_QUERY_CAPABILITY,
};
use crate::application_error::{ApplicationError, ApplicationErrorCode, ApplicationResult};
use crate::application_validation::reject_duplicates;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationCapabilityReference(EnglishNamespace);

impl ApplicationCapabilityReference {
    pub fn new(value: impl Into<String>) -> ApplicationResult<Self> {
        let reference = EnglishNamespace::new("application_capability_reference", value)
            .map_err(ApplicationError::from_domain_rejection)?;
        if !reference.as_str().contains('.') {
            return Err(ApplicationError::new(
                ApplicationErrorCode::CapabilityMismatch,
                "application capability references require namespaced identifiers",
            )?);
        }
        Ok(Self(reference))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationCapabilityDeclaration {
    application_api_version: ApplicationApiVersion,
    admitted_capabilities: Vec<ApplicationCapabilityReference>,
    supported_view_references: Vec<StudioViewReference>,
    supports_commands: bool,
    supports_queries: bool,
}

impl ApplicationCapabilityDeclaration {
    pub fn new(
        application_api_version: ApplicationApiVersion,
        admitted_capabilities: Vec<ApplicationCapabilityReference>,
        supported_view_references: Vec<StudioViewReference>,
        supports_commands: bool,
        supports_queries: bool,
    ) -> ApplicationResult<Self> {
        if admitted_capabilities.is_empty() {
            return Err(ApplicationError::new(
                ApplicationErrorCode::CapabilityMismatch,
                "application capability declarations require at least one admitted capability",
            )?);
        }
        reject_duplicates(
            &admitted_capabilities,
            ApplicationErrorCode::CapabilityMismatch,
            "duplicate application capability reference",
        )?;
        reject_duplicates(
            &supported_view_references,
            ApplicationErrorCode::CapabilityMismatch,
            "duplicate supported Studio view reference",
        )?;
        if !supports_queries && !supported_view_references.is_empty() {
            return Err(ApplicationError::new(
                ApplicationErrorCode::CapabilityMismatch,
                "supported Studio views require query capability admission",
            )?);
        }
        if supports_commands
            && !admitted_capabilities
                .iter()
                .any(|capability| capability.as_str() == APPLICATION_COMMAND_CAPABILITY)
        {
            return Err(ApplicationError::new(
                ApplicationErrorCode::CapabilityMismatch,
                "command admission requires the canonical application command capability",
            )?);
        }
        if supports_queries
            && !admitted_capabilities
                .iter()
                .any(|capability| capability.as_str() == APPLICATION_QUERY_CAPABILITY)
        {
            return Err(ApplicationError::new(
                ApplicationErrorCode::CapabilityMismatch,
                "query admission requires the canonical application query capability",
            )?);
        }
        Ok(Self {
            application_api_version,
            admitted_capabilities,
            supported_view_references,
            supports_commands,
            supports_queries,
        })
    }

    pub fn application_api_version(&self) -> &ApplicationApiVersion {
        &self.application_api_version
    }

    pub fn admitted_capabilities(&self) -> &[ApplicationCapabilityReference] {
        &self.admitted_capabilities
    }

    pub fn supported_view_references(&self) -> &[StudioViewReference] {
        &self.supported_view_references
    }

    pub fn supports_commands(&self) -> bool {
        self.supports_commands
    }

    pub fn supports_queries(&self) -> bool {
        self.supports_queries
    }

    pub fn admits(&self, capability_reference: &ApplicationCapabilityReference) -> bool {
        self.admitted_capabilities
            .iter()
            .any(|capability| capability == capability_reference)
    }

    pub fn supports_view(&self, studio_view_reference: &StudioViewReference) -> bool {
        self.supported_view_references
            .iter()
            .any(|reference| reference == studio_view_reference)
    }

    pub fn require_capability(
        &self,
        capability_reference: &ApplicationCapabilityReference,
        intent_kind: ApplicationIntentKind,
    ) -> ApplicationResult<()> {
        if !self.admits(capability_reference) {
            return Err(ApplicationError::new(
                ApplicationErrorCode::CapabilityMismatch,
                "application capability declaration does not admit the requested capability",
            )?);
        }
        match intent_kind {
            ApplicationIntentKind::Command if !self.supports_commands => {
                return Err(ApplicationError::new(
                    ApplicationErrorCode::CapabilityMismatch,
                    "application capability declaration does not admit command intent",
                )?);
            }
            ApplicationIntentKind::Query if !self.supports_queries => {
                return Err(ApplicationError::new(
                    ApplicationErrorCode::CapabilityMismatch,
                    "application capability declaration does not admit query intent",
                )?);
            }
            _ => {}
        }
        Ok(())
    }
}
