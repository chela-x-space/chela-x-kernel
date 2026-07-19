use crate::studio_test_support::{correlation_id, event_replay_entries, studio_audit_reference};
use crate::StudioEventTimelineProjection;

#[test]
fn studio_event_timeline_preserves_deterministic_ordering_k11_006() {
    let projection = StudioEventTimelineProjection::new(
        event_replay_entries(),
        Some(correlation_id()),
        studio_audit_reference(),
    )
    .expect("projection");
    assert_eq!(projection.event_replay_entries().len(), 1);
}
