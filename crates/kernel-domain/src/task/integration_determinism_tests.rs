#[test]
fn integration_same_full_completion_flow_is_deterministic() {
    let left = super::integration_flow_support::completion_happy_path();
    let right = super::integration_flow_support::completion_happy_path();

    assert_eq!(left, right);
}
