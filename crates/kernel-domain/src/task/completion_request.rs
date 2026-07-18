use super::{TaskCompletionResult, TaskInstance, TaskRecoveryReference, TaskStateSnapshot};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskCompletionValidationRequest {
    task_instance: TaskInstance,
    task_state_snapshot: TaskStateSnapshot,
    task_completion_result: TaskCompletionResult,
    task_recovery_reference: Option<TaskRecoveryReference>,
}

impl TaskCompletionValidationRequest {
    pub fn new(
        task_instance: TaskInstance,
        task_state_snapshot: TaskStateSnapshot,
        task_completion_result: TaskCompletionResult,
        task_recovery_reference: Option<TaskRecoveryReference>,
    ) -> Self {
        Self {
            task_instance,
            task_state_snapshot,
            task_completion_result,
            task_recovery_reference,
        }
    }

    pub fn task_instance(&self) -> &TaskInstance {
        &self.task_instance
    }
    pub fn task_state_snapshot(&self) -> &TaskStateSnapshot {
        &self.task_state_snapshot
    }
    pub fn task_completion_result(&self) -> &TaskCompletionResult {
        &self.task_completion_result
    }
    pub fn task_recovery_reference(&self) -> Option<&TaskRecoveryReference> {
        self.task_recovery_reference.as_ref()
    }
}
