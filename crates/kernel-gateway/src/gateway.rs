use kernel_domain::{AuditEvidenceId, CorrelationId, EventTraceReference, TimeReference};

use crate::gateway_contract::{GatewayApiVersion, GatewayOperationReference};
use crate::gateway_error::{GatewayError, GatewayErrorCode, GatewayResult};
use crate::gateway_protocol::GatewayProtocol;
use crate::gateway_validation::reject_duplicates;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayAuditReference {
    gateway_trace_reference: EventTraceReference,
    correlation_id: Option<CorrelationId>,
    audit_evidence_ids: Vec<AuditEvidenceId>,
}

impl GatewayAuditReference {
    pub fn new(
        gateway_trace_reference: EventTraceReference,
        correlation_id: Option<CorrelationId>,
        audit_evidence_ids: Vec<AuditEvidenceId>,
    ) -> GatewayResult<Self> {
        if audit_evidence_ids.is_empty() {
            return Err(GatewayError::new(
                GatewayErrorCode::InternalContractViolation,
                "gateway audit reference requires audit evidence identifiers",
            )?);
        }
        reject_duplicates(
            &audit_evidence_ids,
            GatewayErrorCode::InternalContractViolation,
            "duplicate gateway audit evidence identifier",
        )?;
        Ok(Self {
            gateway_trace_reference,
            correlation_id,
            audit_evidence_ids,
        })
    }

    pub fn gateway_trace_reference(&self) -> &EventTraceReference {
        &self.gateway_trace_reference
    }

    pub fn correlation_id(&self) -> Option<&CorrelationId> {
        self.correlation_id.as_ref()
    }

    pub fn audit_evidence_ids(&self) -> &[AuditEvidenceId] {
        &self.audit_evidence_ids
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayStatusSnapshot {
    gateway_api_version: GatewayApiVersion,
    supported_protocols: Vec<GatewayProtocol>,
    supported_operations: Vec<GatewayOperationReference>,
    generated_at: TimeReference,
    gateway_audit_reference: GatewayAuditReference,
}

impl GatewayStatusSnapshot {
    pub fn new(
        gateway_api_version: GatewayApiVersion,
        supported_protocols: Vec<GatewayProtocol>,
        supported_operations: Vec<GatewayOperationReference>,
        generated_at: TimeReference,
        gateway_audit_reference: GatewayAuditReference,
    ) -> GatewayResult<Self> {
        if supported_protocols.is_empty() {
            return Err(GatewayError::new(
                GatewayErrorCode::InvalidQuery,
                "gateway status snapshot requires at least one supported protocol",
            )?);
        }
        if supported_operations.is_empty() {
            return Err(GatewayError::new(
                GatewayErrorCode::InvalidQuery,
                "gateway status snapshot requires at least one supported operation",
            )?);
        }
        reject_duplicates(
            &supported_protocols,
            GatewayErrorCode::InvalidQuery,
            "duplicate gateway protocol in status snapshot",
        )?;
        reject_duplicates(
            &supported_operations,
            GatewayErrorCode::InvalidQuery,
            "duplicate gateway operation in status snapshot",
        )?;
        Ok(Self {
            gateway_api_version,
            supported_protocols,
            supported_operations,
            generated_at,
            gateway_audit_reference,
        })
    }

    pub fn gateway_api_version(&self) -> &GatewayApiVersion {
        &self.gateway_api_version
    }

    pub fn supported_protocols(&self) -> &[GatewayProtocol] {
        &self.supported_protocols
    }

    pub fn supported_operations(&self) -> &[GatewayOperationReference] {
        &self.supported_operations
    }

    pub fn generated_at(&self) -> &TimeReference {
        &self.generated_at
    }

    pub fn gateway_audit_reference(&self) -> &GatewayAuditReference {
        &self.gateway_audit_reference
    }
}
