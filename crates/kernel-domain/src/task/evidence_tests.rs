use super::{TaskEvidenceControl, TaskEvidenceRejectionReason, TaskEvidenceValidation};

#[test]
fn task_evidence_preserves_subject_and_metadata() {
    let evidence = super::outcome_test_support::task_evidence(
        "task.evidence.demo",
        Some(super::outcome_test_support::evidence_requirement()),
    );

    assert_eq!(
        evidence.subject_task_instance_reference(),
        &super::outcome_test_support::task_instance_reference()
    );
    assert_eq!(
        evidence.task_evidence_type().as_str(),
        "task.evidence.document"
    );
    assert_eq!(
        evidence
            .task_evidence_metadata()
            .task_evidence_requirement(),
        Some(&super::outcome_test_support::evidence_requirement())
    );
}

#[test]
fn task_evidence_set_rejects_duplicate_identity() {
    let evidence = super::outcome_test_support::task_evidence("task.evidence.dup", None);
    let error = super::TaskEvidenceSet::new(
        super::outcome_test_support::task_instance_reference(),
        vec![evidence.clone(), evidence],
    )
    .expect_err("duplicate evidence identity must reject");

    assert_eq!(
        error,
        crate::errors::DomainError::InvalidTaskEvidence("duplicate task evidence identity")
    );
}

#[test]
fn task_evidence_validation_accepts_declared_requirement() {
    let request = super::TaskEvidenceValidationRequest::new(
        super::outcome_test_support::task_instance(),
        super::outcome_test_support::required_evidence_set(
            super::outcome_test_support::evidence_requirement(),
        ),
    );

    assert!(matches!(
        TaskEvidenceControl::validate(&request),
        TaskEvidenceValidation::Accepted(_)
    ));
}

#[test]
fn task_evidence_validation_rejects_undeclared_requirement() {
    let request = super::TaskEvidenceValidationRequest::new(
        super::outcome_test_support::task_instance(),
        super::outcome_test_support::required_evidence_set(
            super::TaskEvidenceRequirement::new("task.evidence.undeclared")
                .expect("evidence requirement"),
        ),
    );

    let decision = TaskEvidenceControl::validate(&request);
    assert!(matches!(
        decision,
        TaskEvidenceValidation::Rejected(rejected)
            if rejected.reason() == TaskEvidenceRejectionReason::UndeclaredEvidence
    ));
}
