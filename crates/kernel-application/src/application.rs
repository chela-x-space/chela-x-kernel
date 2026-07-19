use kernel_domain::{CorrelationId, StableVersion};

use crate::application_command::ApplicationCommandIntent;
use crate::application_error::{ApplicationError, ApplicationErrorCode, ApplicationResult};
use crate::application_query::ApplicationQueryIntent;

pub const APPLICATION_COMMAND_CAPABILITY: &str = "application.command";
pub const APPLICATION_QUERY_CAPABILITY: &str = "application.query";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplicationIntentKind {
    View,
    Command,
    Query,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplicationResponseKind {
    View,
    Command,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationApiVersion(StableVersion);

impl ApplicationApiVersion {
    pub fn new(value: impl Into<String>) -> ApplicationResult<Self> {
        let value = value.into();
        let trimmed = value.trim();
        if trimmed.is_empty()
            || trimmed.contains('/')
            || !trimmed.chars().all(|character| {
                character.is_ascii_alphanumeric() || matches!(character, '.' | '_' | '-')
            })
        {
            return Err(ApplicationError::new(
                ApplicationErrorCode::UnsupportedApplicationVersion,
                "application API version must be namespace-safe and transport-neutral",
            )?);
        }
        Ok(Self(
            StableVersion::new("application_api_version", trimmed.to_owned())
                .map_err(ApplicationError::from_domain_rejection)?,
        ))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApplicationRequestEnvelope {
    Command(Box<ApplicationCommandIntent>),
    Query(Box<ApplicationQueryIntent>),
}

impl ApplicationRequestEnvelope {
    pub fn command(application_command_intent: ApplicationCommandIntent) -> Self {
        Self::Command(Box::new(application_command_intent))
    }

    pub fn query(application_query_intent: ApplicationQueryIntent) -> Self {
        Self::Query(Box::new(application_query_intent))
    }

    pub fn correlation_id(&self) -> &CorrelationId {
        match self {
            Self::Command(intent) => intent.application_request_context().correlation_id(),
            Self::Query(intent) => intent.application_request_context().correlation_id(),
        }
    }

    pub fn application_request_id(&self) -> &crate::ApplicationRequestId {
        match self {
            Self::Command(intent) => intent
                .application_request_context()
                .application_request_id(),
            Self::Query(intent) => intent
                .application_request_context()
                .application_request_id(),
        }
    }

    pub fn application_identity(&self) -> &crate::ApplicationIdentity {
        match self {
            Self::Command(intent) => intent.application_request_context().application_identity(),
            Self::Query(intent) => intent.application_request_context().application_identity(),
        }
    }

    pub fn application_audit_reference(&self) -> &crate::ApplicationAuditReference {
        match self {
            Self::Command(intent) => intent
                .application_request_context()
                .application_audit_reference(),
            Self::Query(intent) => intent
                .application_request_context()
                .application_audit_reference(),
        }
    }

    pub fn ownership_path(&self) -> &kernel_domain::OwnershipPath {
        match self {
            Self::Command(intent) => intent.application_request_context().ownership_path(),
            Self::Query(intent) => intent.application_request_context().ownership_path(),
        }
    }
}
