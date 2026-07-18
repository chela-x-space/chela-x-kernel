use crate::errors::{DomainError, DomainResult};
use crate::identifier::NonEmptyText;
use crate::state::{TransitionAuthorityReference, TransitionReasonReference};

use super::{
    TaskCompletionRequirement, TaskDefinitionSnapshotReference, TaskEvidenceSet,
    TaskInstanceReference, TaskOutputBinding,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskOutputReference(NonEmptyText);

impl TaskOutputReference {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        NonEmptyText::new("task_output_reference", value).map(Self)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskOutput {
    task_output_reference: TaskOutputReference,
    task_output_binding: TaskOutputBinding,
}

impl TaskOutput {
    pub fn new(
        task_output_reference: TaskOutputReference,
        task_output_binding: TaskOutputBinding,
    ) -> Self {
        Self {
            task_output_reference,
            task_output_binding,
        }
    }

    pub fn task_output_reference(&self) -> &TaskOutputReference {
        &self.task_output_reference
    }
    pub fn task_output_binding(&self) -> &TaskOutputBinding {
        &self.task_output_binding
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskCompletionResult {
    task_instance_reference: TaskInstanceReference,
    task_definition_snapshot_reference: TaskDefinitionSnapshotReference,
    task_completion_requirements: Vec<TaskCompletionRequirement>,
    task_outputs: Vec<TaskOutput>,
    task_evidence_set: TaskEvidenceSet,
    completion_authority_reference: Option<TransitionAuthorityReference>,
    completion_reason_reference: Option<TransitionReasonReference>,
}

impl TaskCompletionResult {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        task_instance_reference: TaskInstanceReference,
        task_definition_snapshot_reference: TaskDefinitionSnapshotReference,
        task_completion_requirements: Vec<TaskCompletionRequirement>,
        task_outputs: Vec<TaskOutput>,
        task_evidence_set: TaskEvidenceSet,
        completion_authority_reference: Option<TransitionAuthorityReference>,
        completion_reason_reference: Option<TransitionReasonReference>,
    ) -> DomainResult<Self> {
        reject_duplicates(
            &task_completion_requirements,
            "duplicate task completion requirement",
        )?;
        reject_duplicates_by(
            task_outputs.iter().map(TaskOutput::task_output_reference),
            "duplicate task output reference",
        )?;

        Ok(Self {
            task_instance_reference,
            task_definition_snapshot_reference,
            task_completion_requirements,
            task_outputs,
            task_evidence_set,
            completion_authority_reference,
            completion_reason_reference,
        })
    }

    pub fn task_instance_reference(&self) -> &TaskInstanceReference {
        &self.task_instance_reference
    }
    pub fn task_definition_snapshot_reference(&self) -> &TaskDefinitionSnapshotReference {
        &self.task_definition_snapshot_reference
    }
    pub fn task_completion_requirements(&self) -> &[TaskCompletionRequirement] {
        &self.task_completion_requirements
    }
    pub fn task_outputs(&self) -> &[TaskOutput] {
        &self.task_outputs
    }
    pub fn task_evidence_set(&self) -> &TaskEvidenceSet {
        &self.task_evidence_set
    }
    pub fn completion_authority_reference(&self) -> Option<&TransitionAuthorityReference> {
        self.completion_authority_reference.as_ref()
    }
    pub fn completion_reason_reference(&self) -> Option<&TransitionReasonReference> {
        self.completion_reason_reference.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskCompletion {
    task_completion_result: TaskCompletionResult,
}

impl TaskCompletion {
    pub(crate) fn new(task_completion_result: TaskCompletionResult) -> Self {
        Self {
            task_completion_result,
        }
    }

    pub fn task_completion_result(&self) -> &TaskCompletionResult {
        &self.task_completion_result
    }
}

fn reject_duplicates<T: PartialEq>(values: &[T], message: &'static str) -> DomainResult<()> {
    reject_duplicates_by(values.iter(), message)
}

fn reject_duplicates_by<'a, T: PartialEq + 'a>(
    values: impl IntoIterator<Item = &'a T>,
    message: &'static str,
) -> DomainResult<()> {
    let values = values.into_iter().collect::<Vec<_>>();
    if values
        .iter()
        .enumerate()
        .any(|(index, value)| values[..index].iter().any(|prior| prior == value))
    {
        return Err(DomainError::InvalidTaskCompletion(message));
    }
    Ok(())
}
