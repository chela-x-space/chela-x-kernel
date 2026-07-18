use crate::agent::{
    AgentCategory, AgentDefinition, AgentDefinitionSpec, AgentRecoveryEvidenceReference,
    AgentRecoveryPlanReference, AgentRecoveryReference, AgentRuntimeReference, AgentType,
};
use crate::authorization::{
    AuthorizationDecisionOutcome, AuthorizationDecisionReference,
    AuthorizationEvaluationOrderVersion, MatchedPolicyEvidenceReference,
};
use crate::identity::AgentIdentity;
use crate::lifecycle::AgentLifecycle;
use crate::ownership::{OrganizationalContext, OwnerReference, OwnershipPath};
use crate::request::TimeReference;
use crate::runtime::{
    assess_recovery_eligibility, AgentRegistration, AgentRegistrationSpec, AgentRegistry,
    CapabilityDescriptor, CapabilityDescriptorSpec, HeartbeatFreshnessPolicy, HeartbeatRecord,
    HeartbeatRecordSpec, LeaseIssuanceSpec, LeaseRecord, PresenceState, RuntimeEntity,
    RuntimeEntitySpec, RuntimeFailureObservation, RuntimeHealth, RuntimeStateSnapshot,
};
use crate::state::{
    StateSequence, TransitionAuthorityReference, TransitionEvidenceReference,
    TransitionReasonReference,
};
use crate::workflow::WorkflowStepReference;
use crate::{
    ActionVerb, AgentFailureCategory, AgentFailureReference, AgentFailureSeverity, AgentId,
    AgentUuid, AuditEvidenceId, AuthorizationDecisionId, AuthorizationRequestId, CapabilityId,
    CorrelationId, DelegationReference, EnglishNamespace, EnterpriseId, ExecutionAuditReference,
    ExecutionContext, ExecutionEvidenceBinding, ExecutionRequest, ExecutionSession,
    ExecutionSessionId, HumanId, LeaseId, PermissionEffectIntent, PermissionId,
    PermissionReference, PolicyId, ProjectId, ResourceType, RuntimeId, StableVersion,
    TaskCompletion, TaskCompletionControl, TaskCompletionOutcome, TaskCompletionRequirement,
    TaskCompletionResult, TaskCompletionValidationRequest, TaskCreationContext, TaskDefinition,
    TaskDefinitionId, TaskDefinitionName, TaskDefinitionVersion, TaskDescription, TaskEvidence,
    TaskEvidenceMetadata, TaskEvidenceReference, TaskEvidenceRequirement, TaskEvidenceSet,
    TaskEvidenceType, TaskFailure, TaskFailureCategory, TaskFailureCode, TaskFailureControl,
    TaskFailureOutcome, TaskFailurePolicyReference, TaskFailureReason, TaskFailureReference,
    TaskFailureValidationRequest, TaskInputBinding, TaskInputContract, TaskInstance,
    TaskInstanceId, TaskInstanceReference, TaskKind, TaskOutput, TaskOutputBinding,
    TaskOutputContract, TaskOutputReference, TaskReadinessControl, TaskReadinessDecision,
    TaskReadinessEvidence, TaskReadinessInput, TaskReadinessRequirement, TaskState,
    TaskStateSnapshot, TaskStepReference, TaskWorkflowReference, WorkspaceId,
};

fn enterprise_id() -> EnterpriseId {
    EnterpriseId::new("CX-ENT-000001").expect("enterprise")
}
fn owner() -> OwnerReference {
    OwnerReference::new(HumanId::new("CX-EMP-000001").expect("owner"))
}
fn ownership_path() -> OwnershipPath {
    OwnershipPath::new(
        enterprise_id(),
        Some(WorkspaceId::new("CX-WS-000001").expect("workspace")),
        Some(ProjectId::new("CX-PROJ-000001").expect("project")),
        None,
    )
    .expect("path")
}

pub(crate) fn execution_session_id() -> ExecutionSessionId {
    ExecutionSessionId::new("execution.session-0001").expect("session id")
}

pub(crate) fn task_instance_id() -> TaskInstanceId {
    TaskInstanceId::new("CX-TASK-000001").expect("task instance id")
}

pub(crate) fn task_instance_reference() -> TaskInstanceReference {
    TaskInstanceReference::new(task_instance_id())
}

pub(crate) fn transition_authority() -> TransitionAuthorityReference {
    TransitionAuthorityReference::new("authority.execution").expect("authority")
}

pub(crate) fn transition_reason() -> TransitionReasonReference {
    TransitionReasonReference::new("execution.terminated").expect("reason")
}

pub(crate) fn transition_evidence() -> TransitionEvidenceReference {
    TransitionEvidenceReference::new("transition.evidence-001").expect("evidence")
}

