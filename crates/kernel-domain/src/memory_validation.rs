use crate::authorization::AuthorizationDecisionReference;
use crate::errors::{DomainError, DomainResult};
use crate::ownership::OwnershipPath;

pub(crate) fn reject_duplicates<T: PartialEq>(
    values: &[T],
    message: &'static str,
) -> DomainResult<()> {
    if values
        .iter()
        .enumerate()
        .any(|(index, value)| values[..index].iter().any(|prior| prior == value))
    {
        return Err(DomainError::InvalidMemory(message));
    }
    Ok(())
}

pub(crate) fn require_allowed(
    authorization_decision_reference: &AuthorizationDecisionReference,
) -> DomainResult<()> {
    if authorization_decision_reference.outcome().is_denied() {
        return Err(DomainError::InvalidMemory(
            "memory operation requires an allowed authorization decision",
        ));
    }
    Ok(())
}

pub(crate) fn scopes_are_compatible(
    source_ownership_path: &OwnershipPath,
    target_ownership_path: &OwnershipPath,
) -> bool {
    source_ownership_path.enterprise_id() == target_ownership_path.enterprise_id()
        && source_ownership_path
            .workspace_id()
            .is_none_or(|workspace_id| target_ownership_path.workspace_id() == Some(workspace_id))
        && source_ownership_path
            .project_id()
            .is_none_or(|project_id| target_ownership_path.project_id() == Some(project_id))
        && source_ownership_path
            .organizational_unit_id()
            .is_none_or(|unit_id| target_ownership_path.organizational_unit_id() == Some(unit_id))
}
