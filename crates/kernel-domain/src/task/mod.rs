mod identity;
mod reference;

pub use identity::{TaskDefinitionId, TaskDependencyId, TaskEvidenceId, TaskInstanceId};
pub use reference::{
    TaskDefinitionReference, TaskDependencyReference, TaskEvidenceReference, TaskInstanceReference,
    TaskStepReference, TaskWorkflowReference,
};

#[cfg(test)]
mod tests;
