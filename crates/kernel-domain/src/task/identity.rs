use std::fmt;
use std::str::FromStr;

use crate::errors::DomainResult;
use crate::identifier::EnglishNamespace;

macro_rules! define_task_namespace_value {
    ($name:ident, $field:literal) => {
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(EnglishNamespace);

        impl $name {
            pub fn new(value: impl Into<String>) -> DomainResult<Self> {
                EnglishNamespace::new($field, value).map(Self)
            }

            pub fn as_str(&self) -> &str {
                self.0.as_str()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(self.as_str())
            }
        }

        impl FromStr for $name {
            type Err = crate::errors::DomainError;

            fn from_str(value: &str) -> DomainResult<Self> {
                Self::new(value.to_owned())
            }
        }
    };
}

define_task_namespace_value!(TaskDefinitionId, "TaskDefinitionId");
define_task_namespace_value!(TaskInstanceId, "TaskInstanceId");
define_task_namespace_value!(TaskDependencyId, "TaskDependencyId");
define_task_namespace_value!(TaskEvidenceId, "TaskEvidenceId");
