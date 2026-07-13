use std::fmt;
use std::str::FromStr;

use crate::errors::{DomainError, DomainResult};

fn validate_non_empty(field: &'static str, value: impl Into<String>) -> DomainResult<String> {
    let value = value.into().trim().to_owned();
    if value.is_empty() {
        return Err(DomainError::EmptyValue { field });
    }
    Ok(value)
}

fn validate_prefixed_identifier(
    kind: &'static str,
    value: impl Into<String>,
    prefix: &'static str,
    digits: usize,
) -> DomainResult<String> {
    let value = validate_non_empty(kind, value)?;
    let expected = "fixed prefix followed by digits";
    let suffix = value
        .strip_prefix(prefix)
        .ok_or_else(|| DomainError::InvalidIdentifier { kind, value: value.clone(), expected })?;
    if suffix.len() != digits || !suffix.chars().all(|char| char.is_ascii_digit()) {
        return Err(DomainError::InvalidIdentifier { kind, value, expected });
    }
    Ok(value)
}

fn validate_namespace(field: &'static str, value: impl Into<String>) -> DomainResult<String> {
    let value = validate_non_empty(field, value)?;
    if value
        .chars()
        .all(|char| char.is_ascii_alphanumeric() || matches!(char, '.' | '_' | '-'))
    {
        Ok(value)
    } else {
        Err(DomainError::InvalidIdentifier {
            kind: field,
            value,
            expected: "ASCII letters, digits, dot, underscore, or hyphen",
        })
    }
}

macro_rules! define_identifier {
    ($name:ident, $kind:literal, $prefix:literal, $digits:literal) => {
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> DomainResult<Self> {
                validate_prefixed_identifier($kind, value, $prefix, $digits).map(Self)
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(self.as_str())
            }
        }

        impl FromStr for $name {
            type Err = DomainError;

            fn from_str(value: &str) -> DomainResult<Self> {
                Self::new(value.to_owned())
            }
        }
    };
}

define_identifier!(EnterpriseId, "EnterpriseId", "CX-ENT-", 6);
define_identifier!(WorkspaceId, "WorkspaceId", "CX-WS-", 6);
define_identifier!(ProjectId, "ProjectId", "CX-PROJ-", 6);
define_identifier!(OrganizationUnitId, "OrganizationUnitId", "CX-OU-", 6);
define_identifier!(OwnershipId, "OwnershipId", "CX-OWN-", 6);
define_identifier!(HumanId, "HumanId", "CX-EMP-", 6);
define_identifier!(AgentId, "AgentId", "CX-AGT-", 6);
define_identifier!(DecisionId, "DecisionId", "CX-DEC-", 6);
define_identifier!(DecisionAuthorityId, "DecisionAuthorityId", "CX-DECAUTH-", 6);
define_identifier!(PrincipalId, "PrincipalId", "CX-PRN-", 6);
define_identifier!(RoleId, "RoleId", "CX-ROLE-", 6);
define_identifier!(PermissionId, "PermissionId", "CX-PERM-", 6);
define_identifier!(ScopeId, "ScopeId", "CX-SCP-", 6);
define_identifier!(AuthorizationRequestId, "AuthorizationRequestId", "CX-AUTHREQ-", 6);
define_identifier!(AuthorizationDecisionId, "AuthorizationDecisionId", "CX-AUTHDEC-", 6);
define_identifier!(AuditEvidenceId, "AuditEvidenceId", "CX-AUD-", 6);
define_identifier!(DelegationId, "DelegationId", "CX-DEL-", 6);
define_identifier!(PolicyId, "PolicyId", "CX-POL-", 6);
define_identifier!(WorkflowId, "WorkflowId", "CX-WF-", 6);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AgentUuid(String);

impl AgentUuid {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        let value = validate_non_empty("AgentUuid", value)?;
        if value.starts_with("CX-UUID-") {
            Ok(Self(value))
        } else {
            Err(DomainError::InvalidIdentifier {
                kind: "AgentUuid",
                value,
                expected: "CX-UUID-######## or equivalent enterprise UUID binding",
            })
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for AgentUuid {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for AgentUuid {
    type Err = DomainError;

    fn from_str(value: &str) -> DomainResult<Self> {
        Self::new(value.to_owned())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NonEmptyText(String);

impl NonEmptyText {
    pub fn new(field: &'static str, value: impl Into<String>) -> DomainResult<Self> {
        validate_non_empty(field, value).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for NonEmptyText {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EnglishNamespace(String);

impl EnglishNamespace {
    pub fn new(field: &'static str, value: impl Into<String>) -> DomainResult<Self> {
        validate_namespace(field, value).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for EnglishNamespace {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StableVersion(String);

impl StableVersion {
    pub fn new(field: &'static str, value: impl Into<String>) -> DomainResult<Self> {
        validate_non_empty(field, value).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for StableVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    use super::{AgentId, EnterpriseId, NonEmptyText};

    #[test]
    fn identifier_accepts_valid_opaque_identifier_ces_b0_025_1() {
        let identifier = EnterpriseId::new("CX-ENT-000001").expect("valid enterprise id");
        assert_eq!(identifier.as_str(), "CX-ENT-000001");
    }

    #[test]
    fn identifier_rejects_empty_value_ces_b0_025_1() {
        let error = EnterpriseId::new("").expect_err("empty identifier must fail");
        assert_eq!(error.to_string(), "empty value: EnterpriseId");
    }

    #[test]
    fn identifier_parsing_and_display_are_stable_ces_b0_026_6() {
        let identifier = AgentId::new("CX-AGT-000101").expect("valid agent id");
        assert_eq!(identifier.to_string(), "CX-AGT-000101");
    }

    #[test]
    fn identifier_hash_is_stable_for_equal_values_ces_b0_022_1() {
        let left = AgentId::new("CX-AGT-000101").expect("left");
        let right = AgentId::new("CX-AGT-000101").expect("right");
        let mut left_hasher = DefaultHasher::new();
        let mut right_hasher = DefaultHasher::new();
        left.hash(&mut left_hasher);
        right.hash(&mut right_hasher);
        assert_eq!(left, right);
        assert_eq!(left_hasher.finish(), right_hasher.finish());
    }

    #[test]
    fn non_empty_text_rejects_blank_values_traceability_k1() {
        let error = NonEmptyText::new("reason", "   ").expect_err("blank text must fail");
        assert_eq!(error.to_string(), "empty value: reason");
    }
}
