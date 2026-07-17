use crate::state::TransitionAuthorityReference;
use crate::{
    AuditEvidenceId, AuthorizationDecisionId, DecisionId, DelegationId, EnglishNamespace,
    EnterpriseId, OwnershipPath, PolicyId, StableVersion, TaskCapabilityRequirement,
    TaskCompletionRequirement, TaskCreationContext, TaskDefinition, TaskDefinitionId,
    TaskDefinitionName, TaskDefinitionVersion, TaskDescription, TaskFailurePolicyReference,
    TaskInputBinding, TaskInputContract, TaskInstance, TaskInstanceId, TaskKind, TaskOutputBinding,
    TaskOutputContract, TaskRequirement, TaskState, TaskStepBinding, TaskStepReference,
    TaskWorkflowBinding, TaskWorkflowReference, WorkflowAuditEvidenceReference, WorkflowId,
    WorkflowLifecycleMapReference, WorkflowRecoveryReference, WorkflowRetryLimit,
    WorkflowRetryPolicyReference, WorkflowState, WorkflowStateSnapshot, WorkflowStepReference,
    WorkflowTerminalOutcomeReference,
};

use crate::workflow::WorkflowInstance;
use crate::WorkflowDefinition as KernelWorkflowDefinition;

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
        Vec::new(),
        vec![TaskCompletionRequirement::new("approval.recorded").expect("completion")],
        Some(TaskFailurePolicyReference::new("CX-POL-700001").expect("policy")),
        Some(TaskWorkflowReference::new(
            WorkflowId::new("CX-WF-700001").expect("workflow"),
        )),
        Some(TaskStepReference::new(
            WorkflowStepReference::new("workflow.step.review").expect("step"),
        )),
    )
    .expect("definition")
}

fn task_creation_context() -> TaskCreationContext {
    TaskCreationContext::new(
        vec![
            TaskInputBinding::new(TaskInputContract::new("submission").expect("input")),
            TaskInputBinding::new(TaskInputContract::new("policy.reference").expect("input")),
        ],
        Some(TransitionAuthorityReference::new("authority.creator").expect("authority")),
    )
    .expect("context")
}

fn workflow_definition() -> KernelWorkflowDefinition {
    let workflow_id = WorkflowId::new("CX-WF-700001").expect("workflow");
    let definition_version =
        StableVersion::new("WorkflowDefinitionVersion", "1.0.0").expect("version");

    KernelWorkflowDefinition::new(
        workflow_id.clone(),
        EnglishNamespace::new("WorkflowDefinitionNamespace", "workflow.review").expect("namespace"),
        definition_version.clone(),
        OwnershipPath::new(
            EnterpriseId::new("CX-ENT-700001").expect("enterprise"),
            None,
            None,
            None,
        )
        .expect("ownership"),
        WorkflowLifecycleMapReference::new("workflow.lifecycle.review").expect("lifecycle"),
        vec![WorkflowStepReference::new("workflow.step.review").expect("step")],
        vec![WorkflowTerminalOutcomeReference::new("completed").expect("terminal")],
        vec![PolicyId::new("CX-POL-700001").expect("policy")],
        Some(WorkflowRetryPolicyReference::new(
            StableVersion::new("RetryPolicyVersion", "1.0.0").expect("policy version"),
            WorkflowRetryLimit::new(2).expect("limit"),
        )),
        Some(WorkflowRetryLimit::new(2).expect("limit")),
        Some(WorkflowRecoveryReference::new("manual.review", true).expect("recovery")),
        vec![WorkflowAuditEvidenceReference::new(
            AuditEvidenceId::new("CX-AUD-700001").expect("evidence"),
            workflow_id,
            definition_version,
            vec![PolicyId::new("CX-POL-700001").expect("policy")],
            vec![AuthorizationDecisionId::new("CX-AUTHZ-700001").expect("authorization decision")],
            vec![DelegationId::new("CX-DEL-700001").expect("delegation")],
            vec![DecisionId::new("CX-DEC-700001").expect("decision")],
        )
        .expect("audit")],
    )
    .expect("workflow definition")
}

fn workflow_instance() -> WorkflowInstance {
    let workflow_id = WorkflowId::new("CX-WF-700001").expect("workflow");
    let definition_version =
        StableVersion::new("WorkflowDefinitionVersionSnapshot", "1.0.0").expect("version");
    let ownership = OwnershipPath::new(
        EnterpriseId::new("CX-ENT-700001").expect("enterprise"),
        None,
        None,
        None,
    )
    .expect("ownership");

    WorkflowInstance::new(
        workflow_id.clone(),
        workflow_definition(),
        definition_version.clone(),
        ownership.clone(),
        WorkflowStateSnapshot::new(
            workflow_id.clone(),
            ownership,
            definition_version.clone(),
            WorkflowState::Validated,
            crate::StateSequence::new(1).expect("seq"),
        ),
        WorkflowAuditEvidenceReference::new(
            AuditEvidenceId::new("CX-AUD-700002").expect("evidence"),
            workflow_id,
            definition_version,
            vec![PolicyId::new("CX-POL-700001").expect("policy")],
            vec![AuthorizationDecisionId::new("CX-AUTHZ-700002").expect("authorization decision")],
            vec![DelegationId::new("CX-DEL-700002").expect("delegation")],
            vec![DecisionId::new("CX-DEC-700002").expect("decision")],
        )
        .expect("creation evidence"),
        None,
        None,
        None,
        vec![],
    )
    .expect("workflow instance")
}

