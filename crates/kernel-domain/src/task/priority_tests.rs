use super::{
    TaskInstanceId, TaskInstanceReference, TaskPriority, TaskPriorityClass, TaskPriorityValue,
};

fn task_instance_reference() -> TaskInstanceReference {
    TaskInstanceReference::new(TaskInstanceId::new("task.instance.priority").expect("instance"))
}

#[test]
fn task_priority_valid_priority_construction_preserves_binding() {
    let priority = TaskPriority::new(
        task_instance_reference(),
        TaskPriorityClass::new("Explicit").expect("class"),
        TaskPriorityValue::new(3).expect("value"),
    );

    assert_eq!(
        priority
            .task_instance_reference()
            .task_instance_id()
            .as_str(),
        "task.instance.priority"
    );
}

#[test]
fn task_priority_same_input_produces_equal_priority() {
    let left = TaskPriority::new(
        task_instance_reference(),
        TaskPriorityClass::new("Explicit").expect("class"),
        TaskPriorityValue::new(3).expect("value"),
    );
    let right = TaskPriority::new(
        task_instance_reference(),
        TaskPriorityClass::new("Explicit").expect("class"),
        TaskPriorityValue::new(3).expect("value"),
    );

    assert_eq!(left, right);
}

#[test]
fn task_priority_different_values_are_not_equal() {
    let left = TaskPriorityValue::new(1).expect("left");
    let right = TaskPriorityValue::new(2).expect("right");

    assert_ne!(left, right);
}

#[test]
fn task_priority_ordering_is_deterministic() {
    let lower = TaskPriorityValue::new(1).expect("lower");
    let higher = TaskPriorityValue::new(2).expect("higher");

    assert!(lower < higher);
}

#[test]
fn task_priority_unsupported_priority_class_is_rejected() {
    let error = TaskPriorityClass::new("Urgent").expect_err("unsupported class must fail");

    assert_eq!(
        error.to_string(),
        "invalid task priority: unsupported task priority class"
    );
}

#[test]
fn task_priority_zero_value_is_rejected() {
    let error = TaskPriorityValue::new(0).expect_err("zero must fail");

    assert_eq!(
        error.to_string(),
        "invalid task priority: task priority value must be greater than zero"
    );
}

#[test]
fn task_priority_binding_does_not_change_lifecycle() {
    let task_instance = crate::task::instance_tests::minimal_task_instance_for_shared_tests();
    let lifecycle_before = task_instance.task_state();
    let _priority = TaskPriority::new(
        TaskInstanceReference::new(task_instance.task_instance_id().clone()),
        TaskPriorityClass::new("Explicit").expect("class"),
        TaskPriorityValue::new(4).expect("value"),
    );

    assert_eq!(task_instance.task_state(), lifecycle_before);
}
