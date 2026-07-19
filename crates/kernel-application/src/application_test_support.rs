use kernel_domain::{
    ActionVerb, AgentId, AuditEvidenceId, AuthorizationDecisionId, AuthorizationDecisionOutcome,
    AuthorizationDecisionReference, AuthorizationEvaluationOrderVersion,
    AuthorizationPrincipalReference, AuthorizationPrincipalType, AuthorizationRequestId,
    AuthorizationRequestRecord, AuthorizationSubject, AuthorizationTarget, CorrelationId,
    CredentialStatusReference, EnterpriseId, EventTraceReference, ExecutionSessionId,
    MatchedPolicyEvidenceReference, MemoryAuditReference, MemoryCaptureDecision,
    MemoryCaptureRequest, MemoryClassification, MemoryProvenance, MemoryRecord, MemoryRecordId,
    MemoryRecordReference, MemoryRetentionPolicyReference, PermissionEffectIntent, PermissionId,
    PermissionReference, PolicyId, PrincipalId, PrincipalLifecycleStateReference, ProjectId,
    ResourceType, ScopeId, ScopeLevel, ScopeReference, TimeReference, WorkflowId, WorkspaceId,
};
use kernel_gateway::{
    GatewayApiVersion, GatewayAuditReference, GatewayAuthenticationContext,
    GatewayAuthorizationBinding, GatewayCommandPayload, GatewayCommandRequest,
    GatewayCommandResponse, GatewayOperationKind, GatewayOperationReference, GatewayProtocol,
    GatewayQueryPayload, GatewayQueryRequest, GatewayRequestContext, GatewayRequestEnvelope,
    GatewayResponseEnvelope, GatewayStatusSnapshot,
};
use kernel_studio::{
    StudioApiVersion, StudioAttentionState, StudioAuditReference, StudioCommandRequest,
    StudioCommandResponse, StudioFilterContext, StudioNavigationReference, StudioSelectionContext,
    StudioStatusSnapshot, StudioTopViewProjection, StudioViewKind, StudioViewProjection,
    StudioViewReference, StudioViewRequest, StudioViewResponse,
};

use crate::{
    application::{APPLICATION_COMMAND_CAPABILITY, APPLICATION_QUERY_CAPABILITY},
    ApplicationApiVersion, ApplicationAuditReference, ApplicationCapabilityDeclaration,
    ApplicationCapabilityReference, ApplicationDependencyCompatibilityReference,
    ApplicationIdentity, ApplicationIdentityKind, ApplicationQueryIntent,
    ApplicationRequestContext, ApplicationRequestId, ApplicationSessionReference,
    ApplicationSessionStatusReference, ApplicationStatusSnapshot, ApplicationValidationStatus,
    ApplicationViewIntent,
};

pub fn enterprise_id() -> EnterpriseId {
    EnterpriseId::new("CX-ENT-000001").expect("enterprise")
}

pub fn workspace_id() -> WorkspaceId {
    WorkspaceId::new("CX-WS-000001").expect("workspace")
}

pub fn workspace_id_mismatch() -> WorkspaceId {
    WorkspaceId::new("CX-WS-000002").expect("workspace")
}

pub fn project_id() -> ProjectId {
    ProjectId::new("CX-PROJ-000001").expect("project")
}

pub fn project_id_mismatch() -> ProjectId {
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
        Some(workspace_id_mismatch()),
        Some(project_id()),
        None,
    )
    .expect("ownership")
}

