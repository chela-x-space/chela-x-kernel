#![allow(dead_code)]

use kernel_domain::{
    ActionVerb, AgentCategory, AgentDefinition, AgentDefinitionSpec, AgentId, AgentIdentity,
    AgentLifecycle, AgentRegistration, AgentRegistrationSpec, AgentRegistry, AgentRuntimeReference,
    AgentType, AgentUuid, AuditEvidenceId, AuthorizationDecisionId, AuthorizationDecisionOutcome,
    AuthorizationDecisionReference, AuthorizationEvaluationOrderVersion,
    AuthorizationPrincipalReference, AuthorizationPrincipalType, AuthorizationRequestId,
    AuthorizationRequestRecord, AuthorizationSubject, AuthorizationTarget, CapabilityDescriptor,
    CapabilityDescriptorSpec, CorrelationId, CredentialStatusReference, EnglishNamespace,
    EnterpriseId, EventActorId, EventCausation, EventClassification, EventComponent, EventEnvelope,
    EventReplayEntry, EventSource, EventSubject, EventSubjectId, EventSubjectType, EventTrace,
    EventTraceReference, EventType, EventVersion, ExecutionSessionId, HeartbeatFreshnessPolicy,
    HeartbeatId, HeartbeatObservationSpec, HeartbeatRecord, HumanId, LeaseId, LeaseIssuanceSpec,
    LeaseRecord, MatchedPolicyEvidenceReference, MemoryAuditReference, MemoryClassification,
    MemoryProjection, MemoryProvenance, MemoryQuery, MemoryQueryResult, MemoryRecord,
    MemoryRecordId, MemoryRecordReference, MemoryRetentionPolicyReference, MemoryRetrievalRequest,
    MemoryRetrievalResult, OrganizationalContext, OwnerReference, OwnershipPath,
    PermissionEffectIntent, PermissionId, PermissionReference, PolicyId, PresenceState,
    PrincipalId, PrincipalLifecycleStateReference, ProjectId, ResourceType, RuntimeEntity,
    RuntimeEntitySpec, RuntimeHealth, RuntimeId, RuntimeStateSnapshot, ScopeId, ScopeLevel,
    ScopeReference, StableVersion, StateSequence, StreamPosition, TaskInstanceId,
    TaskInstanceReference, TaskPriority, TaskPriorityClass, TaskPriorityValue, TaskState,
    TaskStateSnapshot, TimeReference, WorkflowId, WorkflowState, WorkflowStateSnapshot,
    WorkflowStepReference, WorkspaceId,
};
use kernel_gateway::{
    GatewayApiVersion, GatewayAuditReference, GatewayAuthenticationContext,
    GatewayAuthorizationBinding, GatewayOperationKind, GatewayOperationReference,
    GatewayQueryPayload, GatewayQueryRequest, GatewayQueryResponse, GatewayRequestContext,
    GatewayRequestEnvelope, GatewayResponseEnvelope, GatewayStatusSnapshot,
};

use crate::{
    StudioApiVersion, StudioAuditReference, StudioSelectionContext, StudioViewKind,
    StudioViewReference,
};

pub fn enterprise_id() -> EnterpriseId {
    EnterpriseId::new("CX-ENT-000001").expect("enterprise")
}

pub fn workspace_id() -> WorkspaceId {
    WorkspaceId::new("CX-WS-000001").expect("workspace")
}

pub fn project_id() -> ProjectId {
    ProjectId::new("CX-PROJ-000001").expect("project")
}

pub fn ownership_path() -> OwnershipPath {
    OwnershipPath::new(
        enterprise_id(),
        Some(workspace_id()),
        Some(project_id()),
        None,
    )
    .expect("ownership")
}

pub fn time_reference() -> TimeReference {
    TimeReference::new("2026-07-19T00:00:00Z").expect("time")
}

pub fn later_time_reference() -> TimeReference {
    TimeReference::new("2026-07-19T00:10:00Z").expect("time")
}

pub fn correlation_id() -> CorrelationId {
    CorrelationId::new("CX-COR-000001").expect("correlation")
}

pub fn principal_reference() -> AuthorizationPrincipalReference {
    AuthorizationPrincipalReference::new(
        PrincipalId::new("CX-PRN-000001").expect("principal"),
        AuthorizationPrincipalType::Employee,
        "CX-EMP-000001",
        enterprise_id(),
        PrincipalLifecycleStateReference::new("Active").expect("lifecycle"),
        CredentialStatusReference::new("Valid").expect("credential"),
    )
    .expect("principal")
}

