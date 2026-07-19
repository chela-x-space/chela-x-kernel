use crate::studio_test_support::{
    memory_query_result, memory_retrieval_result, ownership_path, studio_audit_reference,
};
use crate::StudioMemoryProjection;

#[test]
fn studio_memory_projection_preserves_scope_and_results_k11_007() {
    let projection = StudioMemoryProjection::new(
        ownership_path(),
        Some(memory_retrieval_result()),
        Some(memory_query_result()),
        studio_audit_reference(),
    )
    .expect("projection");
    assert!(projection.memory_retrieval_result().is_some());
}
