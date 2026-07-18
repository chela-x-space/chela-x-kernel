use crate::errors::DomainResult;
use crate::identifier::EnglishNamespace;
use crate::state::{TransitionAuthorityReference, TransitionEvidenceReference};

use super::{TaskEvidenceReference, TaskEvidenceRequirement, TaskInstanceReference};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskEvidenceType(EnglishNamespace);

impl TaskEvidenceType {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        EnglishNamespace::new("TaskEvidenceType", value).map(Self)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskEvidenceMetadata {
    task_evidence_requirement: Option<TaskEvidenceRequirement>,
    transition_evidence_reference: Option<TransitionEvidenceReference>,
}

impl TaskEvidenceMetadata {
    pub fn new(
        task_evidence_requirement: Option<TaskEvidenceRequirement>,
        transition_evidence_reference: Option<TransitionEvidenceReference>,
    ) -> Self {
        Self {
            task_evidence_requirement,
            transition_evidence_reference,
        }
    }

    pub fn task_evidence_requirement(&self) -> Option<&TaskEvidenceRequirement> {
        self.task_evidence_requirement.as_ref()
    }
    pub fn transition_evidence_reference(&self) -> Option<&TransitionEvidenceReference> {
        self.transition_evidence_reference.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskEvidence {
    task_evidence_reference: TaskEvidenceReference,
    subject_task_instance_reference: TaskInstanceReference,
    task_evidence_type: TaskEvidenceType,
    producer_authority_reference: Option<TransitionAuthorityReference>,
    task_evidence_metadata: TaskEvidenceMetadata,
}

impl TaskEvidence {
    pub fn new(
        task_evidence_reference: TaskEvidenceReference,
        subject_task_instance_reference: TaskInstanceReference,
        task_evidence_type: TaskEvidenceType,
        producer_authority_reference: Option<TransitionAuthorityReference>,
        task_evidence_metadata: TaskEvidenceMetadata,
    ) -> Self {
        Self {
            task_evidence_reference,
            subject_task_instance_reference,
            task_evidence_type,
            producer_authority_reference,
            task_evidence_metadata,
        }
    }

    pub fn task_evidence_reference(&self) -> &TaskEvidenceReference {
        &self.task_evidence_reference
    }
    pub fn subject_task_instance_reference(&self) -> &TaskInstanceReference {
        &self.subject_task_instance_reference
    }
    pub fn task_evidence_type(&self) -> &TaskEvidenceType {
        &self.task_evidence_type
    }
    pub fn producer_authority_reference(&self) -> Option<&TransitionAuthorityReference> {
        self.producer_authority_reference.as_ref()
    }
    pub fn task_evidence_metadata(&self) -> &TaskEvidenceMetadata {
        &self.task_evidence_metadata
    }
}
