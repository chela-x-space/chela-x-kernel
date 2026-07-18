use crate::state::{
    StateSequence, TransitionAuthorityReference, TransitionEvidenceReference,
    TransitionReasonReference,
};

use super::{
    TaskCompletionRequirement, TaskCompletionResult, TaskCompletionValidationRequest,
    TaskCreationContext, TaskDefinition, TaskDefinitionId, TaskDefinitionName,
    TaskDefinitionVersion, TaskDescription, TaskEvidence, TaskEvidenceId, TaskEvidenceMetadata,
    TaskEvidenceReference, TaskEvidenceRequirement, TaskEvidenceSet, TaskEvidenceType, TaskFailure,
    TaskFailureCategory, TaskFailureCode, TaskFailurePolicyReference, TaskFailureReference,
    TaskFailureValidationRequest, TaskInputBinding, TaskInputContract, TaskInstance,
    TaskInstanceId, TaskInstanceReference, TaskKind, TaskOutput, TaskOutputBinding,
    TaskOutputContract, TaskOutputReference, TaskRecoveryReference, TaskState, TaskStateSnapshot,
    TaskTransitionControl, TaskTransitionDecision, TaskTransitionRequest,
};

pub fn task_instance() -> TaskInstance {
    TaskInstance::new(
        task_instance_id(),
        task_definition(),
        TaskCreationContext::new(vec![TaskInputBinding::new(input_contract())], None)
            .expect("creation context"),
        vec![
            TaskOutputBinding::new(output_contract_primary()),
            TaskOutputBinding::new(output_contract_audit()),
        ],
        None,
        None,
        TaskState::Pending,
    )
    .expect("task instance")
}

pub fn task_definition() -> TaskDefinition {
    TaskDefinition::new(
        TaskDefinitionId::new("task.definition.demo").expect("definition id"),
        TaskDefinitionVersion::new("1.0.0").expect("definition version"),
        TaskDefinitionName::new("Demo Task").expect("definition name"),
        Some(TaskDescription::new("demo").expect("description")),
        TaskKind::new("task.kind.demo").expect("task kind"),
        vec![input_contract()],
        vec![output_contract_primary(), output_contract_audit()],
        Vec::new(),
        Vec::new(),
        vec![evidence_requirement()],
        vec![completion_requirement()],
        Some(task_failure_policy_reference()),
        None,
        None,
    )
    .expect("task definition")
}

fn task_instance_id() -> TaskInstanceId {
    TaskInstanceId::new("task.instance.demo").expect("task instance id")
}

pub fn state_snapshot(task_state: TaskState) -> TaskStateSnapshot {
    TaskStateSnapshot::new(
        task_instance_reference(),
        task_state,
        StateSequence::new(1).unwrap(),
    )
}

pub fn task_instance_reference() -> TaskInstanceReference {
    TaskInstanceReference::new(task_instance_id())
}

pub fn required_evidence_set(requirement: super::TaskEvidenceRequirement) -> TaskEvidenceSet {
    TaskEvidenceSet::new(
        task_instance_reference(),
        vec![task_evidence("task.evidence.demo", Some(requirement))],
    )
    .expect("evidence set")
}

pub fn failure_evidence_set() -> TaskEvidenceSet {
    TaskEvidenceSet::new(
        task_instance_reference(),
        vec![task_evidence("task.evidence.demo", None)],
    )
    .expect("evidence set")
}

pub fn task_evidence(
    id: &str,
    requirement: Option<super::TaskEvidenceRequirement>,
) -> TaskEvidence {
    TaskEvidence::new(
        TaskEvidenceReference::new(TaskEvidenceId::new(id).expect("evidence id")),
        task_instance_reference(),
        TaskEvidenceType::new("task.evidence.document").expect("evidence type"),
        Some(TransitionAuthorityReference::new("authority.demo").expect("authority")),
        TaskEvidenceMetadata::new(
            requirement,
            Some(
                TransitionEvidenceReference::new("transition.evidence.demo")
                    .expect("transition evidence"),
            ),
        ),
    )
}

