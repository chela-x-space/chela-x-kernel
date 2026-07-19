use kernel_service::{ServiceError, ServiceErrorCode};

pub type AdapterResult<T> = Result<T, AdapterError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdapterErrorCode {
    InvalidAdapterRequest,
    UnsupportedAdapterVersion,
    InvalidAdapterIdentity,
    AdapterRequestIdentityMismatch,
    CapabilityMismatch,
    ServiceCompatibilityMismatch,
    CorrelationMismatch,
    CommandQueryMismatch,
    ScopeMismatch,
    AuditMismatch,
    ResponseRequestMismatch,
    InternalContractViolation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterError {
    code: AdapterErrorCode,
    detail: String,
    service_error: Option<Box<ServiceError>>,
}

impl AdapterError {
    pub fn new(code: AdapterErrorCode, detail: impl Into<String>) -> AdapterResult<Self> {
        let detail = detail.into();
        if detail.trim().is_empty() {
            return Err(Self {
                code: AdapterErrorCode::InvalidAdapterRequest,
                detail: "adapter error detail must not be empty".to_owned(),
                service_error: None,
            });
        }
        Ok(Self {
            code,
            detail,
            service_error: None,
        })
    }

    pub fn from_service_rejection(service_error: ServiceError) -> Self {
        let code = match service_error.code() {
            ServiceErrorCode::InvalidServiceRequest => AdapterErrorCode::InvalidAdapterRequest,
            ServiceErrorCode::UnsupportedServiceVersion => {
                AdapterErrorCode::UnsupportedAdapterVersion
            }
            ServiceErrorCode::InvalidServiceIdentity => AdapterErrorCode::InvalidAdapterIdentity,
            ServiceErrorCode::ServiceRequestIdentityMismatch => {
                AdapterErrorCode::AdapterRequestIdentityMismatch
            }
            ServiceErrorCode::CapabilityMismatch => AdapterErrorCode::CapabilityMismatch,
            ServiceErrorCode::ApplicationRequestMismatch => {
                AdapterErrorCode::AdapterRequestIdentityMismatch
            }
            ServiceErrorCode::CommandQueryMismatch => AdapterErrorCode::CommandQueryMismatch,
            ServiceErrorCode::ResponseRequestMismatch => AdapterErrorCode::ResponseRequestMismatch,
            ServiceErrorCode::CompatibilityMismatch => {
                AdapterErrorCode::ServiceCompatibilityMismatch
            }
            ServiceErrorCode::InternalContractViolation => {
                AdapterErrorCode::InternalContractViolation
            }
        };
        Self {
            code,
            detail: service_error.detail().to_owned(),
            service_error: Some(Box::new(service_error)),
        }
    }

    pub fn code(&self) -> AdapterErrorCode {
        self.code
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}
