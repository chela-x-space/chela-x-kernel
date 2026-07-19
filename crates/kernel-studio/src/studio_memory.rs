use kernel_domain::{MemoryQueryResult, MemoryRetrievalResult, OwnershipPath};

use crate::studio::{StudioAuditReference, StudioError, StudioErrorCode, StudioResult};
use crate::studio_validation::require_exact_scope;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudioMemoryProjection {
    ownership_path: OwnershipPath,
    memory_retrieval_result: Option<MemoryRetrievalResult>,
    memory_query_result: Option<MemoryQueryResult>,
    studio_audit_reference: StudioAuditReference,
}

impl StudioMemoryProjection {
    pub fn new(
        ownership_path: OwnershipPath,
        memory_retrieval_result: Option<MemoryRetrievalResult>,
        memory_query_result: Option<MemoryQueryResult>,
        studio_audit_reference: StudioAuditReference,
    ) -> StudioResult<Self> {
        if memory_retrieval_result.is_none() && memory_query_result.is_none() {
            return Err(StudioError::new(
                StudioErrorCode::ProjectionMismatch,
                "studio memory projection requires retrieval or query results",
            )?);
        }
        if let Some(memory_retrieval_result) = &memory_retrieval_result {
            require_exact_scope(
                &ownership_path,
                memory_retrieval_result
                    .memory_retrieval_request()
                    .ownership_path(),
                "studio memory retrieval scope must match the selected ownership path",
            )?;
        }
        Ok(Self {
            ownership_path,
            memory_retrieval_result,
            memory_query_result,
            studio_audit_reference,
        })
    }

    pub fn ownership_path(&self) -> &OwnershipPath {
        &self.ownership_path
    }
    pub fn memory_retrieval_result(&self) -> Option<&MemoryRetrievalResult> {
        self.memory_retrieval_result.as_ref()
    }
    pub fn memory_query_result(&self) -> Option<&MemoryQueryResult> {
        self.memory_query_result.as_ref()
    }
    pub fn studio_audit_reference(&self) -> &StudioAuditReference {
        &self.studio_audit_reference
    }
}
