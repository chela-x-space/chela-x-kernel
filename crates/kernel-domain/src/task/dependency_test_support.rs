use crate::state::StateSequence;

use super::{
    TaskDependency, TaskDependencyCoordinationRequest, TaskDependencyFact,
    TaskDependencyGraphReference, TaskDependencyId, TaskDependencyReference,
    TaskDependencyRequirement, TaskDependencySet, TaskDependencySource, TaskDependencyTarget,
    TaskDependencyType, TaskEvidenceReference, TaskInstanceReference, TaskOutputContract,
    TaskState, TaskStateSnapshot,
};

pub(super) fn task_instance_reference(value: &str) -> TaskInstanceReference {
    TaskInstanceReference::new(super::TaskInstanceId::new(value).expect("instance"))
}

pub(super) fn task_dependency(
    dependency_id: &str,
    source: &str,
    target: &str,
    task_dependency_type: TaskDependencyType,
    task_dependency_requirement: TaskDependencyRequirement,
) -> TaskDependency {
    TaskDependency::new(
        TaskDependencyReference::new(TaskDependencyId::new(dependency_id).expect("dependency id")),
        TaskDependencySource::new(task_instance_reference(source)),
        TaskDependencyTarget::new(task_instance_reference(target)),
        task_dependency_type,
        task_dependency_requirement,
    )
    .expect("dependency")
}

pub(super) fn task_dependency_set(task_dependencies: Vec<TaskDependency>) -> TaskDependencySet {
    TaskDependencySet::new(
        TaskDependencyGraphReference::new("task.dependency.graph").expect("graph"),
        task_dependencies,
    )
}

pub(super) fn task_dependency_fact(
    task_instance: &str,
    task_state: TaskState,
    task_evidence_references: Vec<TaskEvidenceReference>,
    task_output_contracts: Vec<TaskOutputContract>,
) -> TaskDependencyFact {
    TaskDependencyFact::new(
        TaskStateSnapshot::new(
            task_instance_reference(task_instance),
            task_state,
            StateSequence::new(1).expect("sequence"),
        ),
        task_evidence_references,
        task_output_contracts,
    )
    .expect("fact")
}

pub(super) fn coordination_request(
    task_dependencies: Vec<TaskDependency>,
    task_dependency_facts: Vec<TaskDependencyFact>,
) -> TaskDependencyCoordinationRequest {
    TaskDependencyCoordinationRequest::new(
        task_dependency_set(task_dependencies),
        task_dependency_facts,
    )
    .expect("request")
}
