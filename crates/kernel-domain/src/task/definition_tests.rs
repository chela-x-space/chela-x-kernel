use super::{
    TaskCapabilityRequirement, TaskCompletionRequirement, TaskDefinition, TaskDefinitionId,
    TaskDefinitionName, TaskDefinitionVersion, TaskDescription, TaskEvidenceRequirement,
    TaskInputContract, TaskKind, TaskOutputContract, TaskRequirement, TaskStepReference,
    TaskWorkflowReference,
};
use crate::{PolicyId, WorkflowId, WorkflowStepReference};

fn task_definition() -> TaskDefinition {
    TaskDefinition::new(
        TaskDefinitionId::new("task.definition.review").expect("id"),
        TaskDefinitionVersion::new("1.0.0").expect("version"),
        TaskDefinitionName::new("Review Submission").expect("name"),
        Some(TaskDescription::new("Review submitted content").expect("description")),
        TaskKind::new("review").expect("kind"),
        vec![
            TaskInputContract::new("submission").expect("input"),
            TaskInputContract::new("policy.reference").expect("input"),
        ],
        vec![TaskOutputContract::new("review.result").expect("output")],
        vec![TaskRequirement::new("qa.required").expect("requirement")],
        vec![TaskCapabilityRequirement::new("human.reviewer").expect("capability")],
        vec![TaskEvidenceRequirement::new("decision.record").expect("evidence")],
        vec![TaskCompletionRequirement::new("approval.recorded").expect("completion")],
        Some(PolicyId::new("CX-POL-700001").expect("policy")),
        Some(TaskWorkflowReference::new(
            WorkflowId::new("CX-WF-700001").expect("workflow"),
        )),
        Some(TaskStepReference::new(
            WorkflowStepReference::new("workflow.step.review").expect("step"),
        )),
    )
    .expect("valid definition")
}

#[test]
fn task_definition_valid_minimal_definition_constructs_successfully() {
    let definition = TaskDefinition::new(
        TaskDefinitionId::new("task.definition.minimal").expect("id"),
        TaskDefinitionVersion::new("1.0.0").expect("version"),
        TaskDefinitionName::new("Minimal Definition").expect("name"),
        None,
        TaskKind::new("review").expect("kind"),
        vec![TaskInputContract::new("submission").expect("input")],
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        vec![TaskCompletionRequirement::new("approval.recorded").expect("completion")],
        None,
        None,
        None,
    )
    .expect("minimal definition");

    assert_eq!(
        definition.task_definition_name().as_str(),
        "Minimal Definition"
    );
    assert!(definition.task_workflow_reference().is_none());
}

#[test]
fn task_definition_valid_complete_definition_preserves_fields() {
    let definition = task_definition();

    assert_eq!(
        definition.task_definition_id().as_str(),
        "task.definition.review"
    );
    assert_eq!(definition.task_definition_version().as_str(), "1.0.0");
    assert_eq!(definition.task_kind().as_str(), "review");
    assert_eq!(definition.task_input_contracts().len(), 2);
    assert_eq!(
        definition.task_output_contracts()[0].as_str(),
        "review.result"
    );
    assert_eq!(
        definition
            .task_failure_policy_reference()
            .expect("policy")
            .as_str(),
        "CX-POL-700001"
    );
}

#[test]
fn task_definition_empty_required_name_is_rejected() {
    let error = TaskDefinitionName::new(" ").expect_err("name must fail");
    assert_eq!(error.to_string(), "empty value: TaskDefinitionName");
}

#[test]
fn task_definition_invalid_identifier_reference_is_rejected() {
    let error = TaskInputContract::new("input contract").expect_err("invalid contract");
    assert!(error
        .to_string()
        .contains("ASCII letters, digits, dot, underscore, or hyphen"));
}

#[test]
fn task_definition_identity_and_version_are_preserved() {
    let definition = task_definition();
    assert_eq!(
        definition.task_definition_id().as_str(),
        "task.definition.review"
    );
    assert_eq!(definition.task_definition_version().as_str(), "1.0.0");
}

