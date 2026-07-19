use crate::application_test_support::{
    application_query_intent, command_capability, query_application_request_context_with_views,
    query_capability, studio_view_reference, studio_view_request,
};
use crate::{ApplicationErrorCode, ApplicationQueryIntent, ApplicationViewIntent};
use kernel_studio::StudioViewKind;

#[test]
fn application_query_intent_accepts_valid_studio_query_k12_003() {
    let intent = application_query_intent();
    assert_eq!(
        intent
            .application_view_intent()
            .studio_view_reference()
            .view_kind(),
        StudioViewKind::TopView
    );
}

#[test]
fn application_query_intent_rejects_view_request_mismatch_k12_002() {
    let mismatched_view_intent = ApplicationViewIntent::new(
        studio_view_reference(StudioViewKind::Runtime),
        crate::application_test_support::studio_selection_context(),
        None,
        vec![],
    )
    .expect("view intent");
    let error = ApplicationQueryIntent::new(
        query_application_request_context_with_views(vec![
            studio_view_reference(StudioViewKind::TopView),
            studio_view_reference(StudioViewKind::Runtime),
        ]),
        query_capability(),
        mismatched_view_intent,
        studio_view_request(),
    )
    .expect_err("view mismatch must fail");
    assert_eq!(error.code(), ApplicationErrorCode::ViewRequestMismatch);
}

#[test]
fn application_query_intent_rejects_capability_mismatch_before_view_mismatch_k12_003() {
    let mismatched_view_intent = ApplicationViewIntent::new(
        studio_view_reference(StudioViewKind::Runtime),
        crate::application_test_support::studio_selection_context(),
        None,
        vec![],
    )
    .expect("view intent");
    let error = ApplicationQueryIntent::new(
        query_application_request_context_with_views(vec![
            studio_view_reference(StudioViewKind::TopView),
            studio_view_reference(StudioViewKind::Runtime),
        ]),
        command_capability(),
        mismatched_view_intent,
        studio_view_request(),
    )
    .expect_err("capability mismatch must take precedence");
    assert_eq!(error.code(), ApplicationErrorCode::CapabilityMismatch);
}
