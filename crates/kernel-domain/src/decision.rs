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
            DecisionStatus::Draft | DecisionStatus::PendingReview | DecisionStatus::PendingApproval => None,
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

impl DecisionRecord {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
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
        rationale: Option<DecisionRationaleReference>,
        created_at: impl Into<String>,
        decided_at: Option<String>,
    ) -> DomainResult<Self> {
        if matches!(status, DecisionStatus::Rejected) && rationale.is_none() {
            return Err(DomainError::InvalidDecisionRecord(
                "rejected decisions require explicit rationale",
            ));
        }
        let decided_at = decided_at
            .map(|value| NonEmptyText::new("decided_at", value))
            .transpose()?;
        if DecisionOutcome::from_status(status).is_some() && decided_at.is_none() {
            return Err(DomainError::InvalidDecisionRecord(
                "authoritative decision outcomes require decided_at evidence",
            ));
        }
        Ok(Self {
            decision_id,
            decision_type,
            authority_id,
            authority_level,
            owner,
            status,
            scope,
            context,
            policy_set,
            subject,
            outcome: DecisionOutcome::from_status(status),
            rationale,
            created_at: NonEmptyText::new("created_at", created_at)?,
            decided_at,
        })
    }

    pub fn decision_id(&self) -> &DecisionId {
        &self.decision_id
    }
}

#[cfg(test)]
mod tests {
    use super::{
        DecisionContextReference, DecisionOwnerReference, DecisionPolicySetReference, DecisionRecord,
        DecisionRationaleReference, DecisionStatus, DecisionSubjectReference, DecisionType,
    };
    use crate::authorization::{AuthorityLevel, ScopeLevel, ScopeReference};
    use crate::identifier::{
        AuthorizationRequestId, DecisionAuthorityId, DecisionId, EnterpriseId, HumanId, PolicyId, ScopeId,
    };
    use crate::ownership::OwnershipPath;

    fn valid_scope() -> ScopeReference {
        ScopeReference::new(
            ScopeId::new("CX-SCP-000001").expect("scope"),
            ScopeLevel::Enterprise,
            OwnershipPath::new(EnterpriseId::new("CX-ENT-000001").expect("enterprise"), None, None, None)
                .expect("path"),
            None,
        )
        .expect("scope")
    }

    #[test]
    fn decision_creates_valid_decision_record_ces_b0_022_1() {
        let record = DecisionRecord::new(
            DecisionId::new("CX-DEC-000001").expect("id"),
            DecisionType::Operational,
            DecisionAuthorityId::new("CX-DECAUTH-000001").expect("authority"),
            AuthorityLevel::Manager,
            DecisionOwnerReference::Human(HumanId::new("CX-EMP-000001").expect("owner")),
            DecisionStatus::Approved,
            valid_scope(),
            DecisionContextReference::new("request scope is valid").expect("context"),
            DecisionPolicySetReference::new(vec![PolicyId::new("CX-POL-000001").expect("policy")])
                .expect("policy set"),
            DecisionSubjectReference::AuthorizationRequest(
                AuthorizationRequestId::new("CX-AUTHREQ-000001").expect("request"),
            ),
            Some(DecisionRationaleReference::new("approved with evidence").expect("rationale")),
            "2026-07-14T00:00:00Z",
            Some("2026-07-14T00:00:05Z".to_owned()),
        )
        .expect("valid decision");
        assert_eq!(record.decision_id().as_str(), "CX-DEC-000001");
    }

    #[test]
    fn decision_rejects_invalid_authority_status_combination_ces_b0_022_6() {
        let error = DecisionRecord::new(
            DecisionId::new("CX-DEC-000001").expect("id"),
            DecisionType::Operational,
            DecisionAuthorityId::new("CX-DECAUTH-000001").expect("authority"),
            AuthorityLevel::Manager,
            DecisionOwnerReference::Human(HumanId::new("CX-EMP-000001").expect("owner")),
            DecisionStatus::Rejected,
            valid_scope(),
            DecisionContextReference::new("request scope is invalid").expect("context"),
            DecisionPolicySetReference::new(vec![PolicyId::new("CX-POL-000001").expect("policy")])
                .expect("policy set"),
            DecisionSubjectReference::AuthorizationRequest(
                AuthorizationRequestId::new("CX-AUTHREQ-000001").expect("request"),
            ),
            None,
            "2026-07-14T00:00:00Z",
            Some("2026-07-14T00:00:05Z".to_owned()),
        )
        .expect_err("rejected decision without rationale must fail");
        assert!(error.to_string().contains("rejected decisions require explicit rationale"));
    }

    #[test]
    fn decision_stable_id_is_preserved_ces_b0_022_1() {
        let id = DecisionId::new("CX-DEC-000111").expect("id");
        assert_eq!(id.to_string(), "CX-DEC-000111");
    }
}
