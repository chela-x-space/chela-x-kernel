use std::fmt;
use std::str::FromStr;

use crate::errors::{DomainError, DomainResult};
use crate::identifier::{
    AuditEvidenceId, AuthorizationDecisionId, DecisionId, DelegationId, EnglishNamespace, PolicyId,
    StableVersion, WorkflowId,
};
use crate::ownership::OwnershipPath;
use crate::state::WorkflowStateSnapshot;

const WORKFLOW_DEFINITION_REFERENCE_EXPECTATION: &str =
    "ASCII letters, digits, dot, underscore, or hyphen";

fn validate_workflow_definition_reference(
    kind: &'static str,
    value: impl Into<String>,
) -> DomainResult<String> {
    let value = value.into().trim().to_owned();

    if value.is_empty() {
        return Err(DomainError::InvalidIdentifier {
            kind,
            value,
            expected: WORKFLOW_DEFINITION_REFERENCE_EXPECTATION,
        });
    }

    if value
        .chars()
        .all(|character| character.is_ascii_alphanumeric() || matches!(character, '.' | '_' | '-'))
    {
        Ok(value)
    } else {
        Err(DomainError::InvalidIdentifier {
            kind,
            value,
            expected: WORKFLOW_DEFINITION_REFERENCE_EXPECTATION,
        })
    }
}

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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorkflowLifecycleMapReference(String);

impl WorkflowLifecycleMapReference {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        validate_workflow_definition_reference("WorkflowLifecycleMapReference", value).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for WorkflowLifecycleMapReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for WorkflowLifecycleMapReference {
    type Err = DomainError;

