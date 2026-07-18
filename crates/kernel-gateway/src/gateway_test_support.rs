use kernel_domain::{
    ActionVerb, AuditEvidenceId, AuthorizationDecisionId, AuthorizationDecisionOutcome,
    AuthorizationDecisionReference, AuthorizationEvaluationOrderVersion,
    AuthorizationPrincipalReference, AuthorizationPrincipalType, AuthorizationRequestId,
    AuthorizationRequestRecord, AuthorizationSubject, AuthorizationTarget, CorrelationId,
    CredentialStatusReference, EnterpriseId, EventTraceReference, MatchedPolicyEvidenceReference,
    MemoryAuditReference, MemoryCaptureDecision, MemoryCaptureRequest, MemoryClassification,
    MemoryProvenance, MemoryQuery, MemoryRecord, MemoryRecordId, MemoryRecordReference,
    MemoryRetentionPolicyReference, MemoryRetrievalRequest, PermissionEffectIntent, PermissionId,
    PermissionReference, PolicyId, PrincipalId, PrincipalLifecycleStateReference, ProjectId,
    ResourceType, ScopeId, ScopeLevel, ScopeReference, StableVersion, StateSequence,
    TaskInstanceId, TaskInstanceReference, TaskState, TaskStateSnapshot, TimeReference, WorkflowId,
    WorkflowState, WorkflowStateSnapshot, WorkflowTransitionControlRequest, WorkspaceId,
};

use crate::{
    GatewayApiVersion, GatewayAuditReference, GatewayAuthenticationContext,
    GatewayAuthorizationBinding, GatewayCommandPayload, GatewayCommandRequest,
    GatewayOperationKind, GatewayOperationReference, GatewayProtocol, GatewayQueryPayload,
    GatewayQueryRequest, GatewayRateGovernanceReference, GatewayRequestContext,
    GatewayStatusSnapshot,
};

pub fn enterprise_id() -> EnterpriseId {
    EnterpriseId::new("CX-ENT-000001").expect("enterprise")
}

pub fn workspace_id() -> WorkspaceId {
    WorkspaceId::new("CX-WS-000001").expect("workspace")
}

pub fn workspace_id_2() -> WorkspaceId {
    WorkspaceId::new("CX-WS-000002").expect("workspace")
}

pub fn project_id() -> ProjectId {
    ProjectId::new("CX-PROJ-000001").expect("project")
}

pub fn project_id_2() -> ProjectId {
    ProjectId::new("CX-PROJ-000002").expect("project")
}

pub fn ownership_path() -> kernel_domain::OwnershipPath {
    kernel_domain::OwnershipPath::new(
        enterprise_id(),
        Some(workspace_id()),
        Some(project_id()),
        None,
    )
    .expect("ownership")
}

pub fn workspace_scope_mismatch_path() -> kernel_domain::OwnershipPath {
    kernel_domain::OwnershipPath::new(
        enterprise_id(),
        Some(workspace_id_2()),
        Some(project_id()),
        None,
    )
    .expect("ownership")
}

