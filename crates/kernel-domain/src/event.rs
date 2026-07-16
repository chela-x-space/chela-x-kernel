use std::collections::HashSet;
use std::fmt;
use std::str::FromStr;

use crate::errors::{DomainError, DomainResult};
use crate::identifier::{AuditEvidenceId, CorrelationId, EventId, RuntimeId, WorkflowId};
use crate::request::TimeReference;

const EVENT_TYPE_EXPECTATION: &str =
    "lowercase dotted event type with an approved category and non-empty ASCII alphanumeric segments";

const EVENT_SUBJECT_TYPE_EXPECTATION: &str =
    "one of the approved lowercase ASCII event subject types";

const EVENT_SUBJECT_ID_EXPECTATION: &str =
    "non-empty ASCII subject reference using a-z, A-Z, 0-9, dot, underscore, or hyphen";

const EVENT_COMPONENT_EXPECTATION: &str =
    "lowercase ASCII component using a-z, 0-9, hyphen, dot, or underscore, without leading, trailing, or adjacent separators";

const EVENT_ACTOR_ID_EXPECTATION: &str =
    "non-empty ASCII actor reference using letters, digits, dot, underscore, or hyphen";

const EVENT_TRACE_REFERENCE_EXPECTATION: &str =
    "non-empty ASCII trace reference using letters, digits, dot, underscore, or hyphen";

const EVENT_VERSION_EXPECTATION: &str =
    "semantic event schema version in MAJOR.MINOR.PATCH format using ASCII digits without leading zeros";

const EVENT_CLASSIFICATION_EXPECTATION: &str =
    "one of PUBLIC, INTERNAL, CONFIDENTIAL, RESTRICTED, or CRITICAL";

const APPROVED_EVENT_CATEGORIES: &[&str] = &[
    "system",
    "runtime",
    "workflow",
    "task",
    "execution",
    "memory",
    "security",
    "audit",
    "api",
    "studio",
];

const APPROVED_EVENT_SUBJECT_TYPES: &[&str] = &[
    "enterprise",
    "workspace",
    "project",
    "organization-unit",
    "ownership",
    "human",
    "agent",
    "runtime",
    "workflow",
    "decision",
    "delegation",
    "policy",
    "api",
    "task",
    "execution",
    "memory",
];

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EventCausation {
    Root,
    CausedBy(EventId),
}

impl EventCausation {
    pub const fn root() -> Self {
        Self::Root
    }

    pub fn caused_by(current_event_id: &EventId, parent_event_id: EventId) -> DomainResult<Self> {
        if current_event_id == &parent_event_id {
            return Err(DomainError::InvalidEventReference(
                "an event cannot directly cause itself",
            ));
        }

        Ok(Self::CausedBy(parent_event_id))
    }

    pub const fn is_root(&self) -> bool {
        matches!(self, Self::Root)
    }

