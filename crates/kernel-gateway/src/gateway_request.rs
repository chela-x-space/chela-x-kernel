use kernel_domain::{CorrelationId, OwnershipPath, TimeReference};

use crate::gateway::GatewayAuditReference;
use crate::gateway_authentication::GatewayAuthenticationContext;
use crate::gateway_authorization::GatewayAuthorizationBinding;
use crate::gateway_command::GatewayCommandRequest;
use crate::gateway_contract::{GatewayApiVersion, GatewayOperationReference};
use crate::gateway_error::{GatewayError, GatewayErrorCode, GatewayResult};
use crate::gateway_protocol::GatewayRateGovernanceReference;
use crate::gateway_query::GatewayQueryRequest;
use crate::gateway_validation::{
    require_exact_scope, require_matching_principal, require_operation_resource,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayRequestContext {
    gateway_api_version: GatewayApiVersion,
    gateway_operation_reference: GatewayOperationReference,
    gateway_authentication_context: GatewayAuthenticationContext,
    gateway_authorization_binding: GatewayAuthorizationBinding,
    ownership_path: OwnershipPath,
    correlation_id: CorrelationId,
    requested_at: TimeReference,
    gateway_rate_governance_reference: Option<GatewayRateGovernanceReference>,
    gateway_audit_reference: GatewayAuditReference,
}

impl GatewayRequestContext {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        gateway_api_version: GatewayApiVersion,
        gateway_operation_reference: GatewayOperationReference,
        gateway_authentication_context: GatewayAuthenticationContext,
        gateway_authorization_binding: GatewayAuthorizationBinding,
        ownership_path: OwnershipPath,
        correlation_id: CorrelationId,
        requested_at: TimeReference,
        gateway_rate_governance_reference: Option<GatewayRateGovernanceReference>,
        gateway_audit_reference: GatewayAuditReference,
    ) -> GatewayResult<Self> {
        gateway_authorization_binding.require_allow()?;
        require_matching_principal(
            &gateway_authentication_context,
            &gateway_authorization_binding,
        )?;
        require_exact_scope(
            &ownership_path,
            gateway_authorization_binding
                .authorization_request_record()
                .target()
                .scope()
                .ownership_path(),
            "gateway ownership path must match the authorization request scope",
        )?;
        require_operation_resource(
            &gateway_operation_reference,
            gateway_authorization_binding
                .authorization_request_record()
                .target()
                .resource_type()
                .as_str(),
        )?;
        if gateway_audit_reference.correlation_id() != Some(&correlation_id) {
            return Err(GatewayError::new(
                GatewayErrorCode::InvalidRequest,
                "gateway audit reference correlation must match the gateway request correlation reference",
            )?);
        }
        if let Some(gateway_rate_governance_reference) = &gateway_rate_governance_reference {
            if gateway_rate_governance_reference
                .gateway_audit_reference()
                .correlation_id()
                != Some(&correlation_id)
            {
                return Err(GatewayError::new(
                    GatewayErrorCode::RateGovernanceRejected,
                    "gateway rate-governance audit correlation must match the gateway request correlation reference",
                )?);
            }
        }
        Ok(Self {
            gateway_api_version,
            gateway_operation_reference,
            gateway_authentication_context,
            gateway_authorization_binding,
            ownership_path,
            correlation_id,
            requested_at,
            gateway_rate_governance_reference,
            gateway_audit_reference,
        })
    }

    pub fn gateway_api_version(&self) -> &GatewayApiVersion {
        &self.gateway_api_version
    }
    pub fn gateway_operation_reference(&self) -> &GatewayOperationReference {
        &self.gateway_operation_reference
    }
    pub fn gateway_authentication_context(&self) -> &GatewayAuthenticationContext {
        &self.gateway_authentication_context
    }
    pub fn gateway_authorization_binding(&self) -> &GatewayAuthorizationBinding {
        &self.gateway_authorization_binding
    }
    pub fn authorization_request_record(&self) -> &kernel_domain::AuthorizationRequestRecord {
        self.gateway_authorization_binding
            .authorization_request_record()
    }
    pub fn authorization_decision_reference(
        &self,
    ) -> &kernel_domain::AuthorizationDecisionReference {
        self.gateway_authorization_binding
            .authorization_decision_reference()
    }
    pub fn ownership_path(&self) -> &OwnershipPath {
        &self.ownership_path
    }
    pub fn correlation_id(&self) -> &CorrelationId {
        &self.correlation_id
    }
    pub fn requested_at(&self) -> &TimeReference {
        &self.requested_at
    }
    pub fn gateway_rate_governance_reference(&self) -> Option<&GatewayRateGovernanceReference> {
        self.gateway_rate_governance_reference.as_ref()
    }
    pub fn gateway_audit_reference(&self) -> &GatewayAuditReference {
        &self.gateway_audit_reference
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GatewayRequestEnvelope {
    Command {
        gateway_request_context: Box<GatewayRequestContext>,
        gateway_command_request: Box<GatewayCommandRequest>,
    },
    Query {
        gateway_request_context: Box<GatewayRequestContext>,
        gateway_query_request: Box<GatewayQueryRequest>,
    },
}

impl GatewayRequestEnvelope {
    pub fn command(
        gateway_request_context: GatewayRequestContext,
        gateway_command_request: GatewayCommandRequest,
    ) -> GatewayResult<Self> {
        gateway_command_request.validate_against_context(&gateway_request_context)?;
        Ok(Self::Command {
            gateway_request_context: Box::new(gateway_request_context),
            gateway_command_request: Box::new(gateway_command_request),
        })
    }

    pub fn query(
        gateway_request_context: GatewayRequestContext,
        gateway_query_request: GatewayQueryRequest,
    ) -> GatewayResult<Self> {
        gateway_query_request.validate_against_context(&gateway_request_context)?;
        Ok(Self::Query {
            gateway_request_context: Box::new(gateway_request_context),
            gateway_query_request: Box::new(gateway_query_request),
        })
    }

    pub fn gateway_request_context(&self) -> &GatewayRequestContext {
        match self {
            Self::Command {
                gateway_request_context,
                ..
            }
            | Self::Query {
                gateway_request_context,
                ..
            } => gateway_request_context,
        }
    }
}
