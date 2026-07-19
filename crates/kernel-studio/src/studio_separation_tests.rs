#[test]
fn studio_contracts_remain_technology_neutral_k11_010() {
    let serialized = format!("{:?}", crate::StudioViewKind::TopView);
    assert!(!serialized.contains("React"));
    assert!(!serialized.contains("HTTP"));
}
