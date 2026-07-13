use crate::authorization::{AuthorityLevel, ScopeReference};
use crate::errors::{DomainError, DomainResult};
use crate::identifier::{
    AgentId, AuthorizationRequestId, DecisionAuthorityId, DecisionId, HumanId, NonEmptyText,
    PolicyId, WorkflowId,
};
use crate::lifecycle::DecisionRecordStatus;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecisionType {
    Architecture,
    Engineering,
    Operational,
    Security,
    Business,
    Policy,
}

pub type DecisionStatus = DecisionRecordStatus;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecisionOutcome {
    Approved,
    Rejected,
    Executed,
    Superseded,
    Archived,
}

impl DecisionOutcome {
    pub fn from_status(status: DecisionStatus) -> Option<Self> {
        match status {
            DecisionStatus::Approved => Some(Self::Approved),
            DecisionStatus::Rejected => Some(Self::Rejected),
            DecisionStatus::Executed => Some(Self::Executed),
            DecisionStatus::Superseded => Some(Self::Superseded),
            DecisionStatus::Archived => Some(Self::Archived),
            DecisionStatus::Draft
            | DecisionStatus::PendingReview
            | DecisionStatus::PendingApproval => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecisionOwnerReference {
    Human(HumanId),
    Agent(AgentId),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionContextReference(NonEmptyText);

impl DecisionContextReference {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        Ok(Self(NonEmptyText::new("decision_context", value)?))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionRationaleReference(NonEmptyText);

impl DecisionRationaleReference {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        Ok(Self(NonEmptyText::new("decision_rationale", value)?))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionPolicySetReference {
    policy_ids: Vec<PolicyId>,
}

impl DecisionPolicySetReference {
    pub fn new(policy_ids: Vec<PolicyId>) -> DomainResult<Self> {
        if policy_ids.is_empty() {
            return Err(DomainError::InvalidDecisionRecord(
                "decision policy set cannot be empty",
            ));
        }
        Ok(Self { policy_ids })
    }

    pub fn policy_ids(&self) -> &[PolicyId] {
        &self.policy_ids
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecisionSubjectReference {
    AuthorizationRequest(AuthorizationRequestId),
    Workflow(WorkflowId),
    Agent(AgentId),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionRecord {
    decision_id: DecisionId,
    decision_type: DecisionType,
    authority_id: DecisionAuthorityId,
    authority_level: AuthorityLevel,
    owner: DecisionOwnerReference,
    status: DecisionStatus,
    scope: ScopeReference,
    context: DecisionContextReference,
    policy_set: DecisionPolicySetReference,
    subject: DecisionSubjectReference,
    outcome: Option<DecisionOutcome>,
    rationale: Option<DecisionRationaleReference>,
    created_at: NonEmptyText,
    decided_at: Option<NonEmptyText>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionRecordSpec {
    pub decision_id: DecisionId,
    pub decision_type: DecisionType,
    pub authority_id: DecisionAuthorityId,
    pub authority_level: AuthorityLevel,
    pub owner: DecisionOwnerReference,
    pub status: DecisionStatus,
    pub scope: ScopeReference,
    pub context: DecisionContextReference,
    pub policy_set: DecisionPolicySetReference,
    pub subject: DecisionSubjectReference,
    pub rationale: Option<DecisionRationaleReference>,
    pub created_at: String,
    pub decided_at: Option<String>,
}

impl DecisionRecord {
    pub fn new(spec: DecisionRecordSpec) -> DomainResult<Self> {
        if matches!(spec.status, DecisionStatus::Rejected) && spec.rationale.is_none() {
            return Err(DomainError::InvalidDecisionRecord(
                "rejected decisions require explicit rationale",
            ));
        }
        let decided_at = spec
            .decided_at
            .map(|value| NonEmptyText::new("decided_at", value))
            .transpose()?;
        if DecisionOutcome::from_status(spec.status).is_some() && decided_at.is_none() {
            return Err(DomainError::InvalidDecisionRecord(
                "authoritative decision outcomes require decided_at evidence",
            ));
        }
        Ok(Self {
            decision_id: spec.decision_id,
            decision_type: spec.decision_type,
            authority_id: spec.authority_id,
            authority_level: spec.authority_level,
            owner: spec.owner,
            status: spec.status,
            scope: spec.scope,
            context: spec.context,
            policy_set: spec.policy_set,
            subject: spec.subject,
            outcome: DecisionOutcome::from_status(spec.status),
            rationale: spec.rationale,
            created_at: NonEmptyText::new("created_at", spec.created_at)?,
            decided_at,
        })
    }

    pub fn decision_id(&self) -> &DecisionId {
        &self.decision_id
    }

    pub fn status(&self) -> DecisionStatus {
        self.status
    }
}

#[cfg(test)]
mod tests {
    use super::{
        DecisionContextReference, DecisionOwnerReference, DecisionPolicySetReference,
        DecisionRationaleReference, DecisionRecord, DecisionRecordSpec, DecisionStatus,
        DecisionSubjectReference, DecisionType,
    };
    use crate::authorization::{AuthorityLevel, ScopeLevel, ScopeReference};
    use crate::identifier::{
        AuthorizationRequestId, DecisionAuthorityId, DecisionId, EnterpriseId, HumanId, PolicyId,
        ScopeId,
    };
    use crate::ownership::OwnershipPath;

    fn valid_scope() -> ScopeReference {
        ScopeReference::new(
            ScopeId::new("CX-SCP-000001").expect("scope"),
            ScopeLevel::Enterprise,
            OwnershipPath::new(
                EnterpriseId::new("CX-ENT-000001").expect("enterprise"),
                None,
                None,
                None,
            )
            .expect("path"),
            None,
        )
        .expect("scope")
    }

    #[test]
    fn decision_creates_valid_decision_record_ces_b0_022_1() {
        let record = DecisionRecord::new(DecisionRecordSpec {
            decision_id: DecisionId::new("CX-DEC-000001").expect("id"),
            decision_type: DecisionType::Operational,
            authority_id: DecisionAuthorityId::new("CX-DECAUTH-000001").expect("authority"),
            authority_level: AuthorityLevel::Manager,
            owner: DecisionOwnerReference::Human(HumanId::new("CX-EMP-000001").expect("owner")),
            status: DecisionStatus::Approved,
            scope: valid_scope(),
            context: DecisionContextReference::new("request scope is valid").expect("context"),
            policy_set: DecisionPolicySetReference::new(vec![
                PolicyId::new("CX-POL-000001").expect("policy")
            ])
            .expect("policy set"),
            subject: DecisionSubjectReference::AuthorizationRequest(
                AuthorizationRequestId::new("CX-AUTHREQ-000001").expect("request"),
            ),
            rationale: Some(
                DecisionRationaleReference::new("approved with evidence").expect("rationale"),
            ),
            created_at: "2026-07-14T00:00:00Z".to_owned(),
            decided_at: Some("2026-07-14T00:00:05Z".to_owned()),
        })
        .expect("valid decision");
        assert_eq!(record.decision_id().as_str(), "CX-DEC-000001");
    }

    #[test]
    fn decision_rejects_invalid_authority_status_combination_ces_b0_022_6() {
        let error = DecisionRecord::new(DecisionRecordSpec {
            decision_id: DecisionId::new("CX-DEC-000001").expect("id"),
            decision_type: DecisionType::Operational,
            authority_id: DecisionAuthorityId::new("CX-DECAUTH-000001").expect("authority"),
            authority_level: AuthorityLevel::Manager,
            owner: DecisionOwnerReference::Human(HumanId::new("CX-EMP-000001").expect("owner")),
            status: DecisionStatus::Rejected,
            scope: valid_scope(),
            context: DecisionContextReference::new("request scope is invalid").expect("context"),
            policy_set: DecisionPolicySetReference::new(vec![
                PolicyId::new("CX-POL-000001").expect("policy")
            ])
            .expect("policy set"),
            subject: DecisionSubjectReference::AuthorizationRequest(
                AuthorizationRequestId::new("CX-AUTHREQ-000001").expect("request"),
            ),
            rationale: None,
            created_at: "2026-07-14T00:00:00Z".to_owned(),
            decided_at: Some("2026-07-14T00:00:05Z".to_owned()),
        })
        .expect_err("rejected decision without rationale must fail");
        assert!(error
            .to_string()
            .contains("rejected decisions require explicit rationale"));
    }

    #[test]
    fn decision_stable_id_is_preserved_ces_b0_022_1() {
        let id = DecisionId::new("CX-DEC-000111").expect("id");
        assert_eq!(id.to_string(), "CX-DEC-000111");
    }
}
