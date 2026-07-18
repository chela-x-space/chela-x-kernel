use crate::authorization::{
    AuthorizationDecisionOutcome, AuthorizationDecisionReference,
    AuthorizationEvaluationOrderVersion, MatchedPolicyEvidenceReference,
};
use crate::memory::{MemoryAuditReference, MemoryRecordId, MemoryRecordReference};
use crate::memory_projection::MemoryProjection;
use crate::memory_query::MemoryQuery;
use crate::memory_record::{
    MemoryClassification, MemoryProvenance, MemoryRecord, MemoryRetentionPolicyReference,
};
use crate::ownership::OwnershipPath;
use crate::request::TimeReference;
use crate::{
    AuditEvidenceId, AuthorizationDecisionId, AuthorizationRequestId, EnterpriseId, EventId,
    ExecutionSessionId, MemoryRetrievalRequest, PolicyId, ProjectId, RuntimeId, TaskEvidenceId,
    TaskEvidenceReference, TaskInstanceId, TaskInstanceReference, WorkflowId, WorkspaceId,
};

pub(crate) fn memory_record_id(value: &str) -> MemoryRecordId {
    MemoryRecordId::new(value).expect("memory record id")
}

pub(crate) fn memory_record_reference(value: &str) -> MemoryRecordReference {
    MemoryRecordReference::new(memory_record_id(value))
}

pub(crate) fn enterprise_ownership() -> OwnershipPath {
    OwnershipPath::new(
        EnterpriseId::new("CX-ENT-000001").expect("enterprise"),
        None,
        None,
        None,
    )
    .expect("ownership")
}

pub(crate) fn workspace_ownership(workspace: &str) -> OwnershipPath {
    OwnershipPath::new(
        EnterpriseId::new("CX-ENT-000001").expect("enterprise"),
        Some(WorkspaceId::new(workspace).expect("workspace")),
        Some(ProjectId::new("CX-PROJ-000001").expect("project")),
        None,
    )
    .expect("ownership")
}

pub(crate) fn memory_classification() -> MemoryClassification {
    MemoryClassification::new("INTERNAL").expect("classification")
}

pub(crate) fn time_reference() -> TimeReference {
    TimeReference::new("2026-07-18T00:00:00Z").expect("time")
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

pub(crate) fn task_instance_reference() -> TaskInstanceReference {
    TaskInstanceReference::new(
        TaskInstanceId::new("task.instance.memory-0001").expect("task instance"),
    )
}

pub(crate) fn task_evidence_reference() -> TaskEvidenceReference {
    TaskEvidenceReference::new(TaskEvidenceId::new("CX-TEVID-000001").expect("task evidence"))
}

pub(crate) fn workflow_id() -> WorkflowId {
    WorkflowId::new("CX-WF-000001").expect("workflow")
}

pub(crate) fn execution_session_id() -> ExecutionSessionId {
    ExecutionSessionId::new("execution.session-0001").expect("execution session")
}

pub(crate) fn runtime_id() -> RuntimeId {
    RuntimeId::new("runtime.memory.primary").expect("runtime")
}

pub(crate) fn memory_provenance() -> MemoryProvenance {
    MemoryProvenance::new(
        EventId::new("CX-EVT-000001").expect("event"),
        Some(workflow_id()),
        Some(task_instance_reference()),
        Some(execution_session_id()),
        Some(runtime_id()),
        Some(authorization_reference(AuthorizationDecisionOutcome::Allow)),
        Some(task_evidence_reference()),
    )
    .expect("provenance")
}

pub(crate) fn event_only_provenance() -> MemoryProvenance {
    MemoryProvenance::new(
        EventId::new("CX-EVT-000099").expect("event"),
        None,
        None,
        None,
        None,
        None,
        None,
    )
    .expect("event-only provenance")
}

pub(crate) fn memory_audit_reference(value: &str) -> MemoryAuditReference {
    MemoryAuditReference::new(
        memory_record_id(value),
        crate::EventTraceReference::new("memory.audit-0001").expect("audit trace"),
        vec![AuditEvidenceId::new("CX-AUD-000001").expect("audit evidence")],
    )
    .expect("memory audit reference")
}

pub(crate) fn memory_record(value: &str) -> MemoryRecord {
    MemoryRecord::new(
        memory_record_reference(value),
        workspace_ownership("CX-WS-000001"),
        "retained enterprise memory",
        memory_classification(),
        memory_provenance(),
        MemoryRetentionPolicyReference::new(
            PolicyId::new("CX-POL-000002").expect("retention policy"),
        ),
        memory_audit_reference(value),
        time_reference(),
    )
    .expect("memory record")
}

pub(crate) fn event_only_memory_record(value: &str) -> MemoryRecord {
    MemoryRecord::new(
        memory_record_reference(value),
        enterprise_ownership(),
        "event only enterprise memory",
        MemoryClassification::new("PUBLIC").expect("classification"),
        event_only_provenance(),
        MemoryRetentionPolicyReference::new(
            PolicyId::new("CX-POL-000003").expect("retention policy"),
        ),
        memory_audit_reference(value),
        time_reference(),
    )
    .expect("memory record")
}

pub(crate) fn memory_projection(value: &str) -> MemoryProjection {
    MemoryProjection::new(&memory_record(value))
}

pub(crate) fn retrieval_request(memory_query: MemoryQuery) -> MemoryRetrievalRequest {
    MemoryRetrievalRequest::new(
        authorization_reference(AuthorizationDecisionOutcome::Allow),
        workspace_ownership("CX-WS-000001"),
        memory_query,
    )
    .expect("retrieval request")
}
