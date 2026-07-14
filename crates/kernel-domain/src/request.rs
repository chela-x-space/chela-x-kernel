use crate::authorization::{AuthorizationSubject, AuthorizationTarget, PermissionReference};
use crate::errors::{DomainError, DomainResult};
use crate::identifier::{AuthorizationRequestId, NonEmptyText};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimeReference(NonEmptyText);

impl TimeReference {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        Ok(Self(NonEmptyText::new("time_reference", value)?))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizationRequestRecord {
    request_id: AuthorizationRequestId,
    requester: AuthorizationSubject,
    requested_permission: PermissionReference,
    target: AuthorizationTarget,
    requested_at: TimeReference,
    reason: NonEmptyText,
}

impl AuthorizationRequestRecord {
    pub fn new(
        request_id: AuthorizationRequestId,
        requester: AuthorizationSubject,
        requested_permission: PermissionReference,
        target: AuthorizationTarget,
        requested_at: TimeReference,
        reason: impl Into<String>,
    ) -> DomainResult<Self> {
        if requester.enterprise_id() != target.scope().enterprise_id() {
            return Err(DomainError::InvalidRequestRecord(
                "requester enterprise must match target scope enterprise",
            ));
        }
        Ok(Self {
            request_id,
            requester,
            requested_permission,
            target,
            requested_at,
            reason: NonEmptyText::new("request_reason", reason)?,
        })
    }

    pub fn request_id(&self) -> &AuthorizationRequestId {
        &self.request_id
    }

    pub fn requester(&self) -> &AuthorizationSubject {
        &self.requester
    }

    pub fn target(&self) -> &AuthorizationTarget {
        &self.target
    }

    pub fn requested_permission(&self) -> &PermissionReference {
        &self.requested_permission
    }

    pub fn requested_at(&self) -> &TimeReference {
        &self.requested_at
    }

    pub fn reason(&self) -> &str {
        self.reason.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::{AuthorizationRequestRecord, TimeReference};
    use crate::authorization::{
        ActionVerb, AuthorizationPrincipalReference, AuthorizationPrincipalType,
        AuthorizationSubject, AuthorizationTarget, CredentialStatusReference,
        PermissionEffectIntent, PermissionReference, PrincipalLifecycleStateReference,
        ResourceType, ScopeLevel, ScopeReference,
    };
    use crate::identifier::{
        AuthorizationRequestId, EnterpriseId, PermissionId, PrincipalId, ProjectId, ScopeId,
        WorkspaceId,
    };
    use crate::ownership::OwnershipPath;

    fn valid_subject() -> AuthorizationSubject {
        AuthorizationSubject::Principal(
            AuthorizationPrincipalReference::new(
                PrincipalId::new("CX-PRN-000001").expect("principal"),
                AuthorizationPrincipalType::Employee,
                "CX-EMP-000001",
                EnterpriseId::new("CX-ENT-000001").expect("enterprise"),
                PrincipalLifecycleStateReference::new("Active").expect("lifecycle"),
                CredentialStatusReference::new("Valid").expect("credential"),
            )
            .expect("principal"),
        )
    }

    fn valid_permission() -> PermissionReference {
        PermissionReference::new(
            PermissionId::new("CX-PERM-000001").expect("permission"),
            ActionVerb::new("approve").expect("verb"),
            ResourceType::new("workflow").expect("type"),
            PermissionEffectIntent::new("Permit").expect("effect"),
        )
    }

    fn valid_target() -> AuthorizationTarget {
        AuthorizationTarget::new(
            ResourceType::new("workflow").expect("type"),
            "WF-001",
            ScopeReference::new(
                ScopeId::new("CX-SCP-000001").expect("scope"),
                ScopeLevel::Project,
                OwnershipPath::new(
                    EnterpriseId::new("CX-ENT-000001").expect("enterprise"),
                    Some(WorkspaceId::new("CX-WS-000001").expect("workspace")),
                    Some(ProjectId::new("CX-PROJ-000001").expect("project")),
                    None,
                )
                .expect("path"),
                None,
            )
            .expect("scope"),
        )
        .expect("target")
    }

    #[test]
    fn request_creates_valid_request_record_ces_b0_026_6() {
        let request = AuthorizationRequestRecord::new(
            AuthorizationRequestId::new("CX-AUTHREQ-000001").expect("id"),
            valid_subject(),
            valid_permission(),
            valid_target(),
            TimeReference::new("2026-07-14T00:00:00Z").expect("time"),
            "approve workflow transition",
        )
        .expect("valid request");
        assert_eq!(request.request_id().as_str(), "CX-AUTHREQ-000001");
    }

    #[test]
    fn request_rejects_invalid_target_reference_ces_b0_026_6() {
        let subject = AuthorizationSubject::Principal(
            AuthorizationPrincipalReference::new(
                PrincipalId::new("CX-PRN-000001").expect("principal"),
                AuthorizationPrincipalType::Employee,
                "CX-EMP-000001",
                EnterpriseId::new("CX-ENT-000001").expect("enterprise"),
                PrincipalLifecycleStateReference::new("Active").expect("lifecycle"),
                CredentialStatusReference::new("Valid").expect("credential"),
            )
            .expect("principal"),
        );
        let target = AuthorizationTarget::new(
            ResourceType::new("workflow").expect("type"),
            "WF-001",
            ScopeReference::new(
                ScopeId::new("CX-SCP-000001").expect("scope"),
                ScopeLevel::Project,
                OwnershipPath::new(
                    EnterpriseId::new("CX-ENT-000002").expect("enterprise"),
                    Some(WorkspaceId::new("CX-WS-000001").expect("workspace")),
                    Some(ProjectId::new("CX-PROJ-000001").expect("project")),
                    None,
                )
                .expect("path"),
                None,
            )
            .expect("scope"),
        )
        .expect("target");
        let error = AuthorizationRequestRecord::new(
            AuthorizationRequestId::new("CX-AUTHREQ-000001").expect("id"),
            subject,
            valid_permission(),
            target,
            TimeReference::new("2026-07-14T00:00:00Z").expect("time"),
            "approve workflow transition",
        )
        .expect_err("enterprise mismatch must fail");
        assert!(error
            .to_string()
            .contains("requester enterprise must match target scope enterprise"));
    }

    #[test]
    fn request_stable_request_id_is_preserved_ces_b0_026_6() {
        let request = AuthorizationRequestRecord::new(
            AuthorizationRequestId::new("CX-AUTHREQ-000111").expect("id"),
            valid_subject(),
            valid_permission(),
            valid_target(),
            TimeReference::new("2026-07-14T00:00:00Z").expect("time"),
            "approve workflow transition",
        )
        .expect("valid request");
        assert_eq!(request.request_id().to_string(), "CX-AUTHREQ-000111");
    }
}
