use std::fmt;

use crate::errors::DomainResult;
use crate::identifier::{EnglishNamespace, NonEmptyText, PolicyId, StableVersion};

macro_rules! define_task_text_value {
    ($name:ident, $field:literal) => {
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(NonEmptyText);

        impl $name {
            pub fn new(value: impl Into<String>) -> DomainResult<Self> {
                NonEmptyText::new($field, value).map(Self)
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
    };
}

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
    };
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TaskDefinitionVersion(StableVersion);

impl TaskDefinitionVersion {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        StableVersion::new("TaskDefinitionVersion", value).map(Self)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

define_task_text_value!(TaskDefinitionName, "TaskDefinitionName");
define_task_text_value!(TaskDescription, "TaskDescription");
define_task_namespace_value!(TaskKind, "TaskKind");
define_task_namespace_value!(TaskInputContract, "TaskInputContract");
define_task_namespace_value!(TaskOutputContract, "TaskOutputContract");
define_task_namespace_value!(TaskRequirement, "TaskRequirement");
define_task_namespace_value!(TaskCapabilityRequirement, "TaskCapabilityRequirement");
define_task_namespace_value!(TaskEvidenceRequirement, "TaskEvidenceRequirement");
define_task_namespace_value!(TaskCompletionRequirement, "TaskCompletionRequirement");

pub type TaskFailurePolicyReference = PolicyId;
