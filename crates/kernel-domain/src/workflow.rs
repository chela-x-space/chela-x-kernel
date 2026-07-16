use crate::errors::{DomainError, DomainResult};
use crate::identifier::{
    AuditEvidenceId, AuthorizationDecisionId, DecisionId, DelegationId, PolicyId, StableVersion,
    WorkflowId,
};
use crate::ownership::OwnershipPath;

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowEngineFoundation {
    workflow_id: WorkflowId,
    ownership: OwnershipPath,
    definition_version: StableVersion,
    retry_policy: Option<WorkflowRetryPolicyReference>,
    retry_limit: Option<WorkflowRetryLimit>,
    recovery_reference: Option<WorkflowRecoveryReference>,
    audit_evidence: Vec<WorkflowAuditEvidenceReference>,
}

impl WorkflowEngineFoundation {
    pub fn new(
        workflow_id: WorkflowId,
        ownership: OwnershipPath,
        definition_version: StableVersion,
        retry_policy: Option<WorkflowRetryPolicyReference>,
        retry_limit: Option<WorkflowRetryLimit>,
        recovery_reference: Option<WorkflowRecoveryReference>,
        audit_evidence: Vec<WorkflowAuditEvidenceReference>,
    ) -> DomainResult<Self> {
        if retry_limit.is_some() && retry_policy.is_none() {
            return Err(DomainError::InvalidWorkflowReference(
                "retry limit requires retry policy",
            ));
        }

        for (index, evidence) in audit_evidence.iter().enumerate() {
            if audit_evidence[..index]
                .iter()
                .any(|prior| prior.audit_evidence_id() == evidence.audit_evidence_id())
            {
                return Err(DomainError::InvalidWorkflowReference(
                    "duplicate workflow audit evidence reference",
                ));
            }
        }

        Ok(Self {
            workflow_id,
            ownership,
            definition_version,
            retry_policy,
            retry_limit,
            recovery_reference,
            audit_evidence,
        })
    }

    pub fn workflow_id(&self) -> &WorkflowId {
        &self.workflow_id
    }

    pub fn ownership(&self) -> &OwnershipPath {
        &self.ownership
    }

    pub fn definition_version(&self) -> &StableVersion {
        &self.definition_version
    }

    pub fn retry_policy(&self) -> Option<&WorkflowRetryPolicyReference> {
        self.retry_policy.as_ref()
    }

    pub fn retry_limit(&self) -> Option<WorkflowRetryLimit> {
        self.retry_limit
    }

    pub fn recovery_reference(&self) -> Option<&WorkflowRecoveryReference> {
        self.recovery_reference.as_ref()
    }

    pub fn audit_evidence(&self) -> &[WorkflowAuditEvidenceReference] {
        &self.audit_evidence
    }
}

#[cfg(test)]
mod tests {
    use super::{
        WorkflowAuditEvidenceReference, WorkflowEngineFoundation, WorkflowRecoveryReference,
        WorkflowRetryLimit, WorkflowRetryPolicyReference,
    };
    use crate::identifier::{
        AuditEvidenceId, AuthorizationDecisionId, DecisionId, DelegationId, EnterpriseId, HumanId,
        OrganizationUnitId, PolicyId, ProjectId, StableVersion, WorkflowId, WorkspaceId,
    };
    use crate::ownership::{OwnerReference, OwnershipPath};

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

    fn workflow_id() -> WorkflowId {
        WorkflowId::new("CX-WF-000001").expect("workflow")
    }

    fn ownership() -> OwnershipPath {
        OwnershipPath::new(
            EnterpriseId::new("CX-ENT-000001").expect("enterprise"),
            Some(WorkspaceId::new("CX-WS-000001").expect("workspace")),
            Some(ProjectId::new("CX-PROJ-000001").expect("project")),
            Some(OrganizationUnitId::new("CX-OU-000001").expect("unit")),
        )
        .expect("ownership")
    }

    fn definition_version() -> StableVersion {
        StableVersion::new("workflow_definition_version", "1.0.0").expect("version")
    }

    fn retry_limit() -> WorkflowRetryLimit {
        WorkflowRetryLimit::new(3).expect("retry limit")
    }

    fn retry_policy() -> WorkflowRetryPolicyReference {
        WorkflowRetryPolicyReference::new(definition_version(), retry_limit())
    }

    fn recovery_reference() -> WorkflowRecoveryReference {
        WorkflowRecoveryReference::new("retry/manual-review", true).expect("recovery")
    }

    fn audit_evidence(id: &str) -> WorkflowAuditEvidenceReference {
        WorkflowAuditEvidenceReference::new(
            AuditEvidenceId::new(id).expect("audit evidence id"),
            workflow_id(),
            definition_version(),
            vec![PolicyId::new("CX-POL-000001").expect("policy")],
            vec![
                AuthorizationDecisionId::new("CX-AUTHDEC-000001").expect("authorization decision"),
            ],
            vec![DelegationId::new("CX-DEL-000001").expect("delegation")],
            vec![DecisionId::new("CX-DEC-000001").expect("decision")],
        )
        .expect("audit evidence")
    }

