use kernel_domain::{CorrelationId, NonEmptyText};
use kernel_gateway::{GatewayError, GatewayErrorCode};
use kernel_studio::{StudioError, StudioErrorCode};

pub type ApplicationResult<T> = Result<T, ApplicationError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplicationErrorCode {
    InvalidApplicationRequest,
    UnsupportedApplicationVersion,
    InvalidApplicationIdentity,
    ApplicationRequestIdentityMismatch,
    AuthorizationEvidenceMismatch,
    ScopeMismatch,
    SessionApplicationMismatch,
    SessionScopeMismatch,
    SessionCorrelationMismatch,
    CapabilityMismatch,
    InvalidNavigationIntent,
    ViewRequestMismatch,
    CommandQueryMismatch,
    ResponseCorrelationMismatch,
    ResponseRequestMismatch,
    AuditEvidenceMismatch,
    GatewayAuthorizationDenied,
    GatewayRequestRejection,
    StudioRequestRejection,
    StudioResponseMismatch,
    InternalContractViolation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationError {
    code: ApplicationErrorCode,
    detail: Box<NonEmptyText>,
    correlation_id: Option<Box<CorrelationId>>,
    gateway_error: Option<Box<GatewayError>>,
    studio_error: Option<Box<StudioError>>,
}

impl ApplicationError {
    pub fn new(code: ApplicationErrorCode, detail: impl Into<String>) -> ApplicationResult<Self> {
        Ok(Self {
            code,
            detail: Box::new(
                NonEmptyText::new("application_error_detail", detail)
                    .map_err(Self::from_domain_rejection)?,
            ),
            correlation_id: None,
            gateway_error: None,
            studio_error: None,
        })
    }

    pub fn from_domain_rejection(domain_error: kernel_domain::DomainError) -> Self {
        Self {
            code: ApplicationErrorCode::InvalidApplicationRequest,
            detail: Box::new(
                NonEmptyText::new("application_error_detail", domain_error.to_string())
                    .expect("application domain rejection detail"),
            ),
            correlation_id: None,
            gateway_error: None,
            studio_error: None,
        }
    }

    pub fn from_gateway_rejection(gateway_error: GatewayError) -> Self {
        let code = match gateway_error.code() {
            GatewayErrorCode::AuthorizationDenied => {
                ApplicationErrorCode::GatewayAuthorizationDenied
            }
            _ => ApplicationErrorCode::GatewayRequestRejection,
        };
        Self {
            code,
            detail: Box::new(
                NonEmptyText::new("application_error_detail", gateway_error.detail())
                    .expect("gateway rejection detail"),
            ),
            correlation_id: gateway_error.correlation_id().cloned().map(Box::new),
            gateway_error: Some(Box::new(gateway_error)),
            studio_error: None,
        }
    }

    pub fn from_studio_rejection(studio_error: StudioError) -> Self {
        let code = match studio_error.code() {
            StudioErrorCode::GatewayAuthorizationDenied => {
                ApplicationErrorCode::GatewayAuthorizationDenied
            }
            StudioErrorCode::GatewayRequestRejection | StudioErrorCode::GatewayDomainRejection => {
                ApplicationErrorCode::StudioRequestRejection
            }
            StudioErrorCode::ResponseCorrelationMismatch => {
                ApplicationErrorCode::ResponseCorrelationMismatch
            }
            StudioErrorCode::AuditReferenceMismatch => ApplicationErrorCode::AuditEvidenceMismatch,
            StudioErrorCode::ScopeMismatch => ApplicationErrorCode::ScopeMismatch,
            StudioErrorCode::CommandOperationMismatch | StudioErrorCode::ViewQueryMismatch => {
                ApplicationErrorCode::CommandQueryMismatch
            }
            StudioErrorCode::UnsupportedStudioVersion => {
                ApplicationErrorCode::UnsupportedApplicationVersion
            }
            _ => ApplicationErrorCode::StudioRequestRejection,
        };
        Self {
            code,
            detail: Box::new(
                NonEmptyText::new("application_error_detail", format!("{studio_error:?}"))
                    .expect("studio rejection detail"),
            ),
            correlation_id: None,
            gateway_error: None,
            studio_error: Some(Box::new(studio_error)),
        }
    }

    pub fn with_correlation_id(mut self, correlation_id: CorrelationId) -> Self {
        self.correlation_id = Some(Box::new(correlation_id));
        self
    }

    pub fn code(&self) -> ApplicationErrorCode {
        self.code
    }

    pub fn detail(&self) -> &str {
        self.detail.as_str()
    }

    pub fn correlation_id(&self) -> Option<&CorrelationId> {
        self.correlation_id.as_deref()
    }
}
