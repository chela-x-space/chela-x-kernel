use kernel_domain::{
    AuthorizationDecisionReference, EnglishNamespace, EventTraceReference, PolicyId,
};

use crate::gateway::GatewayAuditReference;
use crate::gateway_error::{GatewayError, GatewayResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GatewayProtocol {
    Http,
    WebSocket,
    Ipc,
    Internal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayRateGovernanceReference {
    policy_id: PolicyId,
    authorization_decision_reference: Option<AuthorizationDecisionReference>,
    rate_classification: EnglishNamespace,
    window_reference: EventTraceReference,
    gateway_audit_reference: GatewayAuditReference,
}

impl GatewayRateGovernanceReference {
    pub fn new(
        policy_id: PolicyId,
        authorization_decision_reference: Option<AuthorizationDecisionReference>,
        rate_classification: impl Into<String>,
        window_reference: EventTraceReference,
        gateway_audit_reference: GatewayAuditReference,
    ) -> GatewayResult<Self> {
        Ok(Self {
            policy_id,
            authorization_decision_reference,
            rate_classification: EnglishNamespace::new(
                "gateway_rate_classification",
                rate_classification,
            )
            .map_err(GatewayError::from_domain_rejection)?,
            window_reference,
            gateway_audit_reference,
        })
    }

    pub fn policy_id(&self) -> &PolicyId {
        &self.policy_id
    }

    pub fn authorization_decision_reference(&self) -> Option<&AuthorizationDecisionReference> {
        self.authorization_decision_reference.as_ref()
    }

    pub fn rate_classification(&self) -> &str {
        self.rate_classification.as_str()
    }

    pub fn window_reference(&self) -> &EventTraceReference {
        &self.window_reference
    }

    pub fn gateway_audit_reference(&self) -> &GatewayAuditReference {
        &self.gateway_audit_reference
    }
}