    fn second_audit_evidence() -> WorkflowAuditEvidenceReference {
        WorkflowAuditEvidenceReference::new(
            AuditEvidenceId::new("CX-AUD-000002").expect("audit evidence id"),
            workflow_id(),
            StableVersion::new("workflow_definition_version", "1.0.1").expect("version"),
            vec![PolicyId::new("CX-POL-000002").expect("policy")],
            vec![
                AuthorizationDecisionId::new("CX-AUTHDEC-000002").expect("authorization decision"),
            ],
            vec![DelegationId::new("CX-DEL-000002").expect("delegation")],
            vec![DecisionId::new("CX-DEC-000002").expect("decision")],
        )
        .expect("audit evidence")
    }

    #[test]
    fn workflow_engine_foundation_complete_foundation_construction_passes() {
        let foundation = WorkflowEngineFoundation::new(
            workflow_id(),
            ownership(),
            definition_version(),
            Some(retry_policy()),
            Some(retry_limit()),
            Some(recovery_reference()),
            vec![audit_evidence("CX-AUD-000001"), second_audit_evidence()],
        )
        .expect("foundation");

        assert_eq!(foundation.workflow_id().as_str(), "CX-WF-000001");
        assert_eq!(foundation.audit_evidence().len(), 2);
    }

    #[test]
    fn workflow_engine_foundation_mandatory_workflow_identity_is_preserved() {
        let workflow_id = workflow_id();
        let foundation = WorkflowEngineFoundation::new(
            workflow_id.clone(),
            ownership(),
            definition_version(),
            None,
            None,
            None,
            vec![],
        )
        .expect("foundation");

        assert_eq!(foundation.workflow_id(), &workflow_id);
    }

    #[test]
    fn workflow_engine_foundation_ownership_reference_is_preserved() {
        let ownership = ownership();
        let foundation = WorkflowEngineFoundation::new(
            workflow_id(),
            ownership.clone(),
            definition_version(),
            None,
            None,
            None,
            vec![],
        )
        .expect("foundation");

        assert_eq!(foundation.ownership(), &ownership);
    }

    #[test]
    fn workflow_engine_foundation_definition_version_is_preserved() {
        let definition_version = definition_version();
        let foundation = WorkflowEngineFoundation::new(
            workflow_id(),
            ownership(),
            definition_version.clone(),
            None,
            None,
            None,
            vec![],
        )
        .expect("foundation");

        assert_eq!(foundation.definition_version(), &definition_version);
    }

    #[test]
    fn workflow_engine_foundation_retry_policy_and_retry_limit_are_preserved() {
        let retry_policy = retry_policy();
        let retry_limit = retry_limit();
        let foundation = WorkflowEngineFoundation::new(
            workflow_id(),
            ownership(),
            definition_version(),
            Some(retry_policy.clone()),
            Some(retry_limit),
            None,
            vec![],
        )
        .expect("foundation");

        assert_eq!(foundation.retry_policy(), Some(&retry_policy));
        assert_eq!(foundation.retry_limit(), Some(retry_limit));
    }

    #[test]
    fn workflow_engine_foundation_retry_limit_without_policy_is_rejected() {
        let error = WorkflowEngineFoundation::new(
            workflow_id(),
            ownership(),
            definition_version(),
            None,
            Some(retry_limit()),
            None,
            vec![],
        )
        .expect_err("retry limit without policy must fail");

        assert_eq!(
            error,
            crate::errors::DomainError::InvalidWorkflowReference(
                "retry limit requires retry policy",
            )
        );
    }

    #[test]
    fn workflow_engine_foundation_retry_policy_without_limit_is_allowed() {
        let retry_policy = retry_policy();
        let foundation = WorkflowEngineFoundation::new(
            workflow_id(),
            ownership(),
            definition_version(),
            Some(retry_policy.clone()),
            None,
            None,
            vec![],
        )
        .expect("foundation");

        assert_eq!(foundation.retry_policy(), Some(&retry_policy));
        assert_eq!(foundation.retry_limit(), None);
    }

    #[test]
    fn workflow_engine_foundation_recovery_reference_is_optional() {
        let foundation = WorkflowEngineFoundation::new(
            workflow_id(),
            ownership(),
            definition_version(),
            None,
            None,
            None,
            vec![],
        )
        .expect("foundation");

        assert_eq!(foundation.recovery_reference(), None);
    }

