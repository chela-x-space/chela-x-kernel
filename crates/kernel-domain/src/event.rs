use std::fmt;
use std::str::FromStr;

use crate::errors::{DomainError, DomainResult};

const EVENT_TYPE_EXPECTATION: &str =
    "lowercase dotted event type with an approved category and non-empty ASCII alphanumeric segments";

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

#[cfg(test)]
mod tests {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::str::FromStr;

    use super::EventType;

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
}
