use crate::agent::{
    AgentDefinition, AgentFailureReference, AgentRecoveryReference, AgentRuntimeReference,
};
use crate::authorization::PermissionReference;
use crate::errors::{DomainError, DomainResult};
use crate::identifier::{
    AgentId, CapabilityId, HeartbeatId, LeaseId, PolicyId, RuntimeId, StableVersion,
};
use crate::lifecycle::AgentLifecycle;
use crate::ownership::{OrganizationalContext, OwnerReference, OwnershipPath};
use crate::request::TimeReference;
use crate::{EnterpriseId, NonEmptyText};
use std::collections::{BTreeMap, BTreeSet};

/// CES Traceability: CES-B0-027.6, CES-B0-027.10, CES-B0-027.12, CES-B0-027.18
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeHealth {
    Healthy,
    Degraded,
    Critical,
    Unknown,
}

/// CES Traceability: CES-B0-027.8, CES-B0-027.9, CES-B0-027.10, K4.1 runtime presence baseline
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PresenceState {
    Registered,
    Ready,
    Idle,
    Working,
    Paused,
    Recovering,
    Offline,
    Retired,
}

impl PresenceState {
    pub fn can_transition_to(self, target: Self) -> DomainResult<()> {
        let allowed = matches!(
            (self, target),
            (
                Self::Registered,
                Self::Ready | Self::Offline | Self::Retired
            ) | (
                Self::Ready,
                Self::Idle | Self::Paused | Self::Offline | Self::Retired
            ) | (
                Self::Idle,
                Self::Working | Self::Paused | Self::Offline | Self::Retired
            ) | (
                Self::Working,
                Self::Idle | Self::Paused | Self::Recovering | Self::Offline | Self::Retired
            ) | (
                Self::Paused,
                Self::Ready | Self::Idle | Self::Recovering | Self::Offline | Self::Retired
            ) | (
                Self::Recovering,
                Self::Ready | Self::Idle | Self::Offline | Self::Retired
            ) | (Self::Offline, Self::Recovering | Self::Retired)
        );
        if allowed {
            Ok(())
        } else {
            Err(DomainError::InvalidRuntimeReference(
                "presence transition is not allowed by the K4.1 runtime lifecycle map",
            ))
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Retired)
    }
}

