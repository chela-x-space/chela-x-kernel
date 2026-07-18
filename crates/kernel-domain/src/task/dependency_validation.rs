use super::{
    dependency_cycle::{find_invalid_set_reason, semantic_edge_equal},
    dependency_evaluation::evaluate_coordination,
    TaskDependencyCoordinationDecision, TaskDependencyCoordinationRequest,
    TaskDependencyRejectionReason, TaskDependencyValidation, TaskDependencyValidationAccepted,
    TaskDependencyValidationNoOp, TaskDependencyValidationRejected,
    TaskDependencyValidationRequest,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TaskDependencyControl;

impl TaskDependencyControl {
    pub fn validate(request: &TaskDependencyValidationRequest) -> TaskDependencyValidation {
        if request
            .current_task_dependency_set()
            .task_dependencies()
            .iter()
            .any(|dependency| {
                semantic_edge_equal(dependency, request.requested_task_dependency())
                    && dependency.task_dependency_reference()
                        == request
                            .requested_task_dependency()
                            .task_dependency_reference()
            })
        {
            return TaskDependencyValidation::NoOp(TaskDependencyValidationNoOp::new(
                request.current_task_dependency_set().clone(),
            ));
        }
        if request
            .current_task_dependency_set()
            .task_dependencies()
            .iter()
            .any(|dependency| {
                semantic_edge_equal(dependency, request.requested_task_dependency())
                    || dependency.task_dependency_reference()
                        == request
                            .requested_task_dependency()
                            .task_dependency_reference()
            })
        {
            return TaskDependencyValidation::Rejected(TaskDependencyValidationRejected::new(
                request.current_task_dependency_set().clone(),
                request.requested_task_dependency().clone(),
                TaskDependencyRejectionReason::DuplicateDependency,
            ));
        }

        let mut task_dependencies = request
            .current_task_dependency_set()
            .task_dependencies()
            .to_vec();
        task_dependencies.push(request.requested_task_dependency().clone());

        if find_invalid_set_reason(&task_dependencies)
            == Some(TaskDependencyRejectionReason::DependencyCycle)
        {
            return TaskDependencyValidation::Rejected(TaskDependencyValidationRejected::new(
                request.current_task_dependency_set().clone(),
                request.requested_task_dependency().clone(),
                TaskDependencyRejectionReason::DependencyCycle,
            ));
        }

        TaskDependencyValidation::Accepted(TaskDependencyValidationAccepted::new(
            super::TaskDependencySet::new(
                request
                    .current_task_dependency_set()
                    .task_dependency_graph_reference()
                    .clone(),
                task_dependencies,
            ),
        ))
    }

    pub fn evaluate(
        request: &TaskDependencyCoordinationRequest,
    ) -> TaskDependencyCoordinationDecision {
        evaluate_coordination(request)
    }
}
