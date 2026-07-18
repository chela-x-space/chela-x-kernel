use crate::state::StateSequence;

use super::{TaskFailureCategory, TaskFailureCode};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TaskLifecycleGuards {
    expected_current_sequence: Option<StateSequence>,
    assignment_required: bool,
    authorization_allowed: bool,
    dependencies_satisfied: bool,
    completion_conditions_met: bool,
    required_outputs_present: bool,
    required_completion_evidence_present: bool,
    required_failure_evidence_present: bool,
    failure_code: Option<TaskFailureCode>,
    failure_category: Option<TaskFailureCategory>,
}

impl TaskLifecycleGuards {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        expected_current_sequence: Option<StateSequence>,
        assignment_required: bool,
        authorization_allowed: bool,
        dependencies_satisfied: bool,
        completion_conditions_met: bool,
        required_outputs_present: bool,
        required_completion_evidence_present: bool,
        required_failure_evidence_present: bool,
        failure_code: Option<TaskFailureCode>,
        failure_category: Option<TaskFailureCategory>,
    ) -> Self {
        Self {
            expected_current_sequence,
            assignment_required,
            authorization_allowed,
            dependencies_satisfied,
            completion_conditions_met,
            required_outputs_present,
            required_completion_evidence_present,
            required_failure_evidence_present,
            failure_code,
            failure_category,
        }
    }

    pub fn expected_current_sequence(&self) -> Option<StateSequence> {
        self.expected_current_sequence
    }
    pub fn assignment_required(&self) -> bool {
        self.assignment_required
    }
    pub fn authorization_allowed(&self) -> bool {
        self.authorization_allowed
    }
    pub fn dependencies_satisfied(&self) -> bool {
        self.dependencies_satisfied
    }
    pub fn completion_conditions_met(&self) -> bool {
        self.completion_conditions_met
    }
    pub fn required_outputs_present(&self) -> bool {
        self.required_outputs_present
    }
    pub fn required_completion_evidence_present(&self) -> bool {
        self.required_completion_evidence_present
    }
    pub fn required_failure_evidence_present(&self) -> bool {
        self.required_failure_evidence_present
    }
    pub fn failure_code(&self) -> Option<&TaskFailureCode> {
        self.failure_code.as_ref()
    }
    pub fn failure_category(&self) -> Option<&TaskFailureCategory> {
        self.failure_category.as_ref()
    }
}