pub fn gateway_api_version() -> GatewayApiVersion {
    GatewayApiVersion::new("2026.07.19").expect("version")
}

pub fn studio_api_version() -> StudioApiVersion {
    StudioApiVersion::new("2026.07.19").expect("version")
}

pub fn gateway_audit_reference() -> GatewayAuditReference {
    GatewayAuditReference::new(
        EventTraceReference::new("gateway.audit.trace.000001").expect("trace"),
        Some(correlation_id()),
        vec![AuditEvidenceId::new("CX-AUD-000001").expect("audit")],
    )
    .expect("audit")
}

pub fn studio_audit_reference() -> StudioAuditReference {
    StudioAuditReference::new(
        EventTraceReference::new("studio.audit.trace.000001").expect("trace"),
        Some(correlation_id()),
        vec![AuditEvidenceId::new("CX-AUD-000001").expect("audit")],
        Some(gateway_audit_reference()),
    )
    .expect("audit")
}

pub fn operation_gateway_status() -> GatewayOperationReference {
    GatewayOperationReference::new(GatewayOperationKind::Status, "gateway.status")
        .expect("operation")
}

pub fn operation_runtime_state() -> GatewayOperationReference {
    GatewayOperationReference::new(GatewayOperationKind::Query, "runtime.state").expect("operation")
}

pub fn operation_workflow_state() -> GatewayOperationReference {
    GatewayOperationReference::new(GatewayOperationKind::Query, "workflow.state")
        .expect("operation")
}

pub fn operation_task_state() -> GatewayOperationReference {
    GatewayOperationReference::new(GatewayOperationKind::Query, "task.state").expect("operation")
}

pub fn operation_memory_query() -> GatewayOperationReference {
    GatewayOperationReference::new(GatewayOperationKind::Query, "memory.query").expect("operation")
}

pub fn permission_reference(resource_type: &str) -> PermissionReference {
    PermissionReference::new(
        PermissionId::new("CX-PERM-000001").expect("permission"),
        ActionVerb::new("read").expect("verb"),
        ResourceType::new(resource_type).expect("resource type"),
        PermissionEffectIntent::new("Permit").expect("effect"),
    )
}

pub fn authorization_request_record(
    resource_type: &str,
    resource_identifier: &str,
) -> AuthorizationRequestRecord {
    AuthorizationRequestRecord::new(
        AuthorizationRequestId::new("CX-AUTHREQ-000001").expect("request"),
        AuthorizationSubject::Principal(principal_reference()),
        permission_reference(resource_type),
        AuthorizationTarget::new(
            ResourceType::new(resource_type).expect("resource type"),
            resource_identifier,
            ScopeReference::new(
                ScopeId::new("CX-SCP-000001").expect("scope"),
                ScopeLevel::Project,
                ownership_path(),
                None,
            )
            .expect("scope"),
        )
        .expect("target"),
        time_reference(),
        "studio query",
    )
    .expect("auth request")
}

pub fn authorization_binding(
    resource_type: &str,
    resource_identifier: &str,
) -> GatewayAuthorizationBinding {
    let request = authorization_request_record(resource_type, resource_identifier);
    GatewayAuthorizationBinding::new(
        request.clone(),
        AuthorizationDecisionReference::new(
            AuthorizationDecisionId::new("CX-AUTHDEC-000001").expect("decision"),
            request.request_id().clone(),
            PolicyId::new("CX-POL-000001").expect("policy"),
            AuthorizationDecisionOutcome::Allow,
            AuthorizationEvaluationOrderVersion::new("2026.07.19").expect("version"),
            MatchedPolicyEvidenceReference::new("gateway.policy.evidence").expect("evidence"),
            "2026-07-19T00:00:00Z",
        )
        .expect("decision"),
    )
    .expect("binding")
}

pub fn gateway_authentication_context() -> GatewayAuthenticationContext {
    GatewayAuthenticationContext::new(
        principal_reference(),
        "auth.password",
        EventTraceReference::new("gateway.auth.trace.000001").expect("trace"),
        time_reference(),
        None,
        None,
    )
    .expect("auth")
}

pub fn gateway_request_context(
    operation: GatewayOperationReference,
    resource_type: &str,
    resource_identifier: &str,
) -> GatewayRequestContext {
    GatewayRequestContext::new(
        gateway_api_version(),
        operation,
        gateway_authentication_context(),
        authorization_binding(resource_type, resource_identifier),
        ownership_path(),
        correlation_id(),
        time_reference(),
        None,
        gateway_audit_reference(),
    )
    .expect("context")
}

