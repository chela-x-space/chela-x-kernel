use kernel_domain::{CorrelationId, EnglishNamespace, OwnershipPath, TimeReference};

use crate::application_error::{ApplicationError, ApplicationErrorCode, ApplicationResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationSessionStatusReference(EnglishNamespace);

impl ApplicationSessionStatusReference {
    pub fn new(value: impl Into<String>) -> ApplicationResult<Self> {
        Ok(Self(
            EnglishNamespace::new("application_session_status_reference", value)
                .map_err(ApplicationError::from_domain_rejection)?,
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationSessionReference {
    session_identifier: EnglishNamespace,
    application_identifier: EnglishNamespace,
    ownership_path: OwnershipPath,
    correlation_id: CorrelationId,
    issued_at: TimeReference,
    evaluated_at: TimeReference,
    session_status_reference: ApplicationSessionStatusReference,
}

impl ApplicationSessionReference {
    pub fn new(
        session_identifier: impl Into<String>,
        application_identifier: EnglishNamespace,
        ownership_path: OwnershipPath,
        correlation_id: CorrelationId,
        issued_at: TimeReference,
        evaluated_at: TimeReference,
        session_status_reference: ApplicationSessionStatusReference,
    ) -> ApplicationResult<Self> {
        let session_identifier =
            EnglishNamespace::new("application_session_identifier", session_identifier)
                .map_err(ApplicationError::from_domain_rejection)?;
        if issued_at.as_str() > evaluated_at.as_str() {
            return Err(ApplicationError::new(
                ApplicationErrorCode::SessionCorrelationMismatch,
                "application session issued time must not be later than the evaluation time",
            )?);
        }
        Ok(Self {
            session_identifier,
            application_identifier,
            ownership_path,
            correlation_id,
            issued_at,
            evaluated_at,
            session_status_reference,
        })
    }

    pub fn application_identifier(&self) -> &EnglishNamespace {
        &self.application_identifier
    }

    pub fn ownership_path(&self) -> &OwnershipPath {
        &self.ownership_path
    }

    pub fn correlation_id(&self) -> &CorrelationId {
        &self.correlation_id
    }

    pub fn issued_at(&self) -> &TimeReference {
        &self.issued_at
    }

    pub fn evaluated_at(&self) -> &TimeReference {
        &self.evaluated_at
    }
}
