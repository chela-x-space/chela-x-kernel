use kernel_domain::{CorrelationId, OwnershipPath};

use crate::application_error::{ApplicationError, ApplicationErrorCode, ApplicationResult};

pub fn reject_duplicates<T: PartialEq>(
    values: &[T],
    code: ApplicationErrorCode,
    detail: &'static str,
) -> ApplicationResult<()> {
    if values
        .iter()
        .enumerate()
        .any(|(index, value)| values[..index].iter().any(|prior| prior == value))
    {
        return Err(ApplicationError::new(code, detail)?);
    }
    Ok(())
}

pub fn require_exact_scope(
    expected_ownership_path: &OwnershipPath,
    actual_ownership_path: &OwnershipPath,
    code: ApplicationErrorCode,
    detail: &'static str,
) -> ApplicationResult<()> {
    if expected_ownership_path.enterprise_id() != actual_ownership_path.enterprise_id()
        || expected_ownership_path.workspace_id() != actual_ownership_path.workspace_id()
        || expected_ownership_path.project_id() != actual_ownership_path.project_id()
        || expected_ownership_path.organizational_unit_id()
            != actual_ownership_path.organizational_unit_id()
    {
        return Err(ApplicationError::new(code, detail)?);
    }
    Ok(())
}

pub fn require_correlation(
    expected: &CorrelationId,
    actual: &CorrelationId,
    code: ApplicationErrorCode,
    detail: &'static str,
) -> ApplicationResult<()> {
    if expected != actual {
        return Err(ApplicationError::new(code, detail)?);
    }
    Ok(())
}
