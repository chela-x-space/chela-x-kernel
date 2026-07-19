use kernel_domain::{
    AgentId, AuditEvidenceId, CorrelationId, EnglishNamespace, EventTraceReference,
    ExecutionSessionId, MemoryRecordReference, NonEmptyText, OwnershipPath, RuntimeId,
    StableVersion, TaskInstanceReference, TimeReference, WorkflowId,
};
use kernel_gateway::{
    GatewayAuditReference, GatewayError, GatewayErrorCode, GatewayQueryPayload,
    GatewayRequestEnvelope, GatewayResponseEnvelope, GatewayStatusSnapshot,
};

use crate::studio_audit::StudioAuditProjection;
use crate::studio_command::{StudioCommandRequest, StudioCommandResponse};
use crate::studio_digital_twin::StudioDigitalTwinProjection;
use crate::studio_event::StudioEventTimelineProjection;
use crate::studio_memory::StudioMemoryProjection;
use crate::studio_revenue::StudioRevenueReferenceProjection;
use crate::studio_runtime::StudioRuntimeProjection;
use crate::studio_task::StudioTaskProjection;
use crate::studio_top_view::StudioTopViewProjection;
use crate::studio_validation::{
    reject_duplicates, require_correlation, require_exact_scope, require_query_only,
    require_query_responses_only, require_view_query_support,
};
use crate::studio_workflow::StudioWorkflowProjection;

