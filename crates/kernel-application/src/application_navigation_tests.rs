use crate::application_test_support::{
    application_view_intent, studio_navigation_reference, studio_selection_context,
    studio_view_reference,
};
use crate::{ApplicationErrorCode, ApplicationViewIntent};
use kernel_studio::StudioViewKind;

#[test]
fn application_view_intent_accepts_valid_navigation_chain_k12_002() {
    let intent = application_view_intent();
    assert_eq!(
        intent.studio_view_reference().view_kind(),
        StudioViewKind::TopView
    );
}

#[test]
fn application_view_intent_rejects_invalid_navigation_chain_k12_002() {
    let error = ApplicationViewIntent::new(
        studio_view_reference(StudioViewKind::TopView),
        studio_selection_context(),
        None,
        vec![studio_navigation_reference(StudioViewKind::Runtime)],
    )
    .expect_err("mismatched navigation target must fail");
    assert_eq!(error.code(), ApplicationErrorCode::InvalidNavigationIntent);
}
