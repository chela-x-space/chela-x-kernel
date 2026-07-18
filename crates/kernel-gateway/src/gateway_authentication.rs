use kernel_domain::{
    AuthorizationPrincipalReference, EnglishNamespace, EventTraceReference, TimeReference,
};

use crate::gateway_error::{GatewayError, GatewayResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayAuthenticationContext {
    authenticated_principal: AuthorizationPrincipalReference,
    authentication_method_reference: EnglishNamespace,
    authentication_evidence_reference: EventTraceReference,
    authenticated_at: TimeReference,
    credential_reference: Option<EventTraceReference>,
    session_reference: Option<EventTraceReference>,
}

impl GatewayAuthenticationContext {
    pub fn new(
        authenticated_principal: AuthorizationPrincipalReference,
        authentication_method_reference: impl Into<String>,
        authentication_evidence_reference: EventTraceReference,
        authenticated_at: TimeReference,
        credential_reference: Option<EventTraceReference>,
        session_reference: Option<EventTraceReference>,
    ) -> GatewayResult<Self> {
        Ok(Self {
            authenticated_principal,
            authentication_method_reference: EnglishNamespace::new(
                "gateway_authentication_method_reference",
                authentication_method_reference,
            )
            .map_err(GatewayError::from_domain_rejection)?,
            authentication_evidence_reference,
            authenticated_at,
            credential_reference,
            session_reference,
        })
    }

    pub fn authenticated_principal(&self) -> &AuthorizationPrincipalReference {
        &self.authenticated_principal
    }

    pub fn authentication_method_reference(&self) -> &str {
        self.authentication_method_reference.as_str()
    }

    pub fn authentication_evidence_reference(&self) -> &EventTraceReference {
        &self.authentication_evidence_reference
    }

    pub fn authenticated_at(&self) -> &TimeReference {
        &self.authenticated_at
    }

    pub fn credential_reference(&self) -> Option<&EventTraceReference> {
        self.credential_reference.as_ref()
    }

    pub fn session_reference(&self) -> Option<&EventTraceReference> {
        self.session_reference.as_ref()
    }
}