pub(crate) fn authorization_reference(
    outcome: AuthorizationDecisionOutcome,
) -> AuthorizationDecisionReference {
    AuthorizationDecisionReference::new(
        AuthorizationDecisionId::new("CX-AUTHDEC-000001").expect("decision"),
        AuthorizationRequestId::new("CX-AUTHREQ-000001").expect("request"),
        PolicyId::new("CX-POL-000001").expect("policy"),
        outcome,
        AuthorizationEvaluationOrderVersion::new("1.0.0").expect("version"),
        MatchedPolicyEvidenceReference::new("matched.policy-001").expect("evidence"),
        "2026-07-18T00:00:00Z",
    )
    .expect("auth reference")
}

pub(crate) fn readiness_ready() -> TaskReadinessDecision {
    TaskReadinessControl::evaluate(&TaskReadinessInput::new(
        task_instance_reference(),
        TaskState::InProgress,
        None,
        None,
        None,
        vec![TaskReadinessRequirement::DependenciesComplete],
        vec![TaskReadinessEvidence::DependenciesComplete],
        None,
    ))
}

pub(crate) fn readiness_blocked() -> TaskReadinessDecision {
    TaskReadinessControl::evaluate(&TaskReadinessInput::new(
        task_instance_reference(),
        TaskState::Pending,
        None,
        None,
        None,
        vec![TaskReadinessRequirement::DependenciesComplete],
        vec![],
        None,
    ))
}

pub(crate) fn task_state_snapshot(state: TaskState) -> TaskStateSnapshot {
    TaskStateSnapshot::new(
        task_instance_reference(),
        state,
        StateSequence::new(1).expect("sequence"),
    )
}

fn task_definition() -> TaskDefinition {
    let input = TaskInputContract::new("execution.input").expect("input");
    let output = TaskOutputContract::new("execution.output").expect("output");
    TaskDefinition::new(
        TaskDefinitionId::new("CX-TDEF-000001").expect("definition"),
        TaskDefinitionVersion::new("1.0.0").expect("version"),
        TaskDefinitionName::new("Execution Task").expect("name"),
        Some(TaskDescription::new("K8 execution test task").expect("description")),
        TaskKind::new("governed.execution").expect("kind"),
        vec![input.clone()],
        vec![output],
        vec![],
        vec![],
        vec![TaskEvidenceRequirement::new("required.execution.evidence").expect("requirement")],
        vec![TaskCompletionRequirement::new("required.execution.completion").expect("completion")],
        Some(TaskFailurePolicyReference::new("CX-POL-000002").expect("failure policy")),
        None,
        None,
    )
    .expect("task definition")
}

pub(crate) fn task_instance() -> TaskInstance {
    let definition = task_definition();
    let input = definition.task_input_contracts()[0].clone();
    let output = definition.task_output_contracts()[0].clone();
    TaskInstance::new(
        task_instance_id(),
        definition,
        TaskCreationContext::new(
            vec![TaskInputBinding::new(input)],
            Some(transition_authority()),
        )
        .expect("creation context"),
        vec![TaskOutputBinding::new(output)],
        None,
        None,
        TaskState::Pending,
    )
    .expect("task instance")
}

fn task_evidence_set(task_instance: &TaskInstance) -> TaskEvidenceSet {
    let task_instance_reference =
        TaskInstanceReference::new(task_instance.task_instance_id().clone());
    TaskEvidenceSet::new(
        task_instance_reference.clone(),
        vec![TaskEvidence::new(
            TaskEvidenceReference::new(
                crate::TaskEvidenceId::new("CX-TEVID-000001").expect("evidence id"),
            ),
            task_instance_reference,
            TaskEvidenceType::new("task.execution.evidence").expect("type"),
            Some(transition_authority()),
            TaskEvidenceMetadata::new(
                Some(task_instance.task_definition().task_evidence_requirements()[0].clone()),
                Some(transition_evidence()),
            ),
        )],
    )
    .expect("task evidence set")
}

pub(crate) fn accepted_completion() -> TaskCompletion {
    let task_instance = task_instance();
    let output_contract = task_instance.task_definition().task_output_contracts()[0].clone();
    let result = TaskCompletionResult::new(
        TaskInstanceReference::new(task_instance.task_instance_id().clone()),
        task_instance.task_definition_snapshot_reference().clone(),
        task_instance
            .task_definition()
            .task_completion_requirements()
            .to_vec(),
        vec![TaskOutput::new(
            TaskOutputReference::new("execution.output.reference").expect("output ref"),
            TaskOutputBinding::new(output_contract),
        )],
        task_evidence_set(&task_instance),
        Some(transition_authority()),
        Some(transition_reason()),
    )
    .expect("completion result");
    match TaskCompletionControl::evaluate(&TaskCompletionValidationRequest::new(
        task_instance,
        task_state_snapshot(TaskState::InProgress),
        result,
        None,
    )) {
        TaskCompletionOutcome::Accepted(completion) => completion,
        TaskCompletionOutcome::Rejected(_) => panic!("completion fixture must be accepted"),
    }
}

