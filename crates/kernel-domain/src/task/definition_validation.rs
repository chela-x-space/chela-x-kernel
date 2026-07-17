use crate::errors::{DomainError, DomainResult};

pub(super) fn reject_duplicates<T: Eq>(values: &[T], message: &'static str) -> DomainResult<()> {
    for (index, value) in values.iter().enumerate() {
        if values[..index].iter().any(|prior| prior == value) {
            return Err(DomainError::InvalidTaskDefinition(message));
        }
    }

    Ok(())
}