    fn from_str(value: &str) -> DomainResult<Self> {
        Self::new(value.to_owned())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorkflowStepReference(String);

impl WorkflowStepReference {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        validate_workflow_definition_reference("WorkflowStepReference", value).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for WorkflowStepReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for WorkflowStepReference {
    type Err = DomainError;

    fn from_str(value: &str) -> DomainResult<Self> {
        Self::new(value.to_owned())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorkflowTerminalOutcomeReference(String);

impl WorkflowTerminalOutcomeReference {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        validate_workflow_definition_reference("WorkflowTerminalOutcomeReference", value).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for WorkflowTerminalOutcomeReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for WorkflowTerminalOutcomeReference {
    type Err = DomainError;

    fn from_str(value: &str) -> DomainResult<Self> {
        Self::new(value.to_owned())
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowDefinition {
    workflow_id: WorkflowId,
    namespace: EnglishNamespace,
    definition_version: StableVersion,
    ownership: OwnershipPath,
    lifecycle_map: WorkflowLifecycleMapReference,
    entry_steps: Vec<WorkflowStepReference>,
    terminal_outcomes: Vec<WorkflowTerminalOutcomeReference>,
    policy_references: Vec<PolicyId>,
    retry_policy: Option<WorkflowRetryPolicyReference>,
    retry_limit: Option<WorkflowRetryLimit>,
    recovery_reference: Option<WorkflowRecoveryReference>,
    audit_evidence: Vec<WorkflowAuditEvidenceReference>,
}

impl WorkflowDefinition {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        workflow_id: WorkflowId,
        namespace: EnglishNamespace,
        definition_version: StableVersion,
        ownership: OwnershipPath,
        lifecycle_map: WorkflowLifecycleMapReference,
        entry_steps: Vec<WorkflowStepReference>,
        terminal_outcomes: Vec<WorkflowTerminalOutcomeReference>,
        policy_references: Vec<PolicyId>,
        retry_policy: Option<WorkflowRetryPolicyReference>,
        retry_limit: Option<WorkflowRetryLimit>,
        recovery_reference: Option<WorkflowRecoveryReference>,
        audit_evidence: Vec<WorkflowAuditEvidenceReference>,
    ) -> DomainResult<Self> {
        if entry_steps.is_empty() {
            return Err(DomainError::InvalidWorkflowDefinition(
                "workflow definition requires at least one entry step",
            ));
        }

        if retry_limit.is_some() && retry_policy.is_none() {
            return Err(DomainError::InvalidWorkflowDefinition(
                "retry limit requires retry policy",
            ));
        }

        for (index, entry_step) in entry_steps.iter().enumerate() {
            if entry_steps[..index].iter().any(|prior| prior == entry_step) {
                return Err(DomainError::InvalidWorkflowDefinition(
                    "duplicate workflow entry step reference",
                ));
            }
        }

        for (index, terminal_outcome) in terminal_outcomes.iter().enumerate() {
            if terminal_outcomes[..index]
                .iter()
                .any(|prior| prior == terminal_outcome)
            {
                return Err(DomainError::InvalidWorkflowDefinition(
                    "duplicate workflow terminal outcome reference",
                ));
            }
        }

        for (index, policy_reference) in policy_references.iter().enumerate() {
            if policy_references[..index]
                .iter()
                .any(|prior| prior == policy_reference)
            {
                return Err(DomainError::InvalidWorkflowDefinition(
                    "duplicate workflow policy reference",
                ));
            }
        }

        for (index, evidence) in audit_evidence.iter().enumerate() {
            if audit_evidence[..index]
                .iter()
                .any(|prior| prior.audit_evidence_id() == evidence.audit_evidence_id())
            {
                return Err(DomainError::InvalidWorkflowDefinition(
                    "duplicate workflow definition audit evidence reference",
                ));
            }
        }

        Ok(Self {
            workflow_id,
            namespace,
            definition_version,
            ownership,
            lifecycle_map,
            entry_steps,
            terminal_outcomes,
            policy_references,
            retry_policy,
            retry_limit,
            recovery_reference,
            audit_evidence,
        })
    }

    pub fn workflow_id(&self) -> &WorkflowId {
        &self.workflow_id
    }

    pub fn namespace(&self) -> &EnglishNamespace {
        &self.namespace
    }

    pub fn definition_version(&self) -> &StableVersion {
        &self.definition_version
    }

    pub fn ownership(&self) -> &OwnershipPath {
        &self.ownership
    }

    pub fn lifecycle_map(&self) -> &WorkflowLifecycleMapReference {
        &self.lifecycle_map
    }

    pub fn entry_steps(&self) -> &[WorkflowStepReference] {
        &self.entry_steps
    }

    pub fn terminal_outcomes(&self) -> &[WorkflowTerminalOutcomeReference] {
        &self.terminal_outcomes
    }

    pub fn policy_references(&self) -> &[PolicyId] {
        &self.policy_references
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowInstance {
    workflow_id: WorkflowId,
    workflow_definition: WorkflowDefinition,
    definition_version_snapshot: StableVersion,
    ownership_reference: OwnershipPath,
    current_workflow_state_snapshot: WorkflowStateSnapshot,
    creation_evidence: WorkflowAuditEvidenceReference,
    retry_policy_snapshot: Option<WorkflowRetryPolicyReference>,
    retry_limit_snapshot: Option<WorkflowRetryLimit>,
    recovery_reference: Option<WorkflowRecoveryReference>,
    audit_evidence_references: Vec<WorkflowAuditEvidenceReference>,
}

impl WorkflowInstance {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        workflow_id: WorkflowId,
        workflow_definition: WorkflowDefinition,
        definition_version_snapshot: StableVersion,
        ownership_reference: OwnershipPath,
        current_workflow_state_snapshot: WorkflowStateSnapshot,
        creation_evidence: WorkflowAuditEvidenceReference,
        retry_policy_snapshot: Option<WorkflowRetryPolicyReference>,
        retry_limit_snapshot: Option<WorkflowRetryLimit>,
        recovery_reference: Option<WorkflowRecoveryReference>,
        audit_evidence_references: Vec<WorkflowAuditEvidenceReference>,
    ) -> DomainResult<Self> {
        if retry_limit_snapshot.is_some() && retry_policy_snapshot.is_none() {
            return Err(DomainError::InvalidWorkflowInstance(
                "retry limit requires retry policy",
            ));
        }

        for (index, evidence) in audit_evidence_references.iter().enumerate() {
            if audit_evidence_references[..index]
                .iter()
                .any(|prior| prior.audit_evidence_id() == evidence.audit_evidence_id())
            {
                return Err(DomainError::InvalidWorkflowInstance(
                    "duplicate workflow instance audit evidence reference",
                ));
            }
        }

        Ok(Self {
            workflow_id,
            workflow_definition,
            definition_version_snapshot,
            ownership_reference,
            current_workflow_state_snapshot,
            creation_evidence,
            retry_policy_snapshot,
            retry_limit_snapshot,
            recovery_reference,
            audit_evidence_references,
        })
    }

    pub fn workflow_id(&self) -> &WorkflowId {
        &self.workflow_id
    }

    pub fn workflow_definition(&self) -> &WorkflowDefinition {
        &self.workflow_definition
    }

    pub fn definition_version_snapshot(&self) -> &StableVersion {
        &self.definition_version_snapshot
    }

    pub fn ownership_reference(&self) -> &OwnershipPath {
        &self.ownership_reference
    }

    pub fn current_workflow_state_snapshot(&self) -> &WorkflowStateSnapshot {
        &self.current_workflow_state_snapshot
    }

    pub fn creation_evidence(&self) -> &WorkflowAuditEvidenceReference {
        &self.creation_evidence
    }

    pub fn retry_policy_snapshot(&self) -> Option<&WorkflowRetryPolicyReference> {
        self.retry_policy_snapshot.as_ref()
    }

    pub fn retry_limit_snapshot(&self) -> Option<WorkflowRetryLimit> {
        self.retry_limit_snapshot
    }

    pub fn recovery_reference(&self) -> Option<&WorkflowRecoveryReference> {
        self.recovery_reference.as_ref()
    }

    pub fn audit_evidence_references(&self) -> &[WorkflowAuditEvidenceReference] {
        &self.audit_evidence_references
    }
}

#[cfg(test)]
mod tests {
    use super::{
        WorkflowAuditEvidenceReference, WorkflowDefinition, WorkflowEngineFoundation,
        WorkflowInstance, WorkflowLifecycleMapReference, WorkflowRecoveryReference,
        WorkflowRetryLimit, WorkflowRetryPolicyReference, WorkflowStepReference,
        WorkflowTerminalOutcomeReference,
    };
    use crate::errors::DomainError;
    use crate::identifier::EnglishNamespace;
    use crate::identifier::{
        AuditEvidenceId, AuthorizationDecisionId, DecisionId, DelegationId, EnterpriseId, HumanId,
        OrganizationUnitId, PolicyId, ProjectId, StableVersion, WorkflowId, WorkspaceId,
    };
    use crate::lifecycle::WorkflowState;
    use crate::ownership::{OwnerReference, OwnershipPath};
    use crate::state::{StateSequence, WorkflowStateSnapshot};

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

    fn namespace() -> EnglishNamespace {
        EnglishNamespace::new("workflow_namespace", "ops.approval-flow").expect("namespace")
    }

    fn lifecycle_map() -> WorkflowLifecycleMapReference {
        WorkflowLifecycleMapReference::new("workflow.lifecycle.v1").expect("lifecycle map")
    }

    fn entry_step(value: &str) -> WorkflowStepReference {
        WorkflowStepReference::new(value).expect("entry step")
    }

    fn terminal_outcome(value: &str) -> WorkflowTerminalOutcomeReference {
        WorkflowTerminalOutcomeReference::new(value).expect("terminal outcome")
    }

    fn workflow_definition() -> WorkflowDefinition {
        WorkflowDefinition::new(
            workflow_id(),
            namespace(),
            definition_version(),
            ownership(),
            lifecycle_map(),
            vec![entry_step("start.review"), entry_step("collect-input")],
            vec![terminal_outcome("completed"), terminal_outcome("cancelled")],
            vec![
                PolicyId::new("CX-POL-000001").expect("policy"),
                PolicyId::new("CX-POL-000002").expect("policy"),
            ],
            Some(retry_policy()),
            Some(retry_limit()),
            Some(recovery_reference()),
            vec![audit_evidence("CX-AUD-000001"), second_audit_evidence()],
        )
        .expect("workflow definition")
    }

    fn workflow_state_snapshot() -> WorkflowStateSnapshot {
        WorkflowStateSnapshot::new(
            workflow_id(),
            ownership(),
            definition_version(),
            WorkflowState::Validated,
            StateSequence::new(1).expect("sequence"),
        )
    }

    fn workflow_instance() -> WorkflowInstance {
        WorkflowInstance::new(
            workflow_id(),
            workflow_definition(),
            definition_version(),
            ownership(),
            workflow_state_snapshot(),
            audit_evidence("CX-AUD-000003"),
            Some(retry_policy()),
            Some(retry_limit()),
            Some(recovery_reference()),
            vec![audit_evidence("CX-AUD-000001"), second_audit_evidence()],
        )
        .expect("workflow instance")
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

    #[test]
    fn workflow_definition_valid_definition_construction_passes() {
        let definition = workflow_definition();
        assert_eq!(definition.workflow_id().as_str(), "CX-WF-000001");
        assert_eq!(definition.entry_steps().len(), 2);
    }

    #[test]
    fn workflow_definition_identity_is_preserved() {
        let workflow_id = workflow_id();
        let definition = WorkflowDefinition::new(
            workflow_id.clone(),
            namespace(),
            definition_version(),
            ownership(),
            lifecycle_map(),
            vec![entry_step("start.review")],
            vec![],
            vec![],
            None,
            None,
            None,
            vec![],
        )
        .expect("definition");

        assert_eq!(definition.workflow_id(), &workflow_id);
    }

    #[test]
    fn workflow_definition_namespace_is_preserved() {
        let namespace = namespace();
        let definition = WorkflowDefinition::new(
            workflow_id(),
            namespace.clone(),
            definition_version(),
            ownership(),
            lifecycle_map(),
            vec![entry_step("start.review")],
            vec![],
            vec![],
            None,
            None,
            None,
            vec![],
        )
        .expect("definition");

        assert_eq!(definition.namespace(), &namespace);
    }

    #[test]
    fn workflow_definition_definition_version_is_preserved() {
        let definition_version = definition_version();
        let definition = WorkflowDefinition::new(
            workflow_id(),
            namespace(),
            definition_version.clone(),
            ownership(),
            lifecycle_map(),
            vec![entry_step("start.review")],
            vec![],
            vec![],
            None,
            None,
            None,
            vec![],
        )
        .expect("definition");

        assert_eq!(definition.definition_version(), &definition_version);
    }

    #[test]
    fn workflow_definition_ownership_is_preserved() {
        let ownership = ownership();
        let definition = WorkflowDefinition::new(
            workflow_id(),
            namespace(),
            definition_version(),
            ownership.clone(),
            lifecycle_map(),
            vec![entry_step("start.review")],
            vec![],
            vec![],
            None,
            None,
            None,
            vec![],
        )
        .expect("definition");

        assert_eq!(definition.ownership(), &ownership);
    }

    #[test]
    fn workflow_definition_entry_steps_are_preserved_in_caller_order() {
        let first = entry_step("start.review");
        let second = entry_step("collect-input");
        let definition = WorkflowDefinition::new(
            workflow_id(),
            namespace(),
            definition_version(),
            ownership(),
            lifecycle_map(),
            vec![first.clone(), second.clone()],
            vec![],
            vec![],
            None,
            None,
            None,
            vec![],
        )
        .expect("definition");

        assert_eq!(definition.entry_steps(), &[first, second]);
    }

    #[test]
    fn workflow_definition_empty_entry_steps_are_rejected() {
        let error = WorkflowDefinition::new(
            workflow_id(),
            namespace(),
            definition_version(),
            ownership(),
            lifecycle_map(),
            vec![],
            vec![],
            vec![],
            None,
            None,
            None,
            vec![],
        )
        .expect_err("empty entry steps must fail");

        assert_eq!(
            error,
            DomainError::InvalidWorkflowDefinition(
                "workflow definition requires at least one entry step",
            )
        );
    }

    #[test]
    fn workflow_definition_duplicate_entry_step_is_rejected() {
        let error = WorkflowDefinition::new(
            workflow_id(),
            namespace(),
            definition_version(),
            ownership(),
            lifecycle_map(),
            vec![entry_step("start.review"), entry_step("start.review")],
            vec![],
            vec![],
            None,
            None,
            None,
            vec![],
        )
        .expect_err("duplicate entry step must fail");

        assert_eq!(
            error,
            DomainError::InvalidWorkflowDefinition("duplicate workflow entry step reference")
        );
    }

    #[test]
    fn workflow_definition_terminal_outcomes_are_preserved_in_caller_order() {
        let first = terminal_outcome("completed");
        let second = terminal_outcome("cancelled");
        let definition = WorkflowDefinition::new(
            workflow_id(),
            namespace(),
            definition_version(),
            ownership(),
            lifecycle_map(),
            vec![entry_step("start.review")],
            vec![first.clone(), second.clone()],
            vec![],
            None,
            None,
            None,
            vec![],
        )
        .expect("definition");

        assert_eq!(definition.terminal_outcomes(), &[first, second]);
    }

    #[test]
    fn workflow_definition_duplicate_terminal_outcome_is_rejected() {
        let error = WorkflowDefinition::new(
            workflow_id(),
            namespace(),
            definition_version(),
            ownership(),
            lifecycle_map(),
            vec![entry_step("start.review")],
            vec![terminal_outcome("completed"), terminal_outcome("completed")],
            vec![],
            None,
            None,
            None,
            vec![],
        )
        .expect_err("duplicate terminal outcome must fail");

        assert_eq!(
            error,
            DomainError::InvalidWorkflowDefinition("duplicate workflow terminal outcome reference",)
        );
    }

    #[test]
    fn workflow_definition_policy_references_are_preserved() {
        let first = PolicyId::new("CX-POL-000001").expect("policy");
        let second = PolicyId::new("CX-POL-000002").expect("policy");
        let definition = WorkflowDefinition::new(
            workflow_id(),
            namespace(),
            definition_version(),
            ownership(),
            lifecycle_map(),
            vec![entry_step("start.review")],
            vec![],
            vec![first.clone(), second.clone()],
            None,
            None,
            None,
            vec![],
        )
        .expect("definition");

        assert_eq!(definition.policy_references(), &[first, second]);
    }

    #[test]
    fn workflow_definition_duplicate_policy_reference_is_rejected() {
        let error = WorkflowDefinition::new(
            workflow_id(),
            namespace(),
            definition_version(),
            ownership(),
            lifecycle_map(),
            vec![entry_step("start.review")],
            vec![],
            vec![
                PolicyId::new("CX-POL-000001").expect("policy"),
                PolicyId::new("CX-POL-000001").expect("policy"),
            ],
            None,
            None,
            None,
            vec![],
        )
        .expect_err("duplicate policy must fail");

        assert_eq!(
            error,
            DomainError::InvalidWorkflowDefinition("duplicate workflow policy reference")
        );
    }

    #[test]
    fn workflow_definition_audit_evidence_may_be_empty() {
        let definition = WorkflowDefinition::new(
            workflow_id(),
            namespace(),
            definition_version(),
            ownership(),
            lifecycle_map(),
            vec![entry_step("start.review")],
            vec![],
            vec![],
            Some(retry_policy()),
            Some(retry_limit()),
            Some(recovery_reference()),
            vec![],
        )
        .expect("definition");

        assert!(definition.audit_evidence().is_empty());
    }

    #[test]
    fn workflow_definition_audit_evidence_order_is_preserved() {
        let first = audit_evidence("CX-AUD-000001");
        let second = second_audit_evidence();
        let definition = WorkflowDefinition::new(
            workflow_id(),
            namespace(),
            definition_version(),
            ownership(),
            lifecycle_map(),
            vec![entry_step("start.review")],
            vec![],
            vec![],
            None,
            None,
            None,
            vec![first.clone(), second.clone()],
        )
        .expect("definition");

        assert_eq!(definition.audit_evidence(), &[first, second]);
    }

    #[test]
    fn workflow_definition_duplicate_audit_evidence_is_rejected() {
        let error = WorkflowDefinition::new(
            workflow_id(),
            namespace(),
            definition_version(),
            ownership(),
            lifecycle_map(),
            vec![entry_step("start.review")],
            vec![],
            vec![],
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
            DomainError::InvalidWorkflowDefinition(
                "duplicate workflow definition audit evidence reference",
            )
        );
    }

    #[test]
    fn workflow_definition_construction_is_deterministic() {
        let first = workflow_definition();
        let second = workflow_definition();

        assert_eq!(first, second);
    }

    #[test]
    fn workflow_definition_equivalent_invalid_inputs_produce_equivalent_errors() {
        let first = WorkflowDefinition::new(
            workflow_id(),
            namespace(),
            definition_version(),
            ownership(),
            lifecycle_map(),
            vec![],
            vec![],
            vec![],
            None,
            None,
            None,
            vec![],
        )
        .expect_err("invalid");
        let second = WorkflowDefinition::new(
            workflow_id(),
            namespace(),
            definition_version(),
            ownership(),
            lifecycle_map(),
            vec![],
            vec![],
            vec![],
            None,
            None,
            None,
            vec![],
        )
        .expect_err("invalid");

        assert_eq!(first, second);
    }

    #[test]
    fn workflow_definition_value_semantics_are_preserved() {
        let definition = workflow_definition();
        assert_eq!(definition.clone(), definition);
    }

    #[test]
    fn workflow_definition_supplied_values_are_not_mutated() {
        let workflow_id = workflow_id();
        let namespace = namespace();
        let definition_version = definition_version();
        let ownership = ownership();
        let lifecycle_map = lifecycle_map();
        let entry_steps = vec![entry_step("start.review"), entry_step("collect-input")];
        let terminal_outcomes = vec![terminal_outcome("completed"), terminal_outcome("cancelled")];
        let policy_references = vec![
            PolicyId::new("CX-POL-000001").expect("policy"),
            PolicyId::new("CX-POL-000002").expect("policy"),
        ];
        let retry_policy = retry_policy();
        let retry_limit = retry_limit();
        let recovery_reference = recovery_reference();
        let audit_evidence = vec![audit_evidence("CX-AUD-000001"), second_audit_evidence()];

        let workflow_id_before = workflow_id.clone();
        let namespace_before = namespace.clone();
        let definition_version_before = definition_version.clone();
        let ownership_before = ownership.clone();
        let lifecycle_map_before = lifecycle_map.clone();
        let entry_steps_before = entry_steps.clone();
        let terminal_outcomes_before = terminal_outcomes.clone();
        let policy_references_before = policy_references.clone();
        let retry_policy_before = retry_policy.clone();
        let recovery_reference_before = recovery_reference.clone();
        let audit_evidence_before = audit_evidence.clone();

        let definition = WorkflowDefinition::new(
            workflow_id.clone(),
            namespace.clone(),
            definition_version.clone(),
            ownership.clone(),
            lifecycle_map.clone(),
            entry_steps.clone(),
            terminal_outcomes.clone(),
            policy_references.clone(),
            Some(retry_policy.clone()),
            Some(retry_limit),
            Some(recovery_reference.clone()),
            audit_evidence.clone(),
        )
        .expect("definition");

        assert_eq!(workflow_id, workflow_id_before);
        assert_eq!(namespace, namespace_before);
        assert_eq!(definition_version, definition_version_before);
        assert_eq!(ownership, ownership_before);
        assert_eq!(lifecycle_map, lifecycle_map_before);
        assert_eq!(entry_steps, entry_steps_before);
        assert_eq!(terminal_outcomes, terminal_outcomes_before);
        assert_eq!(policy_references, policy_references_before);
        assert_eq!(retry_policy, retry_policy_before);
        assert_eq!(recovery_reference, recovery_reference_before);
        assert_eq!(audit_evidence, audit_evidence_before);
        assert_eq!(definition.namespace(), &namespace_before);
    }

    #[test]
    fn workflow_definition_external_reference_existence_is_not_checked() {
        let definition = WorkflowDefinition::new(
            workflow_id(),
            namespace(),
            definition_version(),
            ownership(),
            WorkflowLifecycleMapReference::new("custom.lifecycle.map").expect("lifecycle map"),
            vec![WorkflowStepReference::new("entry.custom-step").expect("entry step")],
            vec![
                WorkflowTerminalOutcomeReference::new("terminal.custom-outcome")
                    .expect("terminal outcome"),
            ],
            vec![PolicyId::new("CX-POL-999999").expect("policy")],
            None,
            None,
            None,
            vec![],
        )
        .expect("definition");

        assert_eq!(definition.policy_references()[0].as_str(), "CX-POL-999999");
    }

    #[test]
    fn workflow_instance_valid_construction() {
        let instance = workflow_instance();
        assert_eq!(instance.workflow_id().as_str(), "CX-WF-000001");
        assert_eq!(instance.audit_evidence_references().len(), 2);
    }

    #[test]
    fn workflow_instance_definition_preserved() {
        let workflow_definition = workflow_definition();
        let instance = WorkflowInstance::new(
            workflow_id(),
            workflow_definition.clone(),
            definition_version(),
            ownership(),
            workflow_state_snapshot(),
            audit_evidence("CX-AUD-000003"),
            None,
            None,
            None,
            vec![],
        )
        .expect("workflow instance");

        assert_eq!(instance.workflow_definition(), &workflow_definition);
    }

    #[test]
    fn workflow_instance_definition_version_preserved() {
        let definition_version_snapshot = definition_version();
        let instance = WorkflowInstance::new(
            workflow_id(),
            workflow_definition(),
            definition_version_snapshot.clone(),
            ownership(),
            workflow_state_snapshot(),
            audit_evidence("CX-AUD-000003"),
            None,
            None,
            None,
            vec![],
        )
        .expect("workflow instance");

        assert_eq!(
            instance.definition_version_snapshot(),
            &definition_version_snapshot
        );
    }

    #[test]
    fn workflow_instance_ownership_preserved() {
        let ownership_reference = ownership();
        let instance = WorkflowInstance::new(
            workflow_id(),
            workflow_definition(),
            definition_version(),
            ownership_reference.clone(),
            workflow_state_snapshot(),
            audit_evidence("CX-AUD-000003"),
            None,
            None,
            None,
            vec![],
        )
        .expect("workflow instance");

        assert_eq!(instance.ownership_reference(), &ownership_reference);
    }

    #[test]
    fn workflow_instance_workflow_state_preserved() {
        let current_workflow_state_snapshot = workflow_state_snapshot();
        let instance = WorkflowInstance::new(
            workflow_id(),
            workflow_definition(),
            definition_version(),
            ownership(),
            current_workflow_state_snapshot.clone(),
            audit_evidence("CX-AUD-000003"),
            None,
            None,
            None,
            vec![],
        )
        .expect("workflow instance");

        assert_eq!(
            instance.current_workflow_state_snapshot(),
            &current_workflow_state_snapshot
        );
    }

    #[test]
    fn workflow_instance_retry_preserved() {
        let retry_policy_snapshot = retry_policy();
        let retry_limit_snapshot = retry_limit();
        let instance = WorkflowInstance::new(
            workflow_id(),
            workflow_definition(),
            definition_version(),
            ownership(),
            workflow_state_snapshot(),
            audit_evidence("CX-AUD-000003"),
            Some(retry_policy_snapshot.clone()),
            Some(retry_limit_snapshot),
            None,
            vec![],
        )
        .expect("workflow instance");

        assert_eq!(
            instance.retry_policy_snapshot(),
            Some(&retry_policy_snapshot)
        );
        assert_eq!(instance.retry_limit_snapshot(), Some(retry_limit_snapshot));
    }

    #[test]
    fn workflow_instance_retry_validation() {
        let error = WorkflowInstance::new(
            workflow_id(),
            workflow_definition(),
            definition_version(),
            ownership(),
            workflow_state_snapshot(),
            audit_evidence("CX-AUD-000003"),
            None,
            Some(retry_limit()),
            None,
            vec![],
        )
        .expect_err("retry limit without policy must fail");

        assert_eq!(
            error,
            DomainError::InvalidWorkflowInstance("retry limit requires retry policy")
        );
    }

    #[test]
    fn workflow_instance_recovery_optional() {
        let instance = WorkflowInstance::new(
            workflow_id(),
            workflow_definition(),
            definition_version(),
            ownership(),
            workflow_state_snapshot(),
            audit_evidence("CX-AUD-000003"),
            None,
            None,
            None,
            vec![],
        )
        .expect("workflow instance");

        assert_eq!(instance.recovery_reference(), None);
    }

    #[test]
    fn workflow_instance_empty_audit_evidence_allowed() {
        let instance = WorkflowInstance::new(
            workflow_id(),
            workflow_definition(),
            definition_version(),
            ownership(),
            workflow_state_snapshot(),
            audit_evidence("CX-AUD-000003"),
            None,
            None,
            Some(recovery_reference()),
            vec![],
        )
        .expect("workflow instance");

        assert!(instance.audit_evidence_references().is_empty());
    }

    #[test]
    fn workflow_instance_audit_order_preserved() {
        let first = audit_evidence("CX-AUD-000001");
        let second = second_audit_evidence();
        let instance = WorkflowInstance::new(
            workflow_id(),
            workflow_definition(),
            definition_version(),
            ownership(),
            workflow_state_snapshot(),
            audit_evidence("CX-AUD-000003"),
            None,
            None,
            None,
            vec![first.clone(), second.clone()],
        )
        .expect("workflow instance");

        assert_eq!(instance.audit_evidence_references(), &[first, second]);
    }

    #[test]
    fn workflow_instance_duplicate_audit_evidence_rejected() {
        let error = WorkflowInstance::new(
            workflow_id(),
            workflow_definition(),
            definition_version(),
            ownership(),
            workflow_state_snapshot(),
            audit_evidence("CX-AUD-000003"),
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
            DomainError::InvalidWorkflowInstance(
                "duplicate workflow instance audit evidence reference",
            )
        );
    }

    #[test]
    fn workflow_instance_deterministic_construction() {
        let first = WorkflowInstance::new(
            workflow_id(),
            workflow_definition(),
            definition_version(),
            ownership(),
            workflow_state_snapshot(),
            audit_evidence("CX-AUD-000003"),
            Some(retry_policy()),
            Some(retry_limit()),
            Some(recovery_reference()),
            vec![audit_evidence("CX-AUD-000001"), second_audit_evidence()],
        );
        let second = WorkflowInstance::new(
            workflow_id(),
            workflow_definition(),
            definition_version(),
            ownership(),
            workflow_state_snapshot(),
            audit_evidence("CX-AUD-000003"),
            Some(retry_policy()),
            Some(retry_limit()),
            Some(recovery_reference()),
            vec![audit_evidence("CX-AUD-000001"), second_audit_evidence()],
        );

        assert_eq!(first, second);
    }

    #[test]
    fn workflow_instance_equivalent_invalid_inputs() {
        let first = WorkflowInstance::new(
            workflow_id(),
            workflow_definition(),
            definition_version(),
            ownership(),
            workflow_state_snapshot(),
            audit_evidence("CX-AUD-000003"),
            None,
            Some(retry_limit()),
            None,
            vec![],
        )
        .expect_err("invalid");
        let second = WorkflowInstance::new(
            workflow_id(),
            workflow_definition(),
            definition_version(),
            ownership(),
            workflow_state_snapshot(),
            audit_evidence("CX-AUD-000003"),
            None,
            Some(retry_limit()),
            None,
            vec![],
        )
        .expect_err("invalid");

        assert_eq!(first, second);
    }

    #[test]
    fn workflow_instance_value_semantics() {
        let instance = workflow_instance();
        assert_eq!(instance.clone(), instance);
    }

    #[test]
    fn workflow_instance_supplied_values_not_mutated() {
        let workflow_id = workflow_id();
        let workflow_definition = workflow_definition();
        let definition_version_snapshot = definition_version();
        let ownership_reference = ownership();
        let current_workflow_state_snapshot = workflow_state_snapshot();
        let creation_evidence = audit_evidence("CX-AUD-000003");
        let retry_policy_snapshot = retry_policy();
        let retry_limit_snapshot = retry_limit();
        let recovery_reference = recovery_reference();
        let audit_evidence_references =
            vec![audit_evidence("CX-AUD-000001"), second_audit_evidence()];

        let workflow_id_before = workflow_id.clone();
        let workflow_definition_before = workflow_definition.clone();
        let definition_version_snapshot_before = definition_version_snapshot.clone();
        let ownership_reference_before = ownership_reference.clone();
        let current_workflow_state_snapshot_before = current_workflow_state_snapshot.clone();
        let creation_evidence_before = creation_evidence.clone();
        let retry_policy_snapshot_before = retry_policy_snapshot.clone();
        let recovery_reference_before = recovery_reference.clone();
        let audit_evidence_references_before = audit_evidence_references.clone();

        let instance = WorkflowInstance::new(
            workflow_id.clone(),
            workflow_definition.clone(),
            definition_version_snapshot.clone(),
            ownership_reference.clone(),
            current_workflow_state_snapshot.clone(),
            creation_evidence.clone(),
            Some(retry_policy_snapshot.clone()),
            Some(retry_limit_snapshot),
            Some(recovery_reference.clone()),
            audit_evidence_references.clone(),
        )
        .expect("workflow instance");

        assert_eq!(workflow_id, workflow_id_before);
        assert_eq!(workflow_definition, workflow_definition_before);
        assert_eq!(
            definition_version_snapshot,
            definition_version_snapshot_before
        );
        assert_eq!(ownership_reference, ownership_reference_before);
        assert_eq!(
            current_workflow_state_snapshot,
            current_workflow_state_snapshot_before
        );
        assert_eq!(creation_evidence, creation_evidence_before);
        assert_eq!(retry_policy_snapshot, retry_policy_snapshot_before);
        assert_eq!(recovery_reference, recovery_reference_before);
        assert_eq!(audit_evidence_references, audit_evidence_references_before);
        assert_eq!(instance.workflow_id(), &workflow_id_before);
    }

    #[test]
    fn workflow_instance_external_references_not_validated() {
        let workflow_definition = WorkflowDefinition::new(
            workflow_id(),
            namespace(),
            definition_version(),
            ownership(),
            WorkflowLifecycleMapReference::new("custom.lifecycle.map").expect("lifecycle map"),
            vec![WorkflowStepReference::new("entry.custom-step").expect("entry step")],
            vec![],
            vec![PolicyId::new("CX-POL-999999").expect("policy")],
            None,
            None,
            None,
            vec![],
        )
        .expect("workflow definition");
        let creation_evidence =
            WorkflowAuditEvidenceReference::new(
                AuditEvidenceId::new("CX-AUD-999999").expect("audit evidence id"),
                workflow_id(),
                definition_version(),
                vec![PolicyId::new("CX-POL-999999").expect("policy")],
                vec![AuthorizationDecisionId::new("CX-AUTHDEC-999999")
                    .expect("authorization decision")],
                vec![DelegationId::new("CX-DEL-999999").expect("delegation")],
                vec![DecisionId::new("CX-DEC-999999").expect("decision")],
            )
            .expect("creation evidence");

        let instance = WorkflowInstance::new(
            workflow_id(),
            workflow_definition,
            definition_version(),
            ownership(),
            workflow_state_snapshot(),
            creation_evidence,
            None,
            None,
            None,
            vec![],
        )
        .expect("workflow instance");

        assert_eq!(
            instance.creation_evidence().audit_evidence_id().as_str(),
            "CX-AUD-999999"
        );
    }
}