pub(crate) fn accepted_failure() -> TaskFailure {
    let task_instance = task_instance();
    let failure = TaskFailure::new(
        TaskInstanceReference::new(task_instance.task_instance_id().clone()),
        TaskFailureReference::new("task.failure-001").expect("failure reference"),
        TaskFailureCode::new("execution.failure").expect("failure code"),
        TaskFailureCategory::new("execution.failed").expect("failure category"),
        Some(TaskFailureReason::new("execution failed").expect("failure reason")),
        task_evidence_set(&task_instance),
        Some(transition_authority()),
        task_instance
            .task_definition()
            .task_failure_policy_reference()
            .cloned(),
    );
    match TaskFailureControl::evaluate(&TaskFailureValidationRequest::new(
        task_instance,
        task_state_snapshot(TaskState::InProgress),
        failure,
        None,
    )) {
        TaskFailureOutcome::Accepted(failure) => failure,
        TaskFailureOutcome::Rejected(_) => panic!("failure fixture must be accepted"),
    }
}

fn runtime_capability() -> CapabilityDescriptor {
    CapabilityDescriptor::new(CapabilityDescriptorSpec {
        capability_id: CapabilityId::new("CX-CAP-000001").expect("capability"),
        description: "execute governed work".to_owned(),
        dependencies: vec!["governance".to_owned()],
        inputs: vec!["task".to_owned()],
        outputs: vec!["outcome".to_owned()],
        required_permissions: vec![PermissionReference::new(
            PermissionId::new("CX-PERM-000001").expect("permission"),
            ActionVerb::new("execute").expect("verb"),
            ResourceType::new("task").expect("resource"),
            PermissionEffectIntent::new("Permit").expect("effect"),
        )],
        governing_policies: vec![PolicyId::new("CX-POL-000003").expect("policy")],
    })
    .expect("capability")
}

fn runtime_entity() -> RuntimeEntity {
    RuntimeEntity::new(RuntimeEntitySpec {
        runtime_id: RuntimeId::new("runtime.execution.primary").expect("runtime"),
        runtime_reference: AgentRuntimeReference::new("runtime.execution.primary").expect("ref"),
        enterprise_id: enterprise_id(),
        ownership_path: ownership_path(),
        health: RuntimeHealth::Healthy,
    })
    .expect("runtime entity")
}

fn runtime_agent() -> AgentDefinition {
    AgentDefinition::new(AgentDefinitionSpec {
        identity: AgentIdentity::new(
            AgentId::new("CX-AGT-000001").expect("agent"),
            EnglishNamespace::new("agent_namespace", "execution.agent").expect("namespace"),
            StableVersion::new("agent_version", "1.0.0").expect("version"),
            enterprise_id(),
            AgentLifecycle::Registered,
        )
        .expect("identity"),
        agent_uuid: AgentUuid::new("CX-UUID-00000001").expect("uuid"),
        agent_name: "Execution Agent".to_owned(),
        agent_type: AgentType::new("worker").expect("type"),
        agent_category: AgentCategory::new("execution").expect("category"),
        owner: owner(),
        organizational_context: OrganizationalContext::new(ownership_path(), owner()),
        runtime_reference: AgentRuntimeReference::new("runtime.execution.primary").expect("ref"),
    })
    .expect("agent")
}

fn runtime_lease() -> LeaseRecord {
    LeaseRecord::issue(LeaseIssuanceSpec {
        lease_id: LeaseId::new("CX-LEASE-000001").expect("lease"),
        runtime_id: RuntimeId::new("runtime.execution.primary").expect("runtime"),
        agent_id: AgentId::new("CX-AGT-000001").expect("agent"),
        issued_at: TimeReference::new("2026-07-18T00:00:00Z").expect("issued"),
        expires_at: TimeReference::new("2026-07-18T01:00:00Z").expect("expires"),
        supersedes_lease_id: None,
        evidence: "LEASE-EVID-001".to_owned(),
    })
    .expect("lease")
}

fn runtime_heartbeat() -> HeartbeatRecord {
    HeartbeatRecord::new(HeartbeatRecordSpec {
        heartbeat_id: crate::HeartbeatId::new("CX-HB-000001").expect("heartbeat"),
        runtime_id: RuntimeId::new("runtime.execution.primary").expect("runtime"),
        recorded_at: TimeReference::new("2026-07-18T00:10:00Z").expect("recorded"),
        fresh_until: TimeReference::new("2026-07-18T01:00:00Z").expect("fresh"),
    })
    .expect("heartbeat")
}