fn minimal_task_instance() -> TaskInstance {
    TaskInstance::new(
        TaskInstanceId::new("task.instance.000001").expect("id"),
        task_definition(),
        task_creation_context(),
        Vec::new(),
        None,
        None,
        TaskState::Pending,
    )
    .expect("instance")
}

#[test]
fn task_instance_valid_minimal_task_instance_constructs_successfully() {
    let instance = minimal_task_instance();
    assert_eq!(instance.task_state(), TaskState::Pending);
}

#[test]
fn task_instance_valid_complete_task_instance_constructs_successfully() {
    let instance = TaskInstance::new(
        TaskInstanceId::new("task.instance.000002").expect("id"),
        task_definition(),
        task_creation_context(),
        vec![TaskOutputBinding::new(
            TaskOutputContract::new("review.result").expect("output"),
        )],
        Some(TaskWorkflowBinding::from_workflow_instance(
            workflow_instance(),
        )),
        Some(TaskStepBinding::new(TaskStepReference::new(
            WorkflowStepReference::new("workflow.step.review").expect("step"),
        ))),
        TaskState::Pending,
    )
    .expect("instance");

    assert_eq!(instance.task_output_bindings().len(), 1);
    assert!(instance.task_workflow_binding().is_some());
}

#[test]
fn task_instance_identity_and_definition_binding_are_preserved() {
    let instance = minimal_task_instance();
    assert_eq!(instance.task_instance_id().as_str(), "task.instance.000001");
    assert_eq!(
        instance
            .task_definition_snapshot_reference()
            .task_definition_reference()
            .task_definition_id()
            .as_str(),
        "task.definition.review"
    );
}

#[test]
fn task_instance_definition_version_binding_is_preserved() {
    let instance = minimal_task_instance();
    assert_eq!(
        instance
            .task_definition_snapshot_reference()
            .task_definition_version()
            .as_str(),
        "1.0.0"
    );
}

#[test]
fn task_instance_initial_state_is_pending() {
    let instance = minimal_task_instance();
    assert_eq!(instance.task_state(), TaskState::Pending);
}

#[test]
fn task_instance_same_explicit_input_produces_equal_instance() {
    let left = minimal_task_instance();
    let right = minimal_task_instance();
    assert_eq!(left, right);
}

#[test]
fn task_instance_different_instance_id_produces_unequal_instance() {
    let left = minimal_task_instance();
    let right = TaskInstance::new(
        TaskInstanceId::new("task.instance.000003").expect("id"),
        task_definition(),
        task_creation_context(),
        Vec::new(),
        None,
        None,
        TaskState::Pending,
    )
    .expect("instance");

    assert_ne!(left, right);
}

#[test]
fn task_instance_invalid_required_value_is_rejected() {
    let error = TaskCreationContext::new(Vec::new(), None).expect("context");
    let error = TaskInstance::new(
        TaskInstanceId::new("task.instance.000004").expect("id"),
        task_definition(),
        error,
        Vec::new(),
        None,
        None,
        TaskState::Pending,
    )
    .expect_err("missing required inputs must fail");

    assert_eq!(
        error.to_string(),
        "invalid task instance: missing required task input binding"
    );
}

#[test]
fn task_instance_duplicate_input_binding_is_rejected() {
    let error = TaskCreationContext::new(
        vec![
            TaskInputBinding::new(TaskInputContract::new("submission").expect("input")),
            TaskInputBinding::new(TaskInputContract::new("submission").expect("input")),
        ],
        None,
    )
    .expect_err("duplicate binding must fail");

    assert_eq!(
        error.to_string(),
        "invalid task instance: duplicate task input binding"
    );
}

#[test]
fn task_instance_input_ordering_is_preserved() {
    let instance = minimal_task_instance();
    assert_eq!(
        instance.task_creation_context().task_input_bindings()[0]
            .task_input_contract()
            .as_str(),
        "submission"
    );
    assert_eq!(
        instance.task_creation_context().task_input_bindings()[1]
            .task_input_contract()
            .as_str(),
        "policy.reference"
    );
}

#[test]
fn task_instance_step_binding_without_workflow_binding_is_rejected() {
    let error = TaskInstance::new(
        TaskInstanceId::new("task.instance.000005").expect("id"),
        task_definition(),
        task_creation_context(),
        Vec::new(),
        None,
        Some(TaskStepBinding::new(TaskStepReference::new(
            WorkflowStepReference::new("workflow.step.review").expect("step"),
        ))),
        TaskState::Pending,
    )
    .expect_err("step binding without workflow binding must fail");

    assert_eq!(
        error.to_string(),
        "invalid task instance: task step binding requires workflow binding"
    );
}

#[test]
fn task_instance_invalid_initial_state_is_rejected() {
    let error = TaskInstance::new(
        TaskInstanceId::new("task.instance.000006").expect("id"),
        task_definition(),
        task_creation_context(),
        Vec::new(),
        None,
        None,
        TaskState::InProgress,
    )
    .expect_err("initial state must fail");

    assert_eq!(
        error.to_string(),
        "invalid task instance: task instance initial state must be Pending"
    );
}

#[test]
fn task_instance_no_hidden_clock_or_random_identity_generation_occurs() {
    let left = minimal_task_instance();
    let right = minimal_task_instance();

    assert_eq!(left.task_instance_id(), right.task_instance_id());
    assert_eq!(
        left.task_definition_snapshot_reference()
            .task_definition_version(),
        right
            .task_definition_snapshot_reference()
            .task_definition_version()
    );
}
