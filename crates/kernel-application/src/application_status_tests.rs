use crate::application_test_support::{application_status_snapshot, application_view_intent};
use crate::ApplicationValidationStatus;

#[test]
fn application_status_snapshot_preserves_validation_status_k12_009() {
    let status = application_status_snapshot();
    assert_eq!(
        status.application_validation_status(),
        ApplicationValidationStatus::Validated
    );
    let _ = application_view_intent();
}
