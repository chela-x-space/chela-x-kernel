use kernel_application::{ApplicationError, ApplicationErrorCode};

pub type ServiceResult<T> = Result<T, ServiceError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceErrorCode {
    InvalidServiceRequest,
    UnsupportedServiceVersion,
    InvalidServiceIdentity,
    ServiceRequestIdentityMismatch,
    CapabilityMismatch,
    ApplicationRequestMismatch,
    CommandQueryMismatch,
    ResponseRequestMismatch,
    CompatibilityMismatch,
    InternalContractViolation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceError {
    code: ServiceErrorCode,
    detail: String,
    application_error: Option<Box<ApplicationError>>,
}

impl ServiceError {
    pub fn new(code: ServiceErrorCode, detail: impl Into<String>) -> ServiceResult<Self> {
        let detail = detail.into();
        if detail.trim().is_empty() {
            return Err(Self {
                code: ServiceErrorCode::InvalidServiceRequest,
                detail: "service error detail must not be empty".to_owned(),
                application_error: None,
            });
        }
        Ok(Self {
            code,
            detail,
            application_error: None,
        })
    }

    pub fn from_application_rejection(application_error: ApplicationError) -> Self {
        let code = match application_error.code() {
            ApplicationErrorCode::UnsupportedApplicationVersion => {
                ServiceErrorCode::UnsupportedServiceVersion
            }
            ApplicationErrorCode::InvalidApplicationIdentity => {
                ServiceErrorCode::InvalidServiceIdentity
            }
            ApplicationErrorCode::ApplicationRequestIdentityMismatch
            | ApplicationErrorCode::SessionApplicationMismatch
            | ApplicationErrorCode::SessionScopeMismatch
            | ApplicationErrorCode::SessionCorrelationMismatch
            | ApplicationErrorCode::AuthorizationEvidenceMismatch
            | ApplicationErrorCode::ScopeMismatch => ServiceErrorCode::ApplicationRequestMismatch,
            ApplicationErrorCode::CapabilityMismatch => ServiceErrorCode::CapabilityMismatch,
            ApplicationErrorCode::CommandQueryMismatch
            | ApplicationErrorCode::ViewRequestMismatch => ServiceErrorCode::CommandQueryMismatch,
            ApplicationErrorCode::ResponseCorrelationMismatch
            | ApplicationErrorCode::ResponseRequestMismatch
            | ApplicationErrorCode::StudioResponseMismatch => {
                ServiceErrorCode::ResponseRequestMismatch
            }
            ApplicationErrorCode::InternalContractViolation => {
                ServiceErrorCode::InternalContractViolation
            }
            _ => ServiceErrorCode::InvalidServiceRequest,
        };
        Self {
            code,
            detail: application_error.detail().to_owned(),
            application_error: Some(Box::new(application_error)),
        }
    }

    pub fn code(&self) -> ServiceErrorCode {
        self.code
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}
