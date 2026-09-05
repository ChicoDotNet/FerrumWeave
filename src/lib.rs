pub mod managed;

/// Returns the canonical R00 greeting.
#[must_use]
pub const fn greeting() -> &'static str {
    "Hello FerrumWeave"
}

#[cfg(test)]
mod tests {
    use super::greeting;

    #[test]
    fn greeting_is_stable() {
        assert_eq!(greeting(), "Hello FerrumWeave");
    }
}
