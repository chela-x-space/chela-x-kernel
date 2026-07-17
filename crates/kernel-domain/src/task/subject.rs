use crate::identity::IdentityReference;
use crate::ownership::OwnershipSubject;

#[derive(Debug, Clone, PartialEq, Eq)]
enum TaskGovernanceSubject {
    Identity(IdentityReference),
    OwnershipSubject(OwnershipSubject),
}

macro_rules! define_task_subject {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $name(TaskGovernanceSubject);

        impl $name {
            pub fn from_identity(identity_reference: IdentityReference) -> Self {
                Self(TaskGovernanceSubject::Identity(identity_reference))
            }

            pub fn from_ownership_subject(ownership_subject: OwnershipSubject) -> Self {
                Self(TaskGovernanceSubject::OwnershipSubject(ownership_subject))
            }

            pub fn identity_reference(&self) -> Option<&IdentityReference> {
                match &self.0 {
                    TaskGovernanceSubject::Identity(identity_reference) => Some(identity_reference),
                    TaskGovernanceSubject::OwnershipSubject(_) => None,
                }
            }

            pub fn ownership_subject(&self) -> Option<&OwnershipSubject> {
                match &self.0 {
                    TaskGovernanceSubject::Identity(_) => None,
                    TaskGovernanceSubject::OwnershipSubject(ownership_subject) => {
                        Some(ownership_subject)
                    }
                }
            }
        }
    };
}

define_task_subject!(TaskOwner);
define_task_subject!(TaskAssignee);
