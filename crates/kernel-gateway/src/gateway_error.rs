use kernel_domain::{CorrelationId, DomainError, NonEmptyText};

use crate::gateway::GatewayAuditReference;
use crate::gateway_contract::GatewayOperationReference;

pub type GatewayResult<T> = Result<T, GatewayError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GatewayErrorCode {
    InvalidRequest,
    AuthenticationRequired,
    AuthorizationDenied,
    ScopeMismatch,
    UnsupportedOperation,
    UnsupportedGatewayVersion,
    InvalidCommand,
    InvalidQuery,
    DomainRejection,
    InternalContractViolation,
    RateGovernanceRejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayError {
    code: GatewayErrorCode,
    detail: Box<NonEmptyText>,
    gateway_operation_reference: Option<Box<GatewayOperationReference>>,
    correlation_id: Option<Box<CorrelationId>>,
    gateway_audit_reference: Option<Box<GatewayAuditReference>>,
}

impl GatewayError {
    pub fn new(code: GatewayErrorCode, detail: impl Into<String>) -> GatewayResult<Self> {
        Ok(Self {
            code,
            detail: Box::new(
                NonEmptyText::new("gateway_error_detail", detail)
                    .map_err(Self::from_domain_rejection)?,
            ),
            gateway_operation_reference: None,
            correlation_id: None,
            gateway_audit_reference: None,
        })
    }

    pub fn with_context(
        code: GatewayErrorCode,
        detail: impl Into<String>,
        gateway_operation_reference: Option<GatewayOperationReference>,
        correlation_id: Option<CorrelationId>,
        gateway_audit_reference: Option<GatewayAuditReference>,
    ) -> GatewayResult<Self> {
        if let (Some(correlation_id), Some(gateway_audit_reference)) =
            (correlation_id.as_ref(), gateway_audit_reference.as_ref())
        {
            if gateway_audit_reference.correlation_id() != Some(correlation_id) {
                return Err(Self {
                    code: GatewayErrorCode::InternalContractViolation,
                    detail: Box::new(
                        NonEmptyText::new(
                            "gateway_error_detail",
                            "gateway error audit correlation must match the gateway correlation reference",
                        )
                        .map_err(Self::from_domain_rejection)?,
                    ),
                    gateway_operation_reference: None,
                    correlation_id: None,
                    gateway_audit_reference: None,
                });
            }
        }
        Ok(Self {
            code,
            detail: Box::new(
                NonEmptyText::new("gateway_error_detail", detail)
                    .map_err(Self::from_domain_rejection)?,
            ),
            gateway_operation_reference: gateway_operation_reference.map(Box::new),
            correlation_id: correlation_id.map(Box::new),
            gateway_audit_reference: gateway_audit_reference.map(Box::new),
        })
    }

    pub fn from_domain_rejection(domain_error: DomainError) -> Self {
        let code = match domain_error {
            DomainError::EmptyValue { .. }
            | DomainError::InvalidIdentifier { .. }
            | DomainError::InvalidRequestRecord(_)
            | DomainError::InvalidAuthorizationReference(_)
            | DomainError::InvalidOwnershipPath(_) => GatewayErrorCode::InvalidRequest,
            DomainError::MissingAuthorizationEvidence(_)
            | DomainError::UnsupportedAuthorizationSemantics(_) => {
                GatewayErrorCode::AuthorizationDenied
            }
            DomainError::InvalidWorkflowDefinition(_)
            | DomainError::InvalidWorkflowInstance(_)
            | DomainError::InvalidWorkflowTransitionControl(_)
            | DomainError::InvalidWorkflowStepCoordination(_)
            | DomainError::InvalidWorkflowAuthorizationIntegration(_)
            | DomainError::InvalidWorkflowEventIntegration(_)
            | DomainError::InvalidWorkflowFailureRecovery(_)
            | DomainError::InvalidTaskDefinition(_)
            | DomainError::InvalidTaskInstance(_)
            | DomainError::InvalidTaskDependency(_)
            | DomainError::InvalidTaskLifecycle(_)
            | DomainError::InvalidTaskPriority(_)
            | DomainError::InvalidTaskReadiness(_)
            | DomainError::InvalidTaskCompletion(_)
            | DomainError::InvalidTaskFailure(_)
            | DomainError::InvalidTaskEvidence(_)
            | DomainError::InvalidTaskOwnership(_)
            | DomainError::InvalidTaskAssignment(_)
            | DomainError::InvalidExecution(_)
            | DomainError::InvalidMemory(_)
            | DomainError::InvalidEventReference(_)
            | DomainError::InvalidEventTimestamp(_)
            | DomainError::IntegrityFailure(_)
            | DomainError::InvalidStreamAppend(_)
            | DomainError::InvalidReplayOrdering(_)
            | DomainError::InvalidReplayValidation(_) => GatewayErrorCode::DomainRejection,
            _ => GatewayErrorCode::InternalContractViolation,
        };
        Self {
            code,
            detail: Box::new(
                NonEmptyText::new("gateway_error_detail", domain_error.to_string())
                    .expect("domain rejection detail"),
            ),
            gateway_operation_reference: None,
            correlation_id: None,
            gateway_audit_reference: None,
        }
    }

    pub fn code(&self) -> GatewayErrorCode {
        self.code
    }

    pub fn detail(&self) -> &str {
        self.detail.as_str()
    }

    pub fn gateway_operation_reference(&self) -> Option<&GatewayOperationReference> {
        self.gateway_operation_reference.as_deref()
    }

    pub fn correlation_id(&self) -> Option<&CorrelationId> {
        self.correlation_id.as_deref()
    }

    pub fn gateway_audit_reference(&self) -> Option<&GatewayAuditReference> {
        self.gateway_audit_reference.as_deref()
    }
}