pub fn runtime_id() -> RuntimeId {
    RuntimeId::new("CX-RUN-000001").expect("runtime")
}

pub fn workflow_id() -> WorkflowId {
    WorkflowId::new("CX-WF-000001").expect("workflow")
}

pub fn task_instance_reference() -> TaskInstanceReference {
    TaskInstanceReference::new(TaskInstanceId::new("CX-TASK-000001").expect("task"))
}

pub fn runtime_query_envelope() -> GatewayRequestEnvelope {
    GatewayRequestEnvelope::query(
        gateway_request_context(operation_runtime_state(), "runtime", runtime_id().as_str()),
        GatewayQueryRequest::new(GatewayQueryPayload::RuntimeSnapshot(runtime_id())),
    )
    .expect("query")
}

pub fn workflow_query_envelope() -> GatewayRequestEnvelope {
    GatewayRequestEnvelope::query(
        gateway_request_context(
            operation_workflow_state(),
            "workflow",
            workflow_id().as_str(),
        ),
        GatewayQueryRequest::new(GatewayQueryPayload::WorkflowState(workflow_id())),
    )
    .expect("query")
}

pub fn task_query_envelope() -> GatewayRequestEnvelope {
    GatewayRequestEnvelope::query(
        gateway_request_context(
            operation_task_state(),
            "task",
            task_instance_reference().task_instance_id().as_str(),
        ),
        GatewayQueryRequest::new(GatewayQueryPayload::TaskState(task_instance_reference())),
    )
    .expect("query")
}

pub fn gateway_status_envelope() -> GatewayRequestEnvelope {
    GatewayRequestEnvelope::query(
        gateway_request_context(operation_gateway_status(), "gateway", "status"),
        GatewayQueryRequest::new(GatewayQueryPayload::GatewayStatus),
    )
    .expect("query")
}

pub fn memory_record_id() -> MemoryRecordId {
    MemoryRecordId::new("memory.record-000001").expect("memory")
}

pub fn memory_record_reference() -> MemoryRecordReference {
    MemoryRecordReference::new(memory_record_id())
}

pub fn memory_audit_reference() -> MemoryAuditReference {
    MemoryAuditReference::new(
        memory_record_id(),
        EventTraceReference::new("memory.audit.trace.000001").expect("trace"),
        vec![AuditEvidenceId::new("CX-AUD-000001").expect("audit")],
    )
    .expect("audit")
}

pub fn memory_record() -> MemoryRecord {
    MemoryRecord::new(
        memory_record_reference(),
        ownership_path(),
        "Studio memory summary",
        MemoryClassification::new("INTERNAL").expect("classification"),
        MemoryProvenance::new(
            kernel_domain::EventId::new("CX-EVT-000001").expect("event"),
            Some(workflow_id()),
            Some(task_instance_reference()),
            Some(ExecutionSessionId::new("execution.session-0001").expect("execution")),
            Some(runtime_id()),
            Some(
                authorization_binding(
                    "memory",
                    memory_record_reference().memory_record_id().as_str(),
                )
                .authorization_decision_reference()
                .clone(),
            ),
            None,
        )
        .expect("provenance"),
        MemoryRetentionPolicyReference::new(PolicyId::new("CX-POL-000002").expect("policy")),
        memory_audit_reference(),
        time_reference(),
    )
    .expect("memory")
}

pub fn memory_query() -> MemoryQuery {
    MemoryQuery::by_runtime(runtime_id())
}

pub fn memory_query_envelope() -> GatewayRequestEnvelope {
    GatewayRequestEnvelope::query(
        gateway_request_context(
            operation_memory_query(),
            "memory",
            memory_record_reference().memory_record_id().as_str(),
        ),
        GatewayQueryRequest::new(GatewayQueryPayload::MemoryQuery(memory_query())),
    )
    .expect("query")
}

pub fn memory_retrieval_request() -> MemoryRetrievalRequest {
    MemoryRetrievalRequest::new(
        authorization_binding(
            "memory",
            memory_record_reference().memory_record_id().as_str(),
        )
        .authorization_decision_reference()
        .clone(),
        ownership_path(),
        MemoryQuery::by_record_references(vec![memory_record_reference()]).expect("query"),
    )
    .expect("request")
}

pub fn memory_retrieval_result() -> MemoryRetrievalResult {
    MemoryRetrievalResult::new(memory_retrieval_request(), vec![memory_record()]).expect("result")
}

pub fn memory_query_result() -> MemoryQueryResult {
    MemoryQueryResult::new(
        memory_query(),
        vec![MemoryProjection::new(&memory_record())],
    )
    .expect("result")
}

