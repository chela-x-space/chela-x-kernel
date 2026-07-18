use kernel_domain::OwnershipPath;

use crate::gateway_authentication::GatewayAuthenticationContext;
use crate::gateway_authorization::GatewayAuthorizationBinding;
use crate::gateway_contract::GatewayOperationReference;
use crate::gateway_error::{GatewayError, GatewayErrorCode, GatewayResult};

pub fn reject_duplicates<T: PartialEq>(
    values: &[T],
    code: GatewayErrorCode,
    detail: &'static str,
) -> GatewayResult<()> {
    if values
        .iter()
        .enumerate()
        .any(|(index, value)| values[..index].iter().any(|prior| prior == value))
    {
        return Err(GatewayError::new(code, detail)?);
    }
    Ok(())
}

pub fn require_matching_principal(
    gateway_authentication_context: &GatewayAuthenticationContext,
    gateway_authorization_binding: &GatewayAuthorizationBinding,
) -> GatewayResult<()> {
    if gateway_authentication_context.authenticated_principal()
        != gateway_authorization_binding
            .authorization_request_record()
            .requester()
            .principal()
    {
        return Err(GatewayError::new(
            GatewayErrorCode::AuthorizationDenied,
            "authenticated gateway principal must match the authorization requester principal",
        )?);
    }
    Ok(())
}

pub fn require_exact_scope(
    expected_ownership_path: &OwnershipPath,
    actual_ownership_path: &OwnershipPath,
    detail: &'static str,
) -> GatewayResult<()> {
    if expected_ownership_path.enterprise_id() != actual_ownership_path.enterprise_id()
        || expected_ownership_path.workspace_id() != actual_ownership_path.workspace_id()
        || expected_ownership_path.project_id() != actual_ownership_path.project_id()
        || expected_ownership_path.organizational_unit_id()
            != actual_ownership_path.organizational_unit_id()
    {
        return Err(GatewayError::new(GatewayErrorCode::ScopeMismatch, detail)?);
    }
    Ok(())
}

pub fn require_operation_resource(
    gateway_operation_reference: &GatewayOperationReference,
    resource_type: &str,
) -> GatewayResult<()> {
    if gateway_operation_reference.resource_segment() != resource_type {
        return Err(GatewayError::new(
            GatewayErrorCode::UnsupportedOperation,
            "gateway operation resource must match the requested resource type",
        )?);
    }
    Ok(())
}
