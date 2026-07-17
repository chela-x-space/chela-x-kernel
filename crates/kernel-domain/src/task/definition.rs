use crate::errors::{DomainError, DomainResult};

use super::definition_validation::reject_duplicates;
use super::{
    TaskCapabilityRequirement, TaskCompletionRequirement, TaskDefinitionId, TaskDefinitionName,
    TaskDefinitionVersion, TaskDescription, TaskEvidenceRequirement, TaskFailurePolicyReference,
    TaskInputContract, TaskKind, TaskOutputContract, TaskRequirement, TaskStepReference,
    TaskWorkflowReference,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskDefinition {
    task_definition_id: TaskDefinitionId,
    task_definition_version: TaskDefinitionVersion,
    task_definition_name: TaskDefinitionName,
    task_description: Option<TaskDescription>,
    task_kind: TaskKind,
    task_input_contracts: Vec<TaskInputContract>,
    task_output_contracts: Vec<TaskOutputContract>,
    task_requirements: Vec<TaskRequirement>,
    task_capability_requirements: Vec<TaskCapabilityRequirement>,
    task_evidence_requirements: Vec<TaskEvidenceRequirement>,
    task_completion_requirements: Vec<TaskCompletionRequirement>,
    task_failure_policy_reference: Option<TaskFailurePolicyReference>,
    task_workflow_reference: Option<TaskWorkflowReference>,
    task_step_reference: Option<TaskStepReference>,
}

impl TaskDefinition {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        task_definition_id: TaskDefinitionId,
        task_definition_version: TaskDefinitionVersion,
        task_definition_name: TaskDefinitionName,
        task_description: Option<TaskDescription>,
        task_kind: TaskKind,
        task_input_contracts: Vec<TaskInputContract>,
        task_output_contracts: Vec<TaskOutputContract>,
        task_requirements: Vec<TaskRequirement>,
        task_capability_requirements: Vec<TaskCapabilityRequirement>,
        task_evidence_requirements: Vec<TaskEvidenceRequirement>,
        task_completion_requirements: Vec<TaskCompletionRequirement>,
        task_failure_policy_reference: Option<TaskFailurePolicyReference>,
        task_workflow_reference: Option<TaskWorkflowReference>,
        task_step_reference: Option<TaskStepReference>,
    ) -> DomainResult<Self> {
        if task_input_contracts.is_empty() {
            return Err(DomainError::InvalidTaskDefinition(
                "task definition requires at least one input contract",
            ));
        }
        if task_completion_requirements.is_empty() {
            return Err(DomainError::InvalidTaskDefinition(
                "task definition requires at least one completion requirement",
            ));
        }
        if task_step_reference.is_some() && task_workflow_reference.is_none() {
            return Err(DomainError::InvalidTaskDefinition(
                "task step binding requires workflow binding",
            ));
        }

        reject_duplicates(&task_input_contracts, "duplicate task input contract")?;
        reject_duplicates(&task_output_contracts, "duplicate task output contract")?;
        reject_duplicates(&task_requirements, "duplicate task requirement")?;
        reject_duplicates(
            &task_capability_requirements,
            "duplicate task capability requirement",
        )?;
        reject_duplicates(
            &task_evidence_requirements,
            "duplicate task evidence requirement",
        )?;
        reject_duplicates(
            &task_completion_requirements,
            "duplicate task completion requirement",
        )?;

        Ok(Self {
            task_definition_id,
            task_definition_version,
            task_definition_name,
            task_description,
            task_kind,
            task_input_contracts,
            task_output_contracts,
            task_requirements,
            task_capability_requirements,
            task_evidence_requirements,
            task_completion_requirements,
            task_failure_policy_reference,
            task_workflow_reference,
            task_step_reference,
        })
    }

    pub fn task_definition_id(&self) -> &TaskDefinitionId {
        &self.task_definition_id
    }
    pub fn task_definition_version(&self) -> &TaskDefinitionVersion {
        &self.task_definition_version
    }
    pub fn task_definition_name(&self) -> &TaskDefinitionName {
        &self.task_definition_name
    }
    pub fn task_description(&self) -> Option<&TaskDescription> {
        self.task_description.as_ref()
    }
    pub fn task_kind(&self) -> &TaskKind {
        &self.task_kind
    }
    pub fn task_input_contracts(&self) -> &[TaskInputContract] {
        &self.task_input_contracts
    }
    pub fn task_output_contracts(&self) -> &[TaskOutputContract] {
        &self.task_output_contracts
    }
    pub fn task_requirements(&self) -> &[TaskRequirement] {
        &self.task_requirements
    }
    pub fn task_capability_requirements(&self) -> &[TaskCapabilityRequirement] {
        &self.task_capability_requirements
    }
    pub fn task_evidence_requirements(&self) -> &[TaskEvidenceRequirement] {
        &self.task_evidence_requirements
    }
    pub fn task_completion_requirements(&self) -> &[TaskCompletionRequirement] {
        &self.task_completion_requirements
    }
    pub fn task_failure_policy_reference(&self) -> Option<&TaskFailurePolicyReference> {
        self.task_failure_policy_reference.as_ref()
    }
    pub fn task_workflow_reference(&self) -> Option<&TaskWorkflowReference> {
        self.task_workflow_reference.as_ref()
    }
    pub fn task_step_reference(&self) -> Option<&TaskStepReference> {
        self.task_step_reference.as_ref()
    }
}
