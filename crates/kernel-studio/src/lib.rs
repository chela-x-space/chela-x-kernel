#![forbid(unsafe_code)]

pub mod studio;
pub mod studio_audit;
pub mod studio_command;
pub mod studio_digital_twin;
pub mod studio_event;
pub mod studio_memory;
pub mod studio_revenue;
pub mod studio_runtime;
pub mod studio_task;
pub mod studio_top_view;
pub mod studio_validation;
pub mod studio_workflow;

pub use studio::{
    StudioApiVersion, StudioAuditReference, StudioError, StudioErrorCode, StudioFilterContext,
    StudioFilterReference, StudioNavigationReference, StudioRequestEnvelope,
    StudioResponseEnvelope, StudioResult, StudioSelectionContext, StudioSortReference,
    StudioStatusSnapshot, StudioTimeRange, StudioViewKind, StudioViewProjection,
    StudioViewReference, StudioViewRequest, StudioViewResponse,
};
pub use studio_audit::StudioAuditProjection;
pub use studio_command::{StudioCommandRequest, StudioCommandResponse};
pub use studio_digital_twin::StudioDigitalTwinProjection;
pub use studio_event::StudioEventTimelineProjection;
pub use studio_memory::StudioMemoryProjection;
pub use studio_revenue::StudioRevenueReferenceProjection;
pub use studio_runtime::StudioRuntimeProjection;
pub use studio_task::StudioTaskProjection;
pub use studio_top_view::{StudioAttentionState, StudioTopViewProjection};
pub use studio_workflow::StudioWorkflowProjection;

#[cfg(test)]
mod studio_audit_tests;
#[cfg(test)]
mod studio_command_tests;
#[cfg(test)]
mod studio_conformance_tests;
#[cfg(test)]
mod studio_contract_tests;
#[cfg(test)]
mod studio_digital_twin_tests;
#[cfg(test)]
mod studio_event_tests;
#[cfg(test)]
mod studio_memory_tests;
#[cfg(test)]
mod studio_response_tests;
#[cfg(test)]
mod studio_revenue_tests;
#[cfg(test)]
mod studio_runtime_tests;
#[cfg(test)]
mod studio_separation_tests;
#[cfg(test)]
mod studio_task_tests;
#[cfg(test)]
mod studio_test_support;
#[cfg(test)]
mod studio_top_view_tests;
#[cfg(test)]
mod studio_workflow_tests;
