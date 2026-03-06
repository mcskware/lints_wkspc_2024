//! Library crate

use tracing::debug;

/// Returns a greeting message.
#[must_use]
pub fn get_greeting_message() -> String {
    debug!("Getting greeting message...");
    "Hello, world!".to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_greeting_message() {
        assert_eq!(get_greeting_message(), "Hello, world!");
    }
}
