use crate::authorization::{
    AuthorityLevel, AuthorizationAuditEvidenceReference, AuthorizationDecisionOutcome,
    AuthorizationDecisionReference, AuthorizationEvaluationOrderVersion,
    AuthorizationEvaluationStep, AuthorizationPrincipalReference, AuthorizationSubject,
    AuthorizationTarget, MatchedPolicyEvidenceReference, PermissionReference, RoleReference,
    ScopeLevel, ScopeReference,
};
use crate::decision::{
    DecisionContextReference, DecisionOwnerReference, DecisionPolicySetReference,
    DecisionRationaleReference, DecisionRecord, DecisionRecordSpec, DecisionStatus,
    DecisionSubjectReference, DecisionType,
};
use crate::delegation::{DelegationDepth, DelegationReference, SeparationOfDutiesConflict};
use crate::errors::{DomainError, DomainResult};
use crate::identifier::{
    AuditEvidenceId, AuthorizationDecisionId, DecisionAuthorityId, DecisionId, PolicyId,
    PrincipalId, StableVersion,
};
use crate::lifecycle::DelegationLifecycle;
use crate::policy::PolicyEffect;
use crate::request::{AuthorizationRequestRecord, TimeReference};
use std::cmp::Ordering;
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizationRolePermissionRecord {
    role: RoleReference,
    permission: PermissionReference,
    evidence: MatchedPolicyEvidenceReference,
    active: bool,
}

impl AuthorizationRolePermissionRecord {
    pub fn new(
        role: RoleReference,
        permission: PermissionReference,
        evidence: MatchedPolicyEvidenceReference,
        active: bool,
    ) -> Self {
        Self {
            role,
            permission,
            evidence,
            active,
        }
    }

    pub fn role(&self) -> &RoleReference {
        &self.role
    }

    pub fn permission(&self) -> &PermissionReference {
        &self.permission
    }

    pub fn evidence(&self) -> &MatchedPolicyEvidenceReference {
        &self.evidence
    }

