use kernel_domain::{EnglishNamespace, TimeReference};
use kernel_studio::{StudioStatusSnapshot, StudioViewReference};

use crate::application::{ApplicationApiVersion, ApplicationIntentKind};
use crate::application_capability::ApplicationCapabilityDeclaration;
use crate::application_context::ApplicationAuditReference;
use crate::application_error::{ApplicationError, ApplicationErrorCode, ApplicationResult};
use crate::application_identity::ApplicationIdentity;
use crate::application_validation::reject_duplicates;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplicationValidationStatus {
    Validated,
    ValidationPending,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationDependencyCompatibilityReference(EnglishNamespace);

impl ApplicationDependencyCompatibilityReference {
    pub fn new(value: impl Into<String>) -> ApplicationResult<Self> {
        Ok(Self(
            EnglishNamespace::new("application_dependency_compatibility_reference", value)
                .map_err(ApplicationError::from_domain_rejection)?,
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationStatusSnapshot {
    application_api_version: ApplicationApiVersion,
    application_identity: ApplicationIdentity,
    application_capability_declaration: ApplicationCapabilityDeclaration,
    supported_view_references: Vec<StudioViewReference>,
    supported_intent_kinds: Vec<ApplicationIntentKind>,
    dependency_compatibility_references: Vec<ApplicationDependencyCompatibilityReference>,
    application_validation_status: ApplicationValidationStatus,
    studio_status_snapshot: StudioStatusSnapshot,
    application_audit_reference: ApplicationAuditReference,
    generated_at: TimeReference,
}

impl ApplicationStatusSnapshot {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        application_api_version: ApplicationApiVersion,
        application_identity: ApplicationIdentity,
        application_capability_declaration: ApplicationCapabilityDeclaration,
        supported_view_references: Vec<StudioViewReference>,
        supported_intent_kinds: Vec<ApplicationIntentKind>,
        dependency_compatibility_references: Vec<ApplicationDependencyCompatibilityReference>,
        application_validation_status: ApplicationValidationStatus,
        studio_status_snapshot: StudioStatusSnapshot,
        application_audit_reference: ApplicationAuditReference,
        generated_at: TimeReference,
    ) -> ApplicationResult<Self> {
        if supported_intent_kinds.is_empty() {
            return Err(ApplicationError::new(
                ApplicationErrorCode::InternalContractViolation,
                "application status snapshots require at least one supported intent kind",
            )?);
        }
        reject_duplicates(
            &supported_view_references,
            ApplicationErrorCode::InternalContractViolation,
            "duplicate supported Studio view reference in application status snapshot",
        )?;
        reject_duplicates(
            &supported_intent_kinds,
            ApplicationErrorCode::InternalContractViolation,
            "duplicate supported application intent kind in status snapshot",
        )?;
        reject_duplicates(
            &dependency_compatibility_references,
            ApplicationErrorCode::InternalContractViolation,
            "duplicate dependency compatibility reference in application status snapshot",
        )?;
        if application_capability_declaration.application_api_version() != &application_api_version
        {
            return Err(ApplicationError::new(
                ApplicationErrorCode::InternalContractViolation,
                "application status snapshots must preserve the application API version in capability declarations",
            )?);
        }
        if supported_view_references
            != application_capability_declaration.supported_view_references()
        {
            return Err(ApplicationError::new(
                ApplicationErrorCode::CapabilityMismatch,
                "application status snapshots must preserve the declared supported Studio views",
            )?);
        }
        if supported_intent_kinds.contains(&ApplicationIntentKind::Command)
            && !application_capability_declaration.supports_commands()
        {
            return Err(ApplicationError::new(
                ApplicationErrorCode::CapabilityMismatch,
                "command intent support requires command capability admission",
            )?);
        }
        if (supported_intent_kinds.contains(&ApplicationIntentKind::View)
            || supported_intent_kinds.contains(&ApplicationIntentKind::Query))
            && !application_capability_declaration.supports_queries()
        {
            return Err(ApplicationError::new(
                ApplicationErrorCode::CapabilityMismatch,
                "view and query intent support requires query capability admission",
            )?);
        }
        Ok(Self {
            application_api_version,
            application_identity,
            application_capability_declaration,
            supported_view_references,
            supported_intent_kinds,
            dependency_compatibility_references,
            application_validation_status,
            studio_status_snapshot,
            application_audit_reference,
            generated_at,
        })
    }

    pub fn application_validation_status(&self) -> ApplicationValidationStatus {
        self.application_validation_status
    }
}