fn runtime_registry() -> AgentRegistry {
    let registration = AgentRegistration::new(AgentRegistrationSpec {
        agent: runtime_agent(),
        runtime: runtime_entity(),
        supervisor: owner(),
        capabilities: vec![runtime_capability()],
        presence_state: PresenceState::Registered,
        health: RuntimeHealth::Healthy,
        registered_at: TimeReference::new("2026-07-18T00:00:00Z").expect("registered"),
        lease: Some(runtime_lease()),
        last_heartbeat: Some(runtime_heartbeat()),
    })
    .expect("registration");
    let mut registry = AgentRegistry::new();
    registry.register(registration).expect("register");
    registry
}

pub(crate) fn runtime_state_snapshot() -> RuntimeStateSnapshot {
    runtime_registry()
        .runtime_snapshot(
            &AgentId::new("CX-AGT-000001").expect("agent"),
            &TimeReference::new("2026-07-18T00:15:00Z").expect("now"),
            &HeartbeatFreshnessPolicy::new(
                TimeReference::new("2026-07-18T00:14:00Z").expect("late"),
                TimeReference::new("2026-07-18T00:13:00Z").expect("stale"),
                StableVersion::new("heartbeat_policy_version", "2026.07.18").expect("version"),
            )
            .expect("policy"),
            None,
            Some(runtime_recovery_reference()),
        )
        .expect("snapshot")
}

pub(crate) fn runtime_recovery_reference() -> AgentRecoveryReference {
    AgentRecoveryReference::new(
        AgentId::new("CX-AGT-000001").expect("agent"),
        AgentRecoveryPlanReference::new("recovery.plan-001").expect("plan"),
        owner(),
        AgentRecoveryEvidenceReference::new("recovery.evidence-001").expect("evidence"),
    )
}

pub(crate) fn runtime_failure_observation() -> RuntimeFailureObservation {
    RuntimeFailureObservation::new(
        AgentFailureReference::new(
            AgentId::new("CX-AGT-000001").expect("agent"),
            AgentFailureCategory::HeartbeatFailure,
            AgentFailureSeverity::Major,
            true,
        )
        .expect("failure"),
        TimeReference::new("2026-07-18T00:16:00Z").expect("observed"),
        "FAIL-EVID-001",
    )
    .expect("observation")
}

pub(crate) fn recovery_eligibility() -> crate::RecoveryEligibility {
    let snapshot = runtime_state_snapshot();
    let reference = runtime_recovery_reference();
    let failure = runtime_failure_observation();
    assess_recovery_eligibility(&snapshot, Some(&failure), Some(&reference))
}

pub(crate) fn execution_evidence_binding() -> ExecutionEvidenceBinding {
    ExecutionEvidenceBinding::new(
        execution_session_id(),
        task_instance_reference(),
        vec![TaskEvidenceReference::new(
            crate::TaskEvidenceId::new("CX-TEVID-000002").expect("id"),
        )],
        vec![TaskOutputReference::new("execution.output.reference").expect("output ref")],
        vec![transition_evidence()],
    )
    .expect("evidence binding")
}

pub(crate) fn execution_audit_reference() -> ExecutionAuditReference {
    ExecutionAuditReference::new(
        execution_session_id(),
        Some(CorrelationId::new("CX-COR-000001").expect("correlation")),
        vec![AuditEvidenceId::new("CX-AUD-000001").expect("audit")],
    )
    .expect("audit reference")
}

pub(crate) fn execution_request() -> ExecutionRequest {
    ExecutionRequest::new(
        execution_session_id(),
        task_instance_reference(),
        task_state_snapshot(TaskState::InProgress),
        readiness_ready(),
        authorization_reference(AuthorizationDecisionOutcome::Allow),
        TimeReference::new("2026-07-18T00:20:00Z").expect("requested"),
    )
    .expect("request")
}

pub(crate) fn execution_context() -> ExecutionContext {
    ExecutionContext::new(
        execution_session_id(),
        task_instance_reference(),
        runtime_state_snapshot(),
        None::<DelegationReference>,
        Some(TaskWorkflowReference::new(
            crate::WorkflowId::new("CX-WF-000001").expect("wf"),
        )),
        Some(TaskStepReference::new(
            WorkflowStepReference::new("step.execute").expect("step"),
        )),
        task_instance()
            .task_creation_context()
            .task_input_bindings()
            .to_vec(),
    )
    .expect("context")
}

pub(crate) fn execution_session() -> ExecutionSession {
    ExecutionSession::new(
        execution_request(),
        execution_context(),
        execution_evidence_binding(),
        execution_audit_reference(),
        TimeReference::new("2026-07-18T00:21:00Z").expect("started"),
    )
    .expect("session")
}
