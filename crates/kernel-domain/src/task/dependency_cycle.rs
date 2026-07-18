use super::{TaskDependency, TaskDependencyRejectionReason, TaskDependencyTarget};

pub(super) fn find_invalid_set_reason(
    task_dependencies: &[TaskDependency],
) -> Option<TaskDependencyRejectionReason> {
    if has_duplicate(task_dependencies) {
        Some(TaskDependencyRejectionReason::DuplicateDependency)
    } else if has_cycle(task_dependencies) {
        Some(TaskDependencyRejectionReason::DependencyCycle)
    } else {
        None
    }
}

pub(super) fn semantic_edge_equal(left: &TaskDependency, right: &TaskDependency) -> bool {
    left.task_dependency_source() == right.task_dependency_source()
        && left.task_dependency_target() == right.task_dependency_target()
        && left.task_dependency_type() == right.task_dependency_type()
        && left.task_dependency_requirement() == right.task_dependency_requirement()
}

fn has_duplicate(task_dependencies: &[TaskDependency]) -> bool {
    task_dependencies
        .iter()
        .enumerate()
        .any(|(index, dependency)| {
            task_dependencies[..index].iter().any(|prior| {
                semantic_edge_equal(prior, dependency)
                    || prior.task_dependency_reference() == dependency.task_dependency_reference()
            })
        })
}

fn has_cycle(task_dependencies: &[TaskDependency]) -> bool {
    task_dependencies.iter().any(|dependency| {
        reaches_target(
            task_dependencies,
            dependency.task_dependency_target(),
            dependency
                .task_dependency_source()
                .task_instance_reference(),
            &mut Vec::new(),
        )
    })
}

fn reaches_target(
    task_dependencies: &[TaskDependency],
    task_dependency_target: &TaskDependencyTarget,
    expected: &super::TaskInstanceReference,
    visited: &mut Vec<super::TaskInstanceReference>,
) -> bool {
    if visited.contains(task_dependency_target.task_instance_reference()) {
        return false;
    }
    visited.push(task_dependency_target.task_instance_reference().clone());

    task_dependencies.iter().any(|dependency| {
        dependency
            .task_dependency_source()
            .task_instance_reference()
            == task_dependency_target.task_instance_reference()
            && (dependency
                .task_dependency_target()
                .task_instance_reference()
                == expected
                || reaches_target(
                    task_dependencies,
                    dependency.task_dependency_target(),
                    expected,
                    visited,
                ))
    })
}