    pub fn parent_event_id(&self) -> Option<&EventId> {
        match self {
            Self::Root => None,
            Self::CausedBy(event_id) => Some(event_id),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventComponent(String);

impl EventComponent {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        let value = value.into().trim().to_owned();

        if value.is_empty() {
            return Err(DomainError::EmptyValue {
                field: "EventComponent",
            });
        }

        let mut previous_was_separator = false;

        for character in value.chars() {
            let is_separator = matches!(character, '-' | '.' | '_');
            let is_allowed =
                character.is_ascii_lowercase() || character.is_ascii_digit() || is_separator;

            if !is_allowed || (is_separator && previous_was_separator) {
                return Err(Self::invalid(value));
            }

            previous_was_separator = is_separator;
        }

        if value
            .chars()
            .next()
            .is_some_and(|character| matches!(character, '-' | '.' | '_'))
            || value
                .chars()
                .last()
                .is_some_and(|character| matches!(character, '-' | '.' | '_'))
        {
            return Err(Self::invalid(value));
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn invalid(value: String) -> DomainError {
        DomainError::InvalidIdentifier {
            kind: "EventComponent",
            value,
            expected: EVENT_COMPONENT_EXPECTATION,
        }
    }
}

impl fmt::Display for EventComponent {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for EventComponent {
    type Err = DomainError;

    fn from_str(value: &str) -> DomainResult<Self> {
        Self::new(value.to_owned())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventSource {
    component: EventComponent,
    runtime_id: Option<RuntimeId>,
}

impl EventSource {
    pub const fn new(component: EventComponent, runtime_id: Option<RuntimeId>) -> Self {
        Self {
            component,
            runtime_id,
        }
    }

    pub const fn component(&self) -> &EventComponent {
        &self.component
    }

    pub const fn runtime_id(&self) -> Option<&RuntimeId> {
        self.runtime_id.as_ref()
    }

    pub const fn has_runtime_id(&self) -> bool {
        self.runtime_id.is_some()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventSubjectType(String);

impl EventSubjectType {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        let value = value.into().trim().to_owned();

        if value.is_empty() {
            return Err(DomainError::EmptyValue {
                field: "EventSubjectType",
            });
        }

        if !value
            .chars()
            .all(|character| character.is_ascii_lowercase() || character == '-')
            || value.starts_with('-')
            || value.ends_with('-')
            || value.contains("--")
            || !APPROVED_EVENT_SUBJECT_TYPES.contains(&value.as_str())
        {
            return Err(Self::invalid(value));
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn invalid(value: String) -> DomainError {
        DomainError::InvalidIdentifier {
            kind: "EventSubjectType",
            value,
            expected: EVENT_SUBJECT_TYPE_EXPECTATION,
        }
    }
}

impl fmt::Display for EventSubjectType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for EventSubjectType {
    type Err = DomainError;

    fn from_str(value: &str) -> DomainResult<Self> {
        Self::new(value.to_owned())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventActorId(String);

impl EventActorId {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        let value = value.into().trim().to_owned();

        if value.is_empty() {
            return Err(DomainError::EmptyValue {
                field: "EventActorId",
            });
        }

        if !Self::is_namespace_safe(&value) {
            return Err(Self::invalid(value));
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn is_namespace_safe(value: &str) -> bool {
        value.chars().all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '.' | '_' | '-')
        })
    }

    fn invalid(value: String) -> DomainError {
        DomainError::InvalidIdentifier {
            kind: "EventActorId",
            value,
            expected: EVENT_ACTOR_ID_EXPECTATION,
        }
    }
}

impl fmt::Display for EventActorId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for EventActorId {
    type Err = DomainError;

    fn from_str(value: &str) -> DomainResult<Self> {
        Self::new(value.to_owned())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventTraceReference(String);

impl EventTraceReference {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        let value = value.into().trim().to_owned();

        if value.is_empty() {
            return Err(DomainError::EmptyValue {
                field: "EventTraceReference",
            });
        }

        if !Self::is_namespace_safe(&value) {
            return Err(Self::invalid(value));
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn is_namespace_safe(value: &str) -> bool {
        value.chars().all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '.' | '_' | '-')
        })
    }

    fn invalid(value: String) -> DomainError {
        DomainError::InvalidIdentifier {
            kind: "EventTraceReference",
            value,
            expected: EVENT_TRACE_REFERENCE_EXPECTATION,
        }
    }
}

impl fmt::Display for EventTraceReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for EventTraceReference {
    type Err = DomainError;

    fn from_str(value: &str) -> DomainResult<Self> {
        Self::new(value.to_owned())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventTrace {
    actor_id: Option<EventActorId>,
    workflow_id: Option<WorkflowId>,
    task_id: Option<EventTraceReference>,
    execution_id: Option<EventTraceReference>,
    evidence_ids: Vec<AuditEvidenceId>,
}

impl EventTrace {
    pub fn new(
        actor_id: Option<EventActorId>,
        workflow_id: Option<WorkflowId>,
        task_id: Option<EventTraceReference>,
        execution_id: Option<EventTraceReference>,
        evidence_ids: Vec<AuditEvidenceId>,
    ) -> DomainResult<Self> {
        if actor_id.is_none()
            && workflow_id.is_none()
            && task_id.is_none()
            && execution_id.is_none()
            && evidence_ids.is_empty()
        {
            return Err(DomainError::InvalidEventReference(
                "event trace must contain at least one trace reference or evidence",
            ));
        }

        let mut observed_evidence = HashSet::with_capacity(evidence_ids.len());

        if evidence_ids
            .iter()
            .any(|evidence_id| !observed_evidence.insert(evidence_id))
        {
            return Err(DomainError::InvalidEventReference(
                "event trace must not contain duplicate evidence identifiers",
            ));
        }

        Ok(Self {
            actor_id,
            workflow_id,
            task_id,
            execution_id,
            evidence_ids,
        })
    }

    pub const fn actor_id(&self) -> Option<&EventActorId> {
        self.actor_id.as_ref()
    }

    pub const fn workflow_id(&self) -> Option<&WorkflowId> {
        self.workflow_id.as_ref()
    }

    pub const fn task_id(&self) -> Option<&EventTraceReference> {
        self.task_id.as_ref()
    }

    pub const fn execution_id(&self) -> Option<&EventTraceReference> {
        self.execution_id.as_ref()
    }

    pub fn evidence_ids(&self) -> &[AuditEvidenceId] {
        &self.evidence_ids
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventEnvelopeCandidate<P> {
    pub event_id: Option<EventId>,
    pub event_type: Option<EventType>,
    pub event_version: Option<EventVersion>,
    pub occurred_at: Option<TimeReference>,
    pub recorded_at: Option<TimeReference>,
    pub source: Option<EventSource>,
    pub subject: Option<EventSubject>,
    pub payload: Option<P>,
    pub classification: Option<EventClassification>,
    pub trace: Option<EventTrace>,
    pub correlation_id: Option<CorrelationId>,
    pub causation: EventCausation,
}

pub fn validate_event_envelope<P>(
    candidate: EventEnvelopeCandidate<P>,
) -> DomainResult<EventEnvelope<P>> {
    let EventEnvelopeCandidate {
        event_id,
        event_type,
        event_version,
        occurred_at,
        recorded_at,
        source,
        subject,
        payload,
        classification,
        trace,
        correlation_id,
        causation,
    } = candidate;

    let event_id = event_id.ok_or(DomainError::MissingEventField("event_id"))?;
    let event_type = event_type.ok_or(DomainError::MissingEventField("event_type"))?;
    let event_version = event_version.ok_or(DomainError::MissingEventField("event_version"))?;
    let occurred_at = occurred_at.ok_or(DomainError::MissingEventField("occurred_at"))?;
    let recorded_at = recorded_at.ok_or(DomainError::MissingEventField("recorded_at"))?;
    let source = source.ok_or(DomainError::MissingEventField("source"))?;
    let subject = subject.ok_or(DomainError::MissingEventField("subject"))?;
    let payload = payload.ok_or(DomainError::MissingEventField("payload"))?;
    let classification = classification.ok_or(DomainError::MissingEventField("classification"))?;
    let trace = trace.ok_or(DomainError::MissingEventField("trace"))?;

    Ok(EventEnvelope::new(
        event_id,
        event_type,
        event_version,
        occurred_at,
        recorded_at,
        source,
        subject,
        payload,
        classification,
        trace,
        correlation_id,
        causation,
    ))
}

pub fn validate_event_identity<P>(envelope: &EventEnvelope<P>) -> DomainResult<()> {
    match envelope.causation() {
        EventCausation::Root => Ok(()),
        EventCausation::CausedBy(parent_event_id) if parent_event_id != envelope.event_id() => {
            Ok(())
        }
        EventCausation::CausedBy(_) => Err(DomainError::InvalidEventReference(
            "causation_id must not equal event_id",
        )),
    }
}

pub fn validate_event_version<P>(envelope: &EventEnvelope<P>) -> DomainResult<()> {
    if envelope.event_version().major() == 1 {
        return Ok(());
    }

    Err(DomainError::InvalidEventReference(
        "event_version is not supported",
    ))
}

pub fn validate_event_timestamps<P>(envelope: &EventEnvelope<P>) -> DomainResult<()> {
    let occurred_at = parse_event_timestamp(envelope.occurred_at(), "occurred_at")?;
    let recorded_at = parse_event_timestamp(envelope.recorded_at(), "recorded_at")?;

    if recorded_at < occurred_at {
        return Err(DomainError::InvalidEventTimestamp(
            "recorded_at must not precede occurred_at",
        ));
    }

    Ok(())
}

pub fn validate_event_payload<P, F>(envelope: &EventEnvelope<P>, validator: F) -> DomainResult<()>
where
    F: FnOnce(&P) -> DomainResult<()>,
{
    validator(envelope.payload())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct CanonicalEventTimestamp {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
}

fn parse_event_timestamp(
    timestamp: &TimeReference,
    field: &'static str,
) -> DomainResult<CanonicalEventTimestamp> {
    let value = timestamp.as_str();
    let bytes = value.as_bytes();

    if bytes.len() != 20
        || bytes[4] != b'-'
        || bytes[7] != b'-'
        || bytes[10] != b'T'
        || bytes[13] != b':'
        || bytes[16] != b':'
        || bytes[19] != b'Z'
    {
        return Err(DomainError::InvalidEventTimestamp(field));
    }

    for index in [0_usize, 1, 2, 3, 5, 6, 8, 9, 11, 12, 14, 15, 17, 18] {
        if !bytes[index].is_ascii_digit() {
            return Err(DomainError::InvalidEventTimestamp(field));
        }
    }

    let year =
        parse_four_digit_number(bytes, 0).ok_or(DomainError::InvalidEventTimestamp(field))?;
    let month =
        parse_two_digit_number(bytes, 5).ok_or(DomainError::InvalidEventTimestamp(field))?;
    let day = parse_two_digit_number(bytes, 8).ok_or(DomainError::InvalidEventTimestamp(field))?;
    let hour =
        parse_two_digit_number(bytes, 11).ok_or(DomainError::InvalidEventTimestamp(field))?;
    let minute =
        parse_two_digit_number(bytes, 14).ok_or(DomainError::InvalidEventTimestamp(field))?;
    let second =
        parse_two_digit_number(bytes, 17).ok_or(DomainError::InvalidEventTimestamp(field))?;

    if !(1..=12).contains(&month) || hour > 23 || minute > 59 || second > 59 {
        return Err(DomainError::InvalidEventTimestamp(field));
    }

    let max_day = days_in_month(year, month);
    if day == 0 || day > max_day {
        return Err(DomainError::InvalidEventTimestamp(field));
    }

    Ok(CanonicalEventTimestamp {
        year,
        month,
        day,
        hour,
        minute,
        second,
    })
}

fn parse_two_digit_number(bytes: &[u8], start: usize) -> Option<u8> {
    Some((bytes[start] - b'0') * 10 + (bytes[start + 1] - b'0'))
}

fn parse_four_digit_number(bytes: &[u8], start: usize) -> Option<u16> {
    Some(
        u16::from(bytes[start] - b'0') * 1000
            + u16::from(bytes[start + 1] - b'0') * 100
            + u16::from(bytes[start + 2] - b'0') * 10
            + u16::from(bytes[start + 3] - b'0'),
    )
}

fn days_in_month(year: u16, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 0,
    }
}

fn is_leap_year(year: u16) -> bool {
    year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventEnvelope<P> {
    event_id: EventId,
    event_type: EventType,
    event_version: EventVersion,
    occurred_at: TimeReference,
    recorded_at: TimeReference,
    source: EventSource,
    subject: EventSubject,
    payload: P,
    classification: EventClassification,
    trace: EventTrace,
    correlation_id: Option<CorrelationId>,
    causation: EventCausation,
}

impl<P> EventEnvelope<P> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        event_id: EventId,
        event_type: EventType,
        event_version: EventVersion,
        occurred_at: TimeReference,
        recorded_at: TimeReference,
        source: EventSource,
        subject: EventSubject,
        payload: P,
        classification: EventClassification,
        trace: EventTrace,
        correlation_id: Option<CorrelationId>,
        causation: EventCausation,
    ) -> Self {
        Self {
            event_id,
            event_type,
            event_version,
            occurred_at,
            recorded_at,
            source,
            subject,
            payload,
            classification,
            trace,
            correlation_id,
            causation,
        }
    }

    pub const fn event_id(&self) -> &EventId {
        &self.event_id
    }

    pub const fn event_type(&self) -> &EventType {
        &self.event_type
    }

    pub const fn event_version(&self) -> &EventVersion {
        &self.event_version
    }

    pub const fn occurred_at(&self) -> &TimeReference {
        &self.occurred_at
    }

    pub const fn recorded_at(&self) -> &TimeReference {
        &self.recorded_at
    }

    pub const fn source(&self) -> &EventSource {
        &self.source
    }

    pub const fn subject(&self) -> &EventSubject {
        &self.subject
    }

    pub const fn payload(&self) -> &P {
        &self.payload
    }

    pub const fn classification(&self) -> &EventClassification {
        &self.classification
    }

    pub const fn trace(&self) -> &EventTrace {
        &self.trace
    }

    pub const fn correlation_id(&self) -> Option<&CorrelationId> {
        self.correlation_id.as_ref()
    }

    pub const fn causation(&self) -> &EventCausation {
        &self.causation
    }

    pub fn into_payload(self) -> P {
        self.payload
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventSubjectId(String);

impl EventSubjectId {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        let value = value.into().trim().to_owned();

        if value.is_empty() {
            return Err(DomainError::EmptyValue {
                field: "EventSubjectId",
            });
        }

        if !value.chars().all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '.' | '_' | '-')
        }) {
            return Err(Self::invalid(value));
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn invalid(value: String) -> DomainError {
        DomainError::InvalidIdentifier {
            kind: "EventSubjectId",
            value,
            expected: EVENT_SUBJECT_ID_EXPECTATION,
        }
    }
}

impl fmt::Display for EventSubjectId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for EventSubjectId {
    type Err = DomainError;

    fn from_str(value: &str) -> DomainResult<Self> {
        Self::new(value.to_owned())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventSubject {
    subject_type: EventSubjectType,
    subject_id: EventSubjectId,
}

impl EventSubject {
    pub const fn new(subject_type: EventSubjectType, subject_id: EventSubjectId) -> Self {
        Self {
            subject_type,
            subject_id,
        }
    }

    pub const fn subject_type(&self) -> &EventSubjectType {
        &self.subject_type
    }

    pub const fn subject_id(&self) -> &EventSubjectId {
        &self.subject_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventType(String);

impl EventType {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        let value = value.into().trim().to_owned();

        if value.is_empty() {
            return Err(DomainError::EmptyValue { field: "EventType" });
        }

        if value.starts_with('.') || value.ends_with('.') || value.contains("..") {
            return Err(Self::invalid(value));
        }

        let segments: Vec<&str> = value.split('.').collect();

        if segments.len() < 2 {
            return Err(Self::invalid(value));
        }

        if segments.iter().any(|segment| {
            segment.is_empty()
                || !segment
                    .chars()
                    .all(|character| character.is_ascii_lowercase() || character.is_ascii_digit())
        }) {
            return Err(Self::invalid(value));
        }

        let category = segments
            .first()
            .copied()
            .expect("validated event type contains at least two segments");

        if !APPROVED_EVENT_CATEGORIES.contains(&category) {
            return Err(Self::invalid(value));
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn category(&self) -> &str {
        self.0
            .split('.')
            .next()
            .expect("validated event type always contains a category")
    }

    fn invalid(value: String) -> DomainError {
        DomainError::InvalidIdentifier {
            kind: "EventType",
            value,
            expected: EVENT_TYPE_EXPECTATION,
        }
    }
}

impl fmt::Display for EventType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for EventType {
    type Err = DomainError;

    fn from_str(value: &str) -> DomainResult<Self> {
        Self::new(value.to_owned())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventVersion(String);

impl EventVersion {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        let value = value.into().trim().to_owned();

        if value.is_empty() {
            return Err(DomainError::EmptyValue {
                field: "EventVersion",
            });
        }

        let segments: Vec<&str> = value.split('.').collect();

        if segments.len() != 3
            || segments
                .iter()
                .any(|segment| !Self::valid_numeric_segment(segment))
        {
            return Err(Self::invalid(value));
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn major(&self) -> u64 {
        self.segment(0)
    }

    pub fn minor(&self) -> u64 {
        self.segment(1)
    }

    pub fn patch(&self) -> u64 {
        self.segment(2)
    }

    fn valid_numeric_segment(segment: &str) -> bool {
        !segment.is_empty()
            && segment.chars().all(|character| character.is_ascii_digit())
            && (segment == "0" || !segment.starts_with('0'))
            && segment.parse::<u64>().is_ok()
    }

    fn segment(&self, index: usize) -> u64 {
        self.0
            .split('.')
            .nth(index)
            .expect("validated event version contains exactly three segments")
            .parse::<u64>()
            .expect("validated event version segments fit within u64")
    }

    fn invalid(value: String) -> DomainError {
        DomainError::InvalidIdentifier {
            kind: "EventVersion",
            value,
            expected: EVENT_VERSION_EXPECTATION,
        }
    }
}

impl fmt::Display for EventVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for EventVersion {
    type Err = DomainError;

    fn from_str(value: &str) -> DomainResult<Self> {
        Self::new(value.to_owned())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EventClassification {
    Public,
    Internal,
    Confidential,
    Restricted,
    Critical,
}

impl EventClassification {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Public => "PUBLIC",
            Self::Internal => "INTERNAL",
            Self::Confidential => "CONFIDENTIAL",
            Self::Restricted => "RESTRICTED",
            Self::Critical => "CRITICAL",
        }
    }

    pub fn new(value: impl AsRef<str>) -> DomainResult<Self> {
        let value = value.as_ref().trim();

        if value.is_empty() {
            return Err(DomainError::EmptyValue {
                field: "EventClassification",
            });
        }

        match value {
            "PUBLIC" => Ok(Self::Public),
            "INTERNAL" => Ok(Self::Internal),
            "CONFIDENTIAL" => Ok(Self::Confidential),
            "RESTRICTED" => Ok(Self::Restricted),
            "CRITICAL" => Ok(Self::Critical),
            _ => Err(DomainError::InvalidIdentifier {
                kind: "EventClassification",
                value: value.to_owned(),
                expected: EVENT_CLASSIFICATION_EXPECTATION,
            }),
        }
    }
}

impl fmt::Display for EventClassification {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for EventClassification {
    type Err = DomainError;

    fn from_str(value: &str) -> DomainResult<Self> {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::str::FromStr;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::{
        validate_event_envelope, validate_event_identity, validate_event_payload,
        validate_event_timestamps, validate_event_version, EventActorId, EventCausation,
        EventClassification, EventComponent, EventEnvelope, EventEnvelopeCandidate, EventSource,
        EventSubject, EventSubjectId, EventSubjectType, EventTrace, EventTraceReference, EventType,
        EventVersion,
    };
    use crate::errors::{DomainError, DomainResult};
    use crate::identifier::{AuditEvidenceId, CorrelationId, EventId, RuntimeId, WorkflowId};
    use crate::request::TimeReference;

    #[test]
    fn event_type_accepts_canonical_dotted_type_traceability_k5_2() {
        let event_type =
            EventType::new("runtime.health.assessed").expect("valid canonical event type");

        assert_eq!(event_type.as_str(), "runtime.health.assessed");
        assert_eq!(event_type.category(), "runtime");
    }

    #[test]
    fn event_type_accepts_two_segment_type_traceability_k5_2() {
        let event_type = EventType::new("workflow.started").expect("valid workflow event type");

        assert_eq!(event_type.as_str(), "workflow.started");
        assert_eq!(event_type.category(), "workflow");
    }

    #[test]
    fn event_type_display_and_parsing_are_stable_traceability_k5_2() {
        let event_type = EventType::from_str("task.completed").expect("valid parsed event type");

        assert_eq!(event_type.to_string(), "task.completed");
    }

    #[test]
    fn event_type_hash_is_stable_for_equal_values_traceability_k5_2() {
        let left = EventType::new("execution.failed").expect("left event type");
        let right = EventType::new("execution.failed").expect("right event type");

        let mut left_hasher = DefaultHasher::new();
        let mut right_hasher = DefaultHasher::new();

        left.hash(&mut left_hasher);
        right.hash(&mut right_hasher);

        assert_eq!(left, right);
        assert_eq!(left_hasher.finish(), right_hasher.finish());
    }

    #[test]
    fn event_type_rejects_empty_value_traceability_k5_2() {
        let error = EventType::new("").expect_err("empty event type must fail");

        assert_eq!(error.to_string(), "empty value: EventType");
    }

    #[test]
    fn event_type_rejects_uppercase_characters_traceability_k5_2() {
        let error =
            EventType::new("Runtime.Health.Assessed").expect_err("uppercase event type must fail");

        assert!(error.to_string().contains("invalid EventType identifier"));
    }

    #[test]
    fn event_type_rejects_whitespace_traceability_k5_2() {
        let error =
            EventType::new("runtime health assessed").expect_err("spaces must not be accepted");

        assert!(error.to_string().contains("invalid EventType identifier"));
    }

    #[test]
    fn event_type_rejects_empty_segment_traceability_k5_2() {
        let error =
            EventType::new("runtime..assessed").expect_err("empty event type segment must fail");

        assert!(error.to_string().contains("invalid EventType identifier"));
    }

    #[test]
    fn event_type_rejects_leading_separator_traceability_k5_2() {
        let error =
            EventType::new(".runtime.started").expect_err("leading separator must not be accepted");

        assert!(error.to_string().contains("invalid EventType identifier"));
    }

    #[test]
    fn event_type_rejects_trailing_separator_traceability_k5_2() {
        let error = EventType::new("runtime.started.")
            .expect_err("trailing separator must not be accepted");

        assert!(error.to_string().contains("invalid EventType identifier"));
    }

    #[test]
    fn event_type_rejects_single_segment_traceability_k5_2() {
        let error = EventType::new("runtime").expect_err("event type requires an action segment");

        assert!(error.to_string().contains("invalid EventType identifier"));
    }

    #[test]
    fn event_type_rejects_unknown_category_traceability_k5_2() {
        let error = EventType::new("database.connected")
            .expect_err("unknown category must not be accepted");

        assert!(error.to_string().contains("invalid EventType identifier"));
    }

    #[test]
    fn event_version_accepts_canonical_semantic_version_traceability_k5_3() {
        let version = EventVersion::new("1.0.0").expect("valid event version");

        assert_eq!(version.as_str(), "1.0.0");
        assert_eq!(version.major(), 1);
        assert_eq!(version.minor(), 0);
        assert_eq!(version.patch(), 0);
    }

    #[test]
    fn event_version_display_and_parsing_are_stable_traceability_k5_3() {
        let version = EventVersion::from_str("2.10.7").expect("valid parsed event version");

        assert_eq!(version.to_string(), "2.10.7");
        assert_eq!(version.major(), 2);
        assert_eq!(version.minor(), 10);
        assert_eq!(version.patch(), 7);
    }

    #[test]
    fn event_version_hash_is_stable_for_equal_values_traceability_k5_3() {
        let left = EventVersion::new("1.2.3").expect("left version");
        let right = EventVersion::new("1.2.3").expect("right version");

        let mut left_hasher = DefaultHasher::new();
        let mut right_hasher = DefaultHasher::new();

        left.hash(&mut left_hasher);
        right.hash(&mut right_hasher);

        assert_eq!(left, right);
        assert_eq!(left_hasher.finish(), right_hasher.finish());
    }

    #[test]
    fn event_version_rejects_empty_value_traceability_k5_3() {
        let error = EventVersion::new("").expect_err("empty event version must fail");

        assert_eq!(error.to_string(), "empty value: EventVersion");
    }

    #[test]
    fn event_version_rejects_missing_patch_segment_traceability_k5_3() {
        let error = EventVersion::new("1.0").expect_err("three version segments are required");

        assert!(error
            .to_string()
            .contains("invalid EventVersion identifier"));
    }

    #[test]
    fn event_version_rejects_extra_segment_traceability_k5_3() {
        let error =
            EventVersion::new("1.0.0.1").expect_err("extra version segment must be rejected");

        assert!(error
            .to_string()
            .contains("invalid EventVersion identifier"));
    }

    #[test]
    fn event_version_rejects_non_numeric_segment_traceability_k5_3() {
        let error =
            EventVersion::new("1.x.0").expect_err("version segments must contain only digits");

        assert!(error
            .to_string()
            .contains("invalid EventVersion identifier"));
    }

    #[test]
    fn event_version_rejects_leading_zero_traceability_k5_3() {
        let error = EventVersion::new("01.0.0").expect_err("leading zeros must not be accepted");

        assert!(error
            .to_string()
            .contains("invalid EventVersion identifier"));
    }

    #[test]
    fn event_version_rejects_prerelease_suffix_traceability_k5_3() {
        let error =
            EventVersion::new("1.0.0-rc1").expect_err("prerelease versions are not canonical");

        assert!(error
            .to_string()
            .contains("invalid EventVersion identifier"));
    }

    #[test]
    fn event_classification_accepts_all_canonical_values_traceability_k5_4() {
        let cases = [
            ("PUBLIC", EventClassification::Public),
            ("INTERNAL", EventClassification::Internal),
            ("CONFIDENTIAL", EventClassification::Confidential),
            ("RESTRICTED", EventClassification::Restricted),
            ("CRITICAL", EventClassification::Critical),
        ];

        for (value, expected) in cases {
            let classification =
                EventClassification::new(value).expect("valid event classification");

            assert_eq!(classification, expected);
            assert_eq!(classification.as_str(), value);
        }
    }

    #[test]
    fn event_classification_display_and_parsing_are_stable_traceability_k5_4() {
        let classification =
            EventClassification::from_str("CONFIDENTIAL").expect("valid classification");

        assert_eq!(classification, EventClassification::Confidential);
        assert_eq!(classification.to_string(), "CONFIDENTIAL");
    }

    #[test]
    fn event_classification_order_is_stable_traceability_k5_4() {
        assert!(EventClassification::Public < EventClassification::Internal);
        assert!(EventClassification::Internal < EventClassification::Confidential);
        assert!(EventClassification::Confidential < EventClassification::Restricted);
        assert!(EventClassification::Restricted < EventClassification::Critical);
    }

    #[test]
    fn event_classification_rejects_empty_value_traceability_k5_4() {
        let error = EventClassification::new("").expect_err("empty event classification must fail");

        assert_eq!(error.to_string(), "empty value: EventClassification");
    }

    #[test]
    fn event_classification_rejects_lowercase_value_traceability_k5_4() {
        let error =
            EventClassification::new("internal").expect_err("lowercase value must be rejected");

        assert!(error
            .to_string()
            .contains("invalid EventClassification identifier"));
    }

    #[test]
    fn event_classification_rejects_unknown_value_traceability_k5_4() {
        let error =
            EventClassification::new("SECRET").expect_err("unknown classification must fail");

        assert!(error
            .to_string()
            .contains("invalid EventClassification identifier"));
    }
    #[test]
    fn event_causation_represents_root_event_traceability_k5_5() {
        let causation = EventCausation::root();

        assert!(causation.is_root());
        assert_eq!(causation.parent_event_id(), None);
    }

    #[test]
    fn event_causation_references_parent_event_id_traceability_k5_5() {
        let current = EventId::new("CX-EVT-000002").expect("current event id");
        let parent = EventId::new("CX-EVT-000001").expect("parent event id");

        let causation =
            EventCausation::caused_by(&current, parent).expect("valid causation reference");

        assert!(!causation.is_root());
        assert_eq!(
            causation
                .parent_event_id()
                .expect("caused event has parent")
                .as_str(),
            "CX-EVT-000001"
        );
    }

    #[test]
    fn event_causation_rejects_self_causation_traceability_k5_5() {
        let current = EventId::new("CX-EVT-000001").expect("current event id");
        let parent = EventId::new("CX-EVT-000001").expect("same parent event id");

        let error = EventCausation::caused_by(&current, parent)
            .expect_err("event must not directly cause itself");

        assert_eq!(
            error.to_string(),
            "invalid event reference: an event cannot directly cause itself"
        );
    }

    #[test]
    fn event_causation_equality_is_stable_traceability_k5_5() {
        let current = EventId::new("CX-EVT-000010").expect("current event id");

        let left = EventCausation::caused_by(
            &current,
            EventId::new("CX-EVT-000009").expect("left parent"),
        )
        .expect("left causation");

        let right = EventCausation::caused_by(
            &current,
            EventId::new("CX-EVT-000009").expect("right parent"),
        )
        .expect("right causation");

        assert_eq!(left, right);
    }

    #[test]
    fn event_component_accepts_canonical_values() {
        for value in [
            "kernel-runtime",
            "workflow-engine",
            "api-gateway",
            "studio-api",
            "external.news-api",
            "human-command",
            "component_01",
        ] {
            let component =
                EventComponent::new(value).expect("canonical event component must be valid");

            assert_eq!(component.as_str(), value);
            assert_eq!(component.to_string(), value);
        }
    }

    #[test]
    fn event_component_trims_outer_whitespace() {
        let component =
            EventComponent::new("  kernel-runtime  ").expect("outer whitespace must be trimmed");

        assert_eq!(component.as_str(), "kernel-runtime");
    }

    #[test]
    fn event_component_rejects_empty_value() {
        let error = EventComponent::new("   ").expect_err("empty event component must be rejected");

        assert_eq!(error.to_string(), "empty value: EventComponent");
    }

    #[test]
    fn event_component_rejects_internal_whitespace() {
        let error = EventComponent::new("kernel runtime")
            .expect_err("internal whitespace must be rejected");

        assert!(error
            .to_string()
            .contains("invalid EventComponent identifier"));
    }

    #[test]
    fn event_component_rejects_uppercase_characters() {
        let error = EventComponent::new("Kernel-Runtime")
            .expect_err("uppercase characters must be rejected");

        assert!(error
            .to_string()
            .contains("invalid EventComponent identifier"));
    }

    #[test]
    fn event_component_rejects_leading_separator() {
        for value in ["-kernel", ".kernel", "_kernel"] {
            EventComponent::new(value).expect_err("leading separator must be rejected");
        }
    }

    #[test]
    fn event_component_rejects_trailing_separator() {
        for value in ["kernel-", "kernel.", "kernel_"] {
            EventComponent::new(value).expect_err("trailing separator must be rejected");
        }
    }

    #[test]
    fn event_component_rejects_adjacent_separators() {
        for value in [
            "kernel--runtime",
            "kernel..runtime",
            "kernel__runtime",
            "kernel-.runtime",
            "kernel._runtime",
            "kernel_-runtime",
        ] {
            EventComponent::new(value).expect_err("adjacent separators must be rejected");
        }
    }

    #[test]
    fn event_component_supports_from_str() {
        let component: EventComponent = "external.news-api"
            .parse()
            .expect("canonical component must parse");

        assert_eq!(component.as_str(), "external.news-api");
    }

    #[test]
    fn event_source_supports_runtime_origin() {
        let component = EventComponent::new("kernel-runtime").expect("valid event component");
        let runtime_id =
            RuntimeId::new("kernel.runtime.primary").expect("valid runtime identifier");

        let source = EventSource::new(component.clone(), Some(runtime_id.clone()));

        assert_eq!(source.component(), &component);
        assert_eq!(source.runtime_id(), Some(&runtime_id));
        assert!(source.has_runtime_id());
    }

    #[test]
    fn event_source_supports_non_runtime_origin() {
        let component = EventComponent::new("external.news-api").expect("valid event component");

        let source = EventSource::new(component.clone(), None);

        assert_eq!(source.component(), &component);
        assert_eq!(source.runtime_id(), None);
        assert!(!source.has_runtime_id());
    }

    #[test]
    fn event_source_preserves_value_semantics() {
        let left = EventSource::new(
            EventComponent::new("api-gateway").expect("left component"),
            None,
        );
        let right = EventSource::new(
            EventComponent::new("api-gateway").expect("right component"),
            None,
        );

        assert_eq!(left, right);
        assert_eq!(left.clone(), left);
    }

    #[test]
    fn event_subject_type_accepts_approved_values() {
        for value in [
            "enterprise",
            "workspace",
            "project",
            "organization-unit",
            "ownership",
            "human",
            "agent",
            "runtime",
            "workflow",
            "decision",
            "delegation",
            "policy",
            "api",
            "task",
            "execution",
            "memory",
        ] {
            let subject_type =
                EventSubjectType::new(value).expect("approved subject type must be valid");

            assert_eq!(subject_type.as_str(), value);
            assert_eq!(subject_type.to_string(), value);
        }
    }

    #[test]
    fn event_subject_type_trims_outer_whitespace() {
        let subject_type =
            EventSubjectType::new("  runtime  ").expect("outer whitespace must be trimmed");

        assert_eq!(subject_type.as_str(), "runtime");
    }

    #[test]
    fn event_subject_type_rejects_empty_value() {
        let error = EventSubjectType::new("   ").expect_err("empty subject type must be rejected");

        assert_eq!(error.to_string(), "empty value: EventSubjectType");
    }

    #[test]
    fn event_subject_type_rejects_unapproved_value() {
        let error = EventSubjectType::new("unknown-subject")
            .expect_err("unapproved subject type must be rejected");

        assert!(error
            .to_string()
            .contains("invalid EventSubjectType identifier"));
    }

    #[test]
    fn event_subject_type_rejects_non_canonical_values() {
        for value in [
            "Runtime",
            "runtime_type",
            "runtime.type",
            "runtime type",
            "-runtime",
            "runtime-",
            "runtime--agent",
        ] {
            EventSubjectType::new(value).expect_err("non-canonical subject type must be rejected");
        }
    }

    #[test]
    fn event_subject_type_supports_from_str() {
        let subject_type: EventSubjectType = "organization-unit"
            .parse()
            .expect("subject type must parse");

        assert_eq!(subject_type.as_str(), "organization-unit");
    }

    #[test]
    fn event_subject_id_accepts_namespace_safe_values() {
        for value in [
            "runtime.primary",
            "CX-AGT-000001",
            "external_subject_01",
            "workflow.execution-01",
        ] {
            let subject_id =
                EventSubjectId::new(value).expect("namespace-safe subject id must be valid");

            assert_eq!(subject_id.as_str(), value);
            assert_eq!(subject_id.to_string(), value);
        }
    }

    #[test]
    fn event_subject_id_trims_outer_whitespace() {
        let subject_id =
            EventSubjectId::new("  runtime.primary  ").expect("outer whitespace must be trimmed");

        assert_eq!(subject_id.as_str(), "runtime.primary");
    }

    #[test]
    fn event_subject_id_rejects_empty_value() {
        let error = EventSubjectId::new("   ").expect_err("empty subject id must be rejected");

        assert_eq!(error.to_string(), "empty value: EventSubjectId");
    }

    #[test]
    fn event_subject_id_rejects_internal_whitespace() {
        EventSubjectId::new("runtime primary").expect_err("internal whitespace must be rejected");
    }

    #[test]
    fn event_subject_id_rejects_unsafe_characters() {
        for value in [
            "runtime/primary",
            "runtime:primary",
            "runtime@primary",
            "runtime#primary",
        ] {
            EventSubjectId::new(value).expect_err("unsafe subject identifier must be rejected");
        }
    }

    #[test]
    fn event_subject_id_supports_from_str() {
        let subject_id: EventSubjectId = "runtime.primary".parse().expect("subject id must parse");

        assert_eq!(subject_id.as_str(), "runtime.primary");
    }

    #[test]
    fn event_subject_preserves_canonical_reference() {
        let subject_type = EventSubjectType::new("runtime").expect("valid subject type");
        let subject_id = EventSubjectId::new("runtime.primary").expect("valid subject id");

        let subject = EventSubject::new(subject_type.clone(), subject_id.clone());

        assert_eq!(subject.subject_type(), &subject_type);
        assert_eq!(subject.subject_id(), &subject_id);
    }

    #[test]
    fn event_subject_preserves_value_semantics() {
        let left = EventSubject::new(
            EventSubjectType::new("agent").expect("left subject type"),
            EventSubjectId::new("CX-AGT-000001").expect("left subject id"),
        );
        let right = EventSubject::new(
            EventSubjectType::new("agent").expect("right subject type"),
            EventSubjectId::new("CX-AGT-000001").expect("right subject id"),
        );

        assert_eq!(left, right);
        assert_eq!(left.clone(), left);
    }

    #[test]
    fn event_actor_id_accepts_namespace_safe_values() {
        for value in [
            "CX-AGT-000001",
            "CX-EMP-000001",
            "system.scheduler",
            "external_principal-01",
        ] {
            let actor_id = EventActorId::new(value).expect("valid actor reference");

            assert_eq!(actor_id.as_str(), value);
            assert_eq!(actor_id.to_string(), value);
        }
    }

    #[test]
    fn event_actor_id_trims_outer_whitespace() {
        let actor_id =
            EventActorId::new("  system.scheduler  ").expect("outer whitespace must be trimmed");

        assert_eq!(actor_id.as_str(), "system.scheduler");
    }

    #[test]
    fn event_actor_id_rejects_empty_and_unsafe_values() {
        assert_eq!(
            EventActorId::new("   ")
                .expect_err("empty actor id must fail")
                .to_string(),
            "empty value: EventActorId"
        );

        for value in [
            "system actor",
            "actor/system",
            "actor:system",
            "actor@system",
        ] {
            EventActorId::new(value).expect_err("unsafe actor reference must fail");
        }
    }

    #[test]
    fn event_actor_id_supports_from_str() {
        let actor_id: EventActorId = "system.scheduler".parse().expect("actor id must parse");

        assert_eq!(actor_id.as_str(), "system.scheduler");
    }

    #[test]
    fn event_trace_reference_accepts_namespace_safe_values() {
        for value in [
            "task.primary",
            "execution_000001",
            "CX-TASK-000001",
            "execution-runtime-01",
        ] {
            let reference = EventTraceReference::new(value).expect("valid trace reference");

            assert_eq!(reference.as_str(), value);
            assert_eq!(reference.to_string(), value);
        }
    }

    #[test]
    fn event_trace_reference_trims_outer_whitespace() {
        let reference = EventTraceReference::new("  execution.primary  ")
            .expect("outer whitespace must be trimmed");

        assert_eq!(reference.as_str(), "execution.primary");
    }

    #[test]
    fn event_trace_reference_rejects_empty_and_unsafe_values() {
        assert_eq!(
            EventTraceReference::new("   ")
                .expect_err("empty trace reference must fail")
                .to_string(),
            "empty value: EventTraceReference"
        );

        for value in [
            "execution primary",
            "execution/primary",
            "execution:primary",
            "execution@primary",
        ] {
            EventTraceReference::new(value).expect_err("unsafe trace reference must fail");
        }
    }

    #[test]
    fn event_trace_reference_supports_from_str() {
        let reference: EventTraceReference =
            "execution.primary".parse().expect("reference must parse");

        assert_eq!(reference.as_str(), "execution.primary");
    }

    #[test]
    fn event_trace_accepts_actor_only() {
        let actor_id = EventActorId::new("system.scheduler").expect("valid actor id");

        let trace = EventTrace::new(Some(actor_id.clone()), None, None, None, vec![])
            .expect("actor-only trace must be valid");

        assert_eq!(trace.actor_id(), Some(&actor_id));
        assert_eq!(trace.workflow_id(), None);
        assert_eq!(trace.task_id(), None);
        assert_eq!(trace.execution_id(), None);
        assert!(trace.evidence_ids().is_empty());
    }

    #[test]
    fn event_trace_accepts_execution_and_evidence_without_actor() {
        let execution_id =
            EventTraceReference::new("execution.primary").expect("valid execution reference");
        let evidence_id = AuditEvidenceId::new("CX-AUD-000001").expect("valid evidence id");

        let trace = EventTrace::new(
            None,
            None,
            None,
            Some(execution_id.clone()),
            vec![evidence_id.clone()],
        )
        .expect("system trace without actor must be valid");

        assert_eq!(trace.actor_id(), None);
        assert_eq!(trace.execution_id(), Some(&execution_id));
        assert_eq!(trace.evidence_ids(), &[evidence_id]);
    }

    #[test]
    fn event_trace_accepts_workflow_and_task_references() {
        let workflow_id = WorkflowId::new("CX-WF-000001").expect("valid workflow id");
        let task_id = EventTraceReference::new("task.primary").expect("valid task reference");

        let trace = EventTrace::new(
            None,
            Some(workflow_id.clone()),
            Some(task_id.clone()),
            None,
            vec![],
        )
        .expect("workflow and task trace must be valid");

        assert_eq!(trace.workflow_id(), Some(&workflow_id));
        assert_eq!(trace.task_id(), Some(&task_id));
    }

    #[test]
    fn event_trace_rejects_completely_empty_trace() {
        let error =
            EventTrace::new(None, None, None, None, vec![]).expect_err("empty trace must fail");

        assert_eq!(
            error.to_string(),
            "invalid event reference: event trace must contain at least one trace reference or evidence"
        );
    }

    #[test]
    fn event_trace_rejects_duplicate_evidence() {
        let evidence_id = AuditEvidenceId::new("CX-AUD-000001").expect("valid evidence id");

        let error = EventTrace::new(
            None,
            None,
            None,
            None,
            vec![evidence_id.clone(), evidence_id],
        )
        .expect_err("duplicate evidence must fail");

        assert_eq!(
            error.to_string(),
            "invalid event reference: event trace must not contain duplicate evidence identifiers"
        );
    }

    #[test]
    fn event_trace_preserves_evidence_order() {
        let first = AuditEvidenceId::new("CX-AUD-000001").expect("first evidence id");
        let second = AuditEvidenceId::new("CX-AUD-000002").expect("second evidence id");
        let third = AuditEvidenceId::new("CX-AUD-000003").expect("third evidence id");

        let trace = EventTrace::new(
            None,
            None,
            None,
            None,
            vec![first.clone(), second.clone(), third.clone()],
        )
        .expect("evidence-only trace must be valid");

        assert_eq!(trace.evidence_ids(), &[first, second, third]);
    }

    #[test]
    fn event_trace_preserves_value_semantics() {
        let left = EventTrace::new(
            Some(EventActorId::new("system.scheduler").expect("left actor")),
            None,
            None,
            None,
            vec![],
        )
        .expect("left trace");

        let right = EventTrace::new(
            Some(EventActorId::new("system.scheduler").expect("right actor")),
            None,
            None,
            None,
            vec![],
        )
        .expect("right trace");

        assert_eq!(left, right);
        assert_eq!(left.clone(), left);
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestPayload {
        previous_health: &'static str,
        current_health: &'static str,
    }

    fn canonical_test_envelope(
        correlation_id: Option<CorrelationId>,
        causation: EventCausation,
    ) -> EventEnvelope<TestPayload> {
        let source = EventSource::new(
            EventComponent::new("kernel-runtime").expect("valid component"),
            Some(RuntimeId::new("kernel.runtime.primary").expect("valid runtime id")),
        );

        let subject = EventSubject::new(
            EventSubjectType::new("runtime").expect("valid subject type"),
            EventSubjectId::new("kernel.runtime.primary").expect("valid subject id"),
        );

        let trace = EventTrace::new(
            Some(EventActorId::new("system.supervisor").expect("valid actor id")),
            None,
            None,
            Some(
                EventTraceReference::new("execution.health-assessment")
                    .expect("valid execution reference"),
            ),
            vec![],
        )
        .expect("valid trace");

        EventEnvelope::new(
            EventId::new("CX-EVT-000100").expect("valid event id"),
            EventType::new("runtime.health.assessed").expect("valid event type"),
            EventVersion::new("1.0.0").expect("valid event version"),
            TimeReference::new("2026-07-16T12:00:00Z").expect("valid occurrence time"),
            TimeReference::new("2026-07-16T12:00:01Z").expect("valid recording time"),
            source,
            subject,
            TestPayload {
                previous_health: "DEGRADED",
                current_health: "HEALTHY",
            },
            EventClassification::Internal,
            trace,
            correlation_id,
            causation,
        )
    }

    fn canonical_test_candidate(
        correlation_id: Option<CorrelationId>,
        causation: EventCausation,
    ) -> EventEnvelopeCandidate<TestPayload> {
        EventEnvelopeCandidate {
            event_id: Some(EventId::new("CX-EVT-000100").expect("valid event id")),
            event_type: Some(EventType::new("runtime.health.assessed").expect("valid event type")),
            event_version: Some(EventVersion::new("1.0.0").expect("valid event version")),
            occurred_at: Some(
                TimeReference::new("2026-07-16T12:00:00Z").expect("valid occurrence time"),
            ),
            recorded_at: Some(
                TimeReference::new("2026-07-16T12:00:01Z").expect("valid recording time"),
            ),
            source: Some(EventSource::new(
                EventComponent::new("kernel-runtime").expect("valid component"),
                Some(RuntimeId::new("kernel.runtime.primary").expect("valid runtime id")),
            )),
            subject: Some(EventSubject::new(
                EventSubjectType::new("runtime").expect("valid subject type"),
                EventSubjectId::new("kernel.runtime.primary").expect("valid subject id"),
            )),
            payload: Some(TestPayload {
                previous_health: "DEGRADED",
                current_health: "HEALTHY",
            }),
            classification: Some(EventClassification::Internal),
            trace: Some(
                EventTrace::new(
                    Some(EventActorId::new("system.supervisor").expect("valid actor id")),
                    None,
                    None,
                    Some(
                        EventTraceReference::new("execution.health-assessment")
                            .expect("valid execution reference"),
                    ),
                    vec![],
                )
                .expect("valid trace"),
            ),
            correlation_id,
            causation,
        }
    }

    fn self_caused_test_envelope(
        correlation_id: Option<CorrelationId>,
    ) -> EventEnvelope<TestPayload> {
        canonical_test_envelope(
            correlation_id,
            EventCausation::CausedBy(
                EventId::new("CX-EVT-000100").expect("valid self-caused parent event id"),
            ),
        )
    }

    fn test_envelope_with_event_version(
        event_version: EventVersion,
        correlation_id: Option<CorrelationId>,
    ) -> EventEnvelope<TestPayload> {
        let mut envelope = canonical_test_envelope(correlation_id, EventCausation::root());
        envelope.event_version = event_version;
        envelope
    }

    fn test_envelope_with_timestamps(
        occurred_at: &str,
        recorded_at: &str,
        correlation_id: Option<CorrelationId>,
    ) -> EventEnvelope<TestPayload> {
        let mut envelope = canonical_test_envelope(correlation_id, EventCausation::root());
        envelope.occurred_at = TimeReference::new(occurred_at).expect("valid test occurrence time");
        envelope.recorded_at = TimeReference::new(recorded_at).expect("valid test recording time");
        envelope
    }

    fn test_envelope_with_payload<P>(
        payload: P,
        correlation_id: Option<CorrelationId>,
    ) -> EventEnvelope<P> {
        let source = EventSource::new(
            EventComponent::new("kernel-runtime").expect("valid component"),
            Some(RuntimeId::new("kernel.runtime.primary").expect("valid runtime id")),
        );

        let subject = EventSubject::new(
            EventSubjectType::new("runtime").expect("valid subject type"),
            EventSubjectId::new("kernel.runtime.primary").expect("valid subject id"),
        );

        let trace = EventTrace::new(
            Some(EventActorId::new("system.supervisor").expect("valid actor id")),
            None,
            None,
            Some(
                EventTraceReference::new("execution.health-assessment")
                    .expect("valid execution reference"),
            ),
            vec![],
        )
        .expect("valid trace");

        EventEnvelope::new(
            EventId::new("CX-EVT-000100").expect("valid event id"),
            EventType::new("runtime.health.assessed").expect("valid event type"),
            EventVersion::new("1.0.0").expect("valid event version"),
            TimeReference::new("2026-07-16T12:00:00Z").expect("valid occurrence time"),
            TimeReference::new("2026-07-16T12:00:01Z").expect("valid recording time"),
            source,
            subject,
            payload,
            EventClassification::Internal,
            trace,
            correlation_id,
            EventCausation::root(),
        )
    }

    #[test]
    fn event_envelope_preserves_all_mandatory_fields() {
        let envelope = canonical_test_envelope(None, EventCausation::root());

        assert_eq!(envelope.event_id().as_str(), "CX-EVT-000100");
        assert_eq!(envelope.event_type().as_str(), "runtime.health.assessed");
        assert_eq!(envelope.event_version().as_str(), "1.0.0");
        assert_eq!(envelope.occurred_at().as_str(), "2026-07-16T12:00:00Z");
        assert_eq!(envelope.recorded_at().as_str(), "2026-07-16T12:00:01Z");
        assert_eq!(envelope.source().component().as_str(), "kernel-runtime");
        assert_eq!(envelope.subject().subject_type().as_str(), "runtime");
        assert_eq!(
            envelope.subject().subject_id().as_str(),
            "kernel.runtime.primary"
        );
        assert_eq!(envelope.classification(), &EventClassification::Internal);
        assert!(envelope.trace().actor_id().is_some());
        assert!(envelope.causation().is_root());
    }

    #[test]
    fn event_envelope_supports_optional_correlation() {
        let correlation = CorrelationId::new("CX-COR-000100").expect("valid correlation id");

        let envelope = canonical_test_envelope(Some(correlation.clone()), EventCausation::root());

        assert_eq!(envelope.correlation_id(), Some(&correlation));
    }

    #[test]
    fn event_envelope_supports_root_causation() {
        let envelope = canonical_test_envelope(None, EventCausation::root());

        assert!(envelope.causation().is_root());
        assert_eq!(envelope.causation().parent_event_id(), None);
    }

    #[test]
    fn event_envelope_supports_parent_event_causation() {
        let current = EventId::new("CX-EVT-000100").expect("valid current event id");
        let parent = EventId::new("CX-EVT-000099").expect("valid parent event id");

        let causation =
            EventCausation::caused_by(&current, parent.clone()).expect("valid causation");

        let envelope = canonical_test_envelope(None, causation);

        assert_eq!(envelope.causation().parent_event_id(), Some(&parent));
    }

    #[test]
    fn event_envelope_preserves_generic_payload() {
        let envelope = canonical_test_envelope(None, EventCausation::root());

        assert_eq!(envelope.payload().previous_health, "DEGRADED");
        assert_eq!(envelope.payload().current_health, "HEALTHY");
    }

    #[test]
    fn event_envelope_returns_owned_payload() {
        let envelope = canonical_test_envelope(None, EventCausation::root());

        let payload = envelope.into_payload();

        assert_eq!(
            payload,
            TestPayload {
                previous_health: "DEGRADED",
                current_health: "HEALTHY",
            }
        );
    }

    #[test]
    fn event_envelope_preserves_value_semantics() {
        let left = canonical_test_envelope(None, EventCausation::root());
        let right = canonical_test_envelope(None, EventCausation::root());

        assert_eq!(left, right);
        assert_eq!(left.clone(), left);
    }

    #[test]
    fn event_envelope_validation_complete_candidate_succeeds() {
        let candidate = canonical_test_candidate(None, EventCausation::root());

        let envelope = validate_event_envelope(candidate).expect("complete candidate");

        assert_eq!(
            envelope,
            canonical_test_envelope(None, EventCausation::root())
        );
    }

    #[test]
    fn event_envelope_validation_missing_event_id_returns_error() {
        let mut candidate = canonical_test_candidate(None, EventCausation::root());
        candidate.event_id = None;

        let error = validate_event_envelope(candidate).expect_err("event_id is required");

        assert_eq!(error, DomainError::MissingEventField("event_id"));
        assert_eq!(error.to_string(), "missing mandatory event field: event_id");
    }

    #[test]
    fn event_envelope_validation_missing_event_type_returns_error() {
        let mut candidate = canonical_test_candidate(None, EventCausation::root());
        candidate.event_type = None;

        let error = validate_event_envelope(candidate).expect_err("event_type is required");

        assert_eq!(error, DomainError::MissingEventField("event_type"));
    }

    #[test]
    fn event_envelope_validation_missing_event_version_returns_error() {
        let mut candidate = canonical_test_candidate(None, EventCausation::root());
        candidate.event_version = None;

        let error = validate_event_envelope(candidate).expect_err("event_version is required");

        assert_eq!(error, DomainError::MissingEventField("event_version"));
    }

    #[test]
    fn event_envelope_validation_missing_occurred_at_returns_error() {
        let mut candidate = canonical_test_candidate(None, EventCausation::root());
        candidate.occurred_at = None;

        let error = validate_event_envelope(candidate).expect_err("occurred_at is required");

        assert_eq!(error, DomainError::MissingEventField("occurred_at"));
    }

    #[test]
    fn event_envelope_validation_missing_recorded_at_returns_error() {
        let mut candidate = canonical_test_candidate(None, EventCausation::root());
        candidate.recorded_at = None;

        let error = validate_event_envelope(candidate).expect_err("recorded_at is required");

        assert_eq!(error, DomainError::MissingEventField("recorded_at"));
    }

    #[test]
    fn event_envelope_validation_missing_source_returns_error() {
        let mut candidate = canonical_test_candidate(None, EventCausation::root());
        candidate.source = None;

        let error = validate_event_envelope(candidate).expect_err("source is required");

        assert_eq!(error, DomainError::MissingEventField("source"));
    }

    #[test]
    fn event_envelope_validation_missing_subject_returns_error() {
        let mut candidate = canonical_test_candidate(None, EventCausation::root());
        candidate.subject = None;

        let error = validate_event_envelope(candidate).expect_err("subject is required");

        assert_eq!(error, DomainError::MissingEventField("subject"));
    }

    #[test]
    fn event_envelope_validation_missing_payload_returns_error() {
        let mut candidate = canonical_test_candidate(None, EventCausation::root());
        candidate.payload = None;

        let error = validate_event_envelope(candidate).expect_err("payload is required");

        assert_eq!(error, DomainError::MissingEventField("payload"));
    }

    #[test]
    fn event_envelope_validation_missing_classification_returns_error() {
        let mut candidate = canonical_test_candidate(None, EventCausation::root());
        candidate.classification = None;

        let error = validate_event_envelope(candidate).expect_err("classification is required");

        assert_eq!(error, DomainError::MissingEventField("classification"));
    }

    #[test]
    fn event_envelope_validation_missing_trace_returns_error() {
        let mut candidate = canonical_test_candidate(None, EventCausation::root());
        candidate.trace = None;

        let error = validate_event_envelope(candidate).expect_err("trace is required");

        assert_eq!(error, DomainError::MissingEventField("trace"));
    }

    #[test]
    fn event_envelope_validation_event_id_precedes_later_missing_fields() {
        let mut candidate = canonical_test_candidate(None, EventCausation::root());
        candidate.event_id = None;
        candidate.trace = None;

        let error = validate_event_envelope(candidate).expect_err("event_id must win");

        assert_eq!(error, DomainError::MissingEventField("event_id"));
    }

    #[test]
    fn event_envelope_validation_event_type_precedes_later_missing_fields() {
        let mut candidate = canonical_test_candidate(None, EventCausation::root());
        candidate.event_type = None;
        candidate.payload = None;

        let error = validate_event_envelope(candidate).expect_err("event_type must win");

        assert_eq!(error, DomainError::MissingEventField("event_type"));
    }

    #[test]
    fn event_envelope_validation_preserves_optional_correlation() {
        let correlation = CorrelationId::new("CX-COR-000100").expect("valid correlation id");
        let candidate = canonical_test_candidate(Some(correlation.clone()), EventCausation::root());

        let envelope = validate_event_envelope(candidate).expect("complete candidate");

        assert_eq!(envelope.correlation_id(), Some(&correlation));
    }

    #[test]
    fn event_envelope_validation_preserves_root_causation() {
        let candidate = canonical_test_candidate(None, EventCausation::root());

        let envelope = validate_event_envelope(candidate).expect("complete candidate");

        assert!(envelope.causation().is_root());
    }

    #[test]
    fn event_envelope_validation_equivalent_invalid_candidates_produce_equivalent_errors() {
        let mut left = canonical_test_candidate(None, EventCausation::root());
        left.subject = None;

        let mut right = canonical_test_candidate(None, EventCausation::root());
        right.subject = None;

        let left_error = validate_event_envelope(left).expect_err("subject is required");
        let right_error = validate_event_envelope(right).expect_err("subject is required");

        assert_eq!(left_error, right_error);
        assert_eq!(left_error, DomainError::MissingEventField("subject"));
    }

    #[test]
    fn event_identity_validation_root_event_passes() {
        let envelope = canonical_test_envelope(None, EventCausation::root());

        assert_eq!(validate_event_identity(&envelope), Ok(()));
    }

    #[test]
    fn event_identity_validation_distinct_parent_event_passes() {
        let envelope = canonical_test_envelope(
            None,
            EventCausation::CausedBy(EventId::new("CX-EVT-000099").expect("valid parent event id")),
        );

        assert_eq!(validate_event_identity(&envelope), Ok(()));
    }

    #[test]
    fn event_identity_validation_direct_self_causation_is_rejected() {
        let envelope = self_caused_test_envelope(None);

        let error =
            validate_event_identity(&envelope).expect_err("self-causation must be rejected");

        assert_eq!(
            error,
            DomainError::InvalidEventReference("causation_id must not equal event_id")
        );
    }

    #[test]
    fn event_identity_validation_self_causation_error_text_is_canonical() {
        let envelope = self_caused_test_envelope(None);

        let error =
            validate_event_identity(&envelope).expect_err("self-causation must be rejected");

        assert_eq!(
            error.to_string(),
            "invalid event reference: causation_id must not equal event_id"
        );
    }

    #[test]
    fn event_identity_validation_repeated_validation_of_valid_envelope_produces_equivalent_outcomes(
    ) {
        let envelope = canonical_test_envelope(None, EventCausation::root());

        let first = validate_event_identity(&envelope);
        let second = validate_event_identity(&envelope);

        assert_eq!(first, second);
        assert_eq!(first, Ok(()));
    }

    #[test]
    fn event_identity_validation_equivalent_self_caused_envelopes_produce_equivalent_errors() {
        let left = self_caused_test_envelope(None);
        let right = self_caused_test_envelope(None);

        let left_error = validate_event_identity(&left).expect_err("left must be rejected");
        let right_error = validate_event_identity(&right).expect_err("right must be rejected");

        assert_eq!(left_error, right_error);
        assert_eq!(
            left_error,
            DomainError::InvalidEventReference("causation_id must not equal event_id")
        );
    }

    #[test]
    fn event_identity_validation_does_not_mutate_event_envelope() {
        let envelope = canonical_test_envelope(
            None,
            EventCausation::CausedBy(EventId::new("CX-EVT-000099").expect("valid parent event id")),
        );
        let original = envelope.clone();

        let result = validate_event_identity(&envelope);

        assert_eq!(result, Ok(()));
        assert_eq!(envelope, original);
    }

    #[test]
    fn event_identity_validation_correlation_presence_does_not_change_identity_validation_outcome()
    {
        let without_correlation = canonical_test_envelope(None, EventCausation::root());
        let with_correlation = canonical_test_envelope(
            Some(CorrelationId::new("CX-COR-000100").expect("valid correlation id")),
            EventCausation::root(),
        );

        let without_correlation_result = validate_event_identity(&without_correlation);
        let with_correlation_result = validate_event_identity(&with_correlation);

        assert_eq!(without_correlation_result, with_correlation_result);
        assert_eq!(with_correlation_result, Ok(()));
    }

    #[test]
    fn event_version_validation_supported_version_passes() {
        let envelope = canonical_test_envelope(None, EventCausation::root());

        assert_eq!(validate_event_version(&envelope), Ok(()));
    }

    #[test]
    fn event_version_validation_supported_patch_version_passes() {
        let envelope = test_envelope_with_event_version(
            EventVersion::new("1.0.1").expect("valid supported event version"),
            None,
        );

        assert_eq!(validate_event_version(&envelope), Ok(()));
    }

    #[test]
    fn event_version_validation_unsupported_zero_major_version_is_rejected() {
        let envelope = test_envelope_with_event_version(
            EventVersion::new("0.9.0").expect("valid unsupported event version"),
            None,
        );

        let error = validate_event_version(&envelope).expect_err("unsupported version must fail");

        assert_eq!(
            error,
            DomainError::InvalidEventReference("event_version is not supported")
        );
    }

    #[test]
    fn event_version_validation_unsupported_major_version_is_rejected() {
        let envelope = test_envelope_with_event_version(
            EventVersion::new("2.0.0").expect("valid unsupported event version"),
            None,
        );

        let error = validate_event_version(&envelope).expect_err("unsupported version must fail");

        assert_eq!(
            error,
            DomainError::InvalidEventReference("event_version is not supported")
        );
    }

    #[test]
    fn event_version_validation_supported_minor_version_passes() {
        let envelope = test_envelope_with_event_version(
            EventVersion::new("1.1.0").expect("valid supported event version"),
            None,
        );

        assert_eq!(validate_event_version(&envelope), Ok(()));
    }

    #[test]
    fn event_version_validation_error_text_is_canonical() {
        let envelope = test_envelope_with_event_version(
            EventVersion::new("3.0.0").expect("valid unsupported event version"),
            None,
        );

        let error = validate_event_version(&envelope).expect_err("unsupported version must fail");

        assert_eq!(
            error.to_string(),
            "invalid event reference: event_version is not supported"
        );
    }

    #[test]
    fn event_version_validation_repeated_validation_of_supported_version_produces_equivalent_outcomes(
    ) {
        let envelope = canonical_test_envelope(None, EventCausation::root());

        let first = validate_event_version(&envelope);
        let second = validate_event_version(&envelope);

        assert_eq!(first, second);
        assert_eq!(first, Ok(()));
    }

    #[test]
    fn event_version_validation_equivalent_unsupported_envelopes_produce_equivalent_errors() {
        let left = test_envelope_with_event_version(
            EventVersion::new("2.0.0").expect("valid unsupported event version"),
            None,
        );
        let right = test_envelope_with_event_version(
            EventVersion::new("2.0.0").expect("valid unsupported event version"),
            None,
        );

        let left_error = validate_event_version(&left).expect_err("left must fail");
        let right_error = validate_event_version(&right).expect_err("right must fail");

        assert_eq!(left_error, right_error);
        assert_eq!(
            left_error,
            DomainError::InvalidEventReference("event_version is not supported")
        );
    }

    #[test]
    fn event_version_validation_does_not_mutate_event_envelope() {
        let envelope = test_envelope_with_event_version(
            EventVersion::new("1.0.0").expect("valid supported event version"),
            None,
        );
        let original = envelope.clone();

        let result = validate_event_version(&envelope);

        assert_eq!(result, Ok(()));
        assert_eq!(envelope, original);
    }

    #[test]
    fn event_version_validation_correlation_presence_does_not_change_version_validation_outcome() {
        let without_correlation = canonical_test_envelope(None, EventCausation::root());
        let with_correlation = canonical_test_envelope(
            Some(CorrelationId::new("CX-COR-000100").expect("valid correlation id")),
            EventCausation::root(),
        );

        let without_correlation_result = validate_event_version(&without_correlation);
        let with_correlation_result = validate_event_version(&with_correlation);

        assert_eq!(without_correlation_result, with_correlation_result);
        assert_eq!(with_correlation_result, Ok(()));
    }

    #[test]
    fn event_timestamp_validation_valid_increasing_timestamps_pass() {
        let envelope = canonical_test_envelope(None, EventCausation::root());

        assert_eq!(validate_event_timestamps(&envelope), Ok(()));
    }

    #[test]
    fn event_timestamp_validation_equal_timestamps_pass() {
        let envelope =
            test_envelope_with_timestamps("2026-07-16T12:00:00Z", "2026-07-16T12:00:00Z", None);

        assert_eq!(validate_event_timestamps(&envelope), Ok(()));
    }

    #[test]
    fn event_timestamp_validation_recorded_at_before_occurred_at_is_rejected() {
        let envelope =
            test_envelope_with_timestamps("2026-07-16T12:00:01Z", "2026-07-16T12:00:00Z", None);

        let error =
            validate_event_timestamps(&envelope).expect_err("recorded_at ordering must fail");

        assert_eq!(
            error,
            DomainError::InvalidEventTimestamp("recorded_at must not precede occurred_at")
        );
        assert_eq!(
            error.to_string(),
            "recorded_at must not precede occurred_at"
        );
    }

    #[test]
    fn event_timestamp_validation_invalid_occurred_at_fails_before_invalid_recorded_at() {
        let envelope =
            test_envelope_with_timestamps("2026-13-16T12:00:00Z", "2026-13-16T12:00:00Z", None);

        let error = validate_event_timestamps(&envelope).expect_err("occurred_at must win");

        assert_eq!(error, DomainError::InvalidEventTimestamp("occurred_at"));
    }

    #[test]
    fn event_timestamp_validation_invalid_recorded_at_is_rejected() {
        let envelope =
            test_envelope_with_timestamps("2026-07-16T12:00:00Z", "2026-13-16T12:00:00Z", None);

        let error =
            validate_event_timestamps(&envelope).expect_err("invalid recorded_at must fail");

        assert_eq!(error, DomainError::InvalidEventTimestamp("recorded_at"));
        assert_eq!(error.to_string(), "invalid event timestamp: recorded_at");
    }

    #[test]
    fn event_timestamp_validation_invalid_month() {
        let envelope =
            test_envelope_with_timestamps("2026-00-16T12:00:00Z", "2026-07-16T12:00:01Z", None);

        let error = validate_event_timestamps(&envelope).expect_err("invalid month must fail");

        assert_eq!(error, DomainError::InvalidEventTimestamp("occurred_at"));
    }

    #[test]
    fn event_timestamp_validation_invalid_day() {
        let envelope =
            test_envelope_with_timestamps("2026-04-31T12:00:00Z", "2026-07-16T12:00:01Z", None);

        let error = validate_event_timestamps(&envelope).expect_err("invalid day must fail");

        assert_eq!(error, DomainError::InvalidEventTimestamp("occurred_at"));
    }

    #[test]
    fn event_timestamp_validation_leap_year_february_29_passes() {
        let envelope =
            test_envelope_with_timestamps("2024-02-29T12:00:00Z", "2024-02-29T12:00:01Z", None);

        assert_eq!(validate_event_timestamps(&envelope), Ok(()));
    }

    #[test]
    fn event_timestamp_validation_non_leap_february_29_fails() {
        let envelope =
            test_envelope_with_timestamps("2025-02-29T12:00:00Z", "2025-02-29T12:00:01Z", None);

        let error = validate_event_timestamps(&envelope).expect_err("invalid leap day must fail");

        assert_eq!(error, DomainError::InvalidEventTimestamp("occurred_at"));
    }

    #[test]
    fn event_timestamp_validation_invalid_hour() {
        let envelope =
            test_envelope_with_timestamps("2026-07-16T24:00:00Z", "2026-07-16T24:00:01Z", None);

        let error = validate_event_timestamps(&envelope).expect_err("invalid hour must fail");

        assert_eq!(error, DomainError::InvalidEventTimestamp("occurred_at"));
    }

    #[test]
    fn event_timestamp_validation_invalid_minute() {
        let envelope =
            test_envelope_with_timestamps("2026-07-16T12:60:00Z", "2026-07-16T12:60:01Z", None);

        let error = validate_event_timestamps(&envelope).expect_err("invalid minute must fail");

        assert_eq!(error, DomainError::InvalidEventTimestamp("occurred_at"));
    }

    #[test]
    fn event_timestamp_validation_invalid_second() {
        let envelope =
            test_envelope_with_timestamps("2026-07-16T12:00:60Z", "2026-07-16T12:00:59Z", None);

        let error = validate_event_timestamps(&envelope).expect_err("invalid second must fail");

        assert_eq!(error, DomainError::InvalidEventTimestamp("occurred_at"));
    }

    #[test]
    fn event_timestamp_validation_fractional_seconds_rejected() {
        let envelope =
            test_envelope_with_timestamps("2026-07-16T12:00:00.1Z", "2026-07-16T12:00:01Z", None);

        let error = validate_event_timestamps(&envelope).expect_err("fractional seconds must fail");

        assert_eq!(error, DomainError::InvalidEventTimestamp("occurred_at"));
    }

    #[test]
    fn event_timestamp_validation_timezone_offset_rejected() {
        let envelope = test_envelope_with_timestamps(
            "2026-07-16T12:00:00+00:00",
            "2026-07-16T12:00:01Z",
            None,
        );

        let error = validate_event_timestamps(&envelope).expect_err("offset must fail");

        assert_eq!(error, DomainError::InvalidEventTimestamp("occurred_at"));
    }

    #[test]
    fn event_timestamp_validation_lowercase_t_z_rejected() {
        let envelope =
            test_envelope_with_timestamps("2026-07-16t12:00:00z", "2026-07-16T12:00:01Z", None);

        let error = validate_event_timestamps(&envelope).expect_err("lowercase must fail");

        assert_eq!(error, DomainError::InvalidEventTimestamp("occurred_at"));
    }

    #[test]
    fn event_timestamp_validation_is_deterministic() {
        let envelope =
            test_envelope_with_timestamps("2026-07-16T12:00:01Z", "2026-07-16T12:00:00Z", None);

        let first = validate_event_timestamps(&envelope);
        let second = validate_event_timestamps(&envelope);

        assert_eq!(first, second);
        assert_eq!(
            first,
            Err(DomainError::InvalidEventTimestamp(
                "recorded_at must not precede occurred_at",
            ))
        );
    }

    #[test]
    fn event_timestamp_validation_does_not_mutate_envelope() {
        let envelope = canonical_test_envelope(None, EventCausation::root());
        let original = envelope.clone();

        let result = validate_event_timestamps(&envelope);

        assert_eq!(result, Ok(()));
        assert_eq!(envelope, original);
    }

    #[test]
    fn event_timestamp_validation_correlation_presence_does_not_affect_timestamp_result() {
        let without_correlation = canonical_test_envelope(None, EventCausation::root());
        let with_correlation = canonical_test_envelope(
            Some(CorrelationId::new("CX-COR-000100").expect("valid correlation id")),
            EventCausation::root(),
        );

        let without_correlation_result = validate_event_timestamps(&without_correlation);
        let with_correlation_result = validate_event_timestamps(&with_correlation);

        assert_eq!(without_correlation_result, with_correlation_result);
        assert_eq!(with_correlation_result, Ok(()));
    }

    #[test]
    fn event_payload_validation_valid_payload_passes() {
        let envelope = canonical_test_envelope(None, EventCausation::root());

        let result = validate_event_payload(&envelope, |payload| {
            if payload.current_health == "HEALTHY" {
                Ok(())
            } else {
                Err(DomainError::InvalidEventReference(
                    "payload must be healthy",
                ))
            }
        });

        assert_eq!(result, Ok(()));
    }

    #[test]
    fn event_payload_validation_invalid_payload_propagates_canonical_error() {
        let envelope = canonical_test_envelope(None, EventCausation::root());

        let error = validate_event_payload(&envelope, |_| {
            Err(DomainError::InvalidEventReference(
                "payload must satisfy caller validation",
            ))
        })
        .expect_err("invalid payload must fail");

        assert_eq!(
            error,
            DomainError::InvalidEventReference("payload must satisfy caller validation")
        );
        assert_eq!(
            error.to_string(),
            "invalid event reference: payload must satisfy caller validation"
        );
    }

    #[test]
    fn event_payload_validation_validator_receives_payload_by_shared_reference() {
        let envelope = canonical_test_envelope(None, EventCausation::root());
        let payload_reference = envelope.payload() as *const TestPayload;

        let result = validate_event_payload(&envelope, |payload| {
            assert!(std::ptr::eq(payload, envelope.payload()));
            assert_eq!(payload as *const TestPayload, payload_reference);
            Ok(())
        });

        assert_eq!(result, Ok(()));
    }

    #[test]
    fn event_payload_validation_payload_is_not_mutated() {
        let envelope = canonical_test_envelope(None, EventCausation::root());
        let original = envelope.clone();

        let result = validate_event_payload(&envelope, |payload| {
            assert_eq!(payload.previous_health, "DEGRADED");
            assert_eq!(payload.current_health, "HEALTHY");
            Ok(())
        });

        assert_eq!(result, Ok(()));
        assert_eq!(envelope, original);
    }

    #[test]
    fn event_payload_validation_deterministic_repeated_validation() {
        let envelope = canonical_test_envelope(None, EventCausation::root());
        static VALIDATION_CALLS: AtomicUsize = AtomicUsize::new(0);

        fn deterministic_validator(payload: &TestPayload) -> DomainResult<()> {
            VALIDATION_CALLS.fetch_add(1, Ordering::Relaxed);

            if payload.previous_health == "DEGRADED" && payload.current_health == "HEALTHY" {
                Ok(())
            } else {
                Err(DomainError::InvalidEventReference(
                    "payload must match canonical health transition",
                ))
            }
        }

        let first = validate_event_payload(&envelope, deterministic_validator);
        let second = validate_event_payload(&envelope, deterministic_validator);

        assert_eq!(first, second);
        assert_eq!(first, Ok(()));
        assert_eq!(VALIDATION_CALLS.load(Ordering::Relaxed), 2);
    }

    #[test]
    fn event_payload_validation_correlation_does_not_change_result() {
        let without_correlation = canonical_test_envelope(None, EventCausation::root());
        let with_correlation = canonical_test_envelope(
            Some(CorrelationId::new("CX-COR-000100").expect("valid correlation id")),
            EventCausation::root(),
        );

        fn validator(payload: &TestPayload) -> DomainResult<()> {
            if payload.current_health == "HEALTHY" {
                Ok(())
            } else {
                Err(DomainError::InvalidEventReference(
                    "payload must be healthy",
                ))
            }
        }

        let without_correlation_result = validate_event_payload(&without_correlation, validator);
        let with_correlation_result = validate_event_payload(&with_correlation, validator);

        assert_eq!(without_correlation_result, with_correlation_result);
        assert_eq!(with_correlation_result, Ok(()));
    }

    #[test]
    fn event_payload_validation_different_payload_types_are_supported() {
        let string_payload_envelope =
            test_envelope_with_payload(String::from("canonical payload"), None);
        let integer_payload_envelope = test_envelope_with_payload(42_u32, None);

        let string_result = validate_event_payload(&string_payload_envelope, |payload| {
            if payload == "canonical payload" {
                Ok(())
            } else {
                Err(DomainError::InvalidEventReference(
                    "string payload must match",
                ))
            }
        });
        let integer_result = validate_event_payload(&integer_payload_envelope, |payload| {
            if *payload == 42 {
                Ok(())
            } else {
                Err(DomainError::InvalidEventReference(
                    "integer payload must match",
                ))
            }
        });

        assert_eq!(string_result, Ok(()));
        assert_eq!(integer_result, Ok(()));
    }
}
