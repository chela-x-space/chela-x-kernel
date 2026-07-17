use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use super::{
    TaskDefinitionId, TaskDefinitionReference, TaskDependencyId, TaskDependencyReference,
    TaskEvidenceId, TaskEvidenceReference, TaskInstanceId, TaskInstanceReference,
    TaskStepReference, TaskWorkflowReference,
};
use crate::identifier::WorkflowId;
use crate::workflow::WorkflowStepReference;

#[test]
fn task_engine_foundation_valid_definition_identity_constructs_successfully() {
    let identifier = TaskDefinitionId::new("task.definition.primary").expect("valid task id");

    assert_eq!(identifier.as_str(), "task.definition.primary");
    assert_eq!(identifier.to_string(), "task.definition.primary");
}

#[test]
fn task_engine_foundation_empty_definition_identity_is_rejected() {
    let error = TaskDefinitionId::new("   ").expect_err("blank task-definition identity must fail");

    assert_eq!(error.to_string(), "empty value: TaskDefinitionId");
}

#[test]
fn task_engine_foundation_invalid_definition_identity_is_rejected() {
    let error = TaskDefinitionId::new("task definition")
        .expect_err("unsafe identifier characters must fail");

    assert!(error
        .to_string()
        .contains("ASCII letters, digits, dot, underscore, or hyphen"));
}

#[test]
fn task_engine_foundation_clone_preserves_identity_equality() {
    let identifier = TaskInstanceId::new("task.instance.000001").expect("instance");
    let cloned = identifier.clone();

    assert_eq!(identifier, cloned);
}

#[test]
fn task_engine_foundation_different_identity_values_are_unequal() {
    let left = TaskInstanceId::new("task.instance.000001").expect("left");
    let right = TaskInstanceId::new("task.instance.000002").expect("right");

    assert_ne!(left, right);
}

#[test]
fn task_engine_foundation_ordering_is_deterministic() {
    let lower = TaskDependencyId::new("dependency.alpha").expect("lower");
    let higher = TaskDependencyId::new("dependency.beta").expect("higher");

    assert!(lower < higher);
}

#[test]
fn task_engine_foundation_hash_is_stable_for_equal_values() {
    let left = TaskEvidenceId::new("evidence.primary").expect("left");
    let right = TaskEvidenceId::new("evidence.primary").expect("right");
    let mut left_hasher = DefaultHasher::new();
    let mut right_hasher = DefaultHasher::new();

    left.hash(&mut left_hasher);
    right.hash(&mut right_hasher);

    assert_eq!(left, right);
    assert_eq!(left_hasher.finish(), right_hasher.finish());
}

#[test]
fn task_engine_foundation_debug_output_is_stable_for_equal_values() {
    let left = TaskInstanceId::new("task.instance.debug").expect("left");
    let right = TaskInstanceId::new("task.instance.debug").expect("right");

    assert_eq!(format!("{left:?}"), format!("{right:?}"));
}

#[test]
fn task_engine_foundation_definition_reference_preserves_identity() {
    let identifier = TaskDefinitionId::new("task.definition.ref").expect("definition id");
    let reference = TaskDefinitionReference::new(identifier.clone());

    assert_eq!(reference.task_definition_id(), &identifier);
}

#[test]
fn task_engine_foundation_instance_reference_preserves_identity() {
    let identifier = TaskInstanceId::new("task.instance.ref").expect("instance id");
    let reference = TaskInstanceReference::new(identifier.clone());

    assert_eq!(reference.task_instance_id(), &identifier);
}

#[test]
fn task_engine_foundation_dependency_reference_preserves_identity() {
    let identifier = TaskDependencyId::new("task.dependency.ref").expect("dependency id");
    let reference = TaskDependencyReference::new(identifier.clone());

    assert_eq!(reference.task_dependency_id(), &identifier);
}

#[test]
fn task_engine_foundation_evidence_reference_preserves_identity() {
    let identifier = TaskEvidenceId::new("task.evidence.ref").expect("evidence id");
    let reference = TaskEvidenceReference::new(identifier.clone());

    assert_eq!(reference.task_evidence_id(), &identifier);
}

#[test]
fn task_engine_foundation_workflow_reference_preserves_workflow_identity() {
    let workflow_id = WorkflowId::new("CX-WF-000001").expect("workflow");
    let reference = TaskWorkflowReference::new(workflow_id.clone());

    assert_eq!(reference.workflow_id(), &workflow_id);
}

#[test]
fn task_engine_foundation_step_reference_preserves_step_identity() {
    let step = WorkflowStepReference::new("workflow.step.primary").expect("step");
    let reference = TaskStepReference::new(step.clone());

    assert_eq!(reference.workflow_step_reference(), &step);
}

#[test]
fn task_engine_foundation_same_inputs_produce_equal_values_without_side_effects() {
    let left = TaskDefinitionId::new("task.definition.equal").expect("left");
    let right = TaskDefinitionId::new("task.definition.equal").expect("right");

    assert_eq!(left, right);
    assert_eq!(left.as_str(), right.as_str());
}
