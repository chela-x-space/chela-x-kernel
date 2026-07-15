use std::fmt;
use std::str::FromStr;

use crate::errors::{DomainError, DomainResult};
use crate::identifier::{EventId, RuntimeId};

const EVENT_TYPE_EXPECTATION: &str =
    "lowercase dotted event type with an approved category and non-empty ASCII alphanumeric segments";

const EVENT_COMPONENT_EXPECTATION: &str =
    "lowercase ASCII component using a-z, 0-9, hyphen, dot, or underscore, without leading, trailing, or adjacent separators";

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

    use crate::identifier::{EventId, RuntimeId};

    use super::{
        EventCausation, EventClassification, EventComponent, EventSource, EventType, EventVersion,
    };

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
}