    pub fn active(&self) -> bool {
        self.active
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizationGrantRecord {
    policy_id: PolicyId,
    principal_id: PrincipalId,
    scope: ScopeReference,
    role: Option<RoleReference>,
    permission: Option<PermissionReference>,
    inheritable: bool,
    active: bool,
    evidence: MatchedPolicyEvidenceReference,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizationGrantRecordSpec {
    pub policy_id: PolicyId,
    pub principal_id: PrincipalId,
    pub scope: ScopeReference,
    pub role: Option<RoleReference>,
    pub permission: Option<PermissionReference>,
    pub inheritable: bool,
    pub active: bool,
    pub evidence: MatchedPolicyEvidenceReference,
}

impl AuthorizationGrantRecord {
    pub fn new(spec: AuthorizationGrantRecordSpec) -> DomainResult<Self> {
        if spec.role.is_none() && spec.permission.is_none() {
            return Err(DomainError::InvalidAuthorizationEvaluation(
                "grant record requires a role or permission reference",
            ));
        }
        Ok(Self {
            policy_id: spec.policy_id,
            principal_id: spec.principal_id,
            scope: spec.scope,
            role: spec.role,
            permission: spec.permission,
            inheritable: spec.inheritable,
            active: spec.active,
            evidence: spec.evidence,
        })
    }

    pub fn policy_id(&self) -> &PolicyId {
        &self.policy_id
    }

    pub fn principal_id(&self) -> &PrincipalId {
        &self.principal_id
    }

    pub fn scope(&self) -> &ScopeReference {
        &self.scope
    }

    pub fn role(&self) -> Option<&RoleReference> {
        self.role.as_ref()
    }

    pub fn permission(&self) -> Option<&PermissionReference> {
        self.permission.as_ref()
    }

    pub fn inheritable(&self) -> bool {
        self.inheritable
    }

    pub fn active(&self) -> bool {
        self.active
    }

    pub fn evidence(&self) -> &MatchedPolicyEvidenceReference {
        &self.evidence
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizationExplicitDenyRecord {
    policy_id: PolicyId,
    principal_id: PrincipalId,
    permission: PermissionReference,
    scope: ScopeReference,
    active: bool,
    non_waivable: bool,
    evidence: MatchedPolicyEvidenceReference,
}

impl AuthorizationExplicitDenyRecord {
    pub fn new(
        policy_id: PolicyId,
        principal_id: PrincipalId,
        permission: PermissionReference,
        scope: ScopeReference,
        active: bool,
        non_waivable: bool,
        evidence: MatchedPolicyEvidenceReference,
    ) -> Self {
        Self {
            policy_id,
            principal_id,
            permission,
            scope,
            active,
            non_waivable,
            evidence,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorizationPolicyLayer {
    MandatoryEnterprise,
    Scoped,
    ExceptionOrWaiver,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizationPolicyRecord {
    policy_id: PolicyId,
    permission: PermissionReference,
    scope: ScopeReference,
    effect: PolicyEffect,
    authority_level: AuthorityLevel,
    layer: AuthorizationPolicyLayer,
    mandatory: bool,
    active: bool,
    approved: bool,
    priority: u16,
    version: StableVersion,
    evidence: MatchedPolicyEvidenceReference,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizationPolicyRecordSpec {
    pub policy_id: PolicyId,
    pub permission: PermissionReference,
    pub scope: ScopeReference,
    pub effect: PolicyEffect,
    pub authority_level: AuthorityLevel,
    pub layer: AuthorizationPolicyLayer,
    pub mandatory: bool,
    pub active: bool,
    pub approved: bool,
    pub priority: u16,
    pub version: StableVersion,
    pub evidence: MatchedPolicyEvidenceReference,
}

impl AuthorizationPolicyRecord {
    pub fn new(spec: AuthorizationPolicyRecordSpec) -> DomainResult<Self> {
        if matches!(spec.layer, AuthorizationPolicyLayer::ExceptionOrWaiver) && !spec.approved {
            return Err(DomainError::InvalidAuthorizationEvaluation(
                "exception or waiver policy records must be explicitly approved",
            ));
        }
        Ok(Self {
            policy_id: spec.policy_id,
            permission: spec.permission,
            scope: spec.scope,
            effect: spec.effect,
            authority_level: spec.authority_level,
            layer: spec.layer,
            mandatory: spec.mandatory,
            active: spec.active,
            approved: spec.approved,
            priority: spec.priority,
            version: spec.version,
            evidence: spec.evidence,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizationAuthorityRequirement {
    authority_id: DecisionAuthorityId,
    required_level: AuthorityLevel,
    supplied_level: AuthorityLevel,
    evidence: MatchedPolicyEvidenceReference,
}

impl AuthorizationAuthorityRequirement {
    pub fn new(
        authority_id: DecisionAuthorityId,
        required_level: AuthorityLevel,
        supplied_level: AuthorityLevel,
        evidence: MatchedPolicyEvidenceReference,
    ) -> Self {
        Self {
            authority_id,
            required_level,
            supplied_level,
            evidence,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizationDelegationBinding {
    delegate_principal_id: PrincipalId,
    delegation: DelegationReference,
    maximum_depth: DelegationDepth,
    evidence: MatchedPolicyEvidenceReference,
}

impl AuthorizationDelegationBinding {
    pub fn new(
        delegate_principal_id: PrincipalId,
        delegation: DelegationReference,
        maximum_depth: DelegationDepth,
        evidence: MatchedPolicyEvidenceReference,
    ) -> Self {
        Self {
            delegate_principal_id,
            delegation,
            maximum_depth,
            evidence,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizationDecisionIds {
    decision_id: AuthorizationDecisionId,
    audit_evidence_id: AuditEvidenceId,
    decided_at: TimeReference,
}

impl AuthorizationDecisionIds {
    pub fn new(
        decision_id: AuthorizationDecisionId,
        audit_evidence_id: AuditEvidenceId,
        decided_at: TimeReference,
    ) -> Self {
        Self {
            decision_id,
            audit_evidence_id,
            decided_at,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionConstructionInput {
    pub decision_id: DecisionId,
    pub decision_type: DecisionType,
    pub authority_id: DecisionAuthorityId,
    pub authority_level: AuthorityLevel,
    pub owner: DecisionOwnerReference,
    pub context: DecisionContextReference,
    pub created_at: TimeReference,
    pub allow_rationale: DecisionRationaleReference,
    pub deny_rationale: DecisionRationaleReference,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizationEvaluationContext {
    pub principal_scope: ScopeReference,
    pub subject_binding_valid: bool,
    pub evaluation_order_version: AuthorizationEvaluationOrderVersion,
    pub policy_version: StableVersion,
    pub role_permissions: Vec<AuthorizationRolePermissionRecord>,
    pub direct_grants: Vec<AuthorizationGrantRecord>,
    pub inherited_grants: Vec<AuthorizationGrantRecord>,
    pub explicit_denials: Vec<AuthorizationExplicitDenyRecord>,
    pub policies: Vec<AuthorizationPolicyRecord>,
    pub authority_requirement: AuthorizationAuthorityRequirement,
    pub separation_of_duties_conflicts: Vec<SeparationOfDutiesConflict>,
    pub delegation_binding: Option<AuthorizationDelegationBinding>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizationEvaluationInput {
    pub request: AuthorizationRequestRecord,
    pub context: AuthorizationEvaluationContext,
    pub decision_ids: AuthorizationDecisionIds,
    pub decision_construction: Option<DecisionConstructionInput>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthorizationRejectionReason {
    InvalidPrincipal,
    PrincipalInactive,
    InvalidSubjectRelationship,
    InvalidScope,
    TenantIsolationViolation,
    InvalidPermission,
    PermissionMismatch,
    ExplicitDenialMatched,
    PolicyVersionMissing,
    EvidenceIncomplete,
    AuthorityInsufficient,
    SeparationOfDutiesConflict,
    InvalidDelegationBound,
    DenyByDefault,
    UnsupportedDeferredSemantics,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScopeValidationResult {
    pub passed: bool,
    pub rejection_reason: Option<AuthorizationRejectionReason>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PermissionMatchResult {
    pub matched: bool,
    pub matched_policies: Vec<PolicyId>,
    pub matched_evidence: Vec<MatchedPolicyEvidenceReference>,
    pub rejection_reason: Option<AuthorizationRejectionReason>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyMatchResult {
    pub effect: PolicyEffect,
    pub policy_id: Option<PolicyId>,
    pub matched_evidence: Vec<MatchedPolicyEvidenceReference>,
    pub rejection_reason: Option<AuthorizationRejectionReason>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorityValidationResult {
    pub passed: bool,
    pub authority_id: DecisionAuthorityId,
    pub required_level: AuthorityLevel,
    pub supplied_level: AuthorityLevel,
    pub evidence: MatchedPolicyEvidenceReference,
    pub rejection_reason: Option<AuthorizationRejectionReason>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeparationOfDutiesResult {
    pub passed: bool,
    pub conflict: Option<SeparationOfDutiesConflict>,
    pub rejection_reason: Option<AuthorizationRejectionReason>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DelegationBoundResult {
    pub passed: bool,
    pub evidence: Option<MatchedPolicyEvidenceReference>,
    pub rejection_reason: Option<AuthorizationRejectionReason>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizationEvaluationStepResult {
    pub step: AuthorizationEvaluationStep,
    pub passed: bool,
    pub decisive: bool,
    pub rejection_reason: Option<AuthorizationRejectionReason>,
    pub evidence: Vec<MatchedPolicyEvidenceReference>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizationEvaluationTrace {
    pub evaluation_order_version: AuthorizationEvaluationOrderVersion,
    pub steps: Vec<AuthorizationEvaluationStepResult>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizationEvaluationResult {
    pub outcome: AuthorizationDecisionOutcome,
    pub rejection_reason: Option<AuthorizationRejectionReason>,
    pub decision: AuthorizationDecisionReference,
    pub audit_evidence: AuthorizationAuditEvidenceReference,
    pub decision_record: Option<DecisionRecord>,
    pub scope_validation: ScopeValidationResult,
    pub permission_match: PermissionMatchResult,
    pub policy_match: PolicyMatchResult,
    pub authority_validation: AuthorityValidationResult,
    pub separation_of_duties: SeparationOfDutiesResult,
    pub delegation_bound: DelegationBoundResult,
    pub trace: AuthorizationEvaluationTrace,
}

pub fn evaluate_authorization(
    input: &AuthorizationEvaluationInput,
) -> DomainResult<AuthorizationEvaluationResult> {
    if input.context.policy_version.as_str().is_empty() {
        return Err(DomainError::InvalidAuthorizationEvaluation(
            "policy version must be explicit",
        ));
    }
    let principal = input.request.requester().principal();
    let mut trace_steps = Vec::with_capacity(AuthorizationEvaluationStep::ordered().len());

    let principal_step = validate_principal(principal, input.context.subject_binding_valid);
    let mut decisive = None::<(
        AuthorizationDecisionOutcome,
        AuthorizationRejectionReason,
        PolicyId,
        Vec<MatchedPolicyEvidenceReference>,
    )>;
    push_step(
        &mut trace_steps,
        AuthorizationEvaluationStep::VerifyPrincipalIdentityAndLifecycle,
        principal_step.is_ok(),
        principal_step.as_ref().err().cloned(),
        Vec::new(),
        false,
    );
    if let Err(reason) = principal_step {
        decisive = Some((
            AuthorizationDecisionOutcome::DenyValidation,
            reason,
            fallback_policy_id(input)?,
            vec![input.context.authority_requirement.evidence.clone()],
        ));
    }

    let scope_validation = if decisive.is_none() {
        validate_scope(
            input.request.requester(),
            input.request.target(),
            &input.context.principal_scope,
        )
    } else {
        ScopeValidationResult {
            passed: false,
            rejection_reason: None,
        }
    };
    if decisive.is_none() {
        push_step(
            &mut trace_steps,
            AuthorizationEvaluationStep::VerifyTenantIsolationAndScopeLineage,
            scope_validation.passed,
            scope_validation.rejection_reason.clone(),
            Vec::new(),
            false,
        );
        if let Some(reason) = scope_validation.rejection_reason.clone() {
            decisive = Some((
                AuthorizationDecisionOutcome::DenyScope,
                reason,
                fallback_policy_id(input)?,
                vec![input.context.authority_requirement.evidence.clone()],
            ));
        }
    } else {
        push_step(
            &mut trace_steps,
            AuthorizationEvaluationStep::VerifyTenantIsolationAndScopeLineage,
            false,
            None,
            Vec::new(),
            false,
        );
    }

    let explicit_deny_match = if decisive.is_none() {
        find_explicit_deny(input)?
    } else {
        None
    };
    push_step(
        &mut trace_steps,
        AuthorizationEvaluationStep::ResolveExplicitDenials,
        explicit_deny_match.is_none(),
        explicit_deny_match
            .as_ref()
            .map(|_| AuthorizationRejectionReason::ExplicitDenialMatched),
        explicit_deny_match
            .as_ref()
            .map(|record| vec![record.evidence.clone()])
            .unwrap_or_default(),
        false,
    );
    if decisive.is_none() {
        if let Some(deny) = explicit_deny_match {
            decisive = Some((
                AuthorizationDecisionOutcome::DenyExplicit,
                AuthorizationRejectionReason::ExplicitDenialMatched,
                deny.policy_id.clone(),
                vec![deny.evidence.clone()],
            ));
        }
    }

    let direct_grants = if decisive.is_none() {
        matching_grants(
            input.request.requester().principal(),
            input.request.requested_permission(),
            input.request.target(),
            &input.context.direct_grants,
            &input.context.role_permissions,
            false,
        )
    } else {
        Vec::new()
    };
    push_step(
        &mut trace_steps,
        AuthorizationEvaluationStep::ResolveDirectGrants,
        !direct_grants.is_empty(),
        None,
        direct_grants
            .iter()
            .map(|grant| grant.evidence.clone())
            .collect(),
        false,
    );

    let inherited_grants = if decisive.is_none() {
        matching_grants(
            input.request.requester().principal(),
            input.request.requested_permission(),
            input.request.target(),
            &input.context.inherited_grants,
            &input.context.role_permissions,
            true,
        )
    } else {
        Vec::new()
    };
    push_step(
        &mut trace_steps,
        AuthorizationEvaluationStep::ResolveInheritedGrants,
        !inherited_grants.is_empty(),
        None,
        inherited_grants
            .iter()
            .map(|grant| grant.evidence.clone())
            .collect(),
        false,
    );

    let delegation_bound = if decisive.is_none() {
        validate_delegation_bound(input)?
    } else {
        DelegationBoundResult {
            passed: true,
            evidence: None,
            rejection_reason: None,
        }
    };

    let policy_match = if decisive.is_none() {
        resolve_policies(input)?
    } else {
        PolicyMatchResult {
            effect: PolicyEffect::NotApplicable,
            policy_id: None,
            matched_evidence: Vec::new(),
            rejection_reason: None,
        }
    };
    let authority_validation = validate_authority(&input.context.authority_requirement);
    let permission_match = if decisive.is_none() {
        resolve_permission_match(
            input.request.requested_permission(),
            &direct_grants,
            &inherited_grants,
            &policy_match,
            &delegation_bound,
            &authority_validation,
        )
    } else {
        PermissionMatchResult {
            matched: false,
            matched_policies: Vec::new(),
            matched_evidence: Vec::new(),
            rejection_reason: None,
        }
    };

    let permission_step_reason = permission_match
        .rejection_reason
        .clone()
        .or_else(|| policy_match.rejection_reason.clone())
        .or_else(|| delegation_bound.rejection_reason.clone())
        .or_else(|| authority_validation.rejection_reason.clone());
    let mut permission_step_evidence = permission_match.matched_evidence.clone();
    if let Some(evidence) = &delegation_bound.evidence {
        permission_step_evidence.push(evidence.clone());
    }
    permission_step_evidence.push(input.context.authority_requirement.evidence.clone());
    push_step(
        &mut trace_steps,
        AuthorizationEvaluationStep::ResolveRequestedPermissionMatch,
        permission_match.matched,
        permission_step_reason.clone(),
        permission_step_evidence.clone(),
        false,
    );
    if decisive.is_none() {
        if let Some(reason) = permission_step_reason {
            decisive = Some((
                map_rejection_to_outcome(&reason),
                reason,
                permission_match
                    .matched_policies
                    .first()
                    .cloned()
                    .or_else(|| policy_match.policy_id.clone())
                    .unwrap_or_else(|| fallback_policy_id(input).expect("fallback policy id")),
                permission_step_evidence.clone(),
            ));
        }
    }

    let sod = if decisive.is_none() {
        validate_sod(&input.context.separation_of_duties_conflicts)
    } else {
        SeparationOfDutiesResult {
            passed: true,
            conflict: None,
            rejection_reason: None,
        }
    };
    push_step(
        &mut trace_steps,
        AuthorizationEvaluationStep::ApplySeparationOfDutiesConflicts,
        sod.passed,
        sod.rejection_reason.clone(),
        Vec::new(),
        false,
    );
    if decisive.is_none() {
        if let Some(reason) = sod.rejection_reason.clone() {
            decisive = Some((
                AuthorizationDecisionOutcome::DenySeparationOfDuties,
                reason,
                permission_match
                    .matched_policies
                    .first()
                    .cloned()
                    .or_else(|| policy_match.policy_id.clone())
                    .unwrap_or_else(|| fallback_policy_id(input).expect("fallback policy id")),
                permission_match.matched_evidence.clone(),
            ));
        }
    }

    let (outcome, rejection_reason, decisive_policy_id, evidence) = if let Some(decisive) = decisive
    {
        decisive
    } else {
        (
            AuthorizationDecisionOutcome::Allow,
            AuthorizationRejectionReason::DenyByDefault,
            permission_match
                .matched_policies
                .first()
                .cloned()
                .or_else(|| policy_match.policy_id.clone())
                .unwrap_or_else(|| fallback_policy_id(input).expect("fallback policy id")),
            permission_match.matched_evidence.clone(),
        )
    };
    let rejection_reason = if matches!(outcome, AuthorizationDecisionOutcome::Allow) {
        None
    } else {
        Some(rejection_reason)
    };

    let decisive_evidence =
        evidence
            .first()
            .cloned()
            .ok_or(DomainError::MissingAuthorizationEvidence(
                "authorization evaluation requires decisive evidence",
            ))?;
    let all_evidence = dedupe_evidence(evidence);
    let decision = AuthorizationDecisionReference::new(
        input.decision_ids.decision_id.clone(),
        input.request.request_id().clone(),
        decisive_policy_id.clone(),
        outcome,
        input.context.evaluation_order_version.clone(),
        decisive_evidence.clone(),
        input.decision_ids.decided_at.as_str(),
    )?;
    let audit_evidence = AuthorizationAuditEvidenceReference::new(
        input.decision_ids.audit_evidence_id.clone(),
        input.decision_ids.decision_id.clone(),
        principal.principal_id().clone(),
        input.request.target().scope().scope_id().clone(),
        input.context.policy_version.clone(),
        all_evidence.clone(),
        outcome,
    )?;

    let decision_record = input
        .decision_construction
        .as_ref()
        .map(|construction| {
            build_decision_record(
                construction,
                input.request.request_id(),
                input.request.target().scope(),
                &permission_match.matched_policies,
                outcome,
                input.decision_ids.decided_at.as_str(),
            )
        })
        .transpose()?;

    mark_decisive_step(
        &mut trace_steps,
        match outcome {
            AuthorizationDecisionOutcome::DenyExplicit => {
                AuthorizationEvaluationStep::ResolveExplicitDenials
            }
            AuthorizationDecisionOutcome::DenyScope
            | AuthorizationDecisionOutcome::DenyValidation => {
                AuthorizationEvaluationStep::VerifyTenantIsolationAndScopeLineage
            }
            AuthorizationDecisionOutcome::DenySeparationOfDuties => {
                AuthorizationEvaluationStep::ApplySeparationOfDutiesConflicts
            }
            AuthorizationDecisionOutcome::Allow | AuthorizationDecisionOutcome::Deny => {
                AuthorizationEvaluationStep::ResolveRequestedPermissionMatch
            }
        },
    );
    push_step(
        &mut trace_steps,
        AuthorizationEvaluationStep::EmitFinalDecisionAndEvidence,
        true,
        rejection_reason.clone(),
        all_evidence.clone(),
        true,
    );

    Ok(AuthorizationEvaluationResult {
        outcome,
        rejection_reason,
        decision,
        audit_evidence,
        decision_record,
        scope_validation,
        permission_match,
        policy_match,
        authority_validation,
        separation_of_duties: sod,
        delegation_bound,
        trace: AuthorizationEvaluationTrace {
            evaluation_order_version: input.context.evaluation_order_version.clone(),
            steps: trace_steps,
        },
    })
}

fn validate_principal(
    principal: &AuthorizationPrincipalReference,
    subject_binding_valid: bool,
) -> Result<(), AuthorizationRejectionReason> {
    if !subject_binding_valid {
        return Err(AuthorizationRejectionReason::InvalidSubjectRelationship);
    }
    if principal.bound_identity_id().is_empty() {
        return Err(AuthorizationRejectionReason::InvalidPrincipal);
    }
    if principal.lifecycle_state().as_str() != "Active" {
        return Err(AuthorizationRejectionReason::PrincipalInactive);
    }
    if principal.credential_status().as_str() != "Valid" {
        return Err(AuthorizationRejectionReason::InvalidPrincipal);
    }
    Ok(())
}

fn validate_scope(
    subject: &AuthorizationSubject,
    target: &AuthorizationTarget,
    principal_scope: &ScopeReference,
) -> ScopeValidationResult {
    if subject.enterprise_id() != target.scope().enterprise_id()
        || subject.enterprise_id() != principal_scope.enterprise_id()
    {
        return ScopeValidationResult {
            passed: false,
            rejection_reason: Some(AuthorizationRejectionReason::TenantIsolationViolation),
        };
    }
    if !scope_contains(principal_scope, target.scope()) {
        return ScopeValidationResult {
            passed: false,
            rejection_reason: Some(AuthorizationRejectionReason::InvalidScope),
        };
    }
    if matches!(target.scope().level(), ScopeLevel::Resource)
        && target.scope().resource_id() != Some(target.resource_identifier())
    {
        return ScopeValidationResult {
            passed: false,
            rejection_reason: Some(AuthorizationRejectionReason::InvalidScope),
        };
    }
    ScopeValidationResult {
        passed: true,
        rejection_reason: None,
    }
}

fn find_explicit_deny(
    input: &AuthorizationEvaluationInput,
) -> DomainResult<Option<AuthorizationExplicitDenyRecord>> {
    let principal = input.request.requester().principal();
    let request_permission = input.request.requested_permission();
    let target = input.request.target();
    let mut matches = input
        .context
        .explicit_denials
        .iter()
        .filter(|record| {
            record.active
                && record.principal_id == *principal.principal_id()
                && permission_matches(&record.permission, request_permission, target)
                && scope_contains(&record.scope, target.scope())
        })
        .cloned()
        .collect::<Vec<_>>();
    matches.sort_by(|left, right| left.evidence.as_str().cmp(right.evidence.as_str()));
    Ok(matches.into_iter().next())
}

fn matching_grants(
    principal: &AuthorizationPrincipalReference,
    requested_permission: &PermissionReference,
    target: &AuthorizationTarget,
    grants: &[AuthorizationGrantRecord],
    role_permissions: &[AuthorizationRolePermissionRecord],
    require_inheritable: bool,
) -> Vec<AuthorizationGrantRecord> {
    let mut matches = grants
        .iter()
        .filter(|grant| {
            grant.active()
                && grant.principal_id() == principal.principal_id()
                && (!require_inheritable || grant.inheritable())
                && scope_contains(grant.scope(), target.scope())
                && grant_matches_permission(grant, requested_permission, target, role_permissions)
        })
        .cloned()
        .collect::<Vec<_>>();
    matches.sort_by(|left, right| left.evidence.as_str().cmp(right.evidence.as_str()));
    matches
}

fn grant_matches_permission(
    grant: &AuthorizationGrantRecord,
    requested_permission: &PermissionReference,
    target: &AuthorizationTarget,
    role_permissions: &[AuthorizationRolePermissionRecord],
) -> bool {
    if let Some(permission) = grant.permission() {
        return permission_matches(permission, requested_permission, target);
    }
    let Some(role) = grant.role() else {
        return false;
    };
    role_permissions.iter().any(|binding| {
        binding.active()
            && binding.role() == role
            && permission_matches(binding.permission(), requested_permission, target)
    })
}

fn resolve_policies(input: &AuthorizationEvaluationInput) -> DomainResult<PolicyMatchResult> {
    let requested_permission = input.request.requested_permission();
    let target = input.request.target();
    let mut applicable = input
        .context
        .policies
        .iter()
        .filter(|policy| {
            policy.active
                && policy.approved
                && permission_matches(&policy.permission, requested_permission, target)
                && scope_contains(&policy.scope, target.scope())
        })
        .cloned()
        .collect::<Vec<_>>();
    if applicable.is_empty() {
        return Ok(PolicyMatchResult {
            effect: PolicyEffect::NotApplicable,
            policy_id: None,
            matched_evidence: Vec::new(),
            rejection_reason: None,
        });
    }
    applicable.sort_by(compare_policies);
    let primary = applicable.first().expect("non-empty");
    if applicable.len() > 1 {
        let secondary = &applicable[1];
        if compare_policies(primary, secondary) == Ordering::Equal
            && primary.effect != secondary.effect
        {
            return Ok(PolicyMatchResult {
                effect: PolicyEffect::Indeterminate,
                policy_id: Some(primary.policy_id.clone()),
                matched_evidence: vec![primary.evidence.clone(), secondary.evidence.clone()],
                rejection_reason: Some(AuthorizationRejectionReason::DenyByDefault),
            });
        }
    }
    let rejection_reason = match primary.effect {
        PolicyEffect::Permit | PolicyEffect::NotApplicable => None,
        PolicyEffect::Deny => Some(AuthorizationRejectionReason::PermissionMismatch),
        PolicyEffect::Indeterminate => Some(AuthorizationRejectionReason::DenyByDefault),
    };
    Ok(PolicyMatchResult {
        effect: primary.effect,
        policy_id: Some(primary.policy_id.clone()),
        matched_evidence: vec![primary.evidence.clone()],
        rejection_reason,
    })
}

fn compare_policies(
    left: &AuthorizationPolicyRecord,
    right: &AuthorizationPolicyRecord,
) -> Ordering {
    authority_rank(right.authority_level)
        .cmp(&authority_rank(left.authority_level))
        .then_with(|| right.mandatory.cmp(&left.mandatory))
        .then_with(|| policy_layer_rank(left.layer).cmp(&policy_layer_rank(right.layer)))
        .then_with(|| policy_effect_rank(right.effect).cmp(&policy_effect_rank(left.effect)))
        .then_with(|| {
            scope_specificity_rank(right.scope.level())
                .cmp(&scope_specificity_rank(left.scope.level()))
        })
        .then_with(|| right.version.as_str().cmp(left.version.as_str()))
        .then_with(|| right.priority.cmp(&left.priority))
        .then_with(|| left.evidence.as_str().cmp(right.evidence.as_str()))
}

fn validate_authority(
    requirement: &AuthorizationAuthorityRequirement,
) -> AuthorityValidationResult {
    let passed =
        authority_rank(requirement.supplied_level) >= authority_rank(requirement.required_level);
    AuthorityValidationResult {
        passed,
        authority_id: requirement.authority_id.clone(),
        required_level: requirement.required_level,
        supplied_level: requirement.supplied_level,
        evidence: requirement.evidence.clone(),
        rejection_reason: (!passed).then_some(AuthorizationRejectionReason::AuthorityInsufficient),
    }
}

fn validate_delegation_bound(
    input: &AuthorizationEvaluationInput,
) -> DomainResult<DelegationBoundResult> {
    let Some(binding) = &input.context.delegation_binding else {
        return Ok(DelegationBoundResult {
            passed: true,
            evidence: None,
            rejection_reason: None,
        });
    };
    let principal = input.request.requester().principal();
    if principal.principal_type()
        != crate::authorization::AuthorizationPrincipalType::DelegatedAgent
    {
        return Ok(DelegationBoundResult {
            passed: true,
            evidence: None,
            rejection_reason: None,
        });
    }
    if *principal.principal_id() != binding.delegate_principal_id {
        return Ok(DelegationBoundResult {
            passed: false,
            evidence: Some(binding.evidence.clone()),
            rejection_reason: Some(AuthorizationRejectionReason::InvalidDelegationBound),
        });
    }
    if binding.delegation.lifecycle() != DelegationLifecycle::Active {
        return Ok(DelegationBoundResult {
            passed: false,
            evidence: Some(binding.evidence.clone()),
            rejection_reason: Some(AuthorizationRejectionReason::InvalidDelegationBound),
        });
    }
    if binding.delegation.depth().value() > binding.maximum_depth.value() {
        return Ok(DelegationBoundResult {
            passed: false,
            evidence: Some(binding.evidence.clone()),
            rejection_reason: Some(AuthorizationRejectionReason::InvalidDelegationBound),
        });
    }
    if binding.delegation.separation_of_duties().is_some() {
        return Ok(DelegationBoundResult {
            passed: false,
            evidence: Some(binding.evidence.clone()),
            rejection_reason: Some(AuthorizationRejectionReason::SeparationOfDutiesConflict),
        });
    }
    if !delegation_scope_contains(binding.delegation.scope(), input.request.target().scope()) {
        return Ok(DelegationBoundResult {
            passed: false,
            evidence: Some(binding.evidence.clone()),
            rejection_reason: Some(AuthorizationRejectionReason::InvalidDelegationBound),
        });
    }
    let permission_allowed = binding.delegation.delegated_rights().iter().any(|right| {
        permission_matches(
            right.permission(),
            input.request.requested_permission(),
            input.request.target(),
        )
    });
    if !permission_allowed {
        return Ok(DelegationBoundResult {
            passed: false,
            evidence: Some(binding.evidence.clone()),
            rejection_reason: Some(AuthorizationRejectionReason::InvalidDelegationBound),
        });
    }
    Ok(DelegationBoundResult {
        passed: true,
        evidence: Some(binding.evidence.clone()),
        rejection_reason: None,
    })
}

fn resolve_permission_match(
    requested_permission: &PermissionReference,
    direct_grants: &[AuthorizationGrantRecord],
    inherited_grants: &[AuthorizationGrantRecord],
    policy_match: &PolicyMatchResult,
    delegation_bound: &DelegationBoundResult,
    authority_validation: &AuthorityValidationResult,
) -> PermissionMatchResult {
    let matched_grants = direct_grants
        .iter()
        .chain(inherited_grants.iter())
        .collect::<Vec<_>>();
    let mut matched_evidence = matched_grants
        .iter()
        .map(|grant| grant.evidence.clone())
        .collect::<Vec<_>>();
    matched_evidence.extend(policy_match.matched_evidence.clone());
    let mut matched_policies = matched_grants
        .iter()
        .map(|grant| grant.policy_id.clone())
        .collect::<Vec<_>>();
    if let Some(policy_id) = &policy_match.policy_id {
        matched_policies.push(policy_id.clone());
    }
    if !delegation_bound.passed {
        return PermissionMatchResult {
            matched: false,
            matched_policies,
            matched_evidence,
            rejection_reason: delegation_bound.rejection_reason.clone(),
        };
    }
    if !authority_validation.passed {
        return PermissionMatchResult {
            matched: false,
            matched_policies,
            matched_evidence,
            rejection_reason: authority_validation.rejection_reason.clone(),
        };
    }
    if let Some(reason) = policy_match.rejection_reason.clone() {
        return PermissionMatchResult {
            matched: false,
            matched_policies,
            matched_evidence,
            rejection_reason: Some(reason),
        };
    }
    if matched_grants.is_empty() && !matches!(policy_match.effect, PolicyEffect::Permit) {
        return PermissionMatchResult {
            matched: false,
            matched_policies,
            matched_evidence,
            rejection_reason: Some(
                if requested_permission.effect_intent().as_str() == "Permit" {
                    AuthorizationRejectionReason::PermissionMismatch
                } else {
                    AuthorizationRejectionReason::InvalidPermission
                },
            ),
        };
    }
    PermissionMatchResult {
        matched: true,
        matched_policies,
        matched_evidence,
        rejection_reason: None,
    }
}

fn validate_sod(conflicts: &[SeparationOfDutiesConflict]) -> SeparationOfDutiesResult {
    if let Some(conflict) = conflicts.first().cloned() {
        return SeparationOfDutiesResult {
            passed: false,
            conflict: Some(conflict),
            rejection_reason: Some(AuthorizationRejectionReason::SeparationOfDutiesConflict),
        };
    }
    SeparationOfDutiesResult {
        passed: true,
        conflict: None,
        rejection_reason: None,
    }
}

fn build_decision_record(
    input: &DecisionConstructionInput,
    request_id: &crate::identifier::AuthorizationRequestId,
    scope: &ScopeReference,
    matched_policy_ids: &[PolicyId],
    outcome: AuthorizationDecisionOutcome,
    decided_at: &str,
) -> DomainResult<DecisionRecord> {
    let policy_ids = if matched_policy_ids.is_empty() {
        return Err(DomainError::MissingAuthorizationEvidence(
            "decision construction requires at least one matched policy id",
        ));
    } else {
        matched_policy_ids.to_vec()
    };
    let status = if matches!(outcome, AuthorizationDecisionOutcome::Allow) {
        DecisionStatus::Approved
    } else {
        DecisionStatus::Rejected
    };
    let rationale = if matches!(outcome, AuthorizationDecisionOutcome::Allow) {
        Some(input.allow_rationale.clone())
    } else {
        Some(input.deny_rationale.clone())
    };
    DecisionRecord::new(DecisionRecordSpec {
        decision_id: input.decision_id.clone(),
        decision_type: input.decision_type,
        authority_id: input.authority_id.clone(),
        authority_level: input.authority_level,
        owner: input.owner.clone(),
        status,
        scope: scope.clone(),
        context: input.context.clone(),
        policy_set: DecisionPolicySetReference::new(policy_ids)?,
        subject: DecisionSubjectReference::AuthorizationRequest(request_id.clone()),
        rationale,
        created_at: input.created_at.as_str().to_owned(),
        decided_at: Some(decided_at.to_owned()),
    })
}

fn permission_matches(
    candidate: &PermissionReference,
    requested: &PermissionReference,
    target: &AuthorizationTarget,
) -> bool {
    candidate.permission_id() == requested.permission_id()
        && candidate.action_verb() == requested.action_verb()
        && candidate.resource_type() == requested.resource_type()
        && requested.resource_type() == target.resource_type()
        && candidate.effect_intent().as_str() == "Permit"
}

fn scope_contains(parent: &ScopeReference, child: &ScopeReference) -> bool {
    if parent.enterprise_id() != child.enterprise_id() {
        return false;
    }
    let parent_path = parent.ownership_path();
    let child_path = child.ownership_path();
    if parent_path.workspace_id().is_some()
        && parent_path.workspace_id() != child_path.workspace_id()
    {
        return false;
    }
    if parent_path.project_id().is_some() && parent_path.project_id() != child_path.project_id() {
        return false;
    }
    if parent_path.organizational_unit_id().is_some()
        && parent_path.organizational_unit_id() != child_path.organizational_unit_id()
    {
        return false;
    }
    match parent.level() {
        ScopeLevel::Enterprise => true,
        ScopeLevel::Workspace => child_path.workspace_id().is_some(),
        ScopeLevel::Project => child_path.project_id().is_some(),
        ScopeLevel::OrganizationalUnit => child_path.organizational_unit_id().is_some(),
        ScopeLevel::Resource => parent.resource_id() == child.resource_id(),
    }
}

fn delegation_scope_contains(
    parent: &crate::delegation::DelegationScope,
    child: &ScopeReference,
) -> bool {
    if parent.ownership_path().enterprise_id() != child.enterprise_id() {
        return false;
    }
    let parent_path = parent.ownership_path();
    let child_path = child.ownership_path();
    if parent_path.workspace_id().is_some()
        && parent_path.workspace_id() != child_path.workspace_id()
    {
        return false;
    }
    if parent_path.project_id().is_some() && parent_path.project_id() != child_path.project_id() {
        return false;
    }
    if parent_path.organizational_unit_id().is_some()
        && parent_path.organizational_unit_id() != child_path.organizational_unit_id()
    {
        return false;
    }
    true
}

fn authority_rank(level: AuthorityLevel) -> u8 {
    match level {
        AuthorityLevel::Observer => 0,
        AuthorityLevel::Operator => 1,
        AuthorityLevel::Specialist => 2,
        AuthorityLevel::Manager => 3,
        AuthorityLevel::Director => 4,
        AuthorityLevel::Executive => 5,
        AuthorityLevel::Founder => 6,
    }
}

fn scope_specificity_rank(level: ScopeLevel) -> u8 {
    match level {
        ScopeLevel::Enterprise => 0,
        ScopeLevel::Workspace => 1,
        ScopeLevel::Project => 2,
        ScopeLevel::OrganizationalUnit => 3,
        ScopeLevel::Resource => 4,
    }
}

fn policy_layer_rank(layer: AuthorizationPolicyLayer) -> u8 {
    match layer {
        AuthorizationPolicyLayer::MandatoryEnterprise => 0,
        AuthorizationPolicyLayer::Scoped => 1,
        AuthorizationPolicyLayer::ExceptionOrWaiver => 2,
    }
}

fn policy_effect_rank(effect: PolicyEffect) -> u8 {
    match effect {
        PolicyEffect::Deny => 3,
        PolicyEffect::Permit => 2,
        PolicyEffect::Indeterminate => 1,
        PolicyEffect::NotApplicable => 0,
    }
}

fn map_rejection_to_outcome(reason: &AuthorizationRejectionReason) -> AuthorizationDecisionOutcome {
    match reason {
        AuthorizationRejectionReason::ExplicitDenialMatched => {
            AuthorizationDecisionOutcome::DenyExplicit
        }
        AuthorizationRejectionReason::InvalidScope
        | AuthorizationRejectionReason::TenantIsolationViolation => {
            AuthorizationDecisionOutcome::DenyScope
        }
        AuthorizationRejectionReason::SeparationOfDutiesConflict => {
            AuthorizationDecisionOutcome::DenySeparationOfDuties
        }
        _ => AuthorizationDecisionOutcome::Deny,
    }
}

fn fallback_policy_id(input: &AuthorizationEvaluationInput) -> DomainResult<PolicyId> {
    input
        .context
        .policies
        .first()
        .map(|policy| policy.policy_id.clone())
        .or_else(|| {
            input
                .context
                .direct_grants
                .first()
                .map(|grant| grant.policy_id.clone())
        })
        .or_else(|| {
            input
                .context
                .inherited_grants
                .first()
                .map(|grant| grant.policy_id.clone())
        })
        .or_else(|| {
            input
                .context
                .explicit_denials
                .first()
                .map(|deny| deny.policy_id.clone())
        })
        .ok_or(DomainError::MissingAuthorizationEvidence(
            "authorization evaluation requires at least one policy-backed rule reference",
        ))
}

fn dedupe_evidence(
    evidence: Vec<MatchedPolicyEvidenceReference>,
) -> Vec<MatchedPolicyEvidenceReference> {
    let mut seen = BTreeSet::new();
    let mut deduped = Vec::new();
    for reference in evidence {
        if seen.insert(reference.as_str().to_owned()) {
            deduped.push(reference);
        }
    }
    deduped
}

fn push_step(
    steps: &mut Vec<AuthorizationEvaluationStepResult>,
    step: AuthorizationEvaluationStep,
    passed: bool,
    rejection_reason: Option<AuthorizationRejectionReason>,
    evidence: Vec<MatchedPolicyEvidenceReference>,
    decisive: bool,
) {
    steps.push(AuthorizationEvaluationStepResult {
        step,
        passed,
        decisive,
        rejection_reason,
        evidence,
    });
}

fn mark_decisive_step(
    steps: &mut [AuthorizationEvaluationStepResult],
    target: AuthorizationEvaluationStep,
) {
    if let Some(step) = steps.iter_mut().find(|step| step.step == target) {
        step.decisive = true;
    }
}

#[cfg(test)]
mod tests {
    use super::{
        evaluate_authorization, AuthorizationAuthorityRequirement, AuthorizationDecisionIds,
        AuthorizationDelegationBinding, AuthorizationEvaluationContext,
        AuthorizationEvaluationInput, AuthorizationExplicitDenyRecord, AuthorizationGrantRecord,
        AuthorizationGrantRecordSpec, AuthorizationPolicyLayer, AuthorizationPolicyRecord,
        AuthorizationPolicyRecordSpec, AuthorizationRejectionReason,
        AuthorizationRolePermissionRecord, DecisionConstructionInput,
    };
    use crate::agent::AgentReference;
    use crate::authorization::{
        ActionVerb, AuthorityLevel, AuthorizationDecisionOutcome,
        AuthorizationEvaluationOrderVersion, AuthorizationPrincipalReference,
        AuthorizationPrincipalType, AuthorizationSubject, AuthorizationTarget,
        CredentialStatusReference, MatchedPolicyEvidenceReference, PermissionEffectIntent,
        PermissionReference, PrincipalLifecycleStateReference, ResourceType, RoleReference,
        ScopeLevel, ScopeReference,
    };
    use crate::decision::{
        DecisionContextReference, DecisionOwnerReference, DecisionRationaleReference, DecisionType,
    };
    use crate::delegation::{
        AuthoritySourceReference, BeneficiaryReference, DelegateReference, DelegatedRightReference,
        DelegationDepth, DelegationReference, DelegationReferenceSpec, DelegationScope,
        DelegationScopeKind, DelegationVersion, DelegatorReference, PolicyResultReference,
        SeparationOfDutiesConflict,
    };
    use crate::identifier::{
        AgentId, AuditEvidenceId, AuthorizationDecisionId, AuthorizationRequestId,
        DecisionAuthorityId, DecisionId, DelegationId, EnglishNamespace, EnterpriseId, HumanId,
        PermissionId, PolicyId, PrincipalId, ProjectId, RoleId, ScopeId, StableVersion,
        WorkspaceId,
    };
    use crate::identity::AgentIdentity;
    use crate::lifecycle::DelegationLifecycle;
    use crate::ownership::OwnershipPath;
    use crate::policy::PolicyEffect;
    use crate::request::{AuthorizationRequestRecord, TimeReference};

    fn enterprise_id() -> EnterpriseId {
        EnterpriseId::new("CX-ENT-000001").expect("enterprise")
    }

    fn principal_id() -> PrincipalId {
        PrincipalId::new("CX-PRN-000001").expect("principal")
    }

    fn delegated_principal_id() -> PrincipalId {
        PrincipalId::new("CX-PRN-000009").expect("principal")
    }

    fn decision_ids() -> AuthorizationDecisionIds {
        AuthorizationDecisionIds::new(
            AuthorizationDecisionId::new("CX-AUTHDEC-000001").expect("decision"),
            AuditEvidenceId::new("CX-AUD-000001").expect("audit"),
            TimeReference::new("2026-07-14T10:00:05Z").expect("time"),
        )
    }

    fn permission(id: &str, action: &str, resource_type: &str) -> PermissionReference {
        PermissionReference::new(
            PermissionId::new(id).expect("permission"),
            ActionVerb::new(action).expect("action"),
            ResourceType::new(resource_type).expect("type"),
            PermissionEffectIntent::new("Permit").expect("effect"),
        )
    }

    fn role() -> RoleReference {
        RoleReference::new(
            RoleId::new("CX-ROLE-000001").expect("role"),
            enterprise_id(),
        )
    }

    fn project_scope(scope_id: &str, project: &str) -> ScopeReference {
        ScopeReference::new(
            ScopeId::new(scope_id).expect("scope"),
            ScopeLevel::Project,
            OwnershipPath::new(
                enterprise_id(),
                Some(WorkspaceId::new("CX-WS-000001").expect("workspace")),
                Some(ProjectId::new(project).expect("project")),
                None,
            )
            .expect("path"),
            None,
        )
        .expect("scope")
    }

    fn workspace_scope(scope_id: &str) -> ScopeReference {
        ScopeReference::new(
            ScopeId::new(scope_id).expect("scope"),
            ScopeLevel::Workspace,
            OwnershipPath::new(
                enterprise_id(),
                Some(WorkspaceId::new("CX-WS-000001").expect("workspace")),
                None,
                None,
            )
            .expect("path"),
            None,
        )
        .expect("scope")
    }

    fn target(project: &str) -> AuthorizationTarget {
        AuthorizationTarget::new(
            ResourceType::new("workflow").expect("type"),
            "WF-001",
            project_scope("CX-SCP-000101", project),
        )
        .expect("target")
    }

    fn subject_with(
        principal_type: AuthorizationPrincipalType,
        lifecycle: &str,
    ) -> AuthorizationSubject {
        AuthorizationSubject::Principal(
            AuthorizationPrincipalReference::new(
                if matches!(principal_type, AuthorizationPrincipalType::DelegatedAgent) {
                    delegated_principal_id()
                } else {
                    principal_id()
                },
                principal_type,
                "CX-EMP-000001",
                enterprise_id(),
                PrincipalLifecycleStateReference::new(lifecycle).expect("lifecycle"),
                CredentialStatusReference::new("Valid").expect("credential"),
            )
            .expect("principal"),
        )
    }

    fn request_with(
        principal_type: AuthorizationPrincipalType,
        lifecycle: &str,
        requested_permission: PermissionReference,
        target: AuthorizationTarget,
    ) -> AuthorizationRequestRecord {
        AuthorizationRequestRecord::new(
            AuthorizationRequestId::new("CX-AUTHREQ-000001").expect("request"),
            subject_with(principal_type, lifecycle),
            requested_permission,
            target,
            TimeReference::new("2026-07-14T10:00:00Z").expect("time"),
            "authorize governed action",
        )
        .expect("request")
    }

    fn decision_construction() -> DecisionConstructionInput {
        DecisionConstructionInput {
            decision_id: DecisionId::new("CX-DEC-000001").expect("decision"),
            decision_type: DecisionType::Security,
            authority_id: DecisionAuthorityId::new("CX-DECAUTH-000001").expect("authority"),
            authority_level: AuthorityLevel::Manager,
            owner: DecisionOwnerReference::Human(HumanId::new("CX-EMP-000001").expect("owner")),
            context: DecisionContextReference::new("authorization evaluation context")
                .expect("context"),
            created_at: TimeReference::new("2026-07-14T09:59:59Z").expect("time"),
            allow_rationale: DecisionRationaleReference::new("authorization permitted")
                .expect("rationale"),
            deny_rationale: DecisionRationaleReference::new("authorization denied")
                .expect("rationale"),
        }
    }

    fn base_context() -> AuthorizationEvaluationContext {
        AuthorizationEvaluationContext {
            principal_scope: workspace_scope("CX-SCP-000010"),
            subject_binding_valid: true,
            evaluation_order_version: AuthorizationEvaluationOrderVersion::new("v1")
                .expect("version"),
            policy_version: StableVersion::new("policy_version", "2026.07.14").expect("version"),
            role_permissions: vec![AuthorizationRolePermissionRecord::new(
                role(),
                permission("CX-PERM-000001", "approve", "workflow"),
                MatchedPolicyEvidenceReference::new("EVID-ROLE-001").expect("evidence"),
                true,
            )],
            direct_grants: vec![],
            inherited_grants: vec![],
            explicit_denials: vec![],
            policies: vec![],
            authority_requirement: AuthorizationAuthorityRequirement::new(
                DecisionAuthorityId::new("CX-DECAUTH-000001").expect("authority"),
                AuthorityLevel::Manager,
                AuthorityLevel::Manager,
                MatchedPolicyEvidenceReference::new("EVID-AUTH-001").expect("evidence"),
            ),
            separation_of_duties_conflicts: vec![],
            delegation_binding: None,
        }
    }

    fn policy_record(
        effect: PolicyEffect,
        priority: u16,
        evidence: &str,
    ) -> AuthorizationPolicyRecord {
        AuthorizationPolicyRecord::new(AuthorizationPolicyRecordSpec {
            policy_id: PolicyId::new("CX-POL-000001").expect("policy"),
            permission: permission("CX-PERM-000001", "approve", "workflow"),
            scope: workspace_scope("CX-SCP-000011"),
            effect,
            authority_level: AuthorityLevel::Manager,
            layer: AuthorizationPolicyLayer::Scoped,
            mandatory: false,
            active: true,
            approved: true,
            priority,
            version: StableVersion::new("policy_version", "2026.07.14").expect("version"),
            evidence: MatchedPolicyEvidenceReference::new(evidence).expect("evidence"),
        })
        .expect("policy")
    }

    fn direct_permission_grant() -> AuthorizationGrantRecord {
        AuthorizationGrantRecord::new(AuthorizationGrantRecordSpec {
            policy_id: PolicyId::new("CX-POL-000001").expect("policy"),
            principal_id: principal_id(),
            scope: workspace_scope("CX-SCP-000012"),
            role: None,
            permission: Some(permission("CX-PERM-000001", "approve", "workflow")),
            inheritable: false,
            active: true,
            evidence: MatchedPolicyEvidenceReference::new("EVID-GRANT-001").expect("evidence"),
        })
        .expect("grant")
    }

    fn role_grant(inheritable: bool) -> AuthorizationGrantRecord {
        AuthorizationGrantRecord::new(AuthorizationGrantRecordSpec {
            policy_id: PolicyId::new("CX-POL-000001").expect("policy"),
            principal_id: principal_id(),
            scope: workspace_scope("CX-SCP-000013"),
            role: Some(role()),
            permission: None,
            inheritable,
            active: true,
            evidence: MatchedPolicyEvidenceReference::new("EVID-GRANT-ROLE-001").expect("evidence"),
        })
        .expect("grant")
    }

    fn explicit_deny() -> AuthorizationExplicitDenyRecord {
        AuthorizationExplicitDenyRecord::new(
            PolicyId::new("CX-POL-000002").expect("policy"),
            principal_id(),
            permission("CX-PERM-000001", "approve", "workflow"),
            workspace_scope("CX-SCP-000014"),
            true,
            true,
            MatchedPolicyEvidenceReference::new("EVID-DENY-001").expect("evidence"),
        )
    }

    fn active_delegation_binding() -> AuthorizationDelegationBinding {
        let agent = AgentReference::new(
            AgentIdentity::new(
                AgentId::new("CX-AGT-000001").expect("agent"),
                EnglishNamespace::new("agent_namespace", "kernel.security").expect("namespace"),
                StableVersion::new("agent_version", "1.0.0").expect("version"),
                enterprise_id(),
                crate::lifecycle::AgentLifecycle::Registered,
            )
            .expect("agent identity"),
        );
        let auth_decision = crate::authorization::AuthorizationDecisionReference::new(
            AuthorizationDecisionId::new("CX-AUTHDEC-000010").expect("decision"),
            AuthorizationRequestId::new("CX-AUTHREQ-000010").expect("request"),
            PolicyId::new("CX-POL-000010").expect("policy"),
            AuthorizationDecisionOutcome::Allow,
            AuthorizationEvaluationOrderVersion::new("v1").expect("version"),
            MatchedPolicyEvidenceReference::new("EVID-AUTH-DECISION-001").expect("evidence"),
            "2026-07-14T09:00:00Z",
        )
        .expect("decision");
        let authority_source = AuthoritySourceReference::new(
            PolicyResultReference::new(
                PolicyId::new("CX-POL-000010").expect("policy"),
                PolicyEffect::Permit,
                false,
                false,
            )
            .expect("policy result"),
            auth_decision,
        )
        .expect("authority source");
        AuthorizationDelegationBinding::new(
            delegated_principal_id(),
            DelegationReference::new(DelegationReferenceSpec {
                delegation_id: DelegationId::new("CX-DEL-000001").expect("delegation"),
                namespace: EnglishNamespace::new("delegation_namespace", "kernel.authorization")
                    .expect("namespace"),
                version: DelegationVersion::new("1.0.0").expect("version"),
                delegator: DelegatorReference::new(agent.clone()),
                delegate: DelegateReference::new(agent.clone()),
                beneficiary: BeneficiaryReference::Delegate(DelegateReference::new(agent)),
                authority_source,
                scope: DelegationScope::new(
                    DelegationScopeKind::Workspace,
                    OwnershipPath::new(
                        enterprise_id(),
                        Some(WorkspaceId::new("CX-WS-000001").expect("workspace")),
                        None,
                        None,
                    )
                    .expect("path"),
                    None,
                )
                .expect("scope"),
                delegated_rights: vec![DelegatedRightReference::new(
                    permission("CX-PERM-000001", "approve", "workflow"),
                    "review and approve workflow",
                )
                .expect("right")],
                delegated_tasks: vec![],
                conditions: vec![],
                depth: DelegationDepth::new(1).expect("depth"),
                lifecycle: DelegationLifecycle::Active,
                separation_of_duties: None,
            })
            .expect("delegation"),
            DelegationDepth::new(1).expect("depth"),
            MatchedPolicyEvidenceReference::new("EVID-DEL-001").expect("evidence"),
        )
    }

    #[test]
    fn authorization_valid_principal_permits_when_other_requirements_pass_ces_b0_026_1() {
        let mut context = base_context();
        context.direct_grants.push(direct_permission_grant());
        context
            .policies
            .push(policy_record(PolicyEffect::Permit, 1, "EVID-POL-001"));
        let input = AuthorizationEvaluationInput {
            request: request_with(
                AuthorizationPrincipalType::Employee,
                "Active",
                permission("CX-PERM-000001", "approve", "workflow"),
                target("CX-PROJ-000001"),
            ),
            context,
            decision_ids: decision_ids(),
            decision_construction: Some(decision_construction()),
        };
        let result = evaluate_authorization(&input).expect("evaluation");
        assert_eq!(result.outcome, AuthorizationDecisionOutcome::Allow);
        assert!(result.decision_record.is_some());
    }

    #[test]
    fn authorization_rejects_inactive_principal_ces_b0_026_1() {
        let mut context = base_context();
        context.direct_grants.push(direct_permission_grant());
        context
            .policies
            .push(policy_record(PolicyEffect::Permit, 1, "EVID-POL-001"));
        let result = evaluate_authorization(&AuthorizationEvaluationInput {
            request: request_with(
                AuthorizationPrincipalType::Employee,
                "Suspended",
                permission("CX-PERM-000001", "approve", "workflow"),
                target("CX-PROJ-000001"),
            ),
            context,
            decision_ids: decision_ids(),
            decision_construction: None,
        })
        .expect("evaluation");
        assert_eq!(
            result.rejection_reason,
            Some(AuthorizationRejectionReason::PrincipalInactive)
        );
    }

    #[test]
    fn authorization_rejects_invalid_subject_principal_relationship_ces_b0_026_1() {
        let mut context = base_context();
        context.subject_binding_valid = false;
        context.direct_grants.push(direct_permission_grant());
        context
            .policies
            .push(policy_record(PolicyEffect::Permit, 1, "EVID-POL-001"));
        let result = evaluate_authorization(&AuthorizationEvaluationInput {
            request: request_with(
                AuthorizationPrincipalType::Employee,
                "Active",
                permission("CX-PERM-000001", "approve", "workflow"),
                target("CX-PROJ-000001"),
            ),
            context,
            decision_ids: decision_ids(),
            decision_construction: None,
        })
        .expect("evaluation");
        assert_eq!(
            result.rejection_reason,
            Some(AuthorizationRejectionReason::InvalidSubjectRelationship)
        );
    }

    #[test]
    fn authorization_allows_same_enterprise_scope_when_requirements_pass_ces_b0_026_3() {
        let mut context = base_context();
        context.direct_grants.push(direct_permission_grant());
        context
            .policies
            .push(policy_record(PolicyEffect::Permit, 1, "EVID-POL-001"));
        let result = evaluate_authorization(&AuthorizationEvaluationInput {
            request: request_with(
                AuthorizationPrincipalType::Employee,
                "Active",
                permission("CX-PERM-000001", "approve", "workflow"),
                target("CX-PROJ-000001"),
            ),
            context,
            decision_ids: decision_ids(),
            decision_construction: None,
        })
        .expect("evaluation");
        assert_eq!(result.outcome, AuthorizationDecisionOutcome::Allow);
    }

    #[test]
    fn authorization_rejects_cross_enterprise_scope_ces_b0_026_3() {
        let mut context = base_context();
        context.direct_grants.push(direct_permission_grant());
        context
            .policies
            .push(policy_record(PolicyEffect::Permit, 1, "EVID-POL-001"));
        let cross_target = AuthorizationTarget::new(
            ResourceType::new("workflow").expect("type"),
            "WF-001",
            ScopeReference::new(
                ScopeId::new("CX-SCP-000201").expect("scope"),
                ScopeLevel::Project,
                OwnershipPath::new(
                    EnterpriseId::new("CX-ENT-000002").expect("enterprise"),
                    Some(WorkspaceId::new("CX-WS-000001").expect("workspace")),
                    Some(ProjectId::new("CX-PROJ-000001").expect("project")),
                    None,
                )
                .expect("path"),
                None,
            )
            .expect("scope"),
        )
        .expect("target");
        let request = AuthorizationRequestRecord::new(
            AuthorizationRequestId::new("CX-AUTHREQ-000001").expect("request"),
            subject_with(AuthorizationPrincipalType::Employee, "Active"),
            permission("CX-PERM-000001", "approve", "workflow"),
            cross_target,
            TimeReference::new("2026-07-14T10:00:00Z").expect("time"),
            "cross-tenant request",
        );
        assert!(request.is_err());
    }

    #[test]
    fn authorization_rejects_workspace_project_scope_mismatch_ces_b0_026_3() {
        let mut context = base_context();
        context.principal_scope = project_scope("CX-SCP-000300", "CX-PROJ-000002");
        context.direct_grants.push(direct_permission_grant());
        context
            .policies
            .push(policy_record(PolicyEffect::Permit, 1, "EVID-POL-001"));
        let result = evaluate_authorization(&AuthorizationEvaluationInput {
            request: request_with(
                AuthorizationPrincipalType::Employee,
                "Active",
                permission("CX-PERM-000001", "approve", "workflow"),
                target("CX-PROJ-000001"),
            ),
            context,
            decision_ids: decision_ids(),
            decision_construction: None,
        })
        .expect("evaluation");
        assert_eq!(
            result.rejection_reason,
            Some(AuthorizationRejectionReason::InvalidScope)
        );
    }

    #[test]
    fn authorization_rejects_target_outside_permission_scope_ces_b0_026_3() {
        let mut context = base_context();
        context.direct_grants.push(
            AuthorizationGrantRecord::new(AuthorizationGrantRecordSpec {
                policy_id: PolicyId::new("CX-POL-000001").expect("policy"),
                principal_id: principal_id(),
                scope: project_scope("CX-SCP-000301", "CX-PROJ-000002"),
                role: None,
                permission: Some(permission("CX-PERM-000001", "approve", "workflow")),
                inheritable: false,
                active: true,
                evidence: MatchedPolicyEvidenceReference::new("EVID-GRANT-002").expect("evidence"),
            })
            .expect("grant"),
        );
        context
            .policies
            .push(policy_record(PolicyEffect::Permit, 1, "EVID-POL-001"));
        let result = evaluate_authorization(&AuthorizationEvaluationInput {
            request: request_with(
                AuthorizationPrincipalType::Employee,
                "Active",
                permission("CX-PERM-000001", "approve", "workflow"),
                target("CX-PROJ-000001"),
            ),
            context,
            decision_ids: decision_ids(),
            decision_construction: None,
        })
        .expect("evaluation");
        assert_eq!(
            result.rejection_reason,
            Some(AuthorizationRejectionReason::PermissionMismatch)
        );
    }

    #[test]
    fn authorization_matches_action_resource_and_scope_ces_b0_026_2() {
        let mut context = base_context();
        context.direct_grants.push(direct_permission_grant());
        context
            .policies
            .push(policy_record(PolicyEffect::Permit, 1, "EVID-POL-001"));
        let result = evaluate_authorization(&AuthorizationEvaluationInput {
            request: request_with(
                AuthorizationPrincipalType::Employee,
                "Active",
                permission("CX-PERM-000001", "approve", "workflow"),
                target("CX-PROJ-000001"),
            ),
            context,
            decision_ids: decision_ids(),
            decision_construction: None,
        })
        .expect("evaluation");
        assert!(result.permission_match.matched);
    }

    #[test]
    fn authorization_rejects_action_mismatch_ces_b0_026_2() {
        let mut context = base_context();
        context.direct_grants.push(direct_permission_grant());
        context
            .policies
            .push(policy_record(PolicyEffect::Permit, 1, "EVID-POL-001"));
        let result = evaluate_authorization(&AuthorizationEvaluationInput {
            request: request_with(
                AuthorizationPrincipalType::Employee,
                "Active",
                permission("CX-PERM-000001", "delete", "workflow"),
                target("CX-PROJ-000001"),
            ),
            context,
            decision_ids: decision_ids(),
            decision_construction: None,
        })
        .expect("evaluation");
        assert_eq!(
            result.rejection_reason,
            Some(AuthorizationRejectionReason::PermissionMismatch)
        );
    }

    #[test]
    fn authorization_rejects_resource_mismatch_ces_b0_026_2() {
        let mut context = base_context();
        context.direct_grants.push(direct_permission_grant());
        context
            .policies
            .push(policy_record(PolicyEffect::Permit, 1, "EVID-POL-001"));
        let result = evaluate_authorization(&AuthorizationEvaluationInput {
            request: request_with(
                AuthorizationPrincipalType::Employee,
                "Active",
                permission("CX-PERM-000001", "approve", "agent"),
                target("CX-PROJ-000001"),
            ),
            context,
            decision_ids: decision_ids(),
            decision_construction: None,
        })
        .expect("evaluation");
        assert_eq!(
            result.rejection_reason,
            Some(AuthorizationRejectionReason::PermissionMismatch)
        );
    }

    #[test]
    fn authorization_rejects_permission_absence_ces_b0_026_2() {
        let mut context = base_context();
        context.policies.push(policy_record(
            PolicyEffect::NotApplicable,
            1,
            "EVID-POL-NA-001",
        ));
        let result = evaluate_authorization(&AuthorizationEvaluationInput {
            request: request_with(
                AuthorizationPrincipalType::Employee,
                "Active",
                permission("CX-PERM-000001", "approve", "workflow"),
                target("CX-PROJ-000001"),
            ),
            context,
            decision_ids: decision_ids(),
            decision_construction: None,
        })
        .expect("evaluation");
        assert_eq!(result.outcome, AuthorizationDecisionOutcome::Deny);
    }

    #[test]
    fn authorization_policy_permit_supports_allow_ces_b0_028_9() {
        let mut context = base_context();
        context
            .policies
            .push(policy_record(PolicyEffect::Permit, 1, "EVID-POL-001"));
        context.direct_grants.push(direct_permission_grant());
        let result = evaluate_authorization(&AuthorizationEvaluationInput {
            request: request_with(
                AuthorizationPrincipalType::Employee,
                "Active",
                permission("CX-PERM-000001", "approve", "workflow"),
                target("CX-PROJ-000001"),
            ),
            context,
            decision_ids: decision_ids(),
            decision_construction: None,
        })
        .expect("evaluation");
        assert_eq!(result.outcome, AuthorizationDecisionOutcome::Allow);
    }

    #[test]
    fn authorization_explicit_deny_wins_ces_b0_028_7() {
        let mut context = base_context();
        context.direct_grants.push(direct_permission_grant());
        context.explicit_denials.push(explicit_deny());
        context
            .policies
            .push(policy_record(PolicyEffect::Permit, 1, "EVID-POL-001"));
        let result = evaluate_authorization(&AuthorizationEvaluationInput {
            request: request_with(
                AuthorizationPrincipalType::Employee,
                "Active",
                permission("CX-PERM-000001", "approve", "workflow"),
                target("CX-PROJ-000001"),
            ),
            context,
            decision_ids: decision_ids(),
            decision_construction: None,
        })
        .expect("evaluation");
        assert_eq!(result.outcome, AuthorizationDecisionOutcome::DenyExplicit);
        assert!(result.trace.steps[2].decisive);
    }

    #[test]
    fn authorization_conflicting_permit_and_deny_resolve_deterministically_ces_b0_028_8() {
        let mut context = base_context();
        context.direct_grants.push(direct_permission_grant());
        context
            .policies
            .push(policy_record(PolicyEffect::Permit, 1, "EVID-POL-001"));
        context
            .policies
            .push(policy_record(PolicyEffect::Deny, 9, "EVID-POL-002"));
        let result = evaluate_authorization(&AuthorizationEvaluationInput {
            request: request_with(
                AuthorizationPrincipalType::Employee,
                "Active",
                permission("CX-PERM-000001", "approve", "workflow"),
                target("CX-PROJ-000001"),
            ),
            context,
            decision_ids: decision_ids(),
            decision_construction: None,
        })
        .expect("evaluation");
        assert_eq!(result.outcome, AuthorizationDecisionOutcome::Deny);
    }

    #[test]
    fn authorization_evaluation_order_is_stable_ces_b0_026_5() {
        let mut context = base_context();
        context.direct_grants.push(direct_permission_grant());
        context
            .policies
            .push(policy_record(PolicyEffect::Permit, 1, "EVID-POL-001"));
        let input = AuthorizationEvaluationInput {
            request: request_with(
                AuthorizationPrincipalType::Employee,
                "Active",
                permission("CX-PERM-000001", "approve", "workflow"),
                target("CX-PROJ-000001"),
            ),
            context,
            decision_ids: decision_ids(),
            decision_construction: None,
        };
        let result = evaluate_authorization(&input).expect("evaluation");
        assert_eq!(
            result
                .trace
                .steps
                .iter()
                .map(|step| step.step)
                .collect::<Vec<_>>(),
            crate::authorization::AuthorizationEvaluationStep::ordered().to_vec()
        );
    }

    #[test]
    fn authorization_accepts_sufficient_authority_ces_b0_022_3() {
        let mut context = base_context();
        context.direct_grants.push(direct_permission_grant());
        context
            .policies
            .push(policy_record(PolicyEffect::Permit, 1, "EVID-POL-001"));
        context.authority_requirement = AuthorizationAuthorityRequirement::new(
            DecisionAuthorityId::new("CX-DECAUTH-000001").expect("authority"),
            AuthorityLevel::Operator,
            AuthorityLevel::Manager,
            MatchedPolicyEvidenceReference::new("EVID-AUTH-001").expect("evidence"),
        );
        let result = evaluate_authorization(&AuthorizationEvaluationInput {
            request: request_with(
                AuthorizationPrincipalType::Employee,
                "Active",
                permission("CX-PERM-000001", "approve", "workflow"),
                target("CX-PROJ-000001"),
            ),
            context,
            decision_ids: decision_ids(),
            decision_construction: None,
        })
        .expect("evaluation");
        assert!(result.authority_validation.passed);
    }

    #[test]
    fn authorization_rejects_insufficient_authority_ces_b0_022_3() {
        let mut context = base_context();
        context.direct_grants.push(direct_permission_grant());
        context
            .policies
            .push(policy_record(PolicyEffect::Permit, 1, "EVID-POL-001"));
        context.authority_requirement = AuthorizationAuthorityRequirement::new(
            DecisionAuthorityId::new("CX-DECAUTH-000001").expect("authority"),
            AuthorityLevel::Executive,
            AuthorityLevel::Manager,
            MatchedPolicyEvidenceReference::new("EVID-AUTH-001").expect("evidence"),
        );
        let result = evaluate_authorization(&AuthorizationEvaluationInput {
            request: request_with(
                AuthorizationPrincipalType::Employee,
                "Active",
                permission("CX-PERM-000001", "approve", "workflow"),
                target("CX-PROJ-000001"),
            ),
            context,
            decision_ids: decision_ids(),
            decision_construction: None,
        })
        .expect("evaluation");
        assert_eq!(
            result.rejection_reason,
            Some(AuthorizationRejectionReason::AuthorityInsufficient)
        );
    }

    #[test]
    fn authorization_does_not_infer_authority_from_role_alone_ces_b0_022_3() {
        let mut context = base_context();
        context.direct_grants.push(role_grant(false));
        context
            .policies
            .push(policy_record(PolicyEffect::Permit, 1, "EVID-POL-001"));
        context.authority_requirement = AuthorizationAuthorityRequirement::new(
            DecisionAuthorityId::new("CX-DECAUTH-000001").expect("authority"),
            AuthorityLevel::Executive,
            AuthorityLevel::Operator,
            MatchedPolicyEvidenceReference::new("EVID-AUTH-001").expect("evidence"),
        );
        let result = evaluate_authorization(&AuthorizationEvaluationInput {
            request: request_with(
                AuthorizationPrincipalType::Employee,
                "Active",
                permission("CX-PERM-000001", "approve", "workflow"),
                target("CX-PROJ-000001"),
            ),
            context,
            decision_ids: decision_ids(),
            decision_construction: None,
        })
        .expect("evaluation");
        assert_eq!(result.outcome, AuthorizationDecisionOutcome::Deny);
    }

    #[test]
    fn authorization_allows_valid_separation_of_duties_context_ces_b0_026_7() {
        let mut context = base_context();
        context.direct_grants.push(direct_permission_grant());
        context
            .policies
            .push(policy_record(PolicyEffect::Permit, 1, "EVID-POL-001"));
        let result = evaluate_authorization(&AuthorizationEvaluationInput {
            request: request_with(
                AuthorizationPrincipalType::Employee,
                "Active",
                permission("CX-PERM-000001", "approve", "workflow"),
                target("CX-PROJ-000001"),
            ),
            context,
            decision_ids: decision_ids(),
            decision_construction: None,
        })
        .expect("evaluation");
        assert!(result.separation_of_duties.passed);
    }

    #[test]
    fn authorization_rejects_separation_of_duties_conflict_ces_b0_026_7() {
        let mut context = base_context();
        context.direct_grants.push(direct_permission_grant());
        context
            .policies
            .push(policy_record(PolicyEffect::Permit, 1, "EVID-POL-001"));
        context.separation_of_duties_conflicts.push(
            SeparationOfDutiesConflict::new("requester and approver must differ")
                .expect("conflict"),
        );
        let result = evaluate_authorization(&AuthorizationEvaluationInput {
            request: request_with(
                AuthorizationPrincipalType::Employee,
                "Active",
                permission("CX-PERM-000001", "approve", "workflow"),
                target("CX-PROJ-000001"),
            ),
            context,
            decision_ids: decision_ids(),
            decision_construction: None,
        })
        .expect("evaluation");
        assert_eq!(
            result.outcome,
            AuthorizationDecisionOutcome::DenySeparationOfDuties
        );
    }

    #[test]
    fn authorization_accepts_active_valid_delegation_bound_ces_b0_029_4() {
        let mut context = base_context();
        context.delegation_binding = Some(active_delegation_binding());
        context.direct_grants.push(direct_permission_grant());
        context
            .policies
            .push(policy_record(PolicyEffect::Permit, 1, "EVID-POL-001"));
        let result = evaluate_authorization(&AuthorizationEvaluationInput {
            request: request_with(
                AuthorizationPrincipalType::DelegatedAgent,
                "Active",
                permission("CX-PERM-000001", "approve", "workflow"),
                target("CX-PROJ-000001"),
            ),
            context,
            decision_ids: decision_ids(),
            decision_construction: None,
        })
        .expect("evaluation");
        assert!(result.delegation_bound.passed);
    }

    #[test]
    fn authorization_rejects_suspended_or_revoked_delegation_ces_b0_029_11() {
        let mut context = base_context();
        let binding = active_delegation_binding();
        let AuthorizationDelegationBinding { delegation, .. } = binding;
        let binding = AuthorizationDelegationBinding::new(
            delegated_principal_id(),
            DelegationReference::new(DelegationReferenceSpec {
                lifecycle: DelegationLifecycle::Suspended,
                ..delegation_spec_from(delegation)
            })
            .expect("delegation"),
            DelegationDepth::new(1).expect("depth"),
            MatchedPolicyEvidenceReference::new("EVID-DEL-002").expect("evidence"),
        );
        context.delegation_binding = Some(binding);
        context.direct_grants.push(direct_permission_grant());
        context
            .policies
            .push(policy_record(PolicyEffect::Permit, 1, "EVID-POL-001"));
        let result = evaluate_authorization(&AuthorizationEvaluationInput {
            request: request_with(
                AuthorizationPrincipalType::DelegatedAgent,
                "Active",
                permission("CX-PERM-000001", "approve", "workflow"),
                target("CX-PROJ-000001"),
            ),
            context,
            decision_ids: decision_ids(),
            decision_construction: None,
        })
        .expect("evaluation");
        assert_eq!(
            result.rejection_reason,
            Some(AuthorizationRejectionReason::InvalidDelegationBound)
        );
    }

    #[test]
    fn authorization_rejects_right_outside_delegated_scope_ces_b0_029_5() {
        let mut context = base_context();
        let binding = active_delegation_binding();
        context.delegation_binding = Some(binding);
        context.direct_grants.push(direct_permission_grant());
        context
            .policies
            .push(policy_record(PolicyEffect::Permit, 1, "EVID-POL-001"));
        let result = evaluate_authorization(&AuthorizationEvaluationInput {
            request: request_with(
                AuthorizationPrincipalType::DelegatedAgent,
                "Active",
                permission("CX-PERM-999999", "approve", "workflow"),
                target("CX-PROJ-000001"),
            ),
            context,
            decision_ids: decision_ids(),
            decision_construction: None,
        })
        .expect("evaluation");
        assert_eq!(
            result.rejection_reason,
            Some(AuthorizationRejectionReason::InvalidDelegationBound)
        );
    }

    #[test]
    fn authorization_rejects_invalid_delegation_depth_ces_b0_029_9() {
        let mut context = base_context();
        let mut spec = delegation_spec_from(active_delegation_binding().delegation);
        spec.depth = DelegationDepth::new(2).expect("depth");
        spec.conditions =
            vec![
                crate::delegation::DelegationConditionReference::new("redelegation approved")
                    .expect("condition"),
            ];
        let binding = AuthorizationDelegationBinding::new(
            delegated_principal_id(),
            DelegationReference::new(spec).expect("delegation"),
            DelegationDepth::new(1).expect("max depth"),
            MatchedPolicyEvidenceReference::new("EVID-DEL-003").expect("evidence"),
        );
        context.delegation_binding = Some(binding);
        context.direct_grants.push(direct_permission_grant());
        context
            .policies
            .push(policy_record(PolicyEffect::Permit, 1, "EVID-POL-001"));
        let result = evaluate_authorization(&AuthorizationEvaluationInput {
            request: request_with(
                AuthorizationPrincipalType::DelegatedAgent,
                "Active",
                permission("CX-PERM-000001", "approve", "workflow"),
                target("CX-PROJ-000001"),
            ),
            context,
            decision_ids: decision_ids(),
            decision_construction: None,
        })
        .expect("evaluation");
        assert_eq!(
            result.rejection_reason,
            Some(AuthorizationRejectionReason::InvalidDelegationBound)
        );
    }

    #[test]
    fn authorization_constructs_permit_decision_ces_b0_026_6() {
        let mut context = base_context();
        context.direct_grants.push(direct_permission_grant());
        context
            .policies
            .push(policy_record(PolicyEffect::Permit, 1, "EVID-POL-001"));
        let result = evaluate_authorization(&AuthorizationEvaluationInput {
            request: request_with(
                AuthorizationPrincipalType::Employee,
                "Active",
                permission("CX-PERM-000001", "approve", "workflow"),
                target("CX-PROJ-000001"),
            ),
            context,
            decision_ids: decision_ids(),
            decision_construction: Some(decision_construction()),
        })
        .expect("evaluation");
        assert_eq!(
            result.decision.outcome(),
            AuthorizationDecisionOutcome::Allow
        );
        assert_eq!(result.decision.request_id().as_str(), "CX-AUTHREQ-000001");
    }

    #[test]
    fn authorization_constructs_deny_decision_ces_b0_026_6() {
        let mut context = base_context();
        context.explicit_denials.push(explicit_deny());
        context
            .policies
            .push(policy_record(PolicyEffect::Permit, 1, "EVID-POL-001"));
        let result = evaluate_authorization(&AuthorizationEvaluationInput {
            request: request_with(
                AuthorizationPrincipalType::Employee,
                "Active",
                permission("CX-PERM-000001", "approve", "workflow"),
                target("CX-PROJ-000001"),
            ),
            context,
            decision_ids: decision_ids(),
            decision_construction: Some(decision_construction()),
        })
        .expect("evaluation");
        assert_eq!(
            result.decision.outcome(),
            AuthorizationDecisionOutcome::DenyExplicit
        );
    }

    #[test]
    fn authorization_is_deterministic_for_identical_inputs_ces_b0_026_6() {
        let mut context = base_context();
        context.direct_grants.push(direct_permission_grant());
        context
            .policies
            .push(policy_record(PolicyEffect::Permit, 1, "EVID-POL-001"));
        let input = AuthorizationEvaluationInput {
            request: request_with(
                AuthorizationPrincipalType::Employee,
                "Active",
                permission("CX-PERM-000001", "approve", "workflow"),
                target("CX-PROJ-000001"),
            ),
            context,
            decision_ids: decision_ids(),
            decision_construction: None,
        };
        let left = evaluate_authorization(&input).expect("left");
        let right = evaluate_authorization(&input).expect("right");
        assert_eq!(left, right);
    }

    #[test]
    fn authorization_trace_marks_decisive_step_ces_b0_026_8() {
        let mut context = base_context();
        context.explicit_denials.push(explicit_deny());
        context
            .policies
            .push(policy_record(PolicyEffect::Permit, 1, "EVID-POL-001"));
        let result = evaluate_authorization(&AuthorizationEvaluationInput {
            request: request_with(
                AuthorizationPrincipalType::Employee,
                "Active",
                permission("CX-PERM-000001", "approve", "workflow"),
                target("CX-PROJ-000001"),
            ),
            context,
            decision_ids: decision_ids(),
            decision_construction: None,
        })
        .expect("evaluation");
        assert_eq!(
            result
                .trace
                .steps
                .iter()
                .filter(|step| step.decisive)
                .count(),
            2
        );
    }

    fn delegation_spec_from(delegation: DelegationReference) -> DelegationReferenceSpec {
        DelegationReferenceSpec {
            delegation_id: delegation.delegation_id().clone(),
            namespace: EnglishNamespace::new("delegation_namespace", "kernel.authorization")
                .expect("namespace"),
            version: DelegationVersion::new("1.0.0").expect("version"),
            delegator: delegation.delegator().clone(),
            delegate: delegation.delegate().clone(),
            beneficiary: BeneficiaryReference::Delegate(delegation.delegate().clone()),
            authority_source: delegation.authority_source().clone(),
            scope: delegation.scope().clone(),
            delegated_rights: delegation.delegated_rights().to_vec(),
            delegated_tasks: vec![],
            conditions: delegation.conditions().to_vec(),
            depth: delegation.depth(),
            lifecycle: delegation.lifecycle(),
            separation_of_duties: delegation.separation_of_duties().cloned(),
        }
    }
}
