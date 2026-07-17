use crate::ownership::OwnershipScope;
use crate::state::TransitionAuthorityReference;

use super::{subject::TaskOwner, TaskInstanceReference};

pub type TaskOwnershipAuthority = TransitionAuthorityReference;
pub type TaskOwnershipScope = OwnershipScope;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskOwnership {
    task_instance_reference: TaskInstanceReference,
    task_owner: TaskOwner,
    task_ownership_scope: TaskOwnershipScope,
    task_ownership_authority: TaskOwnershipAuthority,
}

impl TaskOwnership {
    pub fn new(
        task_instance_reference: TaskInstanceReference,
        task_owner: TaskOwner,
        task_ownership_scope: TaskOwnershipScope,
        task_ownership_authority: TaskOwnershipAuthority,
    ) -> Self {
        Self {
            task_instance_reference,
            task_owner,
            task_ownership_scope,
            task_ownership_authority,
        }
    }

    pub fn task_instance_reference(&self) -> &TaskInstanceReference {
        &self.task_instance_reference
    }

    pub fn task_owner(&self) -> &TaskOwner {
        &self.task_owner
    }

    pub fn task_ownership_scope(&self) -> &TaskOwnershipScope {
        &self.task_ownership_scope
    }

    pub fn task_ownership_authority(&self) -> &TaskOwnershipAuthority {
        &self.task_ownership_authority
    }
}
