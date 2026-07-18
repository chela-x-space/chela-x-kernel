use crate::errors::DomainResult;
use crate::identifier::NonEmptyText;
use crate::state::TransitionAuthorityReference;

use super::{
    TaskEvidenceSet, TaskFailureCategory, TaskFailureCode, TaskFailurePolicyReference,
    TaskInstanceReference,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskFailureReason(NonEmptyText);

impl TaskFailureReason {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        NonEmptyText::new("task_failure_reason", value).map(Self)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskFailureReference(NonEmptyText);

impl TaskFailureReference {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        NonEmptyText::new("task_failure_reference", value).map(Self)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskRecoveryReference {
    corrective_path: NonEmptyText,
    requires_revalidation: bool,
}

impl TaskRecoveryReference {
    pub fn new(
        corrective_path: impl Into<String>,
        requires_revalidation: bool,
    ) -> DomainResult<Self> {
        Ok(Self {
            corrective_path: NonEmptyText::new("task_recovery_path", corrective_path)?,
            requires_revalidation,
        })
    }

    pub fn corrective_path(&self) -> &str {
        self.corrective_path.as_str()
    }
    pub fn requires_revalidation(&self) -> bool {
        self.requires_revalidation
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskFailure {
    task_instance_reference: TaskInstanceReference,
    task_failure_reference: TaskFailureReference,
    task_failure_code: TaskFailureCode,
    task_failure_category: TaskFailureCategory,
    task_failure_reason: Option<TaskFailureReason>,
    task_failure_evidence_set: TaskEvidenceSet,
    task_failure_authority_reference: Option<TransitionAuthorityReference>,
    task_failure_policy_reference: Option<TaskFailurePolicyReference>,
}

impl TaskFailure {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        task_instance_reference: TaskInstanceReference,
        task_failure_reference: TaskFailureReference,
        task_failure_code: TaskFailureCode,
        task_failure_category: TaskFailureCategory,
        task_failure_reason: Option<TaskFailureReason>,
        task_failure_evidence_set: TaskEvidenceSet,
        task_failure_authority_reference: Option<TransitionAuthorityReference>,
        task_failure_policy_reference: Option<TaskFailurePolicyReference>,
    ) -> Self {
        Self {
            task_instance_reference,
            task_failure_reference,
            task_failure_code,
            task_failure_category,
            task_failure_reason,
            task_failure_evidence_set,
            task_failure_authority_reference,
            task_failure_policy_reference,
        }
    }

    pub fn task_instance_reference(&self) -> &TaskInstanceReference {
        &self.task_instance_reference
    }
    pub fn task_failure_reference(&self) -> &TaskFailureReference {
        &self.task_failure_reference
    }
    pub fn task_failure_code(&self) -> &TaskFailureCode {
        &self.task_failure_code
    }
    pub fn task_failure_category(&self) -> &TaskFailureCategory {
        &self.task_failure_category
    }
    pub fn task_failure_reason(&self) -> Option<&TaskFailureReason> {
        self.task_failure_reason.as_ref()
    }
    pub fn task_failure_evidence_set(&self) -> &TaskEvidenceSet {
        &self.task_failure_evidence_set
    }
    pub fn task_failure_authority_reference(&self) -> Option<&TransitionAuthorityReference> {
        self.task_failure_authority_reference.as_ref()
    }
    pub fn task_failure_policy_reference(&self) -> Option<&TaskFailurePolicyReference> {
        self.task_failure_policy_reference.as_ref()
    }
}
