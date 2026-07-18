use kernel_domain::{AuthorizationDecisionReference, AuthorizationRequestRecord};

use crate::gateway_error::{GatewayError, GatewayErrorCode, GatewayResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayAuthorizationBinding {
    authorization_request_record: AuthorizationRequestRecord,
    authorization_decision_reference: AuthorizationDecisionReference,
}

impl GatewayAuthorizationBinding {
    pub fn new(
        authorization_request_record: AuthorizationRequestRecord,
        authorization_decision_reference: AuthorizationDecisionReference,
    ) -> GatewayResult<Self> {
        if authorization_request_record.request_id()
            != authorization_decision_reference.request_id()
        {
            return Err(GatewayError::new(
                GatewayErrorCode::InvalidRequest,
                "authorization decision must preserve the gateway authorization request identity",
            )?);
        }
        Ok(Self {
            authorization_request_record,
            authorization_decision_reference,
        })
    }

    pub fn require_allow(&self) -> GatewayResult<()> {
        if self.authorization_decision_reference.outcome().is_denied() {
            return Err(GatewayError::new(
                GatewayErrorCode::AuthorizationDenied,
                "gateway request requires an allowed authorization decision",
            )?);
        }
        Ok(())
    }

    pub fn authorization_request_record(&self) -> &AuthorizationRequestRecord {
        &self.authorization_request_record
    }

    pub fn authorization_decision_reference(&self) -> &AuthorizationDecisionReference {
        &self.authorization_decision_reference
    }
}
