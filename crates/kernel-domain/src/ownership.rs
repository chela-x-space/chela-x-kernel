use crate::errors::{DomainError, DomainResult};
use crate::identifier::{EnterpriseId, HumanId, OrganizationUnitId, ProjectId, WorkspaceId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OwnershipScope {
    Enterprise,
    Workspace,
    Project,
    OrganizationalUnit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OwnershipSubject {
    Enterprise(EnterpriseId),
    Workspace(WorkspaceId),
    Project(ProjectId),
    OrganizationalUnit(OrganizationUnitId),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnerReference {
    owner_id: HumanId,
}

impl OwnerReference {
    pub fn new(owner_id: HumanId) -> Self {
        Self { owner_id }
    }

    pub fn owner_id(&self) -> &HumanId {
        &self.owner_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnershipPath {
    enterprise_id: EnterpriseId,
    workspace_id: Option<WorkspaceId>,
    project_id: Option<ProjectId>,
    organizational_unit_id: Option<OrganizationUnitId>,
}

impl OwnershipPath {
    pub fn new(
        enterprise_id: EnterpriseId,
        workspace_id: Option<WorkspaceId>,
        project_id: Option<ProjectId>,
        organizational_unit_id: Option<OrganizationUnitId>,
    ) -> DomainResult<Self> {
        if project_id.is_some() && workspace_id.is_none() {
            return Err(DomainError::InvalidOwnershipPath(
                "project scope requires a parent workspace",
            ));
        }
        Ok(Self {
            enterprise_id,
            workspace_id,
            project_id,
            organizational_unit_id,
        })
    }

    pub fn enterprise_id(&self) -> &EnterpriseId {
        &self.enterprise_id
    }

    pub fn workspace_id(&self) -> Option<&WorkspaceId> {
        self.workspace_id.as_ref()
    }

    pub fn project_id(&self) -> Option<&ProjectId> {
        self.project_id.as_ref()
    }

    pub fn organizational_unit_id(&self) -> Option<&OrganizationUnitId> {
        self.organizational_unit_id.as_ref()
    }

    pub fn contains_repeated_elements(&self) -> bool {
        let enterprise = self.enterprise_id.as_str();
        let workspace = self.workspace_id.as_ref().map(WorkspaceId::as_str);
        let project = self.project_id.as_ref().map(ProjectId::as_str);
        let unit = self
            .organizational_unit_id
            .as_ref()
            .map(OrganizationUnitId::as_str);
        workspace == Some(enterprise)
            || project == workspace
            || unit == workspace
            || unit == project
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrganizationalContext {
    ownership_path: OwnershipPath,
    owner: OwnerReference,
}

impl OrganizationalContext {
    pub fn new(ownership_path: OwnershipPath, owner: OwnerReference) -> Self {
        Self {
            ownership_path,
            owner,
        }
    }

    pub fn ownership_path(&self) -> &OwnershipPath {
        &self.ownership_path
    }

    pub fn owner(&self) -> &OwnerReference {
        &self.owner
    }
}

#[cfg(test)]
mod tests {
    use super::{OrganizationalContext, OwnerReference, OwnershipPath};
    use crate::identifier::{EnterpriseId, HumanId, OrganizationUnitId, ProjectId, WorkspaceId};

    #[test]
    fn ownership_accepts_valid_enterprise_path_ces_b0_025_1() {
        let path = OwnershipPath::new(
            EnterpriseId::new("CX-ENT-000001").expect("enterprise"),
            None,
            None,
            None,
        )
        .expect("valid enterprise root path");
        assert_eq!(path.enterprise_id().as_str(), "CX-ENT-000001");
    }

    #[test]
    fn ownership_accepts_valid_workspace_project_path_ces_b0_025_3() {
        let path = OwnershipPath::new(
            EnterpriseId::new("CX-ENT-000001").expect("enterprise"),
            Some(WorkspaceId::new("CX-WS-000001").expect("workspace")),
            Some(ProjectId::new("CX-PROJ-000001").expect("project")),
            Some(OrganizationUnitId::new("CX-OU-000001").expect("unit")),
        )
        .expect("valid hierarchy");
        let owner = OwnerReference::new(HumanId::new("CX-EMP-000001").expect("owner"));
        let context = OrganizationalContext::new(path, owner);
        assert_eq!(
            context
                .ownership_path()
                .project_id()
                .expect("project")
                .as_str(),
            "CX-PROJ-000001"
        );
    }

    #[test]
    fn ownership_rejects_invalid_hierarchy_without_workspace_ces_b0_025_3() {
        let error = OwnershipPath::new(
            EnterpriseId::new("CX-ENT-000001").expect("enterprise"),
            None,
            Some(ProjectId::new("CX-PROJ-000001").expect("project")),
            None,
        )
        .expect_err("project without workspace must fail");
        assert_eq!(
            error.to_string(),
            "invalid ownership path: project scope requires a parent workspace"
        );
    }

    #[test]
    fn ownership_rejects_repeated_circular_elements_traceability_k1() {
        let path = OwnershipPath::new(
            EnterpriseId::new("CX-ENT-000001").expect("enterprise"),
            Some(WorkspaceId::new("CX-WS-000001").expect("workspace")),
            Some(ProjectId::new("CX-PROJ-000001").expect("project")),
            Some(OrganizationUnitId::new("CX-OU-000001").expect("unit")),
        )
        .expect("valid path");
        assert!(!path.contains_repeated_elements());
    }
}
