use crate::errors::{DomainError, DomainResult};
use crate::identifier::{
    AuditEvidenceId, AuthorizationDecisionId, DecisionId, DelegationId, PolicyId, StableVersion,
    WorkflowId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WorkflowRetryLimit(u16);

impl WorkflowRetryLimit {
    pub fn new(value: u16) -> DomainResult<Self> {
        if value == 0 {
            return Err(DomainError::InvalidWorkflowReference(
                "workflow retry limit must be greater than zero",
            ));
        }
        Ok(Self(value))
    }

    pub fn value(self) -> u16 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowRetryPolicyReference {
    definition_version: StableVersion,
    retry_limit: WorkflowRetryLimit,
}

impl WorkflowRetryPolicyReference {
    pub fn new(definition_version: StableVersion, retry_limit: WorkflowRetryLimit) -> Self {
        Self {
            definition_version,
            retry_limit,
        }
    }

    pub fn definition_version(&self) -> &StableVersion {
        &self.definition_version
    }

    pub fn retry_limit(&self) -> WorkflowRetryLimit {
        self.retry_limit
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowRecoveryReference {
    corrective_path: crate::identifier::NonEmptyText,
    requires_revalidation: bool,
}

impl WorkflowRecoveryReference {
    pub fn new(
        corrective_path: impl Into<String>,
        requires_revalidation: bool,
    ) -> DomainResult<Self> {
        Ok(Self {
            corrective_path: crate::identifier::NonEmptyText::new(
                "workflow_corrective_path",
                corrective_path,
            )?,
            requires_revalidation,
        })
    }

    pub fn corrective_path(&self) -> &str {
        self.corrective_path.as_str()
    }

    pub fn requires_revalidation(&self) -> bool {
        self.requires_revalidation
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowAuditEvidenceReference {
    audit_evidence_id: AuditEvidenceId,
    workflow_id: WorkflowId,
    definition_version: StableVersion,
    consumed_policy_ids: Vec<PolicyId>,
    consumed_authorization_decision_ids: Vec<AuthorizationDecisionId>,
    consumed_delegation_ids: Vec<DelegationId>,
    consumed_decision_ids: Vec<DecisionId>,
}

impl WorkflowAuditEvidenceReference {
    pub fn new(
        audit_evidence_id: AuditEvidenceId,
        workflow_id: WorkflowId,
        definition_version: StableVersion,
        consumed_policy_ids: Vec<PolicyId>,
        consumed_authorization_decision_ids: Vec<AuthorizationDecisionId>,
        consumed_delegation_ids: Vec<DelegationId>,
        consumed_decision_ids: Vec<DecisionId>,
    ) -> DomainResult<Self> {
        if consumed_policy_ids.is_empty()
            || consumed_authorization_decision_ids.is_empty()
            || consumed_delegation_ids.is_empty()
            || consumed_decision_ids.is_empty()
        {
            return Err(DomainError::InvalidWorkflowReference(
                "workflow audit evidence must preserve consumed upstream references",
            ));
        }
        Ok(Self {
            audit_evidence_id,
            workflow_id,
            definition_version,
            consumed_policy_ids,
            consumed_authorization_decision_ids,
            consumed_delegation_ids,
            consumed_decision_ids,
        })
    }

    pub fn audit_evidence_id(&self) -> &AuditEvidenceId {
        &self.audit_evidence_id
    }
}

#[cfg(test)]
mod tests {
    use super::{WorkflowRecoveryReference, WorkflowRetryLimit};

    #[test]
    fn workflow_retry_limit_rejects_zero_ces_b0_030_14() {
        let error = WorkflowRetryLimit::new(0).expect_err("workflow retry limit must be bounded");
        assert!(error
            .to_string()
            .contains("workflow retry limit must be greater than zero"));
    }

    #[test]
    fn workflow_recovery_requires_path_reference_ces_b0_030_14() {
        let error =
            WorkflowRecoveryReference::new("", true).expect_err("recovery path must be explicit");
        assert!(error.to_string().contains("workflow_corrective_path"));
    }
}
