mod definition;
mod definition_validation;
mod definition_value;
mod identity;
mod reference;

pub use definition::TaskDefinition;
pub use definition_value::{
    TaskCapabilityRequirement, TaskCompletionRequirement, TaskDefinitionName,
    TaskDefinitionVersion, TaskDescription, TaskEvidenceRequirement, TaskFailurePolicyReference,
    TaskInputContract, TaskKind, TaskOutputContract, TaskRequirement,
};
pub use identity::{TaskDefinitionId, TaskDependencyId, TaskEvidenceId, TaskInstanceId};
pub use reference::{
    TaskDefinitionReference, TaskDependencyReference, TaskEvidenceReference, TaskInstanceReference,
    TaskStepReference, TaskWorkflowReference,
};

#[cfg(test)]
mod definition_tests;
#[cfg(test)]
mod tests;
