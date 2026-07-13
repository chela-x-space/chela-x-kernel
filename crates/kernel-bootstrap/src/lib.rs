#![forbid(unsafe_code)]

/// Returns the immutable K0 bootstrap marker for the CHELA-X Kernel workspace.
pub fn bootstrap_marker() -> &'static str {
    "chela-x-kernel-k0"
}

#[cfg(test)]
mod tests {
    use super::bootstrap_marker;

    #[test]
    fn marker_is_stable() {
        assert_eq!(bootstrap_marker(), "chela-x-kernel-k0");
    }
}
