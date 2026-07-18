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
    InvalidAuthorizationEvaluation(&'static str),
    MissingAuthorizationEvidence(&'static str),
    InvalidAgentReference(&'static str),
    InvalidRuntimeReference(&'static str),
    InvalidRuntimeRegistry(&'static str),
    InvalidDelegationReference(&'static str),
    InvalidPolicyReference(&'static str),
    InvalidWorkflowReference(&'static str),
    InvalidTaskDefinition(&'static str),
    InvalidTaskInstance(&'static str),
    InvalidTaskDependency(&'static str),
    InvalidTaskLifecycle(&'static str),
    InvalidTaskPriority(&'static str),
    InvalidTaskReadiness(&'static str),
    InvalidTaskCompletion(&'static str),
    InvalidTaskFailure(&'static str),
    InvalidTaskEvidence(&'static str),
    InvalidTaskOwnership(&'static str),
    InvalidTaskAssignment(&'static str),
    InvalidExecution(&'static str),
    InvalidWorkflowDefinition(&'static str),
    InvalidWorkflowInstance(&'static str),
    InvalidWorkflowTransitionControl(&'static str),
    InvalidWorkflowStepCoordination(&'static str),
    InvalidWorkflowAuthorizationIntegration(&'static str),
    InvalidWorkflowEventIntegration(&'static str),
    InvalidWorkflowFailureRecovery(&'static str),
    InvalidEventReference(&'static str),
    InvalidEventTimestamp(&'static str),
    IntegrityFailure(&'static str),
    InvalidStreamAppend(&'static str),
    InvalidReplayOrdering(&'static str),
    InvalidReplayValidation(&'static str),
    MissingEventField(&'static str),
    UnsupportedAuthorizationSemantics(&'static str),
}

impl fmt::Display for DomainError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyValue { field } => {
                write!(formatter, "empty value: {field}")
            }
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
            Self::InvalidIdentity(message) => {
                write!(formatter, "invalid identity: {message}")
            }
            Self::InvalidRequestRecord(message) => {
                write!(formatter, "invalid request record: {message}")
            }
            Self::InvalidDecisionRecord(message) => {
                write!(formatter, "invalid decision record: {message}")
            }
            Self::InvalidAuthorizationReference(message) => {
                write!(formatter, "invalid authorization reference: {message}")
            }
            Self::InvalidAuthorizationEvaluation(message) => {
                write!(formatter, "invalid authorization evaluation: {message}")
            }
            Self::MissingAuthorizationEvidence(message) => {
                write!(formatter, "missing authorization evidence: {message}")
            }
            Self::InvalidAgentReference(message) => {
                write!(formatter, "invalid agent reference: {message}")
            }
            Self::InvalidRuntimeReference(message) => {
                write!(formatter, "invalid runtime reference: {message}")
            }
            Self::InvalidRuntimeRegistry(message) => {
                write!(formatter, "invalid runtime registry: {message}")
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
            Self::InvalidTaskDefinition(message) => {
                write!(formatter, "invalid task definition: {message}")
            }
            Self::InvalidTaskInstance(message) => {
                write!(formatter, "invalid task instance: {message}")
            }
            Self::InvalidTaskDependency(message) => {
                write!(formatter, "invalid task dependency: {message}")
            }
            Self::InvalidTaskLifecycle(message) => {
                write!(formatter, "invalid task lifecycle: {message}")
            }
            Self::InvalidTaskPriority(message) => {
                write!(formatter, "invalid task priority: {message}")
            }
            Self::InvalidTaskReadiness(message) => {
                write!(formatter, "invalid task readiness: {message}")
            }
            Self::InvalidTaskCompletion(message) => {
                write!(formatter, "invalid task completion: {message}")
            }
            Self::InvalidTaskFailure(message) => {
                write!(formatter, "invalid task failure: {message}")
            }
            Self::InvalidTaskEvidence(message) => {
                write!(formatter, "invalid task evidence: {message}")
            }
            Self::InvalidTaskOwnership(message) => {
                write!(formatter, "invalid task ownership: {message}")
            }
            Self::InvalidTaskAssignment(message) => {
                write!(formatter, "invalid task assignment: {message}")
            }
            Self::InvalidExecution(message) => {
                write!(formatter, "invalid execution: {message}")
            }
            Self::InvalidWorkflowDefinition(message) => {
                write!(formatter, "invalid workflow definition: {message}")
            }
            Self::InvalidWorkflowInstance(message) => {
                write!(formatter, "invalid workflow instance: {message}")
            }
            Self::InvalidWorkflowTransitionControl(message) => {
                write!(formatter, "invalid workflow transition control: {message}")
            }
            Self::InvalidWorkflowStepCoordination(message) => {
                write!(formatter, "invalid workflow step coordination: {message}")
            }
            Self::InvalidWorkflowAuthorizationIntegration(message) => {
                write!(
                    formatter,
                    "invalid workflow authorization integration: {message}"
                )
            }
            Self::InvalidWorkflowEventIntegration(message) => {
                write!(formatter, "invalid workflow event integration: {message}")
            }
            Self::InvalidWorkflowFailureRecovery(message) => {
                write!(formatter, "invalid workflow failure recovery: {message}")
            }
            Self::InvalidEventReference(message) => {
                write!(formatter, "invalid event reference: {message}")
            }
            Self::InvalidEventTimestamp(message) => {
                if *message == "recorded_at must not precede occurred_at" {
                    formatter.write_str(message)
                } else {
                    write!(formatter, "invalid event timestamp: {message}")
                }
            }
            Self::IntegrityFailure(message) => {
                write!(formatter, "integrity failure: {message}")
            }
            Self::InvalidStreamAppend(message) => {
                write!(formatter, "invalid stream append: {message}")
            }
            Self::InvalidReplayOrdering(message) => {
                write!(formatter, "invalid replay ordering: {message}")
            }
            Self::InvalidReplayValidation(message) => {
                write!(formatter, "invalid replay validation: {message}")
            }
            Self::MissingEventField(field) => {
                write!(formatter, "missing mandatory event field: {field}")
            }
            Self::UnsupportedAuthorizationSemantics(message) => {
                write!(formatter, "unsupported authorization semantics: {message}")
            }
        }
    }
}

impl Error for DomainError {}