pub fn project_scope_mismatch_path() -> kernel_domain::OwnershipPath {
    kernel_domain::OwnershipPath::new(
        enterprise_id(),
        Some(workspace_id()),
        Some(project_id_mismatch()),
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

pub fn other_correlation_id() -> CorrelationId {
    CorrelationId::new("CX-COR-000002").expect("correlation")
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

pub fn gateway_api_version() -> GatewayApiVersion {
    GatewayApiVersion::new("2026.07.19").expect("version")
}

pub fn studio_api_version() -> StudioApiVersion {
    StudioApiVersion::new("2026.07.19").expect("version")
}

pub fn application_api_version() -> ApplicationApiVersion {
    ApplicationApiVersion::new("2026.07.19").expect("version")
}

pub fn gateway_audit_reference() -> GatewayAuditReference {
    GatewayAuditReference::new(
        EventTraceReference::new("gateway.audit.trace.000001").expect("trace"),
        Some(correlation_id()),
        vec![
            AuditEvidenceId::new("CX-AUD-000001").expect("audit"),
            AuditEvidenceId::new("CX-AUD-000002").expect("audit"),
        ],
    )
    .expect("audit")
}

pub fn studio_audit_reference() -> StudioAuditReference {
    StudioAuditReference::new(
        EventTraceReference::new("studio.audit.trace.000001").expect("trace"),
        Some(correlation_id()),
        vec![
            AuditEvidenceId::new("CX-AUD-000001").expect("audit"),
            AuditEvidenceId::new("CX-AUD-000002").expect("audit"),
        ],
        Some(gateway_audit_reference()),
    )
    .expect("audit")
}

pub fn application_audit_reference() -> ApplicationAuditReference {
    ApplicationAuditReference::new(
        EventTraceReference::new("application.audit.trace.000001").expect("trace"),
        Some(EventTraceReference::new("application.cause.trace.000001").expect("cause")),
        correlation_id(),
        vec![
            AuditEvidenceId::new("CX-AUD-000003").expect("audit"),
            AuditEvidenceId::new("CX-AUD-000004").expect("audit"),
        ],
        Some(studio_audit_reference()),
    )
    .expect("audit")
}

pub fn gateway_authentication_context() -> GatewayAuthenticationContext {
    GatewayAuthenticationContext::new(
        principal_reference(),
        "auth.password",
        EventTraceReference::new("gateway.auth.trace.000001").expect("trace"),
        time_reference(),
        None,
        Some(EventTraceReference::new("gateway.session.trace.000001").expect("session")),
    )
    .expect("auth")
}

pub fn mismatched_gateway_authentication_context() -> GatewayAuthenticationContext {
    GatewayAuthenticationContext::new(
        other_principal_reference(),
        "auth.password",
        EventTraceReference::new("gateway.auth.trace.000002").expect("trace"),
        time_reference(),
        None,
        None,
    )
    .expect("auth")
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
            ResourceType::new(resource_type).expect("type"),
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
        time_reference(),
        "application request",
    )
    .expect("request")
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
            AuthorizationEvaluationOrderVersion::new("2026.07.19").expect("version"),
            MatchedPolicyEvidenceReference::new("gateway.policy.evidence").expect("evidence"),
            "2026-07-19T00:00:00Z",
        )
        .expect("decision"),
    )
    .expect("binding")
}

pub fn gateway_request_context(
    gateway_operation_reference: GatewayOperationReference,
    _resource_type: &str,
    _resource_identifier: &str,
    ownership_path: kernel_domain::OwnershipPath,
    gateway_authentication_context: GatewayAuthenticationContext,
    gateway_authorization_binding: GatewayAuthorizationBinding,
) -> GatewayRequestContext {
    GatewayRequestContext::new(
        gateway_api_version(),
        gateway_operation_reference,
        gateway_authentication_context,
        gateway_authorization_binding,
        ownership_path,
        correlation_id(),
        time_reference(),
        None,
        gateway_audit_reference(),
    )
    .expect("context")
}

pub fn operation_gateway_status() -> GatewayOperationReference {
    GatewayOperationReference::new(GatewayOperationKind::Status, "gateway.status")
        .expect("operation")
}

pub fn operation_memory_capture() -> GatewayOperationReference {
    GatewayOperationReference::new(GatewayOperationKind::Command, "memory.capture")
        .expect("operation")
}

pub fn memory_record_id() -> MemoryRecordId {
    MemoryRecordId::new("application.memory.record.000001").expect("memory")
}

pub fn memory_record_reference() -> MemoryRecordReference {
    MemoryRecordReference::new(memory_record_id())
}

pub fn workflow_id() -> WorkflowId {
    WorkflowId::new("CX-WF-000001").expect("workflow")
}

pub fn memory_record(record_ownership_path: kernel_domain::OwnershipPath) -> MemoryRecord {
    let memory_record_reference = memory_record_reference();
    MemoryRecord::new(
        memory_record_reference.clone(),
        record_ownership_path,
        "application governed memory summary",
        MemoryClassification::new("INTERNAL").expect("classification"),
        MemoryProvenance::new(
            kernel_domain::EventId::new("CX-EVT-000001").expect("event"),
            Some(workflow_id()),
            None,
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
            vec![AuditEvidenceId::new("CX-AUD-000005").expect("audit")],
        )
        .expect("audit"),
        time_reference(),
    )
    .expect("record")
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
        "capture",
    )
    .expect("capture")
}

