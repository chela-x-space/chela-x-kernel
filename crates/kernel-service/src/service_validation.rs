use kernel_application::{ApplicationRequestContext, ApplicationResponseEnvelope};

use crate::service_error::{ServiceError, ServiceErrorCode, ServiceResult};

pub fn validate_namespaced_identifier(
    value: impl Into<String>,
    code: ServiceErrorCode,
    detail: &'static str,
) -> ServiceResult<String> {
    let value = value.into();
    let trimmed = value.trim();
    if trimmed.is_empty()
        || trimmed.contains('/')
        || !trimmed.contains('.')
        || !trimmed.chars().all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '.' | '_' | '-')
        })
    {
        return Err(ServiceError::new(code, detail)?);
    }
    Ok(trimmed.to_owned())
}

pub fn validate_version_reference(
    value: impl Into<String>,
    code: ServiceErrorCode,
    detail: &'static str,
) -> ServiceResult<String> {
    let value = value.into();
    let trimmed = value.trim();
    if trimmed.is_empty()
        || trimmed.contains('/')
        || !trimmed.chars().all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '.' | '_' | '-')
        })
    {
        return Err(ServiceError::new(code, detail)?);
    }
    Ok(trimmed.to_owned())
}

pub fn validate_non_empty_text(
    value: impl Into<String>,
    code: ServiceErrorCode,
    detail: &'static str,
) -> ServiceResult<String> {
    let value = value.into();
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(ServiceError::new(code, detail)?);
    }
    Ok(trimmed.to_owned())
}

pub fn reject_duplicates<T: PartialEq>(
    values: &[T],
    code: ServiceErrorCode,
    detail: &'static str,
) -> ServiceResult<()> {
    if values
        .iter()
        .enumerate()
        .any(|(index, value)| values[..index].iter().any(|prior| prior == value))
    {
        return Err(ServiceError::new(code, detail)?);
    }
    Ok(())
}

pub fn require_application_request_context(
    expected: &ApplicationRequestContext,
    actual: &ApplicationRequestContext,
    code: ServiceErrorCode,
    detail: &'static str,
) -> ServiceResult<()> {
    if expected != actual {
        return Err(ServiceError::new(code, detail)?);
    }
    Ok(())
}

pub fn require_application_response_matches_request(
    application_request_context: &ApplicationRequestContext,
    application_response_envelope: &ApplicationResponseEnvelope,
    code: ServiceErrorCode,
    detail: &'static str,
) -> ServiceResult<()> {
    if application_response_envelope.application_request_id()
        != application_request_context.application_request_id()
        || application_response_envelope.application_identity()
            != application_request_context.application_identity()
        || application_response_envelope.correlation_id()
            != application_request_context.correlation_id()
    {
        return Err(ServiceError::new(code, detail)?);
    }
    Ok(())
}
