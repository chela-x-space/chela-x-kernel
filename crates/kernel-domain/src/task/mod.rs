mod assignment;
mod assignment_control;
mod assignment_decision;
mod assignment_validation;
mod definition;
mod definition_validation;
mod definition_value;
mod identity;
mod instance;
mod instance_binding;
mod instance_validation;
mod instance_value;
mod ownership;
mod priority;
mod readiness;
mod readiness_decision;
mod readiness_input;
mod readiness_validation;
mod reference;
mod subject;

pub use assignment::{
    TaskAssignment, TaskAssignmentAuthority, TaskAssignmentReason, TaskAssignmentRejectionReason,
    TaskAssignmentStatus,
};
pub use assignment_control::TaskAssignmentControl;
pub use assignment_decision::{
    TaskAssignmentChange, TaskAssignmentDecision, TaskAssignmentNoOp, TaskAssignmentRejection,
    TaskAssignmentRequest,
};
pub use definition::TaskDefinition;
pub use definition_value::{
    TaskCapabilityRequirement, TaskCompletionRequirement, TaskDefinitionName,
    TaskDefinitionVersion, TaskDescription, TaskEvidenceRequirement, TaskFailurePolicyReference,
    TaskInputContract, TaskKind, TaskOutputContract, TaskRequirement,
};
pub use identity::{TaskDefinitionId, TaskDependencyId, TaskEvidenceId, TaskInstanceId};
pub use instance::TaskInstance;
pub use instance_binding::{TaskCreationContext, TaskWorkflowBinding};
pub use instance_value::{
    TaskDefinitionSnapshotReference, TaskInputBinding, TaskOutputBinding, TaskState,
    TaskStepBinding,
};
pub use ownership::{TaskOwnership, TaskOwnershipAuthority, TaskOwnershipScope};
pub use priority::{TaskPriority, TaskPriorityClass, TaskPriorityValue};
pub use readiness::{
    TaskReadiness, TaskReadinessBlocker, TaskReadinessEvidence, TaskReadinessRejectionReason,
    TaskReadinessRequirement,
};
pub use readiness_decision::{
    TaskReadinessBlocked, TaskReadinessDecision, TaskReadinessReady, TaskReadinessRejection,
};
pub use readiness_input::TaskReadinessInput;
pub use readiness_validation::TaskReadinessControl;
pub use reference::{
    TaskDefinitionReference, TaskDependencyReference, TaskEvidenceReference, TaskInstanceReference,
    TaskStepReference, TaskWorkflowReference,
};
pub use subject::{TaskAssignee, TaskOwner};

#[cfg(test)]
mod assignment_control_tests;
#[cfg(test)]
mod assignment_eligibility_tests;
#[cfg(test)]
mod assignment_tests;
#[cfg(test)]
mod definition_tests;
#[cfg(test)]
mod instance_tests;
#[cfg(test)]
mod ownership_tests;
#[cfg(test)]
mod priority_tests;
#[cfg(test)]
mod readiness_separation_tests;
#[cfg(test)]
mod readiness_test_support;
#[cfg(test)]
mod readiness_tests;
#[cfg(test)]
mod tests;