#[test]
fn task_definition_immutable_access_preserves_caller_order() {
    let definition = task_definition();
    assert_eq!(
        definition.task_input_contracts(),
        &[
            TaskInputContract::new("submission").expect("first"),
            TaskInputContract::new("policy.reference").expect("second"),
        ]
    );
}

#[test]
fn task_definition_same_input_produces_equal_definition() {
    let left = task_definition();
    let right = task_definition();
    assert_eq!(left, right);
}

#[test]
fn task_definition_different_input_produces_unequal_definition() {
    let left = task_definition();
    let right = TaskDefinition::new(
        TaskDefinitionId::new("task.definition.review.2").expect("id"),
        TaskDefinitionVersion::new("1.0.0").expect("version"),
        TaskDefinitionName::new("Review Submission").expect("name"),
        None,
        TaskKind::new("review").expect("kind"),
        vec![TaskInputContract::new("submission").expect("input")],
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        vec![TaskCompletionRequirement::new("approval.recorded").expect("completion")],
        None,
        None,
        None,
    )
    .expect("valid");

    assert_ne!(left, right);
}

#[test]
fn task_definition_empty_completion_contract_is_rejected() {
    let error = TaskDefinition::new(
        TaskDefinitionId::new("task.definition.invalid").expect("id"),
        TaskDefinitionVersion::new("1.0.0").expect("version"),
        TaskDefinitionName::new("Invalid").expect("name"),
        None,
        TaskKind::new("review").expect("kind"),
        vec![TaskInputContract::new("submission").expect("input")],
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        None,
        None,
        None,
    )
    .expect_err("missing completion requirements must fail");

    assert_eq!(
        error.to_string(),
        "invalid task definition: task definition requires at least one completion requirement"
    );
}

#[test]
fn task_definition_duplicate_requirements_are_rejected() {
    let duplicate = TaskRequirement::new("qa.required").expect("requirement");
    let error = TaskDefinition::new(
        TaskDefinitionId::new("task.definition.invalid").expect("id"),
        TaskDefinitionVersion::new("1.0.0").expect("version"),
        TaskDefinitionName::new("Invalid").expect("name"),
        None,
        TaskKind::new("review").expect("kind"),
        vec![TaskInputContract::new("submission").expect("input")],
        Vec::new(),
        vec![duplicate.clone(), duplicate],
        Vec::new(),
        Vec::new(),
        vec![TaskCompletionRequirement::new("approval.recorded").expect("completion")],
        None,
        None,
        None,
    )
    .expect_err("duplicate requirement must fail");

    assert_eq!(
        error.to_string(),
        "invalid task definition: duplicate task requirement"
    );
}

#[test]
fn task_definition_step_binding_without_workflow_binding_is_rejected() {
    let error = TaskDefinition::new(
        TaskDefinitionId::new("task.definition.invalid").expect("id"),
        TaskDefinitionVersion::new("1.0.0").expect("version"),
        TaskDefinitionName::new("Invalid").expect("name"),
        None,
        TaskKind::new("review").expect("kind"),
        vec![TaskInputContract::new("submission").expect("input")],
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        vec![TaskCompletionRequirement::new("approval.recorded").expect("completion")],
        None,
        None,
        Some(TaskStepReference::new(
            WorkflowStepReference::new("workflow.step.review").expect("step"),
        )),
    )
    .expect_err("step binding without workflow binding must fail");

    assert_eq!(
        error.to_string(),
        "invalid task definition: task step binding requires workflow binding"
    );
}

#[test]
fn task_definition_no_hidden_clock_or_random_generation_occurs() {
    let left = task_definition();
    let right = task_definition();

    assert_eq!(left.task_definition_id(), right.task_definition_id());
    assert_eq!(
        left.task_definition_version(),
        right.task_definition_version()
    );
}
