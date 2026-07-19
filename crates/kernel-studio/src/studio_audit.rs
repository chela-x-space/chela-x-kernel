use kernel_domain::{
    AuditEvidenceId, AuthorizationDecisionReference, CorrelationId, EventTraceReference,
    MemoryAuditReference, NonEmptyText, TimeReference,
};
use kernel_gateway::{GatewayAuditReference, GatewayOperationReference};

use crate::studio::{StudioError, StudioErrorCode, StudioResult};
use crate::studio_validation::reject_duplicates;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudioAuditProjection {
    subject_reference: NonEmptyText,
    operation_reference: Option<GatewayOperationReference>,
    decision_reference: Option<AuthorizationDecisionReference>,
    correlation_id: CorrelationId,
    causation_reference: Option<EventTraceReference>,
    occurred_at: TimeReference,
    audit_evidence_ids: Vec<AuditEvidenceId>,
    outcome_reference: NonEmptyText,
    gateway_audit_reference: Option<GatewayAuditReference>,
    memory_audit_reference: Option<MemoryAuditReference>,
}

impl StudioAuditProjection {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        subject_reference: impl Into<String>,
        operation_reference: Option<GatewayOperationReference>,
        decision_reference: Option<AuthorizationDecisionReference>,
        correlation_id: CorrelationId,
        causation_reference: Option<EventTraceReference>,
        occurred_at: TimeReference,
        audit_evidence_ids: Vec<AuditEvidenceId>,
        outcome_reference: impl Into<String>,
        gateway_audit_reference: Option<GatewayAuditReference>,
        memory_audit_reference: Option<MemoryAuditReference>,
    ) -> StudioResult<Self> {
        if audit_evidence_ids.is_empty() {
            return Err(StudioError::new(
                StudioErrorCode::AuditReferenceMismatch,
                "studio audit projection requires evidence identifiers",
            )?);
        }
        reject_duplicates(
            &audit_evidence_ids,
            StudioErrorCode::AuditReferenceMismatch,
            "duplicate audit evidence identifier in studio audit projection",
        )?;
        if let Some(gateway_audit_reference) = &gateway_audit_reference {
            if gateway_audit_reference.correlation_id() != Some(&correlation_id) {
                return Err(StudioError::new(
                    StudioErrorCode::AuditReferenceMismatch,
                    "studio audit projection gateway correlation must match the projected correlation reference",
                )?);
            }
        }
        Ok(Self {
            subject_reference: NonEmptyText::new(
                "studio_audit_subject_reference",
                subject_reference,
            )
            .map_err(StudioError::from_domain_rejection)?,
            operation_reference,
            decision_reference,
            correlation_id,
            causation_reference,
            occurred_at,
            audit_evidence_ids,
            outcome_reference: NonEmptyText::new(
                "studio_audit_outcome_reference",
                outcome_reference,
            )
            .map_err(StudioError::from_domain_rejection)?,
            gateway_audit_reference,
            memory_audit_reference,
        })
    }

    pub fn correlation_id(&self) -> &CorrelationId {
        &self.correlation_id
    }
    pub fn audit_evidence_ids(&self) -> &[AuditEvidenceId] {
        &self.audit_evidence_ids
    }
}
