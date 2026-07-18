use crate::memory_test_support::{event_only_memory_record, memory_record};
use crate::{
    DomainError, ExecutionMemoryProjection, MemoryProjection, RuntimeMemoryProjection,
    TaskMemoryProjection, WorkflowMemoryProjection,
};

#[test]
fn memory_projection_preserves_record_facts_k9_007() {
    let record = memory_record("memory.record-projection");
    let projection = MemoryProjection::new(&record);
    assert_eq!(
        projection.memory_record_reference(),
        record.memory_record_reference()
    );
    assert_eq!(
        projection.memory_classification(),
        record.memory_classification()
    );
    assert_eq!(
        projection.memory_audit_reference(),
        record.memory_audit_reference()
    );
}

#[test]
fn workflow_memory_projection_constructs_from_workflow_provenance_k9_007() {
    let projection = WorkflowMemoryProjection::new(&memory_record("memory.record-workflow"))
        .expect("projection");
    assert_eq!(projection.workflow_id().as_str(), "CX-WF-000001");
}

#[test]
fn task_memory_projection_constructs_from_task_provenance_k9_007() {
    let projection =
        TaskMemoryProjection::new(&memory_record("memory.record-task")).expect("projection");
    assert_eq!(
        projection
            .task_instance_reference()
            .task_instance_id()
            .as_str(),
        "task.instance.memory-0001"
    );
}

#[test]
fn execution_memory_projection_constructs_from_execution_provenance_k9_007() {
    let projection = ExecutionMemoryProjection::new(&memory_record("memory.record-execution"))
        .expect("projection");
    assert_eq!(
        projection.execution_session_id().as_str(),
        "execution.session-0001"
    );
}

#[test]
fn runtime_memory_projection_constructs_from_runtime_provenance_k9_007() {
    let projection =
        RuntimeMemoryProjection::new(&memory_record("memory.record-runtime")).expect("projection");
    assert_eq!(projection.runtime_id().as_str(), "runtime.memory.primary");
}

#[test]
fn workflow_memory_projection_rejects_missing_workflow_provenance_k9_007() {
    let error =
        WorkflowMemoryProjection::new(&event_only_memory_record("memory.record-event-only"))
            .expect_err("workflow provenance is required");
    assert_eq!(
        error,
        DomainError::InvalidMemory("workflow memory projection requires workflow provenance")
    );
}