/// CES Traceability: CES-B0-027.5
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityDescriptor {
    capability_id: CapabilityId,
    description: NonEmptyText,
    dependencies: Vec<NonEmptyText>,
    inputs: Vec<NonEmptyText>,
    outputs: Vec<NonEmptyText>,
    required_permissions: Vec<PermissionReference>,
    governing_policies: Vec<PolicyId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityDescriptorSpec {
    pub capability_id: CapabilityId,
    pub description: String,
    pub dependencies: Vec<String>,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub required_permissions: Vec<PermissionReference>,
    pub governing_policies: Vec<PolicyId>,
}

impl CapabilityDescriptor {
    pub fn new(spec: CapabilityDescriptorSpec) -> DomainResult<Self> {
        if spec.dependencies.is_empty() || spec.inputs.is_empty() || spec.outputs.is_empty() {
            return Err(DomainError::InvalidRuntimeReference(
                "capability descriptors require dependencies, inputs, and outputs",
            ));
        }
        if spec.required_permissions.is_empty() {
            return Err(DomainError::InvalidRuntimeReference(
                "capability descriptors require at least one permission reference",
            ));
        }
        if spec.governing_policies.is_empty() {
            return Err(DomainError::InvalidRuntimeReference(
                "capability descriptors require at least one governing policy reference",
            ));
        }
        Ok(Self {
            capability_id: spec.capability_id,
            description: NonEmptyText::new("capability_description", spec.description)?,
            dependencies: normalize_text_list("capability_dependency", spec.dependencies)?,
            inputs: normalize_text_list("capability_input", spec.inputs)?,
            outputs: normalize_text_list("capability_output", spec.outputs)?,
            required_permissions: spec.required_permissions,
            governing_policies: spec.governing_policies,
        })
    }

    pub fn capability_id(&self) -> &CapabilityId {
        &self.capability_id
    }

    pub fn governing_policies(&self) -> &[PolicyId] {
        &self.governing_policies
    }
}

/// CES Traceability: CES-B0-027.1, CES-B0-027.13, K4.1 runtime foundation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeEntity {
    runtime_id: RuntimeId,
    runtime_reference: AgentRuntimeReference,
    enterprise_id: EnterpriseId,
    ownership_path: OwnershipPath,
    health: RuntimeHealth,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeEntitySpec {
    pub runtime_id: RuntimeId,
    pub runtime_reference: AgentRuntimeReference,
    pub enterprise_id: EnterpriseId,
    pub ownership_path: OwnershipPath,
    pub health: RuntimeHealth,
}

impl RuntimeEntity {
    pub fn new(spec: RuntimeEntitySpec) -> DomainResult<Self> {
        if spec.enterprise_id != *spec.ownership_path.enterprise_id() {
            return Err(DomainError::InvalidRuntimeReference(
                "runtime entity enterprise must match its ownership path enterprise",
            ));
        }
        Ok(Self {
            runtime_id: spec.runtime_id,
            runtime_reference: spec.runtime_reference,
            enterprise_id: spec.enterprise_id,
            ownership_path: spec.ownership_path,
            health: spec.health,
        })
    }

    pub fn runtime_id(&self) -> &RuntimeId {
        &self.runtime_id
    }

    pub fn runtime_reference(&self) -> &AgentRuntimeReference {
        &self.runtime_reference
    }

    pub fn enterprise_id(&self) -> &EnterpriseId {
        &self.enterprise_id
    }

    pub fn ownership_path(&self) -> &OwnershipPath {
        &self.ownership_path
    }

    pub fn health(&self) -> RuntimeHealth {
        self.health
    }
}

/// CES Traceability: CES-B0-027.7, CES-B0-027.8, CES-B0-027.21
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeaseRecord {
    lease_id: LeaseId,
    runtime_id: RuntimeId,
    issued_at: TimeReference,
    expires_at: TimeReference,
    agent_id: Option<AgentId>,
    supersedes_lease_id: Option<LeaseId>,
    evidence: Option<NonEmptyText>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeaseRecordSpec {
    pub lease_id: LeaseId,
    pub runtime_id: RuntimeId,
    pub issued_at: TimeReference,
    pub expires_at: TimeReference,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeaseIssuanceSpec {
    pub lease_id: LeaseId,
    pub runtime_id: RuntimeId,
    pub agent_id: AgentId,
    pub issued_at: TimeReference,
    pub expires_at: TimeReference,
    pub supersedes_lease_id: Option<LeaseId>,
    pub evidence: String,
}

impl LeaseRecord {
    pub fn new(spec: LeaseRecordSpec) -> DomainResult<Self> {
        validate_lease_window(&spec.issued_at, &spec.expires_at)?;
        Ok(Self {
            lease_id: spec.lease_id,
            runtime_id: spec.runtime_id,
            issued_at: spec.issued_at,
            expires_at: spec.expires_at,
            agent_id: None,
            supersedes_lease_id: None,
            evidence: None,
        })
    }

    pub fn issue(spec: LeaseIssuanceSpec) -> DomainResult<Self> {
        validate_lease_window(&spec.issued_at, &spec.expires_at)?;
        Ok(Self {
            lease_id: spec.lease_id,
            runtime_id: spec.runtime_id,
            issued_at: spec.issued_at,
            expires_at: spec.expires_at,
            agent_id: Some(spec.agent_id),
            supersedes_lease_id: spec.supersedes_lease_id,
            evidence: Some(NonEmptyText::new("lease_evidence", spec.evidence)?),
        })
    }

    pub fn lease_id(&self) -> &LeaseId {
        &self.lease_id
    }

    pub fn runtime_id(&self) -> &RuntimeId {
        &self.runtime_id
    }

    pub fn agent_id(&self) -> Option<&AgentId> {
        self.agent_id.as_ref()
    }

    pub fn issued_at(&self) -> &TimeReference {
        &self.issued_at
    }

    pub fn expires_at(&self) -> &TimeReference {
        &self.expires_at
    }

    pub fn supersedes_lease_id(&self) -> Option<&LeaseId> {
        self.supersedes_lease_id.as_ref()
    }

    pub fn evidence(&self) -> Option<&str> {
        self.evidence.as_ref().map(NonEmptyText::as_str)
    }

    pub fn is_current_at(&self, reference_time: &TimeReference) -> bool {
        self.issued_at.as_str() <= reference_time.as_str()
            && reference_time.as_str() < self.expires_at.as_str()
    }
}

/// CES Traceability: CES-B0-027.6, CES-B0-027.10, CES-B0-027.17, CES-B0-027.21
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeartbeatRecord {
    heartbeat_id: HeartbeatId,
    runtime_id: RuntimeId,
    recorded_at: TimeReference,
    fresh_until: TimeReference,
    agent_id: Option<AgentId>,
    reported_presence: Option<PresenceState>,
    reported_health: Option<RuntimeHealth>,
    active_lease_id: Option<LeaseId>,
    evidence: Option<NonEmptyText>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeartbeatRecordSpec {
    pub heartbeat_id: HeartbeatId,
    pub runtime_id: RuntimeId,
    pub recorded_at: TimeReference,
    pub fresh_until: TimeReference,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeartbeatObservationSpec {
    pub heartbeat_id: HeartbeatId,
    pub runtime_id: RuntimeId,
    pub agent_id: AgentId,
    pub recorded_at: TimeReference,
    pub fresh_until: TimeReference,
    pub reported_presence: PresenceState,
    pub reported_health: RuntimeHealth,
    pub active_lease_id: Option<LeaseId>,
    pub evidence: String,
}

impl HeartbeatRecord {
    pub fn new(spec: HeartbeatRecordSpec) -> DomainResult<Self> {
        validate_heartbeat_window(&spec.recorded_at, &spec.fresh_until)?;
        Ok(Self {
            heartbeat_id: spec.heartbeat_id,
            runtime_id: spec.runtime_id,
            recorded_at: spec.recorded_at,
            fresh_until: spec.fresh_until,
            agent_id: None,
            reported_presence: None,
            reported_health: None,
            active_lease_id: None,
            evidence: None,
        })
    }

    pub fn observe(spec: HeartbeatObservationSpec) -> DomainResult<Self> {
        validate_heartbeat_window(&spec.recorded_at, &spec.fresh_until)?;
        Ok(Self {
            heartbeat_id: spec.heartbeat_id,
            runtime_id: spec.runtime_id,
            recorded_at: spec.recorded_at,
            fresh_until: spec.fresh_until,
            agent_id: Some(spec.agent_id),
            reported_presence: Some(spec.reported_presence),
            reported_health: Some(spec.reported_health),
            active_lease_id: spec.active_lease_id,
            evidence: Some(NonEmptyText::new("heartbeat_evidence", spec.evidence)?),
        })
    }

    pub fn heartbeat_id(&self) -> &HeartbeatId {
        &self.heartbeat_id
    }

    pub fn runtime_id(&self) -> &RuntimeId {
        &self.runtime_id
    }

    pub fn agent_id(&self) -> Option<&AgentId> {
        self.agent_id.as_ref()
    }

    pub fn recorded_at(&self) -> &TimeReference {
        &self.recorded_at
    }

    pub fn fresh_until(&self) -> &TimeReference {
        &self.fresh_until
    }

    pub fn reported_presence(&self) -> Option<PresenceState> {
        self.reported_presence
    }

    pub fn reported_health(&self) -> Option<RuntimeHealth> {
        self.reported_health
    }

    pub fn active_lease_id(&self) -> Option<&LeaseId> {
        self.active_lease_id.as_ref()
    }

    pub fn evidence(&self) -> Option<&str> {
        self.evidence.as_ref().map(NonEmptyText::as_str)
    }

    pub fn is_fresh_at(&self, reference_time: &TimeReference) -> bool {
        reference_time.as_str() <= self.fresh_until.as_str()
    }
}

/// CES Traceability: CES-B0-027.10, CES-B0-027.21
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeartbeatFreshness {
    Fresh,
    Late,
    Stale,
    Missing,
}

/// CES Traceability: CES-B0-027.10, CES-B0-027.21
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeartbeatFreshnessPolicy {
    late_cutoff: TimeReference,
    stale_cutoff: TimeReference,
    rule_version: StableVersion,
}

impl HeartbeatFreshnessPolicy {
    pub fn new(
        late_cutoff: TimeReference,
        stale_cutoff: TimeReference,
        rule_version: StableVersion,
    ) -> DomainResult<Self> {
        if stale_cutoff.as_str() > late_cutoff.as_str() {
            return Err(DomainError::InvalidRuntimeReference(
                "stale cutoff must not be later than the late cutoff",
            ));
        }
        Ok(Self {
            late_cutoff,
            stale_cutoff,
            rule_version,
        })
    }

    pub fn late_cutoff(&self) -> &TimeReference {
        &self.late_cutoff
    }

    pub fn stale_cutoff(&self) -> &TimeReference {
        &self.stale_cutoff
    }

    pub fn rule_version(&self) -> &StableVersion {
        &self.rule_version
    }
}

/// CES Traceability: CES-B0-027.10, CES-B0-027.21
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeartbeatAssessment {
    freshness: HeartbeatFreshness,
    evaluated_at: TimeReference,
    heartbeat_id: Option<HeartbeatId>,
}

impl HeartbeatAssessment {
    pub fn freshness(&self) -> HeartbeatFreshness {
        self.freshness
    }

    pub fn heartbeat_id(&self) -> Option<&HeartbeatId> {
        self.heartbeat_id.as_ref()
    }

    pub fn evaluated_at(&self) -> &TimeReference {
        &self.evaluated_at
    }
}

/// CES Traceability: CES-B0-027.10, CES-B0-027.21
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeartbeatRecordOutcome {
    HeartbeatAccepted,
    HeartbeatRejected,
    HeartbeatDuplicate,
    HeartbeatStale,
    HeartbeatRuntimeMismatch,
    HeartbeatAgentMismatch,
    HeartbeatLeaseMismatch,
    HeartbeatTimestampRegression,
    HeartbeatRuntimeRetired,
}

/// CES Traceability: CES-B0-027.10, CES-B0-027.21
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeartbeatUpdateResult {
    outcome: HeartbeatRecordOutcome,
    assessment: HeartbeatAssessment,
    latest_heartbeat: Option<HeartbeatRecord>,
}

impl HeartbeatUpdateResult {
    pub fn outcome(&self) -> HeartbeatRecordOutcome {
        self.outcome
    }

    pub fn assessment(&self) -> &HeartbeatAssessment {
        &self.assessment
    }

    pub fn latest_heartbeat(&self) -> Option<&HeartbeatRecord> {
        self.latest_heartbeat.as_ref()
    }
}

/// CES Traceability: CES-B0-027.7, CES-B0-027.21
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LeaseValidity {
    Valid,
    Expired,
    Missing,
    Invalid,
}

/// CES Traceability: CES-B0-027.7, CES-B0-027.21
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LeaseRejectionReason {
    LeaseMissing,
    LeaseExpired,
    LeaseRuntimeMismatch,
    LeaseAgentMismatch,
    LeaseRenewalRejected,
    LeaseSequenceRegression,
    RuntimeAlreadyRetired,
}

/// CES Traceability: CES-B0-027.7, CES-B0-027.21
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeaseAssessment {
    validity: LeaseValidity,
    evaluated_at: TimeReference,
    lease_id: Option<LeaseId>,
    rejection_reason: Option<LeaseRejectionReason>,
}

impl LeaseAssessment {
    pub fn validity(&self) -> LeaseValidity {
        self.validity
    }

    pub fn rejection_reason(&self) -> Option<LeaseRejectionReason> {
        self.rejection_reason
    }

    pub fn lease_id(&self) -> Option<&LeaseId> {
        self.lease_id.as_ref()
    }
}

/// CES Traceability: CES-B0-027.7
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeasePolicy {
    allow_renewal_after_expiration: bool,
    rule_version: StableVersion,
}

impl LeasePolicy {
    pub fn new(allow_renewal_after_expiration: bool, rule_version: StableVersion) -> Self {
        Self {
            allow_renewal_after_expiration,
            rule_version,
        }
    }

    pub fn allow_renewal_after_expiration(&self) -> bool {
        self.allow_renewal_after_expiration
    }

    pub fn rule_version(&self) -> &StableVersion {
        &self.rule_version
    }
}

/// CES Traceability: CES-B0-027.7
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeaseRenewalRequest {
    prior_lease_id: LeaseId,
    renewed_lease: LeaseRecord,
    requested_at: TimeReference,
}

impl LeaseRenewalRequest {
    pub fn new(
        prior_lease_id: LeaseId,
        renewed_lease: LeaseRecord,
        requested_at: TimeReference,
    ) -> Self {
        Self {
            prior_lease_id,
            renewed_lease,
            requested_at,
        }
    }

    pub fn prior_lease_id(&self) -> &LeaseId {
        &self.prior_lease_id
    }

    pub fn renewed_lease(&self) -> &LeaseRecord {
        &self.renewed_lease
    }

    pub fn requested_at(&self) -> &TimeReference {
        &self.requested_at
    }
}

/// CES Traceability: CES-B0-027.7
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LeaseRenewalOutcome {
    LeaseRenewed,
    LeaseDuplicate,
    LeaseExpired,
    LeaseRuntimeMismatch,
    LeaseAgentMismatch,
    LeaseRenewalRejected,
    LeaseSequenceRegression,
    RuntimeAlreadyRetired,
}

/// CES Traceability: CES-B0-027.7
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeaseRenewalResult {
    outcome: LeaseRenewalOutcome,
    assessment: LeaseAssessment,
    active_lease: Option<LeaseRecord>,
}

impl LeaseRenewalResult {
    pub fn outcome(&self) -> LeaseRenewalOutcome {
        self.outcome
    }

    pub fn assessment(&self) -> &LeaseAssessment {
        &self.assessment
    }

    pub fn active_lease(&self) -> Option<&LeaseRecord> {
        self.active_lease.as_ref()
    }
}

/// CES Traceability: CES-B0-027.18
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeFailureObservation {
    failure: AgentFailureReference,
    observed_at: TimeReference,
    evidence: NonEmptyText,
}

impl RuntimeFailureObservation {
    pub fn new(
        failure: AgentFailureReference,
        observed_at: TimeReference,
        evidence: impl Into<String>,
    ) -> DomainResult<Self> {
        Ok(Self {
            failure,
            observed_at,
            evidence: NonEmptyText::new("runtime_failure_evidence", evidence)?,
        })
    }

    pub fn failure(&self) -> &AgentFailureReference {
        &self.failure
    }

    pub fn evidence(&self) -> &str {
        self.evidence.as_str()
    }
}

/// CES Traceability: CES-B0-027.19
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryRejectionReason {
    MissingRecoveryEvidence,
    UnderlyingFailureUnresolved,
    LeaseInvalid,
    RuntimeAlreadyRetired,
}

/// CES Traceability: CES-B0-027.19
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryEligibility {
    eligible: bool,
    rejection_reason: Option<RecoveryRejectionReason>,
    recovery_reference: Option<AgentRecoveryReference>,
}

impl RecoveryEligibility {
    pub fn eligible(&self) -> bool {
        self.eligible
    }

    pub fn rejection_reason(&self) -> Option<RecoveryRejectionReason> {
        self.rejection_reason
    }

    pub fn recovery_reference(&self) -> Option<&AgentRecoveryReference> {
        self.recovery_reference.as_ref()
    }
}

/// CES Traceability: CES-B0-027.15, CES-B0-027.21
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeStateSnapshot {
    runtime_id: RuntimeId,
    agent_id: AgentId,
    agent_lifecycle: AgentLifecycle,
    presence: PresenceState,
    health: RuntimeHealth,
    current_lease_id: Option<LeaseId>,
    latest_heartbeat_id: Option<HeartbeatId>,
    heartbeat_freshness: HeartbeatFreshness,
    capability_ids: Vec<CapabilityId>,
    observed_at: TimeReference,
    failure_observation: Option<RuntimeFailureObservation>,
    recovery_reference: Option<AgentRecoveryReference>,
    lease_assessment: LeaseAssessment,
}

impl RuntimeStateSnapshot {
    pub fn runtime_id(&self) -> &RuntimeId {
        &self.runtime_id
    }

    pub fn agent_id(&self) -> &AgentId {
        &self.agent_id
    }

    pub fn presence(&self) -> PresenceState {
        self.presence
    }

    pub fn health(&self) -> RuntimeHealth {
        self.health
    }

    pub fn heartbeat_freshness(&self) -> HeartbeatFreshness {
        self.heartbeat_freshness
    }

    pub fn lease_assessment(&self) -> &LeaseAssessment {
        &self.lease_assessment
    }
}

/// CES Traceability: CES-B0-027.10, CES-B0-027.18, CES-B0-027.19
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupervisorAction {
    NoAction,
    MarkLate,
    MarkUnresponsive,
    TransitionOffline,
    SuspendRuntime,
    RequestRecovery,
    RejectRecovery,
    RetireRuntime,
}

/// CES Traceability: CES-B0-027.10, CES-B0-027.18, CES-B0-027.19
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupervisorStep {
    ObserveRuntimeState,
    AssessHeartbeatFreshness,
    AssessLeaseValidity,
    AssessRuntimeHealth,
    AssessFailureEvidence,
    AssessRecoveryEligibility,
    RecommendSupervisorAction,
}

/// CES Traceability: CES-B0-027.10, CES-B0-027.18, CES-B0-027.19
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupervisorEvidenceReference(NonEmptyText);

impl SupervisorEvidenceReference {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        Ok(Self(NonEmptyText::new(
            "supervisor_evidence_reference",
            value,
        )?))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

/// CES Traceability: CES-B0-027.10, CES-B0-027.18, CES-B0-027.19
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupervisorTraceStep {
    step: SupervisorStep,
    passed: bool,
    decisive: bool,
    evidence: Vec<SupervisorEvidenceReference>,
}

impl SupervisorTraceStep {
    pub fn step(&self) -> SupervisorStep {
        self.step
    }

    pub fn decisive(&self) -> bool {
        self.decisive
    }
}

/// CES Traceability: CES-B0-027.10, CES-B0-027.18, CES-B0-027.19
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupervisorTrace {
    steps: Vec<SupervisorTraceStep>,
}

impl SupervisorTrace {
    pub fn steps(&self) -> &[SupervisorTraceStep] {
        &self.steps
    }
}

/// CES Traceability: CES-B0-027.10, CES-B0-027.18, CES-B0-027.19
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupervisorObservation {
    snapshot: RuntimeStateSnapshot,
    heartbeat_assessment: HeartbeatAssessment,
    lease_assessment: LeaseAssessment,
    recovery_eligibility: RecoveryEligibility,
}

impl SupervisorObservation {
    pub fn snapshot(&self) -> &RuntimeStateSnapshot {
        &self.snapshot
    }
}

/// CES Traceability: CES-B0-027.10, CES-B0-027.18, CES-B0-027.19
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupervisorOutcome {
    runtime_id: RuntimeId,
    agent_id: AgentId,
    observation: SupervisorObservation,
    recommended_action: SupervisorAction,
    proposed_presence: PresenceState,
    resulting_health: RuntimeHealth,
    trace: SupervisorTrace,
}

impl SupervisorOutcome {
    pub fn runtime_id(&self) -> &RuntimeId {
        &self.runtime_id
    }

    pub fn agent_id(&self) -> &AgentId {
        &self.agent_id
    }

    pub fn recommended_action(&self) -> SupervisorAction {
        self.recommended_action
    }

    pub fn proposed_presence(&self) -> PresenceState {
        self.proposed_presence
    }

    pub fn resulting_health(&self) -> RuntimeHealth {
        self.resulting_health
    }

    pub fn trace(&self) -> &SupervisorTrace {
        &self.trace
    }
}

/// CES Traceability: CES-B0-027.8, CES-B0-027.9, CES-B0-027.15, CES-B0-027.22
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentRegistration {
    agent: AgentDefinition,
    runtime: RuntimeEntity,
    supervisor: OwnerReference,
    capabilities: Vec<CapabilityDescriptor>,
    presence_state: PresenceState,
    health: RuntimeHealth,
    registered_at: TimeReference,
    lease: Option<LeaseRecord>,
    last_heartbeat: Option<HeartbeatRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentRegistrationSpec {
    pub agent: AgentDefinition,
    pub runtime: RuntimeEntity,
    pub supervisor: OwnerReference,
    pub capabilities: Vec<CapabilityDescriptor>,
    pub presence_state: PresenceState,
    pub health: RuntimeHealth,
    pub registered_at: TimeReference,
    pub lease: Option<LeaseRecord>,
    pub last_heartbeat: Option<HeartbeatRecord>,
}

impl AgentRegistration {
    pub fn new(spec: AgentRegistrationSpec) -> DomainResult<Self> {
        if spec.capabilities.is_empty() {
            return Err(DomainError::InvalidRuntimeReference(
                "agent registration requires at least one capability",
            ));
        }
        if spec.presence_state != PresenceState::Registered {
            return Err(DomainError::InvalidRuntimeReference(
                "new agent registrations must begin in Registered state",
            ));
        }
        if spec.agent.identity().enterprise_id() != spec.runtime.enterprise_id() {
            return Err(DomainError::InvalidRuntimeReference(
                "agent identity enterprise must match runtime enterprise",
            ));
        }
        let agent_context = spec.agent.organizational_context();
        if !ownership_contains(
            spec.runtime.ownership_path(),
            agent_context.ownership_path(),
        ) {
            return Err(DomainError::InvalidRuntimeReference(
                "runtime ownership path must contain the agent governance path",
            ));
        }
        if let Some(lease) = &spec.lease {
            validate_registration_lease(&spec.agent, &spec.runtime, lease)?;
        }
        if let Some(heartbeat) = &spec.last_heartbeat {
            validate_registration_heartbeat(&spec.agent, &spec.runtime, heartbeat)?;
        }
        Ok(Self {
            agent: spec.agent,
            runtime: spec.runtime,
            supervisor: spec.supervisor,
            capabilities: spec.capabilities,
            presence_state: spec.presence_state,
            health: spec.health,
            registered_at: spec.registered_at,
            lease: spec.lease,
            last_heartbeat: spec.last_heartbeat,
        })
    }

    pub fn agent(&self) -> &AgentDefinition {
        &self.agent
    }

    pub fn agent_id(&self) -> &AgentId {
        self.agent.identity().agent_id()
    }

    pub fn runtime(&self) -> &RuntimeEntity {
        &self.runtime
    }

    pub fn supervisor(&self) -> &OwnerReference {
        &self.supervisor
    }

    pub fn organizational_context(&self) -> &OrganizationalContext {
        self.agent.organizational_context()
    }

    pub fn capabilities(&self) -> &[CapabilityDescriptor] {
        &self.capabilities
    }

    pub fn presence_state(&self) -> PresenceState {
        self.presence_state
    }

    pub fn health(&self) -> RuntimeHealth {
        self.health
    }

    pub fn registered_at(&self) -> &TimeReference {
        &self.registered_at
    }

    pub fn lease(&self) -> Option<&LeaseRecord> {
        self.lease.as_ref()
    }

    pub fn last_heartbeat(&self) -> Option<&HeartbeatRecord> {
        self.last_heartbeat.as_ref()
    }

    pub fn transition_presence(&mut self, next_state: PresenceState) -> DomainResult<()> {
        self.presence_state.can_transition_to(next_state)?;
        self.presence_state = next_state;
        Ok(())
    }

    pub fn replace_lease(&mut self, lease: LeaseRecord) -> DomainResult<()> {
        validate_registration_lease(&self.agent, &self.runtime, &lease)?;
        self.lease = Some(lease);
        Ok(())
    }

    pub fn record_heartbeat(&mut self, heartbeat: HeartbeatRecord) -> DomainResult<()> {
        validate_registration_heartbeat(&self.agent, &self.runtime, &heartbeat)?;
        if self.presence_state.is_terminal() {
            return Err(DomainError::InvalidRuntimeRegistry(
                "retired registrations must not receive new heartbeat records",
            ));
        }
        self.last_heartbeat = Some(heartbeat);
        Ok(())
    }

    pub fn lease_is_current_at(&self, reference_time: &TimeReference) -> bool {
        self.lease
            .as_ref()
            .map(|lease| lease.is_current_at(reference_time))
            .unwrap_or(false)
    }

    fn set_health(&mut self, health: RuntimeHealth) {
        self.health = health;
    }
}

/// CES Traceability: CES-B0-027.8, CES-B0-027.9, CES-B0-027.21, CES-B0-027.22
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct AgentRegistry {
    registrations_by_agent_id: BTreeMap<AgentId, AgentRegistration>,
    capability_index: BTreeMap<CapabilityId, BTreeSet<AgentId>>,
    runtime_index: BTreeMap<RuntimeId, BTreeSet<AgentId>>,
}

impl AgentRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, registration: AgentRegistration) -> DomainResult<()> {
        let agent_id = registration.agent_id().clone();
        if self.registrations_by_agent_id.contains_key(&agent_id) {
            return Err(DomainError::InvalidRuntimeRegistry(
                "agent registration requires unique agent identity",
            ));
        }
        let runtime_id = registration.runtime().runtime_id().clone();
        for capability in registration.capabilities() {
            self.capability_index
                .entry(capability.capability_id().clone())
                .or_default()
                .insert(agent_id.clone());
        }
        self.runtime_index
            .entry(runtime_id)
            .or_default()
            .insert(agent_id.clone());
        self.registrations_by_agent_id
            .insert(agent_id, registration);
        Ok(())
    }

    pub fn lookup(&self, agent_id: &AgentId) -> Option<&AgentRegistration> {
        self.registrations_by_agent_id.get(agent_id)
    }

    pub fn registrations_for_capability(
        &self,
        capability_id: &CapabilityId,
    ) -> Vec<&AgentRegistration> {
        self.capability_index
            .get(capability_id)
            .into_iter()
            .flat_map(|agent_ids| agent_ids.iter())
            .filter_map(|agent_id| self.registrations_by_agent_id.get(agent_id))
            .collect()
    }

    pub fn registrations_for_runtime(&self, runtime_id: &RuntimeId) -> Vec<&AgentRegistration> {
        self.runtime_index
            .get(runtime_id)
            .into_iter()
            .flat_map(|agent_ids| agent_ids.iter())
            .filter_map(|agent_id| self.registrations_by_agent_id.get(agent_id))
            .collect()
    }

    pub fn transition_presence(
        &mut self,
        agent_id: &AgentId,
        next_state: PresenceState,
    ) -> DomainResult<()> {
        let registration = self.registrations_by_agent_id.get_mut(agent_id).ok_or(
            DomainError::InvalidRuntimeRegistry(
                "presence transitions require an existing registration",
            ),
        )?;
        registration.transition_presence(next_state)
    }

    pub fn renew_lease(&mut self, agent_id: &AgentId, lease: LeaseRecord) -> DomainResult<()> {
        let registration = self.registrations_by_agent_id.get_mut(agent_id).ok_or(
            DomainError::InvalidRuntimeRegistry("lease renewal requires an existing registration"),
        )?;
        registration.replace_lease(lease)
    }

    pub fn record_heartbeat(
        &mut self,
        agent_id: &AgentId,
        heartbeat: HeartbeatRecord,
    ) -> DomainResult<()> {
        let registration = self.registrations_by_agent_id.get_mut(agent_id).ok_or(
            DomainError::InvalidRuntimeRegistry(
                "heartbeat updates require an existing registration",
            ),
        )?;
        registration.record_heartbeat(heartbeat)
    }

    pub fn lease_is_current_at(
        &self,
        agent_id: &AgentId,
        reference_time: &TimeReference,
    ) -> DomainResult<bool> {
        let registration = self.registrations_by_agent_id.get(agent_id).ok_or(
            DomainError::InvalidRuntimeRegistry(
                "lease validation requires an existing registration",
            ),
        )?;
        Ok(registration.lease_is_current_at(reference_time))
    }

    pub fn record_heartbeat_validated(
        &mut self,
        agent_id: &AgentId,
        heartbeat: HeartbeatRecord,
        now: &TimeReference,
        freshness_policy: &HeartbeatFreshnessPolicy,
    ) -> DomainResult<HeartbeatUpdateResult> {
        self.update_registration(agent_id, |registration| {
            ensure_validated_heartbeat(&heartbeat)?;
            let assessment = assess_heartbeat(now, Some(&heartbeat), freshness_policy)?;
            if heartbeat.runtime_id() != registration.runtime().runtime_id() {
                return Ok((
                    registration.clone(),
                    HeartbeatUpdateResult {
                        outcome: HeartbeatRecordOutcome::HeartbeatRuntimeMismatch,
                        assessment,
                        latest_heartbeat: registration.last_heartbeat().cloned(),
                    },
                ));
            }
            if heartbeat.agent_id() != Some(registration.agent_id()) {
                return Ok((
                    registration.clone(),
                    HeartbeatUpdateResult {
                        outcome: HeartbeatRecordOutcome::HeartbeatAgentMismatch,
                        assessment,
                        latest_heartbeat: registration.last_heartbeat().cloned(),
                    },
                ));
            }
            if registration.presence_state().is_terminal()
                || registration.agent().identity().lifecycle().is_terminal()
            {
                return Ok((
                    registration.clone(),
                    HeartbeatUpdateResult {
                        outcome: HeartbeatRecordOutcome::HeartbeatRuntimeRetired,
                        assessment,
                        latest_heartbeat: registration.last_heartbeat().cloned(),
                    },
                ));
            }
            if registration.presence_state() == PresenceState::Offline {
                return Ok((
                    registration.clone(),
                    HeartbeatUpdateResult {
                        outcome: HeartbeatRecordOutcome::HeartbeatRejected,
                        assessment,
                        latest_heartbeat: registration.last_heartbeat().cloned(),
                    },
                ));
            }
            if let Some(last) = registration.last_heartbeat() {
                if last == &heartbeat {
                    return Ok((
                        registration.clone(),
                        HeartbeatUpdateResult {
                            outcome: HeartbeatRecordOutcome::HeartbeatDuplicate,
                            assessment,
                            latest_heartbeat: Some(last.clone()),
                        },
                    ));
                }
                if heartbeat.recorded_at().as_str() <= last.recorded_at().as_str() {
                    return Ok((
                        registration.clone(),
                        HeartbeatUpdateResult {
                            outcome: HeartbeatRecordOutcome::HeartbeatTimestampRegression,
                            assessment,
                            latest_heartbeat: Some(last.clone()),
                        },
                    ));
                }
            }
            if registration.lease().is_some()
                && heartbeat.active_lease_id() != registration.lease().map(LeaseRecord::lease_id)
            {
                return Ok((
                    registration.clone(),
                    HeartbeatUpdateResult {
                        outcome: HeartbeatRecordOutcome::HeartbeatLeaseMismatch,
                        assessment,
                        latest_heartbeat: registration.last_heartbeat().cloned(),
                    },
                ));
            }
            if assessment.freshness() == HeartbeatFreshness::Stale {
                return Ok((
                    registration.clone(),
                    HeartbeatUpdateResult {
                        outcome: HeartbeatRecordOutcome::HeartbeatStale,
                        assessment,
                        latest_heartbeat: registration.last_heartbeat().cloned(),
                    },
                ));
            }
            let mut next = registration.clone();
            next.record_heartbeat(heartbeat.clone())?;
            next.set_health(
                heartbeat
                    .reported_health()
                    .unwrap_or(RuntimeHealth::Unknown),
            );
            Ok((
                next,
                HeartbeatUpdateResult {
                    outcome: HeartbeatRecordOutcome::HeartbeatAccepted,
                    assessment,
                    latest_heartbeat: Some(heartbeat),
                },
            ))
        })
    }

    pub fn renew_lease_validated(
        &mut self,
        agent_id: &AgentId,
        request: &LeaseRenewalRequest,
        policy: &LeasePolicy,
    ) -> DomainResult<LeaseRenewalResult> {
        self.update_registration(agent_id, |registration| {
            ensure_validated_lease(request.renewed_lease())?;
            if request.renewed_lease().runtime_id() != registration.runtime().runtime_id() {
                let assessment = evaluate_lease(
                    request.requested_at(),
                    registration.lease(),
                    registration.runtime().runtime_id(),
                    registration.agent_id(),
                );
                return Ok((
                    registration.clone(),
                    LeaseRenewalResult {
                        outcome: LeaseRenewalOutcome::LeaseRuntimeMismatch,
                        assessment,
                        active_lease: registration.lease().cloned(),
                    },
                ));
            }
            if request.renewed_lease().agent_id() != Some(registration.agent_id()) {
                let assessment = evaluate_lease(
                    request.requested_at(),
                    registration.lease(),
                    registration.runtime().runtime_id(),
                    registration.agent_id(),
                );
                return Ok((
                    registration.clone(),
                    LeaseRenewalResult {
                        outcome: LeaseRenewalOutcome::LeaseAgentMismatch,
                        assessment,
                        active_lease: registration.lease().cloned(),
                    },
                ));
            }
            let current = registration.lease().cloned();
            let assessment = evaluate_lease(
                request.requested_at(),
                current.as_ref(),
                registration.runtime().runtime_id(),
                registration.agent_id(),
            );
            if registration.presence_state().is_terminal()
                || registration.agent().identity().lifecycle().is_terminal()
            {
                return Ok((
                    registration.clone(),
                    LeaseRenewalResult {
                        outcome: LeaseRenewalOutcome::RuntimeAlreadyRetired,
                        assessment,
                        active_lease: current,
                    },
                ));
            }
            let Some(current_lease) = current else {
                return Ok((
                    registration.clone(),
                    LeaseRenewalResult {
                        outcome: LeaseRenewalOutcome::LeaseRenewalRejected,
                        assessment,
                        active_lease: None,
                    },
                ));
            };
            if is_duplicate_lease_renewal(&current_lease, request) {
                return Ok((
                    registration.clone(),
                    LeaseRenewalResult {
                        outcome: LeaseRenewalOutcome::LeaseDuplicate,
                        assessment,
                        active_lease: Some(current_lease),
                    },
                ));
            }
            if request.prior_lease_id() != current_lease.lease_id() {
                return Ok((
                    registration.clone(),
                    LeaseRenewalResult {
                        outcome: LeaseRenewalOutcome::LeaseSequenceRegression,
                        assessment,
                        active_lease: Some(current_lease),
                    },
                ));
            }
            if assessment.validity() == LeaseValidity::Expired
                && !policy.allow_renewal_after_expiration()
            {
                return Ok((
                    registration.clone(),
                    LeaseRenewalResult {
                        outcome: LeaseRenewalOutcome::LeaseExpired,
                        assessment,
                        active_lease: Some(current_lease),
                    },
                ));
            }
            if request.renewed_lease().supersedes_lease_id() != Some(current_lease.lease_id()) {
                return Ok((
                    registration.clone(),
                    LeaseRenewalResult {
                        outcome: LeaseRenewalOutcome::LeaseSequenceRegression,
                        assessment,
                        active_lease: Some(current_lease),
                    },
                ));
            }
            let mut next = registration.clone();
            next.replace_lease(request.renewed_lease().clone())?;
            let next_assessment = evaluate_lease(
                request.requested_at(),
                next.lease(),
                next.runtime().runtime_id(),
                next.agent_id(),
            );
            Ok((
                next.clone(),
                LeaseRenewalResult {
                    outcome: LeaseRenewalOutcome::LeaseRenewed,
                    assessment: next_assessment,
                    active_lease: next.lease().cloned(),
                },
            ))
        })
    }

    pub fn expire_lease(
        &mut self,
        agent_id: &AgentId,
        now: &TimeReference,
    ) -> DomainResult<LeaseAssessment> {
        self.update_registration(agent_id, |registration| {
            let assessment = evaluate_lease(
                now,
                registration.lease(),
                registration.runtime().runtime_id(),
                registration.agent_id(),
            );
            let mut next = registration.clone();
            if assessment.validity() == LeaseValidity::Expired
                && !matches!(
                    next.presence_state(),
                    PresenceState::Offline | PresenceState::Retired
                )
            {
                next.transition_presence(PresenceState::Offline)?;
                next.set_health(RuntimeHealth::Critical);
            }
            Ok((next, assessment))
        })
    }

    pub fn runtime_snapshot(
        &self,
        agent_id: &AgentId,
        now: &TimeReference,
        freshness_policy: &HeartbeatFreshnessPolicy,
        failure_observation: Option<RuntimeFailureObservation>,
        recovery_reference: Option<AgentRecoveryReference>,
    ) -> DomainResult<RuntimeStateSnapshot> {
        let registration = self.registrations_by_agent_id.get(agent_id).ok_or(
            DomainError::InvalidRuntimeRegistry(
                "runtime snapshot requires an existing registration",
            ),
        )?;
        let heartbeat_assessment =
            assess_heartbeat(now, registration.last_heartbeat(), freshness_policy)?;
        let lease_assessment = evaluate_lease(
            now,
            registration.lease(),
            registration.runtime().runtime_id(),
            registration.agent_id(),
        );
        let health = assess_runtime_health(
            registration.presence_state(),
            heartbeat_assessment.freshness(),
            &lease_assessment,
            failure_observation.as_ref(),
        );
        Ok(RuntimeStateSnapshot {
            runtime_id: registration.runtime().runtime_id().clone(),
            agent_id: registration.agent_id().clone(),
            agent_lifecycle: registration.agent().identity().lifecycle(),
            presence: registration.presence_state(),
            health,
            current_lease_id: registration.lease().map(|lease| lease.lease_id().clone()),
            latest_heartbeat_id: registration
                .last_heartbeat()
                .map(|heartbeat| heartbeat.heartbeat_id().clone()),
            heartbeat_freshness: heartbeat_assessment.freshness(),
            capability_ids: registration
                .capabilities()
                .iter()
                .map(|capability| capability.capability_id().clone())
                .collect(),
            observed_at: now.clone(),
            failure_observation,
            recovery_reference,
            lease_assessment,
        })
    }

    pub fn supervise_runtime(
        &self,
        agent_id: &AgentId,
        now: &TimeReference,
        freshness_policy: &HeartbeatFreshnessPolicy,
        failure_observation: Option<RuntimeFailureObservation>,
        recovery_reference: Option<AgentRecoveryReference>,
    ) -> DomainResult<SupervisorOutcome> {
        let registration = self.registrations_by_agent_id.get(agent_id).ok_or(
            DomainError::InvalidRuntimeRegistry(
                "runtime supervision requires an existing registration",
            ),
        )?;
        let snapshot = self.runtime_snapshot(
            agent_id,
            now,
            freshness_policy,
            failure_observation,
            recovery_reference,
        )?;
        let heartbeat_assessment =
            assess_heartbeat(now, registration.last_heartbeat(), freshness_policy)?;
        let recovery_eligibility = assess_recovery_eligibility(
            &snapshot,
            snapshot.failure_observation.as_ref(),
            snapshot.recovery_reference.as_ref(),
        );
        supervise_runtime(&snapshot, heartbeat_assessment, recovery_eligibility)
    }

    pub fn apply_supervisor_outcome(&mut self, outcome: &SupervisorOutcome) -> DomainResult<()> {
        self.update_registration(outcome.agent_id(), |registration| {
            if registration.runtime().runtime_id() != outcome.runtime_id() {
                return Err(DomainError::InvalidRuntimeRegistry(
                    "supervisor outcome runtime must match the registered runtime",
                ));
            }
            let mut next = registration.clone();
            if next.presence_state() != outcome.proposed_presence() {
                next.transition_presence(outcome.proposed_presence())?;
            }
            next.set_health(outcome.resulting_health());
            Ok((next, ()))
        })
    }

    pub fn apply_presence_transition(
        &mut self,
        agent_id: &AgentId,
        next_state: PresenceState,
    ) -> DomainResult<()> {
        self.update_registration(agent_id, |registration| {
            let mut next = registration.clone();
            next.transition_presence(next_state)?;
            Ok((next, ()))
        })
    }

    pub fn deregister(&mut self, agent_id: &AgentId) -> DomainResult<AgentRegistration> {
        let mut registration = self.registrations_by_agent_id.remove(agent_id).ok_or(
            DomainError::InvalidRuntimeRegistry("deregistration requires an existing registration"),
        )?;
        let _ = registration.transition_presence(PresenceState::Retired);
        for capability in registration.capabilities() {
            if let Some(agent_ids) = self.capability_index.get_mut(capability.capability_id()) {
                agent_ids.remove(agent_id);
                if agent_ids.is_empty() {
                    self.capability_index.remove(capability.capability_id());
                }
            }
        }
        if let Some(agent_ids) = self
            .runtime_index
            .get_mut(registration.runtime().runtime_id())
        {
            agent_ids.remove(agent_id);
            if agent_ids.is_empty() {
                self.runtime_index
                    .remove(registration.runtime().runtime_id());
            }
        }
        Ok(registration)
    }

    fn update_registration<T, F>(&mut self, agent_id: &AgentId, operation: F) -> DomainResult<T>
    where
        F: FnOnce(&AgentRegistration) -> DomainResult<(AgentRegistration, T)>,
    {
        let current = self.registrations_by_agent_id.get(agent_id).ok_or(
            DomainError::InvalidRuntimeRegistry("operation requires an existing registration"),
        )?;
        let (next, result) = operation(current)?;
        self.registrations_by_agent_id
            .insert(agent_id.clone(), next);
        Ok(result)
    }
}

pub fn assess_heartbeat(
    now: &TimeReference,
    heartbeat: Option<&HeartbeatRecord>,
    policy: &HeartbeatFreshnessPolicy,
) -> DomainResult<HeartbeatAssessment> {
    if policy.late_cutoff().as_str() > now.as_str() || policy.stale_cutoff().as_str() > now.as_str()
    {
        return Err(DomainError::InvalidRuntimeReference(
            "heartbeat freshness cutoffs must not be later than the evaluation time",
        ));
    }
    let Some(heartbeat) = heartbeat else {
        return Ok(HeartbeatAssessment {
            freshness: HeartbeatFreshness::Missing,
            evaluated_at: now.clone(),
            heartbeat_id: None,
        });
    };
    let freshness = if heartbeat.recorded_at().as_str() < policy.stale_cutoff().as_str() {
        HeartbeatFreshness::Stale
    } else if heartbeat.recorded_at().as_str() < policy.late_cutoff().as_str() {
        HeartbeatFreshness::Late
    } else {
        HeartbeatFreshness::Fresh
    };
    Ok(HeartbeatAssessment {
        freshness,
        evaluated_at: now.clone(),
        heartbeat_id: Some(heartbeat.heartbeat_id().clone()),
    })
}

pub fn evaluate_lease(
    now: &TimeReference,
    lease: Option<&LeaseRecord>,
    runtime_id: &RuntimeId,
    agent_id: &AgentId,
) -> LeaseAssessment {
    let Some(lease) = lease else {
        return LeaseAssessment {
            validity: LeaseValidity::Missing,
            evaluated_at: now.clone(),
            lease_id: None,
            rejection_reason: Some(LeaseRejectionReason::LeaseMissing),
        };
    };
    let (validity, rejection_reason) = if lease.runtime_id() != runtime_id {
        (
            LeaseValidity::Invalid,
            Some(LeaseRejectionReason::LeaseRuntimeMismatch),
        )
    } else if lease.agent_id().is_some() && lease.agent_id() != Some(agent_id) {
        (
            LeaseValidity::Invalid,
            Some(LeaseRejectionReason::LeaseAgentMismatch),
        )
    } else if !lease.is_current_at(now) {
        (
            LeaseValidity::Expired,
            Some(LeaseRejectionReason::LeaseExpired),
        )
    } else {
        (LeaseValidity::Valid, None)
    };
    LeaseAssessment {
        validity,
        evaluated_at: now.clone(),
        lease_id: Some(lease.lease_id().clone()),
        rejection_reason,
    }
}

pub fn assess_runtime_health(
    presence: PresenceState,
    heartbeat_freshness: HeartbeatFreshness,
    lease_assessment: &LeaseAssessment,
    failure_observation: Option<&RuntimeFailureObservation>,
) -> RuntimeHealth {
    if presence == PresenceState::Retired {
        return RuntimeHealth::Unknown;
    }
    if presence == PresenceState::Offline {
        return RuntimeHealth::Critical;
    }
    if let Some(failure) = failure_observation {
        if failure.failure().severity().critical_level() {
            return RuntimeHealth::Critical;
        }
        if failure.failure().severity().major_level() {
            return RuntimeHealth::Degraded;
        }
    }
    if matches!(
        heartbeat_freshness,
        HeartbeatFreshness::Missing | HeartbeatFreshness::Stale
    ) {
        return RuntimeHealth::Critical;
    }
    if matches!(
        lease_assessment.validity(),
        LeaseValidity::Expired | LeaseValidity::Invalid
    ) {
        return RuntimeHealth::Critical;
    }
    if matches!(heartbeat_freshness, HeartbeatFreshness::Late)
        || presence == PresenceState::Recovering
    {
        return RuntimeHealth::Degraded;
    }
    if presence == PresenceState::Registered
        && lease_assessment.validity() == LeaseValidity::Missing
    {
        return RuntimeHealth::Unknown;
    }
    RuntimeHealth::Healthy
}

pub fn assess_recovery_eligibility(
    snapshot: &RuntimeStateSnapshot,
    failure_observation: Option<&RuntimeFailureObservation>,
    recovery_reference: Option<&AgentRecoveryReference>,
) -> RecoveryEligibility {
    if snapshot.presence() == PresenceState::Retired || snapshot.agent_lifecycle.is_terminal() {
        return RecoveryEligibility {
            eligible: false,
            rejection_reason: Some(RecoveryRejectionReason::RuntimeAlreadyRetired),
            recovery_reference: recovery_reference.cloned(),
        };
    }
    if failure_observation.is_none() {
        return RecoveryEligibility {
            eligible: false,
            rejection_reason: Some(RecoveryRejectionReason::UnderlyingFailureUnresolved),
            recovery_reference: recovery_reference.cloned(),
        };
    }
    if recovery_reference.is_none() {
        return RecoveryEligibility {
            eligible: false,
            rejection_reason: Some(RecoveryRejectionReason::MissingRecoveryEvidence),
            recovery_reference: None,
        };
    }
    if snapshot.lease_assessment().validity() != LeaseValidity::Valid {
        return RecoveryEligibility {
            eligible: false,
            rejection_reason: Some(RecoveryRejectionReason::LeaseInvalid),
            recovery_reference: recovery_reference.cloned(),
        };
    }
    RecoveryEligibility {
        eligible: true,
        rejection_reason: None,
        recovery_reference: recovery_reference.cloned(),
    }
}

pub fn supervise_runtime(
    snapshot: &RuntimeStateSnapshot,
    heartbeat_assessment: HeartbeatAssessment,
    recovery_eligibility: RecoveryEligibility,
) -> DomainResult<SupervisorOutcome> {
    let mut steps = Vec::new();
    push_supervisor_step(
        &mut steps,
        SupervisorStep::ObserveRuntimeState,
        true,
        false,
        collect_snapshot_evidence(snapshot),
    );
    push_supervisor_step(
        &mut steps,
        SupervisorStep::AssessHeartbeatFreshness,
        !matches!(
            heartbeat_assessment.freshness(),
            HeartbeatFreshness::Missing
        ),
        false,
        heartbeat_assessment
            .heartbeat_id()
            .map(|heartbeat_id| {
                vec![SupervisorEvidenceReference::new(format!(
                    "heartbeat:{}",
                    heartbeat_id.as_str()
                ))
                .expect("heartbeat evidence")]
            })
            .unwrap_or_default(),
    );
    push_supervisor_step(
        &mut steps,
        SupervisorStep::AssessLeaseValidity,
        snapshot.lease_assessment().validity() == LeaseValidity::Valid,
        false,
        snapshot
            .lease_assessment()
            .lease_id()
            .map(|lease_id| {
                vec![
                    SupervisorEvidenceReference::new(format!("lease:{}", lease_id.as_str()))
                        .expect("lease evidence"),
                ]
            })
            .unwrap_or_default(),
    );
    push_supervisor_step(
        &mut steps,
        SupervisorStep::AssessRuntimeHealth,
        snapshot.health() != RuntimeHealth::Critical,
        false,
        Vec::new(),
    );
    push_supervisor_step(
        &mut steps,
        SupervisorStep::AssessFailureEvidence,
        snapshot.failure_observation.is_none(),
        false,
        snapshot
            .failure_observation
            .as_ref()
            .map(|failure| {
                vec![
                    SupervisorEvidenceReference::new(failure.evidence().to_owned())
                        .expect("failure evidence"),
                ]
            })
            .unwrap_or_default(),
    );
    push_supervisor_step(
        &mut steps,
        SupervisorStep::AssessRecoveryEligibility,
        recovery_eligibility.eligible(),
        false,
        recovery_eligibility
            .recovery_reference()
            .map(|_| {
                vec![
                    SupervisorEvidenceReference::new("recovery-reference".to_owned())
                        .expect("recovery evidence"),
                ]
            })
            .unwrap_or_default(),
    );

    let (recommended_action, proposed_presence, resulting_health, decisive_step) =
        if snapshot.presence() == PresenceState::Retired {
            (
                SupervisorAction::NoAction,
                PresenceState::Retired,
                RuntimeHealth::Unknown,
                SupervisorStep::RecommendSupervisorAction,
            )
        } else if snapshot
            .failure_observation
            .as_ref()
            .map(|failure| failure.failure().severity().critical_level())
            .unwrap_or(false)
        {
            (
                SupervisorAction::SuspendRuntime,
                PresenceState::Offline,
                RuntimeHealth::Critical,
                SupervisorStep::AssessFailureEvidence,
            )
        } else if matches!(
            snapshot.lease_assessment().validity(),
            LeaseValidity::Expired | LeaseValidity::Invalid
        ) {
            (
                SupervisorAction::TransitionOffline,
                PresenceState::Offline,
                RuntimeHealth::Critical,
                SupervisorStep::AssessLeaseValidity,
            )
        } else if matches!(
            heartbeat_assessment.freshness(),
            HeartbeatFreshness::Missing | HeartbeatFreshness::Stale
        ) {
            (
                SupervisorAction::TransitionOffline,
                PresenceState::Offline,
                RuntimeHealth::Critical,
                SupervisorStep::AssessHeartbeatFreshness,
            )
        } else if heartbeat_assessment.freshness() == HeartbeatFreshness::Late {
            (
                SupervisorAction::MarkLate,
                snapshot.presence(),
                RuntimeHealth::Degraded,
                SupervisorStep::AssessHeartbeatFreshness,
            )
        } else if snapshot.presence() == PresenceState::Offline {
            if recovery_eligibility.eligible() {
                (
                    SupervisorAction::RequestRecovery,
                    PresenceState::Recovering,
                    RuntimeHealth::Degraded,
                    SupervisorStep::AssessRecoveryEligibility,
                )
            } else {
                (
                    SupervisorAction::RejectRecovery,
                    PresenceState::Offline,
                    RuntimeHealth::Critical,
                    SupervisorStep::AssessRecoveryEligibility,
                )
            }
        } else {
            (
                SupervisorAction::NoAction,
                snapshot.presence(),
                snapshot.health(),
                SupervisorStep::RecommendSupervisorAction,
            )
        };
    mark_supervisor_decisive_step(&mut steps, decisive_step);
    push_supervisor_step(
        &mut steps,
        SupervisorStep::RecommendSupervisorAction,
        true,
        true,
        vec![
            SupervisorEvidenceReference::new(format!("action:{recommended_action:?}"))
                .expect("action evidence"),
        ],
    );
    Ok(SupervisorOutcome {
        runtime_id: snapshot.runtime_id().clone(),
        agent_id: snapshot.agent_id().clone(),
        observation: SupervisorObservation {
            snapshot: snapshot.clone(),
            heartbeat_assessment,
            lease_assessment: snapshot.lease_assessment().clone(),
            recovery_eligibility,
        },
        recommended_action,
        proposed_presence,
        resulting_health,
        trace: SupervisorTrace { steps },
    })
}

fn validate_registration_lease(
    agent: &AgentDefinition,
    runtime: &RuntimeEntity,
    lease: &LeaseRecord,
) -> DomainResult<()> {
    if lease.runtime_id() != runtime.runtime_id() {
        return Err(DomainError::InvalidRuntimeReference(
            "lease runtime must match the registration runtime",
        ));
    }
    if lease.agent_id().is_some() && lease.agent_id() != Some(agent.identity().agent_id()) {
        return Err(DomainError::InvalidRuntimeReference(
            "lease agent must match the registered agent identity",
        ));
    }
    Ok(())
}

fn validate_registration_heartbeat(
    agent: &AgentDefinition,
    runtime: &RuntimeEntity,
    heartbeat: &HeartbeatRecord,
) -> DomainResult<()> {
    if heartbeat.runtime_id() != runtime.runtime_id() {
        return Err(DomainError::InvalidRuntimeReference(
            "heartbeat runtime must match the registration runtime",
        ));
    }
    if heartbeat.agent_id().is_some() && heartbeat.agent_id() != Some(agent.identity().agent_id()) {
        return Err(DomainError::InvalidRuntimeReference(
            "heartbeat agent must match the registered agent identity",
        ));
    }
    if let Some(lease_id) = heartbeat.active_lease_id() {
        if lease_id.as_str().is_empty() {
            return Err(DomainError::InvalidRuntimeReference(
                "heartbeat lease references must not be empty",
            ));
        }
    }
    Ok(())
}

fn ensure_validated_heartbeat(heartbeat: &HeartbeatRecord) -> DomainResult<()> {
    if heartbeat.agent_id().is_none()
        || heartbeat.reported_presence().is_none()
        || heartbeat.reported_health().is_none()
        || heartbeat.evidence().is_none()
    {
        return Err(DomainError::InvalidRuntimeReference(
            "validated heartbeat operations require bound agent identity, reported presence, reported health, and evidence",
        ));
    }
    Ok(())
}

fn ensure_validated_lease(lease: &LeaseRecord) -> DomainResult<()> {
    if lease.agent_id().is_none() || lease.evidence().is_none() {
        return Err(DomainError::InvalidRuntimeReference(
            "validated lease operations require bound agent identity and evidence",
        ));
    }
    Ok(())
}

fn is_duplicate_lease_renewal(current_lease: &LeaseRecord, request: &LeaseRenewalRequest) -> bool {
    request.renewed_lease() == current_lease
        && current_lease.supersedes_lease_id() == Some(request.prior_lease_id())
}

fn validate_lease_window(
    issued_at: &TimeReference,
    expires_at: &TimeReference,
) -> DomainResult<()> {
    if expires_at.as_str() <= issued_at.as_str() {
        return Err(DomainError::InvalidRuntimeReference(
            "lease expiration must be after lease issuance",
        ));
    }
    Ok(())
}

fn validate_heartbeat_window(
    recorded_at: &TimeReference,
    fresh_until: &TimeReference,
) -> DomainResult<()> {
    if fresh_until.as_str() < recorded_at.as_str() {
        return Err(DomainError::InvalidRuntimeReference(
            "heartbeat freshness must not precede the recorded timestamp",
        ));
    }
    Ok(())
}

fn normalize_text_list(
    field: &'static str,
    values: Vec<String>,
) -> DomainResult<Vec<NonEmptyText>> {
    values
        .into_iter()
        .map(|value| NonEmptyText::new(field, value))
        .collect()
}

fn ownership_contains(parent: &OwnershipPath, child: &OwnershipPath) -> bool {
    if parent.enterprise_id() != child.enterprise_id() {
        return false;
    }
    if parent.workspace_id().is_some() && parent.workspace_id() != child.workspace_id() {
        return false;
    }
    if parent.project_id().is_some() && parent.project_id() != child.project_id() {
        return false;
    }
    if parent.organizational_unit_id().is_some()
        && parent.organizational_unit_id() != child.organizational_unit_id()
    {
        return false;
    }
    true
}

fn push_supervisor_step(
    steps: &mut Vec<SupervisorTraceStep>,
    step: SupervisorStep,
    passed: bool,
    decisive: bool,
    evidence: Vec<SupervisorEvidenceReference>,
) {
    steps.push(SupervisorTraceStep {
        step,
        passed,
        decisive,
        evidence,
    });
}

fn mark_supervisor_decisive_step(steps: &mut [SupervisorTraceStep], target: SupervisorStep) {
    if let Some(step) = steps.iter_mut().find(|step| step.step == target) {
        step.decisive = true;
    }
}

fn collect_snapshot_evidence(snapshot: &RuntimeStateSnapshot) -> Vec<SupervisorEvidenceReference> {
    let mut evidence = vec![
        SupervisorEvidenceReference::new(format!("runtime:{}", snapshot.runtime_id().as_str()))
            .expect("runtime evidence"),
        SupervisorEvidenceReference::new(format!("agent:{}", snapshot.agent_id().as_str()))
            .expect("agent evidence"),
    ];
    if let Some(lease_id) = &snapshot.current_lease_id {
        evidence.push(
            SupervisorEvidenceReference::new(format!("lease:{}", lease_id.as_str()))
                .expect("lease evidence"),
        );
    }
    if let Some(heartbeat_id) = &snapshot.latest_heartbeat_id {
        evidence.push(
            SupervisorEvidenceReference::new(format!("heartbeat:{}", heartbeat_id.as_str()))
                .expect("heartbeat evidence"),
        );
    }
    evidence
}

trait FailureSeverityExt {
    fn critical_level(self) -> bool;
    fn major_level(self) -> bool;
}

impl FailureSeverityExt for crate::agent::AgentFailureSeverity {
    fn critical_level(self) -> bool {
        matches!(self, crate::agent::AgentFailureSeverity::Critical)
    }

    fn major_level(self) -> bool {
        matches!(
            self,
            crate::agent::AgentFailureSeverity::Major
                | crate::agent::AgentFailureSeverity::Critical
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{
        assess_heartbeat, assess_recovery_eligibility, assess_runtime_health, evaluate_lease,
        supervise_runtime, AgentRegistration, AgentRegistrationSpec, AgentRegistry,
        CapabilityDescriptor, CapabilityDescriptorSpec, HeartbeatAssessment, HeartbeatFreshness,
        HeartbeatFreshnessPolicy, HeartbeatObservationSpec, HeartbeatRecord,
        HeartbeatRecordOutcome, HeartbeatRecordSpec, LeaseAssessment, LeaseIssuanceSpec,
        LeasePolicy, LeaseRecord, LeaseRecordSpec, LeaseRenewalOutcome, LeaseRenewalRequest,
        LeaseValidity, PresenceState, RecoveryRejectionReason, RuntimeEntity, RuntimeEntitySpec,
        RuntimeFailureObservation, RuntimeHealth, SupervisorAction,
    };
    use crate::agent::{
        AgentCategory, AgentDefinition, AgentDefinitionSpec, AgentFailureCategory,
        AgentFailureReference, AgentFailureSeverity, AgentRecoveryEvidenceReference,
        AgentRecoveryPlanReference, AgentRecoveryReference, AgentRuntimeReference, AgentType,
    };
    use crate::authorization::{
        ActionVerb, PermissionEffectIntent, PermissionReference, ResourceType,
    };
    use crate::identifier::{
        AgentId, AgentUuid, CapabilityId, EnglishNamespace, EnterpriseId, HeartbeatId, HumanId,
        LeaseId, PermissionId, PolicyId, ProjectId, RuntimeId, StableVersion, WorkspaceId,
    };
    use crate::identity::AgentIdentity;
    use crate::lifecycle::AgentLifecycle;
    use crate::ownership::{OrganizationalContext, OwnerReference, OwnershipPath};
    use crate::request::TimeReference;

    fn enterprise_id() -> EnterpriseId {
        EnterpriseId::new("CX-ENT-000001").expect("enterprise")
    }

    fn owner() -> OwnerReference {
        OwnerReference::new(HumanId::new("CX-EMP-000001").expect("owner"))
    }

    fn ownership_path(project_id: &str) -> OwnershipPath {
        OwnershipPath::new(
            enterprise_id(),
            Some(WorkspaceId::new("CX-WS-000001").expect("workspace")),
            Some(ProjectId::new(project_id).expect("project")),
            None,
        )
        .expect("path")
    }

    fn permission(permission_id: &str, action: &str) -> PermissionReference {
        PermissionReference::new(
            PermissionId::new(permission_id).expect("permission"),
            ActionVerb::new(action).expect("action"),
            ResourceType::new("workflow").expect("resource"),
            PermissionEffectIntent::new("Permit").expect("effect"),
        )
    }

    fn capability(capability_id: &str) -> CapabilityDescriptor {
        CapabilityDescriptor::new(CapabilityDescriptorSpec {
            capability_id: CapabilityId::new(capability_id).expect("capability"),
            description: "render governed output".to_owned(),
            dependencies: vec!["model".to_owned()],
            inputs: vec!["prompt".to_owned()],
            outputs: vec!["artifact".to_owned()],
            required_permissions: vec![permission("CX-PERM-000001", "approve")],
            governing_policies: vec![PolicyId::new("CX-POL-000001").expect("policy")],
        })
        .expect("capability")
    }

    fn runtime_entity(runtime_id: &str, project_id: &str) -> RuntimeEntity {
        RuntimeEntity::new(RuntimeEntitySpec {
            runtime_id: RuntimeId::new(runtime_id).expect("runtime"),
            runtime_reference: AgentRuntimeReference::new("runtime.primary").expect("runtime ref"),
            enterprise_id: enterprise_id(),
            ownership_path: ownership_path(project_id),
            health: RuntimeHealth::Healthy,
        })
        .expect("runtime entity")
    }

    fn agent_definition(
        agent_id: &str,
        project_id: &str,
        lifecycle: AgentLifecycle,
    ) -> AgentDefinition {
        AgentDefinition::new(AgentDefinitionSpec {
            identity: AgentIdentity::new(
                AgentId::new(agent_id).expect("agent"),
                EnglishNamespace::new("agent_namespace", "enterprise.agent").expect("namespace"),
                StableVersion::new("agent_version", "1.0.0").expect("version"),
                enterprise_id(),
                lifecycle,
            )
            .expect("identity"),
            agent_uuid: AgentUuid::new("CX-UUID-00000001").expect("uuid"),
            agent_name: "Agent One".to_owned(),
            agent_type: AgentType::new("worker").expect("type"),
            agent_category: AgentCategory::new("creative").expect("category"),
            owner: owner(),
            organizational_context: OrganizationalContext::new(ownership_path(project_id), owner()),
            runtime_reference: AgentRuntimeReference::new("runtime.primary").expect("runtime ref"),
        })
        .expect("agent definition")
    }

    fn lease(runtime_id: &str, expires_at: &str) -> LeaseRecord {
        LeaseRecord::new(LeaseRecordSpec {
            lease_id: LeaseId::new("CX-LEASE-000001").expect("lease"),
            runtime_id: RuntimeId::new(runtime_id).expect("runtime"),
            issued_at: TimeReference::new("2026-07-15T00:00:00Z").expect("issued"),
            expires_at: TimeReference::new(expires_at).expect("expires"),
        })
        .expect("lease")
    }

    fn governed_lease(
        lease_id: &str,
        runtime_id: &str,
        agent_id: &str,
        issued_at: &str,
        expires_at: &str,
        supersedes: Option<&str>,
    ) -> LeaseRecord {
        LeaseRecord::issue(LeaseIssuanceSpec {
            lease_id: LeaseId::new(lease_id).expect("lease"),
            runtime_id: RuntimeId::new(runtime_id).expect("runtime"),
            agent_id: AgentId::new(agent_id).expect("agent"),
            issued_at: TimeReference::new(issued_at).expect("issued"),
            expires_at: TimeReference::new(expires_at).expect("expires"),
            supersedes_lease_id: supersedes.map(|lease_id| LeaseId::new(lease_id).expect("lease")),
            evidence: "LEASE-EVID-001".to_owned(),
        })
        .expect("governed lease")
    }

    fn heartbeat(runtime_id: &str, fresh_until: &str) -> HeartbeatRecord {
        HeartbeatRecord::new(HeartbeatRecordSpec {
            heartbeat_id: HeartbeatId::new("CX-HB-000001").expect("heartbeat"),
            runtime_id: RuntimeId::new(runtime_id).expect("runtime"),
            recorded_at: TimeReference::new("2026-07-15T00:10:00Z").expect("recorded"),
            fresh_until: TimeReference::new(fresh_until).expect("fresh until"),
        })
        .expect("heartbeat")
    }

    #[allow(clippy::too_many_arguments)]
    fn governed_heartbeat(
        heartbeat_id: &str,
        runtime_id: &str,
        agent_id: &str,
        recorded_at: &str,
        fresh_until: &str,
        lease_id: Option<&str>,
        reported_presence: PresenceState,
        reported_health: RuntimeHealth,
    ) -> HeartbeatRecord {
        HeartbeatRecord::observe(HeartbeatObservationSpec {
            heartbeat_id: HeartbeatId::new(heartbeat_id).expect("heartbeat"),
            runtime_id: RuntimeId::new(runtime_id).expect("runtime"),
            agent_id: AgentId::new(agent_id).expect("agent"),
            recorded_at: TimeReference::new(recorded_at).expect("recorded"),
            fresh_until: TimeReference::new(fresh_until).expect("fresh"),
            reported_presence,
            reported_health,
            active_lease_id: lease_id.map(|lease_id| LeaseId::new(lease_id).expect("lease")),
            evidence: "HB-EVID-001".to_owned(),
        })
        .expect("governed heartbeat")
    }

    fn heartbeat_policy() -> HeartbeatFreshnessPolicy {
        HeartbeatFreshnessPolicy::new(
            TimeReference::new("2026-07-15T00:09:00Z").expect("late"),
            TimeReference::new("2026-07-15T00:05:00Z").expect("stale"),
            StableVersion::new("heartbeat_policy_version", "2026.07.15").expect("version"),
        )
        .expect("policy")
    }

    fn lease_policy() -> LeasePolicy {
        LeasePolicy::new(
            false,
            StableVersion::new("lease_policy_version", "2026.07.15").expect("version"),
        )
    }

    fn recovery_reference(agent_id: &str) -> AgentRecoveryReference {
        AgentRecoveryReference::new(
            AgentId::new(agent_id).expect("agent"),
            AgentRecoveryPlanReference::new("recovery-plan-001").expect("plan"),
            owner(),
            AgentRecoveryEvidenceReference::new("recovery-evidence-001").expect("evidence"),
        )
    }

    fn failure_observation(
        agent_id: &str,
        severity: AgentFailureSeverity,
        recovery_eligible: bool,
    ) -> RuntimeFailureObservation {
        RuntimeFailureObservation::new(
            AgentFailureReference::new(
                AgentId::new(agent_id).expect("agent"),
                AgentFailureCategory::HeartbeatFailure,
                severity,
                recovery_eligible,
            )
            .expect("failure"),
            TimeReference::new("2026-07-15T00:15:00Z").expect("observed"),
            "FAIL-EVID-001",
        )
        .expect("observation")
    }

    fn registration(
        agent_id: &str,
        capability_id: &str,
        runtime_id: &str,
        presence: PresenceState,
        lifecycle: AgentLifecycle,
        lease: Option<LeaseRecord>,
        heartbeat: Option<HeartbeatRecord>,
    ) -> AgentRegistration {
        AgentRegistration::new(AgentRegistrationSpec {
            agent: agent_definition(agent_id, "CX-PROJ-000001", lifecycle),
            runtime: runtime_entity(runtime_id, "CX-PROJ-000001"),
            supervisor: owner(),
            capabilities: vec![capability(capability_id)],
            presence_state: presence,
            health: RuntimeHealth::Healthy,
            registered_at: TimeReference::new("2026-07-15T00:00:00Z").expect("registered"),
            lease,
            last_heartbeat: heartbeat,
        })
        .expect("registration")
    }

    fn governed_registration(agent_id: &str, runtime_id: &str) -> AgentRegistration {
        registration(
            agent_id,
            "CX-CAP-000001",
            runtime_id,
            PresenceState::Registered,
            AgentLifecycle::Registered,
            Some(governed_lease(
                "CX-LEASE-000001",
                runtime_id,
                agent_id,
                "2026-07-15T00:00:00Z",
                "2026-07-15T01:00:00Z",
                None,
            )),
            None,
        )
    }

    fn registry_with_retired_runtime(agent_id: &str, runtime_id: &str) -> (AgentRegistry, AgentId) {
        let mut registry = AgentRegistry::new();
        let registration = governed_registration(agent_id, runtime_id);
        let agent_id = registration.agent_id().clone();
        registry.register(registration).expect("register");
        registry
            .transition_presence(&agent_id, PresenceState::Retired)
            .expect("retire presence");
        assert_eq!(
            registry.lookup(&agent_id).expect("lookup").presence_state(),
            PresenceState::Retired
        );
        (registry, agent_id)
    }

    #[test]
    fn runtime_registration_is_stable_and_lookup_is_deterministic_ces_b0_027_8() {
        let mut registry = AgentRegistry::new();
        let registration = registration(
            "CX-AGT-000001",
            "CX-CAP-000001",
            "runtime.primary",
            PresenceState::Registered,
            AgentLifecycle::Registered,
            Some(lease("runtime.primary", "2026-07-15T01:00:00Z")),
            None,
        );
        let agent_id = registration.agent_id().clone();
        registry.register(registration).expect("register");
        let lookup = registry.lookup(&agent_id).expect("lookup");
        assert_eq!(lookup.agent_id().as_str(), "CX-AGT-000001");
    }

    #[test]
    fn runtime_registration_rejects_duplicate_agent_identity_ces_b0_027_8() {
        let mut registry = AgentRegistry::new();
        registry
            .register(governed_registration("CX-AGT-000001", "runtime.primary"))
            .expect("first registration");
        let error = registry
            .register(governed_registration("CX-AGT-000001", "runtime.primary"))
            .expect_err("duplicate registration must fail");
        assert!(error
            .to_string()
            .contains("agent registration requires unique agent identity"));
    }

    #[test]
    fn runtime_capability_lookup_uses_indexed_registration_state_ces_b0_027_9() {
        let mut registry = AgentRegistry::new();
        registry
            .register(governed_registration("CX-AGT-000001", "runtime.primary"))
            .expect("register one");
        registry
            .register(registration(
                "CX-AGT-000002",
                "CX-CAP-000001",
                "runtime.primary",
                PresenceState::Registered,
                AgentLifecycle::Registered,
                Some(governed_lease(
                    "CX-LEASE-000002",
                    "runtime.primary",
                    "CX-AGT-000002",
                    "2026-07-15T00:00:00Z",
                    "2026-07-15T01:00:00Z",
                    None,
                )),
                None,
            ))
            .expect("register two");
        let registrations = registry
            .registrations_for_capability(&CapabilityId::new("CX-CAP-000001").expect("cap"))
            .iter()
            .map(|registration| registration.agent_id().to_string())
            .collect::<Vec<_>>();
        assert_eq!(
            registrations,
            vec!["CX-AGT-000001".to_owned(), "CX-AGT-000002".to_owned()]
        );
    }

    #[test]
    fn runtime_runtime_lookup_groups_registrations_by_runtime_id_k4_1() {
        let mut registry = AgentRegistry::new();
        registry
            .register(governed_registration("CX-AGT-000001", "runtime.primary"))
            .expect("register one");
        registry
            .register(registration(
                "CX-AGT-000002",
                "CX-CAP-000002",
                "runtime.secondary",
                PresenceState::Registered,
                AgentLifecycle::Registered,
                Some(governed_lease(
                    "CX-LEASE-000002",
                    "runtime.secondary",
                    "CX-AGT-000002",
                    "2026-07-15T00:00:00Z",
                    "2026-07-15T01:00:00Z",
                    None,
                )),
                None,
            ))
            .expect("register two");
        assert_eq!(
            registry
                .registrations_for_runtime(&RuntimeId::new("runtime.primary").expect("runtime"))
                .len(),
            1
        );
    }

    #[test]
    fn runtime_presence_follows_k4_1_transition_order() {
        let mut registry = AgentRegistry::new();
        let registration = governed_registration("CX-AGT-000001", "runtime.primary");
        let agent_id = registration.agent_id().clone();
        registry.register(registration).expect("register");
        registry
            .transition_presence(&agent_id, PresenceState::Ready)
            .expect("ready");
        registry
            .transition_presence(&agent_id, PresenceState::Idle)
            .expect("idle");
        registry
            .transition_presence(&agent_id, PresenceState::Working)
            .expect("working");
        assert_eq!(
            registry.lookup(&agent_id).expect("lookup").presence_state(),
            PresenceState::Working
        );
    }

    #[test]
    fn runtime_presence_rejects_invalid_transition_k4_1() {
        let mut registry = AgentRegistry::new();
        let registration = governed_registration("CX-AGT-000001", "runtime.primary");
        let agent_id = registration.agent_id().clone();
        registry.register(registration).expect("register");
        let error = registry
            .transition_presence(&agent_id, PresenceState::Working)
            .expect_err("registered must not jump directly to working");
        assert!(error
            .to_string()
            .contains("presence transition is not allowed"));
    }

    #[test]
    fn runtime_lease_validation_detects_current_and_expired_leases_ces_b0_027_7() {
        let registration = governed_registration("CX-AGT-000001", "runtime.primary");
        let assessment = evaluate_lease(
            &TimeReference::new("2026-07-15T00:30:00Z").expect("time"),
            registration.lease(),
            registration.runtime().runtime_id(),
            registration.agent_id(),
        );
        assert_eq!(assessment.validity(), LeaseValidity::Valid);
        let expired = evaluate_lease(
            &TimeReference::new("2026-07-15T02:00:00Z").expect("time"),
            registration.lease(),
            registration.runtime().runtime_id(),
            registration.agent_id(),
        );
        assert_eq!(expired.validity(), LeaseValidity::Expired);
    }

    #[test]
    fn runtime_lease_renewal_rejects_runtime_mismatch_k4_2() {
        let mut registry = AgentRegistry::new();
        let registration = governed_registration("CX-AGT-000001", "runtime.primary");
        let agent_id = registration.agent_id().clone();
        registry.register(registration).expect("register");
        let result = registry
            .renew_lease_validated(
                &agent_id,
                &LeaseRenewalRequest::new(
                    LeaseId::new("CX-LEASE-000001").expect("lease"),
                    governed_lease(
                        "CX-LEASE-000002",
                        "runtime.secondary",
                        "CX-AGT-000001",
                        "2026-07-15T00:30:00Z",
                        "2026-07-15T02:00:00Z",
                        Some("CX-LEASE-000001"),
                    ),
                    TimeReference::new("2026-07-15T00:30:00Z").expect("time"),
                ),
                &lease_policy(),
            )
            .expect("result");
        assert_eq!(result.outcome(), LeaseRenewalOutcome::LeaseRuntimeMismatch);
    }

    #[test]
    fn runtime_heartbeat_updates_last_seen_state_ces_b0_027_10() {
        let mut registry = AgentRegistry::new();
        let registration = governed_registration("CX-AGT-000001", "runtime.primary");
        let agent_id = registration.agent_id().clone();
        registry.register(registration).expect("register");
        let result = registry
            .record_heartbeat_validated(
                &agent_id,
                governed_heartbeat(
                    "CX-HB-000001",
                    "runtime.primary",
                    "CX-AGT-000001",
                    "2026-07-15T00:10:00Z",
                    "2026-07-15T00:20:00Z",
                    Some("CX-LEASE-000001"),
                    PresenceState::Registered,
                    RuntimeHealth::Healthy,
                ),
                &TimeReference::new("2026-07-15T00:10:00Z").expect("now"),
                &heartbeat_policy(),
            )
            .expect("heartbeat");
        assert_eq!(result.outcome(), HeartbeatRecordOutcome::HeartbeatAccepted);
        assert_eq!(
            registry
                .lookup(&agent_id)
                .expect("lookup")
                .last_heartbeat()
                .expect("heartbeat")
                .recorded_at()
                .as_str(),
            "2026-07-15T00:10:00Z"
        );
    }

    #[test]
    fn runtime_heartbeat_rejects_runtime_mismatch_k4_2() {
        let mut registry = AgentRegistry::new();
        let registration = governed_registration("CX-AGT-000001", "runtime.primary");
        let agent_id = registration.agent_id().clone();
        registry.register(registration).expect("register");
        let result = registry
            .record_heartbeat_validated(
                &agent_id,
                governed_heartbeat(
                    "CX-HB-000001",
                    "runtime.secondary",
                    "CX-AGT-000001",
                    "2026-07-15T00:10:00Z",
                    "2026-07-15T00:20:00Z",
                    Some("CX-LEASE-000001"),
                    PresenceState::Registered,
                    RuntimeHealth::Healthy,
                ),
                &TimeReference::new("2026-07-15T00:10:00Z").expect("now"),
                &heartbeat_policy(),
            )
            .expect("result");
        assert_eq!(
            result.outcome(),
            HeartbeatRecordOutcome::HeartbeatRuntimeMismatch
        );
    }

    #[test]
    fn runtime_deregistration_removes_capability_indexes_ces_b0_027_9() {
        let mut registry = AgentRegistry::new();
        let registration = governed_registration("CX-AGT-000001", "runtime.primary");
        let agent_id = registration.agent_id().clone();
        registry.register(registration).expect("register");
        let retired = registry.deregister(&agent_id).expect("deregister");
        assert_eq!(retired.presence_state(), PresenceState::Retired);
        assert!(registry.lookup(&agent_id).is_none());
        assert!(registry
            .registrations_for_capability(&CapabilityId::new("CX-CAP-000001").expect("cap"))
            .is_empty());
    }

    #[test]
    fn runtime_registration_requires_registered_initial_presence_k4_1() {
        let error = AgentRegistration::new(AgentRegistrationSpec {
            agent: agent_definition(
                "CX-AGT-000001",
                "CX-PROJ-000001",
                AgentLifecycle::Registered,
            ),
            runtime: runtime_entity("runtime.primary", "CX-PROJ-000001"),
            supervisor: owner(),
            capabilities: vec![capability("CX-CAP-000001")],
            presence_state: PresenceState::Ready,
            health: RuntimeHealth::Healthy,
            registered_at: TimeReference::new("2026-07-15T00:00:00Z").expect("registered"),
            lease: Some(lease("runtime.primary", "2026-07-15T01:00:00Z")),
            last_heartbeat: None,
        })
        .expect_err("registrations must begin in registered state");
        assert!(error
            .to_string()
            .contains("new agent registrations must begin in Registered state"));
    }

    #[test]
    fn runtime_registration_requires_capability_descriptors_ces_b0_027_5() {
        let error = AgentRegistration::new(AgentRegistrationSpec {
            agent: agent_definition(
                "CX-AGT-000001",
                "CX-PROJ-000001",
                AgentLifecycle::Registered,
            ),
            runtime: runtime_entity("runtime.primary", "CX-PROJ-000001"),
            supervisor: owner(),
            capabilities: vec![],
            presence_state: PresenceState::Registered,
            health: RuntimeHealth::Healthy,
            registered_at: TimeReference::new("2026-07-15T00:00:00Z").expect("registered"),
            lease: Some(lease("runtime.primary", "2026-07-15T01:00:00Z")),
            last_heartbeat: None,
        })
        .expect_err("registrations require capabilities");
        assert!(error
            .to_string()
            .contains("agent registration requires at least one capability"));
    }

    #[test]
    fn runtime_heartbeat_freshness_is_deterministic_ces_b0_027_10() {
        let heartbeat = heartbeat("runtime.primary", "2026-07-15T00:20:00Z");
        assert!(heartbeat.is_fresh_at(&TimeReference::new("2026-07-15T00:19:59Z").expect("time")));
        assert!(!heartbeat.is_fresh_at(&TimeReference::new("2026-07-15T00:20:01Z").expect("time")));
    }

    #[test]
    fn runtime_valid_heartbeat_accepted_ces_b0_027_10() {
        let assessment = assess_heartbeat(
            &TimeReference::new("2026-07-15T00:10:00Z").expect("now"),
            Some(&governed_heartbeat(
                "CX-HB-000001",
                "runtime.primary",
                "CX-AGT-000001",
                "2026-07-15T00:10:00Z",
                "2026-07-15T00:20:00Z",
                Some("CX-LEASE-000001"),
                PresenceState::Registered,
                RuntimeHealth::Healthy,
            )),
            &heartbeat_policy(),
        )
        .expect("assessment");
        assert_eq!(assessment.freshness(), HeartbeatFreshness::Fresh);
    }

    #[test]
    fn runtime_heartbeat_agent_mismatch_rejected_ces_b0_027_21() {
        let mut registry = AgentRegistry::new();
        let registration = governed_registration("CX-AGT-000001", "runtime.primary");
        let agent_id = registration.agent_id().clone();
        registry.register(registration).expect("register");
        let result = registry
            .record_heartbeat_validated(
                &agent_id,
                governed_heartbeat(
                    "CX-HB-000001",
                    "runtime.primary",
                    "CX-AGT-000002",
                    "2026-07-15T00:10:00Z",
                    "2026-07-15T00:20:00Z",
                    Some("CX-LEASE-000001"),
                    PresenceState::Registered,
                    RuntimeHealth::Healthy,
                ),
                &TimeReference::new("2026-07-15T00:10:00Z").expect("now"),
                &heartbeat_policy(),
            )
            .expect("result");
        assert_eq!(
            result.outcome(),
            HeartbeatRecordOutcome::HeartbeatAgentMismatch
        );
    }

    #[test]
    fn runtime_stale_heartbeat_rejected_ces_b0_027_21() {
        let mut registry = AgentRegistry::new();
        let registration = governed_registration("CX-AGT-000001", "runtime.primary");
        let agent_id = registration.agent_id().clone();
        registry.register(registration).expect("register");
        let result = registry
            .record_heartbeat_validated(
                &agent_id,
                governed_heartbeat(
                    "CX-HB-000001",
                    "runtime.primary",
                    "CX-AGT-000001",
                    "2026-07-15T00:00:01Z",
                    "2026-07-15T00:05:00Z",
                    Some("CX-LEASE-000001"),
                    PresenceState::Registered,
                    RuntimeHealth::Healthy,
                ),
                &TimeReference::new("2026-07-15T00:10:00Z").expect("now"),
                &heartbeat_policy(),
            )
            .expect("result");
        assert_eq!(result.outcome(), HeartbeatRecordOutcome::HeartbeatStale);
    }

    #[test]
    fn runtime_duplicate_heartbeat_is_deterministic_ces_b0_027_21() {
        let mut registry = AgentRegistry::new();
        let registration = governed_registration("CX-AGT-000001", "runtime.primary");
        let agent_id = registration.agent_id().clone();
        registry.register(registration).expect("register");
        let heartbeat = governed_heartbeat(
            "CX-HB-000001",
            "runtime.primary",
            "CX-AGT-000001",
            "2026-07-15T00:10:00Z",
            "2026-07-15T00:20:00Z",
            Some("CX-LEASE-000001"),
            PresenceState::Registered,
            RuntimeHealth::Healthy,
        );
        let left = registry
            .record_heartbeat_validated(
                &agent_id,
                heartbeat.clone(),
                &TimeReference::new("2026-07-15T00:10:00Z").expect("now"),
                &heartbeat_policy(),
            )
            .expect("first");
        let right = registry
            .record_heartbeat_validated(
                &agent_id,
                heartbeat,
                &TimeReference::new("2026-07-15T00:10:00Z").expect("now"),
                &heartbeat_policy(),
            )
            .expect("second");
        assert_eq!(left.outcome(), HeartbeatRecordOutcome::HeartbeatAccepted);
        assert_eq!(right.outcome(), HeartbeatRecordOutcome::HeartbeatDuplicate);
    }

    #[test]
    fn runtime_heartbeat_timestamp_regression_rejected_ces_b0_027_21() {
        let mut registry = AgentRegistry::new();
        let registration = governed_registration("CX-AGT-000001", "runtime.primary");
        let agent_id = registration.agent_id().clone();
        registry.register(registration).expect("register");
        let _ = registry
            .record_heartbeat_validated(
                &agent_id,
                governed_heartbeat(
                    "CX-HB-000002",
                    "runtime.primary",
                    "CX-AGT-000001",
                    "2026-07-15T00:10:00Z",
                    "2026-07-15T00:20:00Z",
                    Some("CX-LEASE-000001"),
                    PresenceState::Registered,
                    RuntimeHealth::Healthy,
                ),
                &TimeReference::new("2026-07-15T00:10:00Z").expect("now"),
                &heartbeat_policy(),
            )
            .expect("first");
        let result = registry
            .record_heartbeat_validated(
                &agent_id,
                governed_heartbeat(
                    "CX-HB-000003",
                    "runtime.primary",
                    "CX-AGT-000001",
                    "2026-07-15T00:09:59Z",
                    "2026-07-15T00:20:00Z",
                    Some("CX-LEASE-000001"),
                    PresenceState::Registered,
                    RuntimeHealth::Healthy,
                ),
                &TimeReference::new("2026-07-15T00:10:00Z").expect("now"),
                &heartbeat_policy(),
            )
            .expect("result");
        assert_eq!(
            result.outcome(),
            HeartbeatRecordOutcome::HeartbeatTimestampRegression
        );
    }

    #[test]
    fn runtime_heartbeat_for_retired_runtime_rejected_ces_b0_027_7() {
        let (mut registry, agent_id) =
            registry_with_retired_runtime("CX-AGT-000001", "runtime.primary");
        let result = registry
            .record_heartbeat_validated(
                &agent_id,
                governed_heartbeat(
                    "CX-HB-000001",
                    "runtime.primary",
                    "CX-AGT-000001",
                    "2026-07-15T00:10:00Z",
                    "2026-07-15T00:20:00Z",
                    Some("CX-LEASE-000001"),
                    PresenceState::Retired,
                    RuntimeHealth::Critical,
                ),
                &TimeReference::new("2026-07-15T00:10:00Z").expect("now"),
                &heartbeat_policy(),
            )
            .expect("result");
        assert_eq!(
            result.outcome(),
            HeartbeatRecordOutcome::HeartbeatRuntimeRetired
        );
    }

    #[test]
    fn runtime_retired_heartbeat_rejection_precedes_duplicate_and_preserves_registry_ces_b0_027_21()
    {
        let (mut registry, agent_id) =
            registry_with_retired_runtime("CX-AGT-000001", "runtime.primary");
        let retired_heartbeat = governed_heartbeat(
            "CX-HB-000001",
            "runtime.primary",
            "CX-AGT-000001",
            "2026-07-15T00:10:00Z",
            "2026-07-15T00:20:00Z",
            Some("CX-LEASE-000001"),
            PresenceState::Retired,
            RuntimeHealth::Critical,
        );
        let before = registry.lookup(&agent_id).expect("lookup").clone();

        let left = registry
            .record_heartbeat_validated(
                &agent_id,
                retired_heartbeat.clone(),
                &TimeReference::new("2026-07-15T00:10:00Z").expect("now"),
                &heartbeat_policy(),
            )
            .expect("left");
        let right = registry
            .record_heartbeat_validated(
                &agent_id,
                retired_heartbeat,
                &TimeReference::new("2026-07-15T00:10:00Z").expect("now"),
                &heartbeat_policy(),
            )
            .expect("right");

        assert_eq!(
            left.outcome(),
            HeartbeatRecordOutcome::HeartbeatRuntimeRetired
        );
        assert_eq!(
            right.outcome(),
            HeartbeatRecordOutcome::HeartbeatRuntimeRetired
        );
        assert_eq!(registry.lookup(&agent_id).expect("lookup"), &before);
    }

    #[test]
    fn runtime_heartbeat_does_not_change_immutable_identity_ces_b0_027_2() {
        let mut registry = AgentRegistry::new();
        let registration = governed_registration("CX-AGT-000001", "runtime.primary");
        let agent_id = registration.agent_id().clone();
        let runtime_id = registration.runtime().runtime_id().clone();
        registry.register(registration).expect("register");
        let _ = registry
            .record_heartbeat_validated(
                &agent_id,
                governed_heartbeat(
                    "CX-HB-000001",
                    "runtime.primary",
                    "CX-AGT-000001",
                    "2026-07-15T00:10:00Z",
                    "2026-07-15T00:20:00Z",
                    Some("CX-LEASE-000001"),
                    PresenceState::Registered,
                    RuntimeHealth::Healthy,
                ),
                &TimeReference::new("2026-07-15T00:10:00Z").expect("now"),
                &heartbeat_policy(),
            )
            .expect("result");
        let updated = registry.lookup(&agent_id).expect("lookup");
        assert_eq!(updated.agent_id(), &agent_id);
        assert_eq!(updated.runtime().runtime_id(), &runtime_id);
    }

    #[test]
    fn runtime_freshness_classifies_fresh_late_stale_and_missing_ces_b0_027_10() {
        let now = TimeReference::new("2026-07-15T00:10:00Z").expect("now");
        assert_eq!(
            assess_heartbeat(
                &now,
                Some(&governed_heartbeat(
                    "CX-HB-000001",
                    "runtime.primary",
                    "CX-AGT-000001",
                    "2026-07-15T00:10:00Z",
                    "2026-07-15T00:20:00Z",
                    Some("CX-LEASE-000001"),
                    PresenceState::Registered,
                    RuntimeHealth::Healthy
                )),
                &heartbeat_policy(),
            )
            .expect("fresh")
            .freshness(),
            HeartbeatFreshness::Fresh
        );
        assert_eq!(
            assess_heartbeat(
                &now,
                Some(&governed_heartbeat(
                    "CX-HB-000002",
                    "runtime.primary",
                    "CX-AGT-000001",
                    "2026-07-15T00:08:59Z",
                    "2026-07-15T00:20:00Z",
                    Some("CX-LEASE-000001"),
                    PresenceState::Registered,
                    RuntimeHealth::Healthy
                )),
                &heartbeat_policy(),
            )
            .expect("late")
            .freshness(),
            HeartbeatFreshness::Late
        );
        assert_eq!(
            assess_heartbeat(
                &now,
                Some(&governed_heartbeat(
                    "CX-HB-000003",
                    "runtime.primary",
                    "CX-AGT-000001",
                    "2026-07-15T00:04:59Z",
                    "2026-07-15T00:20:00Z",
                    Some("CX-LEASE-000001"),
                    PresenceState::Registered,
                    RuntimeHealth::Healthy
                )),
                &heartbeat_policy(),
            )
            .expect("stale")
            .freshness(),
            HeartbeatFreshness::Stale
        );
        assert_eq!(
            assess_heartbeat(&now, None, &heartbeat_policy())
                .expect("missing")
                .freshness(),
            HeartbeatFreshness::Missing
        );
    }

    #[test]
    fn runtime_invalid_freshness_policy_rejected_ces_b0_027_21() {
        let error = HeartbeatFreshnessPolicy::new(
            TimeReference::new("2026-07-15T00:05:00Z").expect("late"),
            TimeReference::new("2026-07-15T00:09:00Z").expect("stale"),
            StableVersion::new("heartbeat_policy_version", "2026.07.15").expect("version"),
        )
        .expect_err("invalid policy");
        assert!(error
            .to_string()
            .contains("stale cutoff must not be later than the late cutoff"));
    }

    #[test]
    fn runtime_repeated_freshness_assessment_is_deterministic_ces_b0_027_21() {
        let now = TimeReference::new("2026-07-15T00:10:00Z").expect("now");
        let heartbeat = governed_heartbeat(
            "CX-HB-000001",
            "runtime.primary",
            "CX-AGT-000001",
            "2026-07-15T00:08:59Z",
            "2026-07-15T00:20:00Z",
            Some("CX-LEASE-000001"),
            PresenceState::Registered,
            RuntimeHealth::Healthy,
        );
        let left = assess_heartbeat(&now, Some(&heartbeat), &heartbeat_policy()).expect("left");
        let right = assess_heartbeat(&now, Some(&heartbeat), &heartbeat_policy()).expect("right");
        assert_eq!(left, right);
    }

    #[test]
    fn runtime_valid_lease_is_detected_ces_b0_027_7() {
        let lease = governed_lease(
            "CX-LEASE-000001",
            "runtime.primary",
            "CX-AGT-000001",
            "2026-07-15T00:00:00Z",
            "2026-07-15T01:00:00Z",
            None,
        );
        let assessment = evaluate_lease(
            &TimeReference::new("2026-07-15T00:30:00Z").expect("now"),
            Some(&lease),
            &RuntimeId::new("runtime.primary").expect("runtime"),
            &AgentId::new("CX-AGT-000001").expect("agent"),
        );
        assert_eq!(assessment.validity(), LeaseValidity::Valid);
    }

    #[test]
    fn runtime_expired_lease_is_detected_ces_b0_027_7() {
        let lease = governed_lease(
            "CX-LEASE-000001",
            "runtime.primary",
            "CX-AGT-000001",
            "2026-07-15T00:00:00Z",
            "2026-07-15T00:30:00Z",
            None,
        );
        let assessment = evaluate_lease(
            &TimeReference::new("2026-07-15T00:30:00Z").expect("now"),
            Some(&lease),
            &RuntimeId::new("runtime.primary").expect("runtime"),
            &AgentId::new("CX-AGT-000001").expect("agent"),
        );
        assert_eq!(assessment.validity(), LeaseValidity::Expired);
    }

    #[test]
    fn runtime_lease_agent_mismatch_detected_ces_b0_027_21() {
        let lease = governed_lease(
            "CX-LEASE-000001",
            "runtime.primary",
            "CX-AGT-000002",
            "2026-07-15T00:00:00Z",
            "2026-07-15T01:00:00Z",
            None,
        );
        let assessment = evaluate_lease(
            &TimeReference::new("2026-07-15T00:30:00Z").expect("now"),
            Some(&lease),
            &RuntimeId::new("runtime.primary").expect("runtime"),
            &AgentId::new("CX-AGT-000001").expect("agent"),
        );
        assert_eq!(assessment.validity(), LeaseValidity::Invalid);
    }

    #[test]
    fn runtime_valid_lease_renewal_succeeds_ces_b0_027_7() {
        let mut registry = AgentRegistry::new();
        let registration = governed_registration("CX-AGT-000001", "runtime.primary");
        let agent_id = registration.agent_id().clone();
        registry.register(registration).expect("register");
        let result = registry
            .renew_lease_validated(
                &agent_id,
                &LeaseRenewalRequest::new(
                    LeaseId::new("CX-LEASE-000001").expect("lease"),
                    governed_lease(
                        "CX-LEASE-000002",
                        "runtime.primary",
                        "CX-AGT-000001",
                        "2026-07-15T00:30:00Z",
                        "2026-07-15T02:00:00Z",
                        Some("CX-LEASE-000001"),
                    ),
                    TimeReference::new("2026-07-15T00:30:00Z").expect("time"),
                ),
                &lease_policy(),
            )
            .expect("result");
        assert_eq!(result.outcome(), LeaseRenewalOutcome::LeaseRenewed);
    }

    #[test]
    fn runtime_lease_renewal_after_expiration_rejected_ces_b0_027_7() {
        let mut registry = AgentRegistry::new();
        let registration = governed_registration("CX-AGT-000001", "runtime.primary");
        let agent_id = registration.agent_id().clone();
        registry.register(registration).expect("register");
        let result = registry
            .renew_lease_validated(
                &agent_id,
                &LeaseRenewalRequest::new(
                    LeaseId::new("CX-LEASE-000001").expect("lease"),
                    governed_lease(
                        "CX-LEASE-000002",
                        "runtime.primary",
                        "CX-AGT-000001",
                        "2026-07-15T01:00:01Z",
                        "2026-07-15T02:00:00Z",
                        Some("CX-LEASE-000001"),
                    ),
                    TimeReference::new("2026-07-15T01:00:01Z").expect("time"),
                ),
                &lease_policy(),
            )
            .expect("result");
        assert_eq!(result.outcome(), LeaseRenewalOutcome::LeaseExpired);
    }

    #[test]
    fn runtime_lease_renewal_for_retired_runtime_rejected_ces_b0_027_19() {
        let (mut registry, agent_id) =
            registry_with_retired_runtime("CX-AGT-000001", "runtime.primary");
        let result = registry
            .renew_lease_validated(
                &agent_id,
                &LeaseRenewalRequest::new(
                    LeaseId::new("CX-LEASE-000001").expect("lease"),
                    governed_lease(
                        "CX-LEASE-000002",
                        "runtime.primary",
                        "CX-AGT-000001",
                        "2026-07-15T00:30:00Z",
                        "2026-07-15T02:00:00Z",
                        Some("CX-LEASE-000001"),
                    ),
                    TimeReference::new("2026-07-15T00:30:00Z").expect("time"),
                ),
                &lease_policy(),
            )
            .expect("result");
        assert_eq!(result.outcome(), LeaseRenewalOutcome::RuntimeAlreadyRetired);
    }

    #[test]
    fn runtime_retired_lease_rejection_precedes_duplicate_and_preserves_registry_ces_b0_027_22() {
        let (mut registry, agent_id) =
            registry_with_retired_runtime("CX-AGT-000001", "runtime.primary");
        let current_lease = registry
            .lookup(&agent_id)
            .expect("lookup")
            .lease()
            .expect("lease")
            .clone();
        let before = registry.lookup(&agent_id).expect("lookup").clone();

        let request = LeaseRenewalRequest::new(
            LeaseId::new("CX-LEASE-000001").expect("lease"),
            current_lease,
            TimeReference::new("2026-07-15T00:30:00Z").expect("time"),
        );
        let left = registry
            .renew_lease_validated(&agent_id, &request, &lease_policy())
            .expect("left");
        let right = registry
            .renew_lease_validated(&agent_id, &request, &lease_policy())
            .expect("right");

        assert_eq!(left.outcome(), LeaseRenewalOutcome::RuntimeAlreadyRetired);
        assert_eq!(right.outcome(), LeaseRenewalOutcome::RuntimeAlreadyRetired);
        assert_eq!(registry.lookup(&agent_id).expect("lookup"), &before);
    }

    #[test]
    fn runtime_lease_sequence_regression_rejected_ces_b0_027_21() {
        let mut registry = AgentRegistry::new();
        let registration = governed_registration("CX-AGT-000001", "runtime.primary");
        let agent_id = registration.agent_id().clone();
        registry.register(registration).expect("register");
        let result = registry
            .renew_lease_validated(
                &agent_id,
                &LeaseRenewalRequest::new(
                    LeaseId::new("CX-LEASE-999999").expect("lease"),
                    governed_lease(
                        "CX-LEASE-000002",
                        "runtime.primary",
                        "CX-AGT-000001",
                        "2026-07-15T00:30:00Z",
                        "2026-07-15T02:00:00Z",
                        Some("CX-LEASE-000001"),
                    ),
                    TimeReference::new("2026-07-15T00:30:00Z").expect("time"),
                ),
                &lease_policy(),
            )
            .expect("result");
        assert_eq!(
            result.outcome(),
            LeaseRenewalOutcome::LeaseSequenceRegression
        );
    }

    #[test]
    fn runtime_health_healthy_requires_fresh_heartbeat_and_valid_lease_ces_b0_027_12() {
        let lease = LeaseAssessment {
            validity: LeaseValidity::Valid,
            evaluated_at: TimeReference::new("2026-07-15T00:10:00Z").expect("time"),
            lease_id: Some(LeaseId::new("CX-LEASE-000001").expect("lease")),
            rejection_reason: None,
        };
        assert_eq!(
            assess_runtime_health(PresenceState::Idle, HeartbeatFreshness::Fresh, &lease, None,),
            RuntimeHealth::Healthy
        );
    }

    #[test]
    fn runtime_health_stale_heartbeat_becomes_critical_ces_b0_027_18() {
        let lease = LeaseAssessment {
            validity: LeaseValidity::Valid,
            evaluated_at: TimeReference::new("2026-07-15T00:10:00Z").expect("time"),
            lease_id: Some(LeaseId::new("CX-LEASE-000001").expect("lease")),
            rejection_reason: None,
        };
        assert_eq!(
            assess_runtime_health(PresenceState::Idle, HeartbeatFreshness::Stale, &lease, None,),
            RuntimeHealth::Critical
        );
    }

    #[test]
    fn runtime_health_expired_lease_affects_health_ces_b0_027_7() {
        let lease = LeaseAssessment {
            validity: LeaseValidity::Expired,
            evaluated_at: TimeReference::new("2026-07-15T00:10:00Z").expect("time"),
            lease_id: Some(LeaseId::new("CX-LEASE-000001").expect("lease")),
            rejection_reason: Some(super::LeaseRejectionReason::LeaseExpired),
        };
        assert_eq!(
            assess_runtime_health(PresenceState::Idle, HeartbeatFreshness::Fresh, &lease, None,),
            RuntimeHealth::Critical
        );
    }

    #[test]
    fn runtime_health_idle_is_not_failure_ces_b0_027_15() {
        let lease = LeaseAssessment {
            validity: LeaseValidity::Valid,
            evaluated_at: TimeReference::new("2026-07-15T00:10:00Z").expect("time"),
            lease_id: Some(LeaseId::new("CX-LEASE-000001").expect("lease")),
            rejection_reason: None,
        };
        assert_eq!(
            assess_runtime_health(PresenceState::Idle, HeartbeatFreshness::Fresh, &lease, None,),
            RuntimeHealth::Healthy
        );
    }

    #[test]
    fn runtime_health_recovery_state_is_represented_ces_b0_027_19() {
        let lease = LeaseAssessment {
            validity: LeaseValidity::Valid,
            evaluated_at: TimeReference::new("2026-07-15T00:10:00Z").expect("time"),
            lease_id: Some(LeaseId::new("CX-LEASE-000001").expect("lease")),
            rejection_reason: None,
        };
        assert_eq!(
            assess_runtime_health(
                PresenceState::Recovering,
                HeartbeatFreshness::Fresh,
                &lease,
                None,
            ),
            RuntimeHealth::Degraded
        );
    }

    #[test]
    fn runtime_health_assessment_is_deterministic_ces_b0_027_21() {
        let lease = LeaseAssessment {
            validity: LeaseValidity::Valid,
            evaluated_at: TimeReference::new("2026-07-15T00:10:00Z").expect("time"),
            lease_id: Some(LeaseId::new("CX-LEASE-000001").expect("lease")),
            rejection_reason: None,
        };
        let left =
            assess_runtime_health(PresenceState::Idle, HeartbeatFreshness::Late, &lease, None);
        let right =
            assess_runtime_health(PresenceState::Idle, HeartbeatFreshness::Late, &lease, None);
        assert_eq!(left, right);
    }

    #[test]
    fn runtime_supervisor_healthy_runtime_recommends_no_action_ces_b0_027_10() {
        let registration = governed_registration("CX-AGT-000001", "runtime.primary");
        let snapshot = super::RuntimeStateSnapshot {
            runtime_id: registration.runtime().runtime_id().clone(),
            agent_id: registration.agent_id().clone(),
            agent_lifecycle: AgentLifecycle::Registered,
            presence: PresenceState::Registered,
            health: RuntimeHealth::Healthy,
            current_lease_id: registration.lease().map(|lease| lease.lease_id().clone()),
            latest_heartbeat_id: Some(HeartbeatId::new("CX-HB-000001").expect("heartbeat")),
            heartbeat_freshness: HeartbeatFreshness::Fresh,
            capability_ids: vec![CapabilityId::new("CX-CAP-000001").expect("capability")],
            observed_at: TimeReference::new("2026-07-15T00:10:00Z").expect("time"),
            failure_observation: None,
            recovery_reference: None,
            lease_assessment: evaluate_lease(
                &TimeReference::new("2026-07-15T00:10:00Z").expect("time"),
                registration.lease(),
                registration.runtime().runtime_id(),
                registration.agent_id(),
            ),
        };
        let outcome = supervise_runtime(
            &snapshot,
            HeartbeatAssessment {
                freshness: HeartbeatFreshness::Fresh,
                evaluated_at: TimeReference::new("2026-07-15T00:10:00Z").expect("time"),
                heartbeat_id: Some(HeartbeatId::new("CX-HB-000001").expect("heartbeat")),
            },
            assess_recovery_eligibility(&snapshot, None, None),
        )
        .expect("outcome");
        assert_eq!(outcome.recommended_action(), SupervisorAction::NoAction);
    }

    #[test]
    fn runtime_supervisor_late_heartbeat_marks_late_ces_b0_027_10() {
        let mut registry = AgentRegistry::new();
        let mut registration = governed_registration("CX-AGT-000001", "runtime.primary");
        registration
            .transition_presence(PresenceState::Ready)
            .expect("ready");
        let agent_id = registration.agent_id().clone();
        registry.register(registration).expect("register");
        let outcome = registry
            .supervise_runtime(
                &agent_id,
                &TimeReference::new("2026-07-15T00:10:00Z").expect("now"),
                &heartbeat_policy(),
                None,
                None,
            )
            .expect("outcome");
        assert_eq!(
            outcome.recommended_action(),
            SupervisorAction::TransitionOffline
        );
    }

    #[test]
    fn runtime_supervisor_stale_or_missing_heartbeat_transitions_offline_ces_b0_027_10() {
        let registration = governed_registration("CX-AGT-000001", "runtime.primary");
        let snapshot = super::RuntimeStateSnapshot {
            runtime_id: registration.runtime().runtime_id().clone(),
            agent_id: registration.agent_id().clone(),
            agent_lifecycle: AgentLifecycle::Registered,
            presence: PresenceState::Ready,
            health: RuntimeHealth::Degraded,
            current_lease_id: registration.lease().map(|lease| lease.lease_id().clone()),
            latest_heartbeat_id: None,
            heartbeat_freshness: HeartbeatFreshness::Missing,
            capability_ids: vec![CapabilityId::new("CX-CAP-000001").expect("capability")],
            observed_at: TimeReference::new("2026-07-15T00:10:00Z").expect("time"),
            failure_observation: None,
            recovery_reference: None,
            lease_assessment: evaluate_lease(
                &TimeReference::new("2026-07-15T00:10:00Z").expect("time"),
                registration.lease(),
                registration.runtime().runtime_id(),
                registration.agent_id(),
            ),
        };
        let outcome = supervise_runtime(
            &snapshot,
            HeartbeatAssessment {
                freshness: HeartbeatFreshness::Missing,
                evaluated_at: TimeReference::new("2026-07-15T00:10:00Z").expect("time"),
                heartbeat_id: None,
            },
            assess_recovery_eligibility(&snapshot, None, None),
        )
        .expect("outcome");
        assert_eq!(
            outcome.recommended_action(),
            SupervisorAction::TransitionOffline
        );
        assert_eq!(outcome.proposed_presence(), PresenceState::Offline);
    }

    #[test]
    fn runtime_supervisor_expired_lease_transitions_offline_ces_b0_027_7() {
        let mut registry = AgentRegistry::new();
        let registration = governed_registration("CX-AGT-000001", "runtime.primary");
        let agent_id = registration.agent_id().clone();
        registry.register(registration).expect("register");
        let _ = registry
            .expire_lease(
                &agent_id,
                &TimeReference::new("2026-07-15T02:00:00Z").expect("time"),
            )
            .expect("expired");
        assert_eq!(
            registry.lookup(&agent_id).expect("lookup").presence_state(),
            PresenceState::Offline
        );
    }

    #[test]
    fn runtime_supervisor_retired_runtime_rejects_recovery_ces_b0_027_19() {
        let snapshot = super::RuntimeStateSnapshot {
            runtime_id: RuntimeId::new("runtime.primary").expect("runtime"),
            agent_id: AgentId::new("CX-AGT-000001").expect("agent"),
            agent_lifecycle: AgentLifecycle::Retired,
            presence: PresenceState::Retired,
            health: RuntimeHealth::Unknown,
            current_lease_id: None,
            latest_heartbeat_id: None,
            heartbeat_freshness: HeartbeatFreshness::Missing,
            capability_ids: vec![CapabilityId::new("CX-CAP-000001").expect("capability")],
            observed_at: TimeReference::new("2026-07-15T00:10:00Z").expect("time"),
            failure_observation: Some(failure_observation(
                "CX-AGT-000001",
                AgentFailureSeverity::Major,
                false,
            )),
            recovery_reference: Some(recovery_reference("CX-AGT-000001")),
            lease_assessment: LeaseAssessment {
                validity: LeaseValidity::Missing,
                evaluated_at: TimeReference::new("2026-07-15T00:10:00Z").expect("time"),
                lease_id: None,
                rejection_reason: Some(super::LeaseRejectionReason::LeaseMissing),
            },
        };
        let eligibility = assess_recovery_eligibility(
            &snapshot,
            snapshot.failure_observation.as_ref(),
            snapshot.recovery_reference.as_ref(),
        );
        assert_eq!(
            eligibility.rejection_reason(),
            Some(RecoveryRejectionReason::RuntimeAlreadyRetired)
        );
    }

    #[test]
    fn runtime_supervision_is_deterministic_for_same_input_ces_b0_027_21() {
        let registration = governed_registration("CX-AGT-000001", "runtime.primary");
        let snapshot = super::RuntimeStateSnapshot {
            runtime_id: registration.runtime().runtime_id().clone(),
            agent_id: registration.agent_id().clone(),
            agent_lifecycle: AgentLifecycle::Registered,
            presence: PresenceState::Ready,
            health: RuntimeHealth::Healthy,
            current_lease_id: registration.lease().map(|lease| lease.lease_id().clone()),
            latest_heartbeat_id: Some(HeartbeatId::new("CX-HB-000001").expect("heartbeat")),
            heartbeat_freshness: HeartbeatFreshness::Late,
            capability_ids: vec![CapabilityId::new("CX-CAP-000001").expect("capability")],
            observed_at: TimeReference::new("2026-07-15T00:10:00Z").expect("time"),
            failure_observation: None,
            recovery_reference: None,
            lease_assessment: evaluate_lease(
                &TimeReference::new("2026-07-15T00:10:00Z").expect("time"),
                registration.lease(),
                registration.runtime().runtime_id(),
                registration.agent_id(),
            ),
        };
        let left = supervise_runtime(
            &snapshot,
            HeartbeatAssessment {
                freshness: HeartbeatFreshness::Late,
                evaluated_at: TimeReference::new("2026-07-15T00:10:00Z").expect("time"),
                heartbeat_id: Some(HeartbeatId::new("CX-HB-000001").expect("heartbeat")),
            },
            assess_recovery_eligibility(&snapshot, None, None),
        )
        .expect("left");
        let right = supervise_runtime(
            &snapshot,
            HeartbeatAssessment {
                freshness: HeartbeatFreshness::Late,
                evaluated_at: TimeReference::new("2026-07-15T00:10:00Z").expect("time"),
                heartbeat_id: Some(HeartbeatId::new("CX-HB-000001").expect("heartbeat")),
            },
            assess_recovery_eligibility(&snapshot, None, None),
        )
        .expect("right");
        assert_eq!(left, right);
    }

    #[test]
    fn runtime_supervisor_does_not_execute_action_ces_b0_027_10() {
        let mut registry = AgentRegistry::new();
        let registration = governed_registration("CX-AGT-000001", "runtime.primary");
        let agent_id = registration.agent_id().clone();
        registry.register(registration).expect("register");
        let before = registry.lookup(&agent_id).expect("lookup").clone();
        let _ = registry
            .supervise_runtime(
                &agent_id,
                &TimeReference::new("2026-07-15T00:10:00Z").expect("now"),
                &heartbeat_policy(),
                None,
                None,
            )
            .expect("outcome");
        let after = registry.lookup(&agent_id).expect("lookup").clone();
        assert_eq!(before, after);
    }

    #[test]
    fn runtime_registry_rejected_heartbeat_leaves_state_unchanged_ces_b0_027_21() {
        let mut registry = AgentRegistry::new();
        let registration = governed_registration("CX-AGT-000001", "runtime.primary");
        let agent_id = registration.agent_id().clone();
        registry.register(registration).expect("register");
        let before = registry.lookup(&agent_id).expect("lookup").clone();
        let _ = registry
            .record_heartbeat_validated(
                &agent_id,
                governed_heartbeat(
                    "CX-HB-000001",
                    "runtime.secondary",
                    "CX-AGT-000001",
                    "2026-07-15T00:10:00Z",
                    "2026-07-15T00:20:00Z",
                    Some("CX-LEASE-000001"),
                    PresenceState::Registered,
                    RuntimeHealth::Healthy,
                ),
                &TimeReference::new("2026-07-15T00:10:00Z").expect("now"),
                &heartbeat_policy(),
            )
            .expect("result");
        let after = registry.lookup(&agent_id).expect("lookup").clone();
        assert_eq!(before, after);
    }

    #[test]
    fn runtime_registry_rejected_lease_renewal_leaves_state_unchanged_ces_b0_027_21() {
        let mut registry = AgentRegistry::new();
        let registration = governed_registration("CX-AGT-000001", "runtime.primary");
        let agent_id = registration.agent_id().clone();
        registry.register(registration).expect("register");
        let before = registry.lookup(&agent_id).expect("lookup").clone();
        let _ = registry
            .renew_lease_validated(
                &agent_id,
                &LeaseRenewalRequest::new(
                    LeaseId::new("CX-LEASE-999999").expect("lease"),
                    governed_lease(
                        "CX-LEASE-000002",
                        "runtime.primary",
                        "CX-AGT-000001",
                        "2026-07-15T00:30:00Z",
                        "2026-07-15T02:00:00Z",
                        Some("CX-LEASE-000001"),
                    ),
                    TimeReference::new("2026-07-15T00:30:00Z").expect("time"),
                ),
                &lease_policy(),
            )
            .expect("result");
        let after = registry.lookup(&agent_id).expect("lookup").clone();
        assert_eq!(before, after);
    }

    #[test]
    fn runtime_registry_supervisor_outcome_applies_only_after_validation_ces_b0_027_10() {
        let mut registry = AgentRegistry::new();
        let mut registration = governed_registration("CX-AGT-000001", "runtime.primary");
        registration
            .transition_presence(PresenceState::Ready)
            .expect("ready");
        let agent_id = registration.agent_id().clone();
        registry.register(registration).expect("register");
        let outcome = registry
            .supervise_runtime(
                &agent_id,
                &TimeReference::new("2026-07-15T00:10:00Z").expect("now"),
                &heartbeat_policy(),
                None,
                None,
            )
            .expect("outcome");
        registry
            .apply_supervisor_outcome(&outcome)
            .expect("apply outcome");
        assert_eq!(
            registry.lookup(&agent_id).expect("lookup").presence_state(),
            PresenceState::Offline
        );
    }

    #[test]
    fn runtime_registry_duplicate_operations_are_deterministic_ces_b0_027_21() {
        let mut registry = AgentRegistry::new();
        let registration = governed_registration("CX-AGT-000001", "runtime.primary");
        let agent_id = registration.agent_id().clone();
        registry.register(registration).expect("register");
        let request = LeaseRenewalRequest::new(
            LeaseId::new("CX-LEASE-000001").expect("lease"),
            governed_lease(
                "CX-LEASE-000002",
                "runtime.primary",
                "CX-AGT-000001",
                "2026-07-15T00:30:00Z",
                "2026-07-15T02:00:00Z",
                Some("CX-LEASE-000001"),
            ),
            TimeReference::new("2026-07-15T00:30:00Z").expect("time"),
        );
        let left = registry
            .renew_lease_validated(&agent_id, &request, &lease_policy())
            .expect("left");
        let right = registry
            .renew_lease_validated(&agent_id, &request, &lease_policy())
            .expect("right");
        assert_eq!(left.outcome(), LeaseRenewalOutcome::LeaseRenewed);
        assert_eq!(right.outcome(), LeaseRenewalOutcome::LeaseDuplicate);
    }

    #[test]
    fn runtime_lower_non_identical_lease_sequence_is_regression_ces_b0_027_21() {
        let mut registry = AgentRegistry::new();
        let registration = governed_registration("CX-AGT-000001", "runtime.primary");
        let agent_id = registration.agent_id().clone();
        registry.register(registration).expect("register");
        let accepted = LeaseRenewalRequest::new(
            LeaseId::new("CX-LEASE-000001").expect("lease"),
            governed_lease(
                "CX-LEASE-000002",
                "runtime.primary",
                "CX-AGT-000001",
                "2026-07-15T00:30:00Z",
                "2026-07-15T02:00:00Z",
                Some("CX-LEASE-000001"),
            ),
            TimeReference::new("2026-07-15T00:30:00Z").expect("time"),
        );
        let _ = registry
            .renew_lease_validated(&agent_id, &accepted, &lease_policy())
            .expect("accepted");

        let before = registry.lookup(&agent_id).expect("lookup").clone();
        let regression = LeaseRenewalRequest::new(
            LeaseId::new("CX-LEASE-000001").expect("lease"),
            governed_lease(
                "CX-LEASE-000003",
                "runtime.primary",
                "CX-AGT-000001",
                "2026-07-15T00:45:00Z",
                "2026-07-15T02:30:00Z",
                Some("CX-LEASE-000001"),
            ),
            TimeReference::new("2026-07-15T00:45:00Z").expect("time"),
        );

        let result = registry
            .renew_lease_validated(&agent_id, &regression, &lease_policy())
            .expect("result");
        assert_eq!(
            result.outcome(),
            LeaseRenewalOutcome::LeaseSequenceRegression
        );
        assert_eq!(registry.lookup(&agent_id).expect("lookup"), &before);
    }

    #[test]
    fn runtime_deregistration_blocks_future_runtime_operations_ces_b0_027_22() {
        let mut registry = AgentRegistry::new();
        let registration = governed_registration("CX-AGT-000001", "runtime.primary");
        let agent_id = registration.agent_id().clone();
        registry.register(registration).expect("register");
        let _ = registry.deregister(&agent_id).expect("deregister");
        let error = registry
            .record_heartbeat_validated(
                &agent_id,
                governed_heartbeat(
                    "CX-HB-000001",
                    "runtime.primary",
                    "CX-AGT-000001",
                    "2026-07-15T00:10:00Z",
                    "2026-07-15T00:20:00Z",
                    Some("CX-LEASE-000001"),
                    PresenceState::Registered,
                    RuntimeHealth::Healthy,
                ),
                &TimeReference::new("2026-07-15T00:10:00Z").expect("now"),
                &heartbeat_policy(),
            )
            .expect_err("deregistered runtime operations must fail");
        assert!(error
            .to_string()
            .contains("operation requires an existing registration"));
    }
}
