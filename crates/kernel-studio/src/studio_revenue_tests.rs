use crate::studio_test_support::{ownership_path, studio_audit_reference, time_reference};
use crate::StudioRevenueReferenceProjection;

#[test]
fn studio_revenue_projection_preserves_reference_only_semantics_k11_009() {
    let projection = StudioRevenueReferenceProjection::new(
        "revenue.metric.reference",
        time_reference(),
        "revenue.source.reference",
        "USD",
        ownership_path(),
        studio_audit_reference(),
    )
    .expect("projection");
    assert_eq!(
        projection.ownership_path().enterprise_id().as_str(),
        "CX-ENT-000001"
    );
}