pub fn memory_projection() -> MemoryProjection {
    MemoryProjection::new(&memory_record())
}

pub fn memory_query_response_envelope() -> GatewayResponseEnvelope {
    GatewayResponseEnvelope::new(
        &memory_query_envelope(),
        correlation_id(),
        kernel_gateway::gateway_response::GatewayEnvelopeResponse::Query(Box::new(
            GatewayQueryResponse::MemoryQuery(Box::new(memory_query_result())),
        )),
        gateway_audit_reference(),
        later_time_reference(),
    )
    .expect("response")
}

pub fn gateway_status_snapshot() -> GatewayStatusSnapshot {
    GatewayStatusSnapshot::new(
        gateway_api_version(),
        vec![kernel_gateway::GatewayProtocol::Internal],
        vec![operation_gateway_status()],
        time_reference(),
        gateway_audit_reference(),
    )
    .expect("status")
}

pub fn gateway_status_response_envelope() -> GatewayResponseEnvelope {
    GatewayResponseEnvelope::new(
        &gateway_status_envelope(),
        correlation_id(),
        kernel_gateway::gateway_response::GatewayEnvelopeResponse::Query(Box::new(
            GatewayQueryResponse::StatusSnapshot(gateway_status_snapshot()),
        )),
        gateway_audit_reference(),
        later_time_reference(),
    )
    .expect("response")
}

pub fn workflow_state_snapshot() -> WorkflowStateSnapshot {
    WorkflowStateSnapshot::new(
        workflow_id(),
        ownership_path(),
        StableVersion::new("workflow_version", "1.0.0").expect("version"),
        WorkflowState::Running,
        StateSequence::new(2).expect("sequence"),
    )
}

pub fn task_state_snapshot() -> TaskStateSnapshot {
    TaskStateSnapshot::new(
        task_instance_reference(),
        TaskState::InProgress,
        StateSequence::new(3).expect("sequence"),
    )
}

pub fn workflow_step_reference(value: &str) -> WorkflowStepReference {
    WorkflowStepReference::new(value).expect("step")
}

pub fn task_priority() -> TaskPriority {
    TaskPriority::new(
        task_instance_reference(),
        TaskPriorityClass::new("Explicit").expect("class"),
        TaskPriorityValue::new(5).expect("value"),
    )
}

pub fn runtime_state_snapshot() -> RuntimeStateSnapshot {
    let agent_id = AgentId::new("CX-AGT-000001").expect("agent");
    let runtime_id = runtime_id();
    let agent_identity = AgentIdentity::new(
        agent_id.clone(),
        EnglishNamespace::new("agent_namespace", "enterprise.agent").expect("namespace"),
        StableVersion::new("agent_version", "1.0.0").expect("version"),
        enterprise_id(),
        AgentLifecycle::Registered,
    )
    .expect("identity");
    let owner = OwnerReference::new(HumanId::new("CX-EMP-000001").expect("owner"));
    let organizational_context = OrganizationalContext::new(ownership_path(), owner.clone());
    let agent_definition = AgentDefinition::new(AgentDefinitionSpec {
        identity: agent_identity,
        agent_uuid: AgentUuid::new("CX-UUID-00000001").expect("uuid"),
        agent_name: "Kernel Agent".to_owned(),
        agent_type: AgentType::new("Supervisor").expect("type"),
        agent_category: AgentCategory::new("Operations").expect("category"),
        owner: owner.clone(),
        organizational_context,
        runtime_reference: AgentRuntimeReference::new("runtime.ref.000001").expect("runtime ref"),
    })
    .expect("agent");
    let runtime_entity = RuntimeEntity::new(RuntimeEntitySpec {
        runtime_id: runtime_id.clone(),
        runtime_reference: AgentRuntimeReference::new("runtime.ref.000001").expect("runtime ref"),
        enterprise_id: enterprise_id(),
        ownership_path: ownership_path(),
        health: RuntimeHealth::Healthy,
    })
    .expect("runtime");
    let capability = CapabilityDescriptor::new(CapabilityDescriptorSpec {
        capability_id: kernel_domain::CapabilityId::new("CX-CAP-000001").expect("capability"),
        description: "Studio projection".to_owned(),
        dependencies: vec!["gateway".to_owned()],
        inputs: vec!["request".to_owned()],
        outputs: vec!["response".to_owned()],
        required_permissions: vec![permission_reference("runtime")],
        governing_policies: vec![PolicyId::new("CX-POL-000003").expect("policy")],
    })
    .expect("capability");
    let lease = LeaseRecord::issue(LeaseIssuanceSpec {
        lease_id: LeaseId::new("CX-LEASE-000001").expect("lease"),
        runtime_id: runtime_id.clone(),
        agent_id: agent_id.clone(),
        issued_at: time_reference(),
        expires_at: TimeReference::new("2026-07-19T01:00:00Z").expect("expires"),
        supersedes_lease_id: None,
        evidence: "lease evidence".to_owned(),
    })
    .expect("lease");
    let heartbeat = HeartbeatRecord::observe(HeartbeatObservationSpec {
        heartbeat_id: HeartbeatId::new("CX-HB-000001").expect("heartbeat"),
        runtime_id: runtime_id.clone(),
        agent_id: agent_id.clone(),
        recorded_at: time_reference(),
        fresh_until: TimeReference::new("2026-07-19T00:30:00Z").expect("fresh"),
        reported_presence: PresenceState::Registered,
        reported_health: RuntimeHealth::Healthy,
        active_lease_id: Some(lease.lease_id().clone()),
        evidence: "heartbeat evidence".to_owned(),
    })
    .expect("heartbeat");
    let registration = AgentRegistration::new(AgentRegistrationSpec {
        agent: agent_definition,
        runtime: runtime_entity,
        supervisor: owner,
        capabilities: vec![capability],
        presence_state: PresenceState::Registered,
        health: RuntimeHealth::Healthy,
        registered_at: time_reference(),
        lease: Some(lease),
        last_heartbeat: Some(heartbeat),
    })
    .expect("registration");
    let mut registry = AgentRegistry::new();
    registry.register(registration).expect("register");
    registry
        .runtime_snapshot(
            &agent_id,
            &time_reference(),
            &HeartbeatFreshnessPolicy::new(
                TimeReference::new("2026-07-19T00:20:00Z").expect("late"),
                TimeReference::new("2026-07-19T00:10:00Z").expect("stale"),
                StableVersion::new("freshness_version", "1.0.0").expect("version"),
            )
            .expect("policy"),
            None,
            None,
        )
        .expect("snapshot")
}