pub fn completion_result(
    outputs: Vec<TaskOutput>,
    evidence_set: TaskEvidenceSet,
    requirements: Vec<TaskCompletionRequirement>,
) -> TaskCompletionResult {
    TaskCompletionResult::new(
        task_instance_reference(),
        task_instance().task_definition_snapshot_reference().clone(),
        requirements,
        outputs,
        evidence_set,
        Some(TransitionAuthorityReference::new("completion.authority").expect("authority")),
        Some(TransitionReasonReference::new("completion.reason").expect("reason")),
    )
    .expect("completion result")
}

pub fn completion_request(task_state: TaskState) -> TaskCompletionValidationRequest {
    TaskCompletionValidationRequest::new(
        task_instance(),
        state_snapshot(task_state),
        completion_result(
            valid_outputs(),
            required_evidence_set(evidence_requirement()),
            vec![completion_requirement()],
        ),
        None,
    )
}

pub fn failure(
    evidence_set: TaskEvidenceSet,
    policy: Option<TaskFailurePolicyReference>,
) -> TaskFailure {
    TaskFailure::new(
        task_instance_reference(),
        TaskFailureReference::new("task.failure.demo").expect("failure reference"),
        TaskFailureCode::new("task.failure.timeout").expect("failure code"),
        TaskFailureCategory::new("task.failure_category.execution").expect("failure category"),
        Some(super::TaskFailureReason::new("supporting detail").expect("failure reason")),
        evidence_set,
        Some(TransitionAuthorityReference::new("failure.authority").expect("authority")),
        policy,
    )
}

pub fn failure_request(
    task_completion: Option<super::TaskCompletion>,
) -> TaskFailureValidationRequest {
    TaskFailureValidationRequest::new(
        task_instance(),
        state_snapshot(TaskState::InProgress),
        failure(
            failure_evidence_set(),
            Some(task_failure_policy_reference()),
        ),
        task_completion,
    )
}

pub fn lifecycle_completion_decision() -> TaskTransitionDecision {
    TaskTransitionControl::evaluate(
        &TaskTransitionRequest::new(
            state_snapshot(TaskState::InProgress),
            TaskState::Completed,
            Some(TransitionReasonReference::new("transition.reason").expect("reason")),
            Some(TransitionAuthorityReference::new("transition.authority").expect("authority")),
            vec![TransitionEvidenceReference::new("transition.evidence").expect("evidence")],
            None,
            None,
            super::TaskLifecycleGuards::new(
                Some(StateSequence::new(1).unwrap()),
                false,
                true,
                true,
                true,
                true,
                true,
                true,
                None,
                None,
            ),
        )
        .expect("transition request"),
    )
}

pub fn valid_outputs() -> Vec<TaskOutput> {
    vec![
        TaskOutput::new(
            TaskOutputReference::new("task.output.primary").expect("output reference"),
            TaskOutputBinding::new(output_contract_primary()),
        ),
        TaskOutput::new(
            TaskOutputReference::new("task.output.audit").expect("output reference"),
            TaskOutputBinding::new(output_contract_audit()),
        ),
    ]
}

pub fn input_contract() -> TaskInputContract {
    TaskInputContract::new("task.input.primary").expect("input contract")
}

pub fn output_contract_primary() -> TaskOutputContract {
    TaskOutputContract::new("task.output.primary").expect("output contract")
}

pub fn output_contract_audit() -> TaskOutputContract {
    TaskOutputContract::new("task.output.audit").expect("output contract")
}

pub fn evidence_requirement() -> TaskEvidenceRequirement {
    TaskEvidenceRequirement::new("task.evidence.required").expect("evidence requirement")
}

pub fn completion_requirement() -> TaskCompletionRequirement {
    TaskCompletionRequirement::new("task.completion.required").expect("completion requirement")
}

pub fn recovery_reference() -> TaskRecoveryReference {
    TaskRecoveryReference::new("recovery/manual-review", true).expect("recovery reference")
}

pub fn task_failure_policy_reference() -> TaskFailurePolicyReference {
    TaskFailurePolicyReference::new("CX-POL-900001").expect("policy")
}
