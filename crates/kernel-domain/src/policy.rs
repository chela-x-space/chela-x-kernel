use crate::errors::DomainResult;
use crate::identifier::{AuditEvidenceId, PolicyId, StableVersion};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyEffect {
    Permit,
    Deny,
    NotApplicable,
    Indeterminate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyEvaluationStep {
    TenantAndScopeBoundaryValidation,
    SecurityClassificationValidation,
    ExplicitDenyEvaluation,
    MandatoryEnterprisePolicy,
    MoreSpecificScopedPolicy,
    ApprovedExceptionOrWaiver,
    PermitEvaluation,
    DefaultOutcome,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyEvaluationOrderVersion(StableVersion);

impl PolicyEvaluationOrderVersion {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        Ok(Self(StableVersion::new(
            "policy_evaluation_order_version",
            value,
        )?))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyAuditEvidenceReference {
    audit_evidence_id: AuditEvidenceId,
    policy_id: PolicyId,
    evaluation_order_version: PolicyEvaluationOrderVersion,
}

impl PolicyAuditEvidenceReference {
    pub fn new(
        audit_evidence_id: AuditEvidenceId,
        policy_id: PolicyId,
        evaluation_order_version: PolicyEvaluationOrderVersion,
    ) -> Self {
        Self {
            audit_evidence_id,
            policy_id,
            evaluation_order_version,
        }
    }

    pub fn audit_evidence_id(&self) -> &AuditEvidenceId {
        &self.audit_evidence_id
    }

    pub fn policy_id(&self) -> &PolicyId {
        &self.policy_id
    }

    pub fn evaluation_order_version(&self) -> &PolicyEvaluationOrderVersion {
        &self.evaluation_order_version
    }
}

impl PolicyEffect {
    pub fn permits(self) -> bool {
        matches!(self, Self::Permit)
    }

    pub fn denies(self) -> bool {
        matches!(self, Self::Deny)
    }
}

impl PolicyEvaluationStep {
    pub fn ordered() -> [Self; 8] {
        [
            Self::TenantAndScopeBoundaryValidation,
            Self::SecurityClassificationValidation,
            Self::ExplicitDenyEvaluation,
            Self::MandatoryEnterprisePolicy,
            Self::MoreSpecificScopedPolicy,
            Self::ApprovedExceptionOrWaiver,
            Self::PermitEvaluation,
            Self::DefaultOutcome,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::{PolicyEffect, PolicyEvaluationOrderVersion, PolicyEvaluationStep};
    use crate::errors::DomainError;

    #[test]
    fn policy_effect_distinguishes_permit_and_deny_ces_b0_028_7() {
        assert!(PolicyEffect::Permit.permits());
        assert!(PolicyEffect::Deny.denies());
    }

    #[test]
    fn policy_evaluation_order_is_total_and_stable_ces_b0_028_9() {
        let steps = PolicyEvaluationStep::ordered();
        assert_eq!(steps.len(), 8);
        assert_eq!(
            steps[0],
            PolicyEvaluationStep::TenantAndScopeBoundaryValidation
        );
        assert_eq!(steps[7], PolicyEvaluationStep::DefaultOutcome);
    }

    #[test]
    fn policy_evaluation_order_version_rejects_empty_value_ces_b0_028_9() {
        let error = PolicyEvaluationOrderVersion::new("")
            .expect_err("policy evaluation version must be explicit");
        assert_eq!(
            error,
            DomainError::EmptyValue {
                field: "policy_evaluation_order_version",
            }
        );
    }
}