pub fn event_replay_entries() -> Vec<EventReplayEntry<String>> {
    let event = EventEnvelope::new(
        kernel_domain::EventId::new("CX-EVT-000001").expect("event"),
        EventType::new("studio.view").expect("type"),
        EventVersion::new("1.0.0").expect("version"),
        time_reference(),
        time_reference(),
        EventSource::new(
            EventComponent::new("studio").expect("component"),
            Some(runtime_id()),
        ),
        EventSubject::new(
            EventSubjectType::new("workflow").expect("type"),
            EventSubjectId::new("CX-WF-000001").expect("subject"),
        ),
        "payload".to_owned(),
        EventClassification::new("INTERNAL").expect("classification"),
        EventTrace::new(
            Some(EventActorId::new("studio.user").expect("actor")),
            Some(workflow_id()),
            None,
            None,
            vec![AuditEvidenceId::new("CX-AUD-000001").expect("audit")],
        )
        .expect("trace"),
        Some(correlation_id()),
        EventCausation::root(),
    );
    vec![EventReplayEntry::new(
        StreamPosition::new(
            kernel_domain::EventStreamId::new("studio.stream").expect("stream"),
            kernel_domain::EventSequence::new(1).expect("sequence"),
        ),
        event,
    )]
}

pub fn studio_view_reference(kind: StudioViewKind) -> StudioViewReference {
    let name = match kind {
        StudioViewKind::TopView => "studio.top.view",
        StudioViewKind::DigitalTwin => "studio.digital.twin",
        StudioViewKind::Runtime => "studio.runtime.view",
        StudioViewKind::Workflow => "studio.workflow.view",
        StudioViewKind::Task => "studio.task.view",
        StudioViewKind::EventTimeline => "studio.event.timeline",
        StudioViewKind::Memory => "studio.memory.view",
        StudioViewKind::Audit => "studio.audit.view",
        StudioViewKind::Revenue => "studio.revenue.view",
        StudioViewKind::CommandConsole => "studio.command.console",
    };
    StudioViewReference::new(kind, name).expect("view")
}

pub fn studio_selection_context() -> StudioSelectionContext {
    StudioSelectionContext::new(
        ownership_path(),
        Some(AgentId::new("CX-AGT-000001").expect("agent")),
        Some(runtime_id()),
        Some(workflow_id()),
        Some(task_instance_reference()),
        Some(ExecutionSessionId::new("execution.session-0001").expect("execution")),
        Some(memory_record_reference()),
    )
    .expect("selection")
}
