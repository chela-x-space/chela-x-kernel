use kernel_service::{ServiceRequestContext, ServiceResponseEnvelope};

use crate::adapter_error::{AdapterError, AdapterErrorCode, AdapterResult};

pub fn validate_namespaced_identifier(
    value: impl Into<String>,
    code: AdapterErrorCode,
    detail: &'static str,
) -> AdapterResult<String> {
    let value = value.into();
    let trimmed = value.trim();
    if trimmed.is_empty()
        || trimmed.contains('/')
        || !trimmed.contains('.')
        || !trimmed.chars().all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '.' | '_' | '-')
        })
    {
        return Err(AdapterError::new(code, detail)?);
    }
    Ok(trimmed.to_owned())
}

pub fn validate_version_reference(
    value: impl Into<String>,
    code: AdapterErrorCode,
    detail: &'static str,
) -> AdapterResult<String> {
    let value = value.into();
    let trimmed = value.trim();
    if trimmed.is_empty()
        || trimmed.contains('/')
        || !trimmed.chars().all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '.' | '_' | '-')
        })
    {
        return Err(AdapterError::new(code, detail)?);
    }
    Ok(trimmed.to_owned())
}

pub fn validate_non_empty_text(
    value: impl Into<String>,
    code: AdapterErrorCode,
    detail: &'static str,
) -> AdapterResult<String> {
    let value = value.into();
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(AdapterError::new(code, detail)?);
    }
    Ok(trimmed.to_owned())
}

pub fn reject_duplicates<T: PartialEq>(
    values: &[T],
    code: AdapterErrorCode,
    detail: &'static str,
) -> AdapterResult<()> {
    if values
        .iter()
        .enumerate()
        .any(|(index, value)| values[..index].iter().any(|prior| prior == value))
    {
        return Err(AdapterError::new(code, detail)?);
    }
    Ok(())
}

pub fn require_service_request_context(
    expected: &ServiceRequestContext,
    actual: &ServiceRequestContext,
    code: AdapterErrorCode,
    detail: &'static str,
) -> AdapterResult<()> {
    if expected != actual {
        return Err(AdapterError::new(code, detail)?);
    }
    Ok(())
}

pub fn require_correlation_continuity(
    service_request_context: &ServiceRequestContext,
    service_response_envelope: &ServiceResponseEnvelope,
    code: AdapterErrorCode,
    detail: &'static str,
) -> AdapterResult<()> {
    if service_request_context
        .application_request_context()
        .correlation_id()
        != service_response_envelope
            .application_response_envelope()
            .correlation_id()
    {
        return Err(AdapterError::new(code, detail)?);
    }
    Ok(())
}
