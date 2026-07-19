use kernel_domain::{validate_replay_ordering, CorrelationId, EventReplayEntry};

use crate::studio::{StudioAuditReference, StudioError, StudioErrorCode, StudioResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudioEventTimelineProjection {
    event_replay_entries: Vec<EventReplayEntry<String>>,
    correlation_id: Option<CorrelationId>,
    studio_audit_reference: StudioAuditReference,
}

impl StudioEventTimelineProjection {
    pub fn new(
        event_replay_entries: Vec<EventReplayEntry<String>>,
        correlation_id: Option<CorrelationId>,
        studio_audit_reference: StudioAuditReference,
    ) -> StudioResult<Self> {
        if event_replay_entries.is_empty() {
            return Err(StudioError::new(
                StudioErrorCode::ProjectionMismatch,
                "studio event timeline requires at least one replay entry",
            )?);
        }
        validate_replay_ordering(&event_replay_entries)
            .map_err(StudioError::from_domain_rejection)?;
        if let Some(correlation_id) = &correlation_id {
            if event_replay_entries.iter().any(|entry| {
                entry.event().correlation_id().is_some()
                    && entry.event().correlation_id() != Some(correlation_id)
            }) {
                return Err(StudioError::new(
                    StudioErrorCode::EventSequenceInconsistency,
                    "studio event timeline correlation continuity must be preserved",
                )?);
            }
        }
        Ok(Self {
            event_replay_entries,
            correlation_id,
            studio_audit_reference,
        })
    }

    pub fn event_replay_entries(&self) -> &[EventReplayEntry<String>] {
        &self.event_replay_entries
    }
    pub fn correlation_id(&self) -> Option<&CorrelationId> {
        self.correlation_id.as_ref()
    }
    pub fn studio_audit_reference(&self) -> &StudioAuditReference {
        &self.studio_audit_reference
    }
}