pub fn project_scope_mismatch_path() -> kernel_domain::OwnershipPath {
    kernel_domain::OwnershipPath::new(
        enterprise_id(),
        Some(workspace_id()),
        Some(project_id_2()),
        None,
    )
    .expect("ownership")
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

pub fn other_principal_reference() -> AuthorizationPrincipalReference {
    AuthorizationPrincipalReference::new(
        PrincipalId::new("CX-PRN-000002").expect("principal"),
        AuthorizationPrincipalType::Employee,
        "CX-EMP-000002",
        enterprise_id(),
        PrincipalLifecycleStateReference::new("Active").expect("lifecycle"),
        CredentialStatusReference::new("Valid").expect("credential"),
    )
    .expect("principal")
}

pub fn authentication_context() -> GatewayAuthenticationContext {
    GatewayAuthenticationContext::new(
        principal_reference(),
        "auth.password",
        EventTraceReference::new("gateway.auth.trace.000001").expect("trace"),
        requested_at(),
        Some(EventTraceReference::new("gateway.credential.000001").expect("credential")),
        Some(EventTraceReference::new("gateway.session.000001").expect("session")),
    )
    .expect("auth context")
}

pub fn mismatched_authentication_context() -> GatewayAuthenticationContext {
    GatewayAuthenticationContext::new(
        other_principal_reference(),
        "auth.password",
        EventTraceReference::new("gateway.auth.trace.000002").expect("trace"),
        requested_at(),
        None,
        None,
    )
    .expect("auth context")
}

pub fn correlation_id() -> CorrelationId {
    CorrelationId::new("CX-COR-000001").expect("correlation")
}

pub fn requested_at() -> TimeReference {
    TimeReference::new("2026-07-18T00:00:00Z").expect("time")
}

pub fn responded_at() -> TimeReference {
    TimeReference::new("2026-07-18T00:10:00Z").expect("time")
}

pub fn gateway_audit_reference() -> GatewayAuditReference {
    GatewayAuditReference::new(
        EventTraceReference::new("gateway.audit.trace.000001").expect("trace"),
        Some(correlation_id()),
        vec![AuditEvidenceId::new("CX-AUD-000001").expect("audit")],
    )
    .expect("audit")
}

pub fn gateway_rate_governance_reference() -> GatewayRateGovernanceReference {
    GatewayRateGovernanceReference::new(
        PolicyId::new("CX-POL-000001").expect("policy"),
        None,
        "gateway.rate.standard",
        EventTraceReference::new("gateway.window.000001").expect("window"),
        gateway_audit_reference(),
    )
    .expect("rate")
}

pub fn gateway_api_version() -> GatewayApiVersion {
    GatewayApiVersion::new("2026.07.18").expect("version")
}

pub fn operation_memory_capture() -> GatewayOperationReference {
    GatewayOperationReference::new(GatewayOperationKind::Command, "memory.capture")
        .expect("operation")
}

pub fn operation_memory_query() -> GatewayOperationReference {
    GatewayOperationReference::new(GatewayOperationKind::Query, "memory.query").expect("operation")
}

pub fn operation_memory_retrieve() -> GatewayOperationReference {
    GatewayOperationReference::new(GatewayOperationKind::Query, "memory.retrieve")
        .expect("operation")
}

pub fn operation_workflow_transition() -> GatewayOperationReference {
    GatewayOperationReference::new(GatewayOperationKind::Command, "workflow.transition")
        .expect("operation")
}

pub fn operation_task_query() -> GatewayOperationReference {
    GatewayOperationReference::new(GatewayOperationKind::Query, "task.state").expect("operation")
}

pub fn operation_workflow_query() -> GatewayOperationReference {
    GatewayOperationReference::new(GatewayOperationKind::Query, "workflow.state")
        .expect("operation")
}

pub fn operation_gateway_status() -> GatewayOperationReference {
    GatewayOperationReference::new(GatewayOperationKind::Status, "gateway.status")
        .expect("operation")
}

pub fn permission_reference(resource_type: &str, action: &str) -> PermissionReference {
    PermissionReference::new(
        PermissionId::new("CX-PERM-000001").expect("permission"),
        ActionVerb::new(action).expect("verb"),
        ResourceType::new(resource_type).expect("resource type"),
        PermissionEffectIntent::new("Permit").expect("effect"),
    )
}

pub fn authorization_request_record(
    resource_type: &str,
    resource_identifier: &str,
    ownership_path: kernel_domain::OwnershipPath,
    request_id: AuthorizationRequestId,
) -> AuthorizationRequestRecord {
    AuthorizationRequestRecord::new(
        request_id,
        AuthorizationSubject::Principal(principal_reference()),
        permission_reference(resource_type, "read"),
        AuthorizationTarget::new(
            ResourceType::new(resource_type).expect("resource type"),
            resource_identifier,
            ScopeReference::new(
                ScopeId::new("CX-SCP-000001").expect("scope"),
                ScopeLevel::Project,
                ownership_path,
                None,
            )
            .expect("scope"),
        )
        .expect("target"),
        requested_at(),
        "gateway request",
    )
    .expect("auth request")
}

pub fn authorization_binding(
    resource_type: &str,
    resource_identifier: &str,
    ownership_path: kernel_domain::OwnershipPath,
    outcome: AuthorizationDecisionOutcome,
) -> GatewayAuthorizationBinding {
    let request_id = AuthorizationRequestId::new("CX-AUTHREQ-000001").expect("request");
    let request = authorization_request_record(
        resource_type,
        resource_identifier,
        ownership_path,
        request_id.clone(),
    );
    GatewayAuthorizationBinding::new(
        request,
        AuthorizationDecisionReference::new(
            AuthorizationDecisionId::new("CX-AUTHDEC-000001").expect("decision"),
            request_id,
            PolicyId::new("CX-POL-000001").expect("policy"),
            outcome,
            AuthorizationEvaluationOrderVersion::new("2026.07.18").expect("version"),
            MatchedPolicyEvidenceReference::new("gateway.policy.evidence").expect("evidence"),
            "2026-07-18T00:00:00Z",
        )
        .expect("decision"),
    )
    .expect("binding")
}

pub fn gateway_request_context(
    gateway_operation_reference: GatewayOperationReference,
    resource_type: &str,
    resource_identifier: &str,
    ownership_path: kernel_domain::OwnershipPath,
) -> GatewayRequestContext {
    GatewayRequestContext::new(
        gateway_api_version(),
        gateway_operation_reference,
        authentication_context(),
        authorization_binding(
            resource_type,
            resource_identifier,
            ownership_path.clone(),
            AuthorizationDecisionOutcome::Allow,
        ),
        ownership_path,
        correlation_id(),
        requested_at(),
        Some(gateway_rate_governance_reference()),
        gateway_audit_reference(),
    )
    .expect("context")
}

pub fn memory_record_id() -> MemoryRecordId {
    MemoryRecordId::new("gateway.memory.record.000001").expect("memory id")
}

pub fn memory_record_reference() -> MemoryRecordReference {
    MemoryRecordReference::new(memory_record_id())
}

pub fn workflow_id() -> WorkflowId {
    WorkflowId::new("CX-WF-000001").expect("workflow")
}

pub fn task_instance_reference() -> TaskInstanceReference {
    TaskInstanceReference::new(TaskInstanceId::new("task.instance.000001").expect("task"))
}

pub fn memory_record(record_ownership_path: kernel_domain::OwnershipPath) -> MemoryRecord {
    let memory_record_reference = memory_record_reference();
    MemoryRecord::new(
        memory_record_reference.clone(),
        record_ownership_path,
        "governed memory summary",
        MemoryClassification::new("INTERNAL").expect("classification"),
        MemoryProvenance::new(
            kernel_domain::EventId::new("CX-EVT-000001").expect("event"),
            Some(workflow_id()),
            Some(task_instance_reference()),
            None,
            None,
            Some(
                authorization_binding(
                    "memory",
                    memory_record_reference.memory_record_id().as_str(),
                    ownership_path(),
                    AuthorizationDecisionOutcome::Allow,
                )
                .authorization_decision_reference()
                .clone(),
            ),
            None,
        )
        .expect("provenance"),
        MemoryRetentionPolicyReference::new(PolicyId::new("CX-POL-000002").expect("policy")),
        MemoryAuditReference::new(
            memory_record_id(),
            EventTraceReference::new("memory.audit.trace.000001").expect("trace"),
            vec![AuditEvidenceId::new("CX-AUD-000002").expect("audit")],
        )
        .expect("memory audit"),
        requested_at(),
    )
    .expect("memory record")
}

pub fn memory_capture_request() -> MemoryCaptureRequest {
    MemoryCaptureRequest::new(
        memory_record(ownership_path()),
        authorization_binding(
            "memory",
            memory_record_id().as_str(),
            ownership_path(),
            AuthorizationDecisionOutcome::Allow,
        )
        .authorization_decision_reference()
        .clone(),
        "capture memory",
    )
    .expect("capture")
}

pub fn accepted_memory_capture_decision() -> MemoryCaptureDecision {
    MemoryCaptureDecision::evaluate(&memory_capture_request())
}

pub fn memory_query() -> MemoryQuery {
    MemoryQuery::by_workflow(workflow_id())
}

pub fn memory_retrieval_request() -> MemoryRetrievalRequest {
    MemoryRetrievalRequest::new(
        authorization_binding(
            "memory",
            memory_record_id().as_str(),
            ownership_path(),
            AuthorizationDecisionOutcome::Allow,
        )
        .authorization_decision_reference()
        .clone(),
        ownership_path(),
        memory_query(),
    )
    .expect("retrieval")
}

pub fn workflow_state_snapshot() -> WorkflowStateSnapshot {
    WorkflowStateSnapshot::new(
        workflow_id(),
        ownership_path(),
        StableVersion::new("workflow_definition_version", "2026.07.18").expect("version"),
        WorkflowState::Ready,
        StateSequence::new(1).expect("sequence"),
    )
}

pub fn workflow_transition_control_request() -> WorkflowTransitionControlRequest {
    WorkflowTransitionControlRequest::new(
        workflow_state_snapshot(),
        WorkflowState::Running,
        None,
        None,
        Vec::new(),
        None,
        kernel_domain::WorkflowLifecycleGuards {
            policy_valid: true,
            authorization_valid: true,
            delegation_valid: true,
            decision_valid: true,
            scope_valid: true,
            participants_valid: true,
            audit_evidence: None,
            upstream_outcomes_allow: true,
            retry_limit_respected: true,
            recovery_revalidated: true,
            failure_code: None,
        },
    )
    .expect("workflow transition control request")
}

pub fn task_state_snapshot() -> TaskStateSnapshot {
    TaskStateSnapshot::new(
        task_instance_reference(),
        TaskState::Pending,
        StateSequence::new(1).expect("sequence"),
    )
}

pub fn gateway_status_snapshot() -> GatewayStatusSnapshot {
    GatewayStatusSnapshot::new(
        gateway_api_version(),
        vec![GatewayProtocol::Http, GatewayProtocol::Internal],
        vec![operation_gateway_status(), operation_memory_query()],
        requested_at(),
        gateway_audit_reference(),
    )
    .expect("status")
}

pub fn memory_command_request() -> GatewayCommandRequest {
    GatewayCommandRequest::new(GatewayCommandPayload::MemoryCapture(Box::new(
        memory_capture_request(),
    )))
}

pub fn memory_query_request() -> GatewayQueryRequest {
    GatewayQueryRequest::new(GatewayQueryPayload::MemoryRetrieval(Box::new(
        memory_retrieval_request(),
    )))
}