    #[test]
    fn workflow_engine_foundation_empty_audit_evidence_is_allowed() {
        let foundation = WorkflowEngineFoundation::new(
            workflow_id(),
            ownership(),
            definition_version(),
            None,
            None,
            Some(recovery_reference()),
            vec![],
        )
        .expect("foundation");

        assert!(foundation.audit_evidence().is_empty());
    }

    #[test]
    fn workflow_engine_foundation_audit_evidence_order_is_preserved() {
        let first = audit_evidence("CX-AUD-000001");
        let second = second_audit_evidence();
        let foundation = WorkflowEngineFoundation::new(
            workflow_id(),
            ownership(),
            definition_version(),
            None,
            None,
            None,
            vec![first.clone(), second.clone()],
        )
        .expect("foundation");

        assert_eq!(foundation.audit_evidence(), &[first, second]);
    }

    #[test]
    fn workflow_engine_foundation_duplicate_audit_evidence_is_rejected() {
        let error = WorkflowEngineFoundation::new(
            workflow_id(),
            ownership(),
            definition_version(),
            None,
            None,
            None,
            vec![
                audit_evidence("CX-AUD-000001"),
                audit_evidence("CX-AUD-000001"),
            ],
        )
        .expect_err("duplicate audit evidence must fail");

        assert_eq!(
            error,
            crate::errors::DomainError::InvalidWorkflowReference(
                "duplicate workflow audit evidence reference",
            )
        );
    }

    #[test]
    fn workflow_engine_foundation_equivalent_invalid_inputs_produce_equivalent_errors() {
        let first = WorkflowEngineFoundation::new(
            workflow_id(),
            ownership(),
            definition_version(),
            None,
            Some(retry_limit()),
            None,
            vec![],
        )
        .expect_err("invalid");
        let second = WorkflowEngineFoundation::new(
            workflow_id(),
            ownership(),
            definition_version(),
            None,
            Some(retry_limit()),
            None,
            vec![],
        )
        .expect_err("invalid");

        assert_eq!(first, second);
    }

    #[test]
    fn workflow_engine_foundation_construction_is_deterministic() {
        let first = WorkflowEngineFoundation::new(
            workflow_id(),
            ownership(),
            definition_version(),
            Some(retry_policy()),
            Some(retry_limit()),
            Some(recovery_reference()),
            vec![audit_evidence("CX-AUD-000001"), second_audit_evidence()],
        );
        let second = WorkflowEngineFoundation::new(
            workflow_id(),
            ownership(),
            definition_version(),
            Some(retry_policy()),
            Some(retry_limit()),
            Some(recovery_reference()),
            vec![audit_evidence("CX-AUD-000001"), second_audit_evidence()],
        );

        assert_eq!(first, second);
    }

    #[test]
    fn workflow_engine_foundation_value_semantics_are_preserved() {
        let foundation = WorkflowEngineFoundation::new(
            workflow_id(),
            ownership(),
            definition_version(),
            Some(retry_policy()),
            Some(retry_limit()),
            Some(recovery_reference()),
            vec![audit_evidence("CX-AUD-000001"), second_audit_evidence()],
        )
        .expect("foundation");

        assert_eq!(foundation.clone(), foundation);
    }

    #[test]
    fn workflow_engine_foundation_does_not_mutate_supplied_values() {
        let workflow_id = workflow_id();
        let ownership = ownership();
        let definition_version = definition_version();
        let retry_policy = retry_policy();
        let retry_limit = retry_limit();
        let recovery_reference = recovery_reference();
        let audit_evidence = vec![audit_evidence("CX-AUD-000001"), second_audit_evidence()];

        let workflow_id_before = workflow_id.clone();
        let ownership_before = ownership.clone();
        let definition_version_before = definition_version.clone();
        let retry_policy_before = retry_policy.clone();
        let recovery_reference_before = recovery_reference.clone();
        let audit_evidence_before = audit_evidence.clone();

        let foundation = WorkflowEngineFoundation::new(
            workflow_id.clone(),
            ownership.clone(),
            definition_version.clone(),
            Some(retry_policy.clone()),
            Some(retry_limit),
            Some(recovery_reference.clone()),
            audit_evidence.clone(),
        )
        .expect("foundation");

        assert_eq!(workflow_id, workflow_id_before);
        assert_eq!(ownership, ownership_before);
        assert_eq!(definition_version, definition_version_before);
        assert_eq!(retry_policy, retry_policy_before);
        assert_eq!(recovery_reference, recovery_reference_before);
        assert_eq!(audit_evidence, audit_evidence_before);
        assert_eq!(foundation.workflow_id(), &workflow_id_before);
    }

    #[test]
    fn workflow_engine_foundation_complete_foundation_owner_reference_remains_external() {
        let owner = OwnerReference::new(HumanId::new("CX-EMP-000001").expect("owner"));
        assert_eq!(owner.owner_id().as_str(), "CX-EMP-000001");
    }
}