pub fn gateway_status_snapshot() -> GatewayStatusSnapshot {
    GatewayStatusSnapshot::new(
        gateway_api_version(),
        vec![GatewayProtocol::Http, GatewayProtocol::Internal],
        vec![operation_gateway_status()],
        time_reference(),
        gateway_audit_reference(),
    )
    .expect("status")
}

pub fn gateway_status_envelope() -> GatewayRequestEnvelope {
    let binding = authorization_binding(
        "gateway",
        "gateway.status",
        ownership_path(),
        AuthorizationDecisionOutcome::Allow,
    );
    GatewayRequestEnvelope::query(
        gateway_request_context(
            operation_gateway_status(),
            "gateway",
            "gateway.status",
            ownership_path(),
            gateway_authentication_context(),
            binding,
        ),
        GatewayQueryRequest::new(GatewayQueryPayload::GatewayStatus),
    )
    .expect("query")
}

pub fn gateway_status_response_envelope() -> GatewayResponseEnvelope {
    GatewayResponseEnvelope::new(
        &gateway_status_envelope(),
        correlation_id(),
        kernel_gateway::gateway_response::GatewayEnvelopeResponse::Query(Box::new(
            kernel_gateway::GatewayQueryResponse::StatusSnapshot(gateway_status_snapshot()),
        )),
        gateway_audit_reference(),
        later_time_reference(),
    )
    .expect("response")
}

pub fn memory_command_envelope() -> GatewayRequestEnvelope {
    let binding = authorization_binding(
        "memory",
        memory_record_id().as_str(),
        ownership_path(),
        AuthorizationDecisionOutcome::Allow,
    );
    GatewayRequestEnvelope::command(
        gateway_request_context(
            operation_memory_capture(),
            "memory",
            memory_record_id().as_str(),
            ownership_path(),
            gateway_authentication_context(),
            binding,
        ),
        GatewayCommandRequest::new(GatewayCommandPayload::MemoryCapture(Box::new(
            memory_capture_request(),
        ))),
    )
    .expect("command")
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
    StudioSelectionContext::new(ownership_path(), None, None, None, None, None, None)
        .expect("selection")
}

pub fn studio_filter_context() -> StudioFilterContext {
    StudioFilterContext::new(Vec::new(), Vec::new(), None).expect("filter")
}

pub fn studio_navigation_reference(kind: StudioViewKind) -> StudioNavigationReference {
    StudioNavigationReference::new(
        studio_view_reference(kind),
        "studio.navigation.target.000001",
    )
    .expect("navigation")
}

pub fn studio_view_request() -> StudioViewRequest {
    StudioViewRequest::new(
        studio_api_version(),
        studio_view_reference(StudioViewKind::TopView),
        studio_selection_context(),
        Some(studio_filter_context()),
        Some(studio_navigation_reference(StudioViewKind::TopView)),
        correlation_id(),
        time_reference(),
        vec![gateway_status_envelope()],
        studio_audit_reference(),
    )
    .expect("request")
}

pub fn studio_command_request() -> StudioCommandRequest {
    StudioCommandRequest::new(
        studio_api_version(),
        studio_view_reference(StudioViewKind::CommandConsole),
        StudioSelectionContext::new(
            ownership_path(),
            None,
            None,
            None,
            None,
            None,
            Some(memory_record_reference()),
        )
        .expect("selection"),
        correlation_id(),
        time_reference(),
        memory_command_envelope(),
        studio_audit_reference(),
    )
    .expect("request")
}

pub fn studio_view_response() -> StudioViewResponse {
    let projection = StudioViewProjection::TopView(Box::new(
        StudioTopViewProjection::new(
            ownership_path(),
            vec![AgentId::new("CX-AGT-000001").expect("agent")],
            vec![kernel_domain::RuntimeId::new("runtime.primary").expect("runtime")],
            vec![workflow_id()],
            Vec::new(),
            vec![ExecutionSessionId::new("execution.session-000001").expect("execution")],
            StudioAttentionState::Nominal,
            studio_audit_reference(),
        )
        .expect("projection"),
    ));
    StudioViewResponse::new(
        &studio_view_request(),
        correlation_id(),
        projection,
        vec![gateway_status_response_envelope()],
        studio_audit_reference(),
        later_time_reference(),
    )
    .expect("response")
}

pub fn studio_command_response() -> StudioCommandResponse {
    StudioCommandResponse::new(
        &studio_command_request(),
        correlation_id(),
        GatewayCommandResponse::MemoryCapture(Box::new(MemoryCaptureDecision::Accepted(Box::new(
            memory_record(ownership_path()),
        )))),
        studio_audit_reference(),
        later_time_reference(),
    )
    .expect("response")
}

