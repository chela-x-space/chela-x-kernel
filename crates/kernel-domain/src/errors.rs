use std::error::Error;
use std::fmt;

pub type DomainResult<T> = Result<T, DomainError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DomainError {
    EmptyValue {
        field: &'static str,
    },
    InvalidIdentifier {
        kind: &'static str,
        value: String,
        expected: &'static str,
    },
    InvalidOwnershipPath(&'static str),
    InvalidLifecycleTransition {
        lifecycle: &'static str,
        from: &'static str,
        to: &'static str,
    },
    InvalidIdentity(&'static str),
    InvalidRequestRecord(&'static str),
    InvalidDecisionRecord(&'static str),
    InvalidAuthorizationReference(&'static str),
    InvalidAgentReference(&'static str),
    InvalidDelegationReference(&'static str),
    InvalidPolicyReference(&'static str),
    InvalidWorkflowReference(&'static str),
}

impl fmt::Display for DomainError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyValue { field } => write!(formatter, "empty value: {field}"),
            Self::InvalidIdentifier {
                kind,
                value,
                expected,
            } => {
                write!(
                    formatter,
                    "invalid {kind} identifier `{value}` expected {expected}"
                )
            }
            Self::InvalidOwnershipPath(message) => {
                write!(formatter, "invalid ownership path: {message}")
            }
            Self::InvalidLifecycleTransition {
                lifecycle,
                from,
                to,
            } => {
                write!(
                    formatter,
                    "invalid {lifecycle} transition from {from} to {to}"
                )
            }
            Self::InvalidIdentity(message) => write!(formatter, "invalid identity: {message}"),
            Self::InvalidRequestRecord(message) => {
                write!(formatter, "invalid request record: {message}")
            }
            Self::InvalidDecisionRecord(message) => {
                write!(formatter, "invalid decision record: {message}")
            }
            Self::InvalidAuthorizationReference(message) => {
                write!(formatter, "invalid authorization reference: {message}")
            }
            Self::InvalidAgentReference(message) => {
                write!(formatter, "invalid agent reference: {message}")
            }
            Self::InvalidDelegationReference(message) => {
                write!(formatter, "invalid delegation reference: {message}")
            }
            Self::InvalidPolicyReference(message) => {
                write!(formatter, "invalid policy reference: {message}")
            }
            Self::InvalidWorkflowReference(message) => {
                write!(formatter, "invalid workflow reference: {message}")
            }
        }
    }
}

impl Error for DomainError {}
