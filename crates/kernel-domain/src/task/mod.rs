mod assignment;
mod assignment_control;
mod assignment_decision;
mod assignment_validation;
mod completion;
mod completion_request;
mod completion_rules;
mod completion_validation;
mod definition;
mod definition_validation;
mod definition_value;
mod dependency;
mod dependency_coordination;
mod dependency_cycle;
mod dependency_decision;
mod dependency_evaluation;
mod dependency_fact;
mod dependency_set;
mod dependency_validation;
mod evidence;
mod evidence_set;
mod evidence_validation;
mod failure;
mod failure_validation;
mod identity;
mod instance;
mod instance_binding;
mod instance_validation;
mod instance_value;
mod lifecycle;
mod lifecycle_decision;
mod lifecycle_guard;
mod lifecycle_request;
mod lifecycle_transition;
mod lifecycle_validation;
mod outcome_decision;
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
pub use completion::{TaskCompletion, TaskCompletionResult, TaskOutput, TaskOutputReference};
pub use completion_request::TaskCompletionValidationRequest;
pub use completion_validation::{
    TaskCompletionControl, TaskCompletionOutcome, TaskCompletionRejected,
    TaskCompletionRejectionReason,
};
pub use definition::TaskDefinition;
pub use definition_value::{
    TaskCapabilityRequirement, TaskCompletionRequirement, TaskDefinitionName,
    TaskDefinitionVersion, TaskDescription, TaskEvidenceRequirement, TaskFailurePolicyReference,
    TaskInputContract, TaskKind, TaskOutputContract, TaskRequirement,
};
pub use dependency::{
    TaskDependency, TaskDependencyGraphReference, TaskDependencyRequirement, TaskDependencySource,
    TaskDependencyStatus, TaskDependencyTarget, TaskDependencyType,
};
pub use dependency_coordination::{TaskDependencyCoordinationDecision, TaskDependencyDecision};
pub use dependency_decision::{
    TaskDependencyBlocker, TaskDependencyRejectionReason, TaskDependencyUnresolvedReason,
    TaskDependencyValidation, TaskDependencyValidationAccepted, TaskDependencyValidationNoOp,
    TaskDependencyValidationRejected,
};
pub use dependency_fact::TaskDependencyFact;
pub use dependency_set::{
    TaskDependencyCoordinationRequest, TaskDependencySet, TaskDependencyValidationRequest,
};
pub use dependency_validation::TaskDependencyControl;
pub use evidence::{TaskEvidence, TaskEvidenceMetadata, TaskEvidenceType};
pub use evidence_set::TaskEvidenceSet;
pub use evidence_validation::{
    TaskEvidenceControl, TaskEvidenceRejected, TaskEvidenceRejectionReason, TaskEvidenceValidation,
    TaskEvidenceValidationRequest,
};
pub use failure::{TaskFailure, TaskFailureReason, TaskFailureReference, TaskRecoveryReference};
pub use failure_validation::{
    TaskFailureControl, TaskFailureOutcome, TaskFailureRejected, TaskFailureRejectionReason,
    TaskFailureValidationRequest,
};
pub use identity::{TaskDefinitionId, TaskDependencyId, TaskEvidenceId, TaskInstanceId};
pub use instance::TaskInstance;
pub use instance_binding::{TaskCreationContext, TaskWorkflowBinding};
pub use instance_value::{
    TaskDefinitionSnapshotReference, TaskInputBinding, TaskOutputBinding, TaskState,
    TaskStepBinding,
};
pub use lifecycle::{TaskFailureCategory, TaskFailureCode, TaskStateSnapshot};
pub use lifecycle_decision::{
    TaskAllowedTransition, TaskNoOpTransition, TaskRejectedTransition, TaskTransitionDecision,
    TaskTransitionRejectionReason,
};
pub use lifecycle_guard::TaskLifecycleGuards;
pub use lifecycle_request::TaskTransitionRequest;
pub use lifecycle_validation::TaskTransitionControl;
pub use outcome_decision::{TaskOutcomeDecision, TaskOutcomeRejectionReason};
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
mod completion_tests;
#[cfg(test)]
mod definition_tests;
#[cfg(test)]
mod dependency_construction_tests;
#[cfg(test)]
mod dependency_coordination_tests;
#[cfg(test)]
mod dependency_cycle_tests;
#[cfg(test)]
mod dependency_satisfaction_tests;
#[cfg(test)]
mod dependency_separation_tests;
#[cfg(test)]
mod dependency_test_support;
#[cfg(test)]
mod evidence_tests;
#[cfg(test)]
mod instance_tests;
#[cfg(test)]
mod lifecycle_allowed_tests;
#[cfg(test)]
mod lifecycle_noop_tests;
#[cfg(test)]
mod lifecycle_rejected_tests;
#[cfg(test)]
mod lifecycle_separation_tests;
#[cfg(test)]
mod lifecycle_test_support;
#[cfg(test)]
mod outcome_separation_tests;
#[cfg(test)]
mod outcome_test_support;
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