pub fn application_request_id() -> ApplicationRequestId {
    ApplicationRequestId::new("application.request.000001").expect("request id")
}

pub fn command_capability() -> ApplicationCapabilityReference {
    ApplicationCapabilityReference::new(APPLICATION_COMMAND_CAPABILITY).expect("capability")
}

pub fn query_capability() -> ApplicationCapabilityReference {
    ApplicationCapabilityReference::new(APPLICATION_QUERY_CAPABILITY).expect("capability")
}

pub fn application_identity() -> ApplicationIdentity {
    ApplicationIdentity::new(
        "application.integration.primary",
        ApplicationIdentityKind::ExternalApplication,
        "application.integration",
        "2026.07.19",
        vec![command_capability(), query_capability()],
        Some("application.environment.primary".to_owned()),
    )
    .expect("identity")
}

pub fn application_capability_declaration() -> ApplicationCapabilityDeclaration {
    ApplicationCapabilityDeclaration::new(
        application_api_version(),
        vec![command_capability(), query_capability()],
        vec![studio_view_reference(StudioViewKind::TopView)],
        true,
        true,
    )
    .expect("capability")
}

pub fn query_application_request_context() -> ApplicationRequestContext {
    ApplicationRequestContext::new(
        application_api_version(),
        application_request_id(),
        application_identity(),
        application_capability_declaration(),
        gateway_authentication_context(),
        authorization_binding(
            "gateway",
            "gateway.status",
            ownership_path(),
            AuthorizationDecisionOutcome::Allow,
        ),
        studio_selection_context(),
        Some(application_session_reference()),
        correlation_id(),
        Some(EventTraceReference::new("application.cause.trace.000001").expect("cause")),
        time_reference(),
        application_audit_reference(),
    )
    .expect("context")
}

pub fn command_application_request_context() -> ApplicationRequestContext {
    ApplicationRequestContext::new(
        application_api_version(),
        application_request_id(),
        application_identity(),
        application_capability_declaration(),
        gateway_authentication_context(),
        authorization_binding(
            "memory",
            memory_record_id().as_str(),
            ownership_path(),
            AuthorizationDecisionOutcome::Allow,
        ),
        StudioSelectionContext::new(
            ownership_path(),
            None,
            None,
            None,
            None,
            None,
            Some(memory_record_reference()),
        )
        .expect("selection"),
        Some(application_session_reference()),
        correlation_id(),
        Some(EventTraceReference::new("application.cause.trace.000001").expect("cause")),
        time_reference(),
        application_audit_reference(),
    )
    .expect("context")
}

pub fn application_session_reference() -> ApplicationSessionReference {
    ApplicationSessionReference::new(
        "application.session.000001",
        application_identity().application_identifier().clone(),
        ownership_path(),
        correlation_id(),
        time_reference(),
        later_time_reference(),
        ApplicationSessionStatusReference::new("application.session.active").expect("status"),
    )
    .expect("session")
}

pub fn application_view_intent() -> ApplicationViewIntent {
    ApplicationViewIntent::new(
        studio_view_reference(StudioViewKind::TopView),
        studio_selection_context(),
        Some(studio_filter_context()),
        vec![studio_navigation_reference(StudioViewKind::TopView)],
    )
    .expect("view intent")
}

pub fn application_query_intent() -> ApplicationQueryIntent {
    ApplicationQueryIntent::new(
        query_application_request_context(),
        query_capability(),
        application_view_intent(),
        studio_view_request(),
    )
    .expect("query intent")
}

pub fn application_status_snapshot() -> ApplicationStatusSnapshot {
    ApplicationStatusSnapshot::new(
        application_api_version(),
        application_identity(),
        application_capability_declaration(),
        vec![studio_view_reference(StudioViewKind::TopView)],
        vec![
            crate::ApplicationIntentKind::View,
            crate::ApplicationIntentKind::Query,
            crate::ApplicationIntentKind::Command,
        ],
        vec![
            ApplicationDependencyCompatibilityReference::new("dependency.kernel-studio.2026")
                .expect("dependency"),
        ],
        ApplicationValidationStatus::Validated,
        StudioStatusSnapshot::new(
            studio_api_version(),
            vec![studio_view_reference(StudioViewKind::TopView)],
            gateway_status_snapshot(),
            time_reference(),
            studio_audit_reference(),
        )
        .expect("status"),
        application_audit_reference(),
        time_reference(),
    )
    .expect("status")
}
