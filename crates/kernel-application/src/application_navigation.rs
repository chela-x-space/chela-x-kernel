use kernel_studio::{
    StudioFilterContext, StudioNavigationReference, StudioSelectionContext, StudioViewKind,
    StudioViewReference,
};

use crate::application_error::{ApplicationError, ApplicationErrorCode, ApplicationResult};
use crate::application_validation::reject_duplicates;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationViewIntent {
    studio_view_reference: StudioViewReference,
    studio_selection_context: StudioSelectionContext,
    studio_filter_context: Option<StudioFilterContext>,
    studio_navigation_references: Vec<StudioNavigationReference>,
}

impl ApplicationViewIntent {
    pub fn new(
        studio_view_reference: StudioViewReference,
        studio_selection_context: StudioSelectionContext,
        studio_filter_context: Option<StudioFilterContext>,
        studio_navigation_references: Vec<StudioNavigationReference>,
    ) -> ApplicationResult<Self> {
        if studio_view_reference.view_kind() == StudioViewKind::CommandConsole {
            return Err(ApplicationError::new(
                ApplicationErrorCode::ViewRequestMismatch,
                "application view intent does not permit the command console view",
            )?);
        }
        reject_duplicates(
            &studio_navigation_references,
            ApplicationErrorCode::InvalidNavigationIntent,
            "duplicate application navigation reference",
        )?;
        if studio_navigation_references
            .iter()
            .any(|reference| reference.target_view_reference() != &studio_view_reference)
        {
            return Err(ApplicationError::new(
                ApplicationErrorCode::InvalidNavigationIntent,
                "application navigation chains must preserve the requested Studio view identity",
            )?);
        }
        Ok(Self {
            studio_view_reference,
            studio_selection_context,
            studio_filter_context,
            studio_navigation_references,
        })
    }

    pub fn studio_view_reference(&self) -> &StudioViewReference {
        &self.studio_view_reference
    }

    pub fn studio_selection_context(&self) -> &StudioSelectionContext {
        &self.studio_selection_context
    }

    pub fn studio_filter_context(&self) -> Option<&StudioFilterContext> {
        self.studio_filter_context.as_ref()
    }

    pub fn studio_navigation_references(&self) -> &[StudioNavigationReference] {
        &self.studio_navigation_references
    }
}
