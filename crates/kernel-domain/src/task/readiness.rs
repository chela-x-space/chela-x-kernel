#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskReadiness {
    Ready,
    Blocked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskReadinessRequirement {
    OwnershipRequired,
    AssignmentRequired,
    AcceptedAssignmentRequired,
    RequiredInputAvailable,
    DependenciesComplete,
    AuthorizationAllowed,
    EvidencePrerequisitesAvailable,
    LaterAssignmentPermitted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskReadinessEvidence {
    RequiredInputAvailable,
    DependenciesComplete,
    AuthorizationAllowed,
    EvidencePrerequisitesAvailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskReadinessBlocker {
    MissingOwner,
    MissingAssignment,
    AssignmentNotAccepted,
    MissingRequiredInput,
    DependencyIncomplete,
    AuthorizationDenied,
    TerminalTaskState,
    MissingRequiredEvidence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskReadinessRejectionReason {
    ContradictoryRequirement,
}
