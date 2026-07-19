use kernel_domain::{EnglishNamespace, OwnershipPath, TimeReference};

use crate::studio::{StudioAuditReference, StudioError, StudioErrorCode, StudioResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudioRevenueReferenceProjection {
    revenue_metric_reference: EnglishNamespace,
    period_reference: TimeReference,
    source_reference: EnglishNamespace,
    currency_reference: EnglishNamespace,
    ownership_path: OwnershipPath,
    studio_audit_reference: StudioAuditReference,
}

impl StudioRevenueReferenceProjection {
    pub fn new(
        revenue_metric_reference: impl Into<String>,
        period_reference: TimeReference,
        source_reference: impl Into<String>,
        currency_reference: impl Into<String>,
        ownership_path: OwnershipPath,
        studio_audit_reference: StudioAuditReference,
    ) -> StudioResult<Self> {
        let currency_reference =
            EnglishNamespace::new("studio_revenue_currency_reference", currency_reference)
                .map_err(crate::StudioError::from_domain_rejection)?;
        if !currency_reference
            .as_str()
            .chars()
            .all(|character| character.is_ascii_uppercase() || character.is_ascii_digit())
        {
            return Err(StudioError::new(
                StudioErrorCode::InvalidStudioRequest,
                "studio revenue currency reference must remain technology-neutral and uppercase",
            )?);
        }
        Ok(Self {
            revenue_metric_reference: EnglishNamespace::new(
                "studio_revenue_metric_reference",
                revenue_metric_reference,
            )
            .map_err(crate::StudioError::from_domain_rejection)?,
            period_reference,
            source_reference: EnglishNamespace::new(
                "studio_revenue_source_reference",
                source_reference,
            )
            .map_err(crate::StudioError::from_domain_rejection)?,
            currency_reference,
            ownership_path,
            studio_audit_reference,
        })
    }

    pub fn ownership_path(&self) -> &OwnershipPath {
        &self.ownership_path
    }
}