pub type StudioResult<T> = Result<T, StudioError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StudioViewKind {
    TopView,
    DigitalTwin,
    Runtime,
    Workflow,
    Task,
    EventTimeline,
    Memory,
    Audit,
    Revenue,
    CommandConsole,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudioApiVersion(StableVersion);

impl StudioApiVersion {
    pub fn new(value: impl Into<String>) -> StudioResult<Self> {
        let value = value.into();
        let trimmed = value.trim();
        if trimmed.is_empty()
            || trimmed.contains('/')
            || !trimmed.chars().all(|character| {
                character.is_ascii_alphanumeric() || matches!(character, '.' | '_' | '-')
            })
        {
            return Err(StudioError::new(
                StudioErrorCode::UnsupportedStudioVersion,
                "studio API version must be namespace-safe and transport-neutral",
            )?);
        }
        Ok(Self(
            StableVersion::new("studio_api_version", trimmed.to_owned())
                .map_err(StudioError::from_domain_rejection)?,
        ))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudioViewReference {
    view_kind: StudioViewKind,
    view_name: EnglishNamespace,
}

impl StudioViewReference {
    pub fn new(view_kind: StudioViewKind, view_name: impl Into<String>) -> StudioResult<Self> {
        let view_name = EnglishNamespace::new("studio_view_name", view_name)
            .map_err(StudioError::from_domain_rejection)?;
        if !view_name.as_str().contains('.') {
            return Err(StudioError::new(
                StudioErrorCode::UnsupportedView,
                "studio view reference requires a namespaced logical view identity",
            )?);
        }
        Ok(Self {
            view_kind,
            view_name,
        })
    }

    pub fn view_kind(&self) -> StudioViewKind {
        self.view_kind
    }

    pub fn view_name(&self) -> &str {
        self.view_name.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudioNavigationReference {
    target_view_reference: StudioViewReference,
    navigation_target_reference: EnglishNamespace,
}

impl StudioNavigationReference {
    pub fn new(
        target_view_reference: StudioViewReference,
        navigation_target_reference: impl Into<String>,
    ) -> StudioResult<Self> {
        Ok(Self {
            target_view_reference,
            navigation_target_reference: EnglishNamespace::new(
                "studio_navigation_target_reference",
                navigation_target_reference,
            )
            .map_err(StudioError::from_domain_rejection)?,
        })
    }

    pub fn target_view_reference(&self) -> &StudioViewReference {
        &self.target_view_reference
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudioFilterReference {
    filter_reference: EnglishNamespace,
    filter_value: NonEmptyText,
    negated: bool,
}

impl StudioFilterReference {
    pub fn new(
        filter_reference: impl Into<String>,
        filter_value: impl Into<String>,
        negated: bool,
    ) -> StudioResult<Self> {
        Ok(Self {
            filter_reference: EnglishNamespace::new("studio_filter_reference", filter_reference)
                .map_err(StudioError::from_domain_rejection)?,
            filter_value: NonEmptyText::new("studio_filter_value", filter_value)
                .map_err(StudioError::from_domain_rejection)?,
            negated,
        })
    }

    pub fn filter_reference(&self) -> &str {
        self.filter_reference.as_str()
    }

    pub fn filter_value(&self) -> &str {
        self.filter_value.as_str()
    }

    pub fn negated(&self) -> bool {
        self.negated
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudioSortReference {
    sort_reference: EnglishNamespace,
    descending: bool,
}

impl StudioSortReference {
    pub fn new(sort_reference: impl Into<String>, descending: bool) -> StudioResult<Self> {
        Ok(Self {
            sort_reference: EnglishNamespace::new("studio_sort_reference", sort_reference)
                .map_err(StudioError::from_domain_rejection)?,
            descending,
        })
    }

    pub fn sort_reference(&self) -> &str {
        self.sort_reference.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudioTimeRange {
    started_at: TimeReference,
    ended_at: TimeReference,
}

impl StudioTimeRange {
    pub fn new(started_at: TimeReference, ended_at: TimeReference) -> StudioResult<Self> {
        if started_at.as_str() > ended_at.as_str() {
            return Err(StudioError::new(
                StudioErrorCode::InvalidFilterReference,
                "studio time range start must not follow the end reference",
            )?);
        }
        Ok(Self {
            started_at,
            ended_at,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudioFilterContext {
    filter_references: Vec<StudioFilterReference>,
    sort_references: Vec<StudioSortReference>,
    time_range: Option<StudioTimeRange>,
}

impl StudioFilterContext {
    pub fn new(
        filter_references: Vec<StudioFilterReference>,
        sort_references: Vec<StudioSortReference>,
        time_range: Option<StudioTimeRange>,
    ) -> StudioResult<Self> {
        reject_duplicates(
            &sort_references,
            StudioErrorCode::InvalidFilterReference,
            "duplicate studio sort reference",
        )?;
        for (index, filter_reference) in filter_references.iter().enumerate() {
            if filter_references[..index].iter().any(|prior| {
                prior.filter_reference() == filter_reference.filter_reference()
                    && prior.filter_value() == filter_reference.filter_value()
                    && prior.negated() != filter_reference.negated()
            }) {
                return Err(StudioError::new(
                    StudioErrorCode::ContradictoryFilters,
                    "studio filter references must not contain contradictory filter polarity",
                )?);
            }
        }
        Ok(Self {
            filter_references,
            sort_references,
            time_range,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudioSelectionContext {
    ownership_path: OwnershipPath,
    selected_agent_id: Option<AgentId>,
    selected_runtime_id: Option<RuntimeId>,
    selected_workflow_id: Option<WorkflowId>,
    selected_task_instance_reference: Option<TaskInstanceReference>,
    selected_execution_session_id: Option<ExecutionSessionId>,
    selected_memory_record_reference: Option<MemoryRecordReference>,
}

impl StudioSelectionContext {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ownership_path: OwnershipPath,
        selected_agent_id: Option<AgentId>,
        selected_runtime_id: Option<RuntimeId>,
        selected_workflow_id: Option<WorkflowId>,
        selected_task_instance_reference: Option<TaskInstanceReference>,
        selected_execution_session_id: Option<ExecutionSessionId>,
        selected_memory_record_reference: Option<MemoryRecordReference>,
    ) -> StudioResult<Self> {
        Ok(Self {
            ownership_path,
            selected_agent_id,
            selected_runtime_id,
            selected_workflow_id,
            selected_task_instance_reference,
            selected_execution_session_id,
            selected_memory_record_reference,
        })
    }

    pub fn ownership_path(&self) -> &OwnershipPath {
        &self.ownership_path
    }
    pub fn selected_agent_id(&self) -> Option<&AgentId> {
        self.selected_agent_id.as_ref()
    }
    pub fn selected_runtime_id(&self) -> Option<&RuntimeId> {
        self.selected_runtime_id.as_ref()
    }
    pub fn selected_workflow_id(&self) -> Option<&WorkflowId> {
        self.selected_workflow_id.as_ref()
    }
    pub fn selected_task_instance_reference(&self) -> Option<&TaskInstanceReference> {
        self.selected_task_instance_reference.as_ref()
    }
    pub fn selected_execution_session_id(&self) -> Option<&ExecutionSessionId> {
        self.selected_execution_session_id.as_ref()
    }
    pub fn selected_memory_record_reference(&self) -> Option<&MemoryRecordReference> {
        self.selected_memory_record_reference.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudioAuditReference {
    studio_trace_reference: EventTraceReference,
    correlation_id: Option<CorrelationId>,
    audit_evidence_ids: Vec<AuditEvidenceId>,
    gateway_audit_reference: Option<GatewayAuditReference>,
}

impl StudioAuditReference {
    pub fn new(
        studio_trace_reference: EventTraceReference,
        correlation_id: Option<CorrelationId>,
        audit_evidence_ids: Vec<AuditEvidenceId>,
        gateway_audit_reference: Option<GatewayAuditReference>,
    ) -> StudioResult<Self> {
        if audit_evidence_ids.is_empty() {
            return Err(StudioError::new(
                StudioErrorCode::InternalContractViolation,
                "studio audit reference requires audit evidence identifiers",
            )?);
        }
        reject_duplicates(
            &audit_evidence_ids,
            StudioErrorCode::InternalContractViolation,
            "duplicate studio audit evidence identifier",
        )?;
        if let (Some(correlation_id), Some(gateway_audit_reference)) =
            (correlation_id.as_ref(), gateway_audit_reference.as_ref())
        {
            if gateway_audit_reference.correlation_id() != Some(correlation_id) {
                return Err(StudioError::new(
                    StudioErrorCode::AuditReferenceMismatch,
                    "studio audit reference correlation must match the preserved gateway audit correlation reference",
                )?);
            }
        }
        Ok(Self {
            studio_trace_reference,
            correlation_id,
            audit_evidence_ids,
            gateway_audit_reference,
        })
    }

    pub fn correlation_id(&self) -> Option<&CorrelationId> {
        self.correlation_id.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudioStatusSnapshot {
    studio_api_version: StudioApiVersion,
    supported_views: Vec<StudioViewReference>,
    gateway_status_snapshot: GatewayStatusSnapshot,
    generated_at: TimeReference,
    studio_audit_reference: StudioAuditReference,
}

impl StudioStatusSnapshot {
    pub fn new(
        studio_api_version: StudioApiVersion,
        supported_views: Vec<StudioViewReference>,
        gateway_status_snapshot: GatewayStatusSnapshot,
        generated_at: TimeReference,
        studio_audit_reference: StudioAuditReference,
    ) -> StudioResult<Self> {
        if supported_views.is_empty() {
            return Err(StudioError::new(
                StudioErrorCode::InvalidStudioRequest,
                "studio status snapshot requires at least one supported view",
            )?);
        }
        reject_duplicates(
            &supported_views,
            StudioErrorCode::InvalidStudioRequest,
            "duplicate studio view reference in status snapshot",
        )?;
        Ok(Self {
            studio_api_version,
            supported_views,
            gateway_status_snapshot,
            generated_at,
            studio_audit_reference,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StudioErrorCode {
    InvalidStudioRequest,
    UnsupportedStudioVersion,
    UnsupportedView,
    InvalidSelection,
    ScopeMismatch,
    ViewQueryMismatch,
    CommandOperationMismatch,
    ResponseCorrelationMismatch,
    ProjectionMismatch,
    RuntimeProjectionMismatch,
    WorkflowProjectionMismatch,
    TaskProjectionMismatch,
    EventSequenceInconsistency,
    MemoryProjectionMismatch,
    AuditReferenceMismatch,
    InvalidFilterReference,
    ContradictoryFilters,
    GatewayRequestRejection,
    GatewayAuthorizationDenied,
    GatewayDomainRejection,
    InternalContractViolation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudioError {
    code: StudioErrorCode,
    detail: Box<NonEmptyText>,
    studio_view_reference: Option<Box<StudioViewReference>>,
    correlation_id: Option<Box<CorrelationId>>,
    studio_audit_reference: Option<Box<StudioAuditReference>>,
    gateway_error: Option<Box<GatewayError>>,
}

impl StudioError {
    pub fn new(code: StudioErrorCode, detail: impl Into<String>) -> StudioResult<Self> {
        Ok(Self {
            code,
            detail: Box::new(
                NonEmptyText::new("studio_error_detail", detail)
                    .map_err(Self::from_domain_rejection)?,
            ),
            studio_view_reference: None,
            correlation_id: None,
            studio_audit_reference: None,
            gateway_error: None,
        })
    }

    pub fn from_domain_rejection(domain_error: kernel_domain::DomainError) -> Self {
        Self {
            code: StudioErrorCode::InvalidStudioRequest,
            detail: Box::new(
                NonEmptyText::new("studio_error_detail", domain_error.to_string())
                    .expect("domain rejection detail"),
            ),
            studio_view_reference: None,
            correlation_id: None,
            studio_audit_reference: None,
            gateway_error: None,
        }
    }

    pub fn from_gateway_rejection(gateway_error: GatewayError) -> Self {
        let code = match gateway_error.code() {
            GatewayErrorCode::AuthorizationDenied => StudioErrorCode::GatewayAuthorizationDenied,
            GatewayErrorCode::DomainRejection => StudioErrorCode::GatewayDomainRejection,
            _ => StudioErrorCode::GatewayRequestRejection,
        };
        Self {
            code,
            detail: Box::new(
                NonEmptyText::new("studio_error_detail", gateway_error.detail())
                    .expect("gateway rejection detail"),
            ),
            studio_view_reference: None,
            correlation_id: gateway_error.correlation_id().cloned().map(Box::new),
            studio_audit_reference: None,
            gateway_error: Some(Box::new(gateway_error)),
        }
    }

    pub fn code(&self) -> StudioErrorCode {
        self.code
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StudioViewProjection {
    TopView(Box<StudioTopViewProjection>),
    DigitalTwin(Box<StudioDigitalTwinProjection>),
    Runtime(Box<StudioRuntimeProjection>),
    Workflow(Box<StudioWorkflowProjection>),
    Task(Box<StudioTaskProjection>),
    EventTimeline(Box<StudioEventTimelineProjection>),
    Memory(Box<StudioMemoryProjection>),
    Audit(Box<StudioAuditProjection>),
    Revenue(Box<StudioRevenueReferenceProjection>),
}

impl StudioViewProjection {
    pub fn view_kind(&self) -> StudioViewKind {
        match self {
            Self::TopView(_) => StudioViewKind::TopView,
            Self::DigitalTwin(_) => StudioViewKind::DigitalTwin,
            Self::Runtime(_) => StudioViewKind::Runtime,
            Self::Workflow(_) => StudioViewKind::Workflow,
            Self::Task(_) => StudioViewKind::Task,
            Self::EventTimeline(_) => StudioViewKind::EventTimeline,
            Self::Memory(_) => StudioViewKind::Memory,
            Self::Audit(_) => StudioViewKind::Audit,
            Self::Revenue(_) => StudioViewKind::Revenue,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudioViewRequest {
    studio_api_version: StudioApiVersion,
    studio_view_reference: StudioViewReference,
    studio_selection_context: StudioSelectionContext,
    studio_filter_context: Option<StudioFilterContext>,
    studio_navigation_reference: Option<StudioNavigationReference>,
    correlation_id: CorrelationId,
    requested_at: TimeReference,
    gateway_request_envelopes: Vec<GatewayRequestEnvelope>,
    studio_audit_reference: StudioAuditReference,
}

impl StudioViewRequest {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        studio_api_version: StudioApiVersion,
        studio_view_reference: StudioViewReference,
        studio_selection_context: StudioSelectionContext,
        studio_filter_context: Option<StudioFilterContext>,
        studio_navigation_reference: Option<StudioNavigationReference>,
        correlation_id: CorrelationId,
        requested_at: TimeReference,
        gateway_request_envelopes: Vec<GatewayRequestEnvelope>,
        studio_audit_reference: StudioAuditReference,
    ) -> StudioResult<Self> {
        if studio_view_reference.view_kind() == StudioViewKind::CommandConsole {
            return Err(StudioError::new(
                StudioErrorCode::ViewQueryMismatch,
                "command console requests must use studio command contracts rather than studio view requests",
            )?);
        }
        require_query_only(&gateway_request_envelopes)?;
        if studio_audit_reference.correlation_id() != Some(&correlation_id) {
            return Err(StudioError::new(
                StudioErrorCode::AuditReferenceMismatch,
                "studio view audit correlation must match the studio view correlation reference",
            )?);
        }
        for gateway_request_envelope in &gateway_request_envelopes {
            let GatewayRequestEnvelope::Query {
                gateway_request_context,
                gateway_query_request,
            } = gateway_request_envelope
            else {
                unreachable!("validated query-only");
            };
            require_correlation(
                &correlation_id,
                gateway_request_context.correlation_id(),
                "studio view correlation must match the gateway request correlation reference",
            )?;
            require_exact_scope(
                studio_selection_context.ownership_path(),
                gateway_request_context.ownership_path(),
                "studio view scope must match the gateway request ownership path",
            )?;
            require_view_query_support(
                studio_view_reference.view_kind(),
                gateway_query_request.gateway_query_payload(),
            )?;
            match gateway_query_request.gateway_query_payload() {
                GatewayQueryPayload::RuntimeSnapshot(runtime_id) => {
                    if let Some(selected_runtime_id) =
                        studio_selection_context.selected_runtime_id()
                    {
                        if selected_runtime_id != runtime_id {
                            return Err(StudioError::new(
                                StudioErrorCode::InvalidSelection,
                                "studio runtime selection must match the gateway runtime query target",
                            )?);
                        }
                    }
                }
                GatewayQueryPayload::WorkflowState(workflow_id) => {
                    if let Some(selected_workflow_id) =
                        studio_selection_context.selected_workflow_id()
                    {
                        if selected_workflow_id != workflow_id {
                            return Err(StudioError::new(
                                StudioErrorCode::InvalidSelection,
                                "studio workflow selection must match the gateway workflow query target",
                            )?);
                        }
                    }
                }
                GatewayQueryPayload::TaskState(task_instance_reference) => {
                    if let Some(selected_task_instance_reference) =
                        studio_selection_context.selected_task_instance_reference()
                    {
                        if selected_task_instance_reference != task_instance_reference {
                            return Err(StudioError::new(
                                StudioErrorCode::InvalidSelection,
                                "studio task selection must match the gateway task query target",
                            )?);
                        }
                    }
                }
                GatewayQueryPayload::ExecutionSession(execution_session_id) => {
                    if let Some(selected_execution_session_id) =
                        studio_selection_context.selected_execution_session_id()
                    {
                        if selected_execution_session_id != execution_session_id {
                            return Err(StudioError::new(
                                StudioErrorCode::InvalidSelection,
                                "studio execution selection must match the gateway execution query target",
                            )?);
                        }
                    }
                }
                GatewayQueryPayload::MemoryRetrieval(memory_retrieval_request) => {
                    require_exact_scope(
                        studio_selection_context.ownership_path(),
                        memory_retrieval_request.ownership_path(),
                        "studio memory retrieval scope must match the studio selection scope",
                    )?;
                }
                GatewayQueryPayload::MemoryQuery(_) | GatewayQueryPayload::GatewayStatus => {}
            }
        }
        Ok(Self {
            studio_api_version,
            studio_view_reference,
            studio_selection_context,
            studio_filter_context,
            studio_navigation_reference,
            correlation_id,
            requested_at,
            gateway_request_envelopes,
            studio_audit_reference,
        })
    }

    pub fn correlation_id(&self) -> &CorrelationId {
        &self.correlation_id
    }
    pub fn studio_view_reference(&self) -> &StudioViewReference {
        &self.studio_view_reference
    }
    pub fn studio_selection_context(&self) -> &StudioSelectionContext {
        &self.studio_selection_context
    }
    pub fn gateway_request_envelopes(&self) -> &[GatewayRequestEnvelope] {
        &self.gateway_request_envelopes
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudioViewResponse {
    studio_view_reference: StudioViewReference,
    correlation_id: CorrelationId,
    studio_view_projection: StudioViewProjection,
    gateway_response_envelopes: Vec<GatewayResponseEnvelope>,
    studio_audit_reference: StudioAuditReference,
    responded_at: TimeReference,
}

impl StudioViewResponse {
    pub fn new(
        studio_view_request: &StudioViewRequest,
        correlation_id: CorrelationId,
        studio_view_projection: StudioViewProjection,
        gateway_response_envelopes: Vec<GatewayResponseEnvelope>,
        studio_audit_reference: StudioAuditReference,
        responded_at: TimeReference,
    ) -> StudioResult<Self> {
        require_correlation(
            studio_view_request.correlation_id(),
            &correlation_id,
            "studio view response correlation must match the original studio view correlation reference",
        )?;
        require_query_responses_only(&gateway_response_envelopes)?;
        if studio_view_projection.view_kind()
            != studio_view_request.studio_view_reference().view_kind()
        {
            return Err(StudioError::new(
                StudioErrorCode::ProjectionMismatch,
                "studio view response projection must match the original studio view identity",
            )?);
        }
        if studio_audit_reference.correlation_id() != Some(&correlation_id) {
            return Err(StudioError::new(
                StudioErrorCode::AuditReferenceMismatch,
                "studio view response audit correlation must match the studio response correlation reference",
            )?);
        }
        for gateway_response_envelope in &gateway_response_envelopes {
            let GatewayResponseEnvelope::Query {
                correlation_id: response_correlation_id,
                ..
            } = gateway_response_envelope
            else {
                unreachable!("validated query-only");
            };
            require_correlation(
                &correlation_id,
                response_correlation_id,
                "studio view response correlation must match the preserved gateway response correlation reference",
            )?;
        }
        Ok(Self {
            studio_view_reference: studio_view_request.studio_view_reference().clone(),
            correlation_id,
            studio_view_projection,
            gateway_response_envelopes,
            studio_audit_reference,
            responded_at,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StudioRequestEnvelope {
    View(Box<StudioViewRequest>),
    Command(Box<StudioCommandRequest>),
}

impl StudioRequestEnvelope {
    pub fn view(studio_view_request: StudioViewRequest) -> Self {
        Self::View(Box::new(studio_view_request))
    }

    pub fn command(studio_command_request: StudioCommandRequest) -> Self {
        Self::Command(Box::new(studio_command_request))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StudioResponseEnvelope {
    View(Box<StudioViewResponse>),
    Command(Box<StudioCommandResponse>),
}

impl StudioResponseEnvelope {
    pub fn view(studio_view_response: StudioViewResponse) -> Self {
        Self::View(Box::new(studio_view_response))
    }

    pub fn command(studio_command_response: StudioCommandResponse) -> Self {
        Self::Command(Box::new(studio_command_response))
    }
}
