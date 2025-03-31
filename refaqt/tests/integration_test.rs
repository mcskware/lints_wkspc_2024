//! integration tests

#![cfg(test)]

use refaqt::get_greeting_message;

#[test]
fn integration_get_greeting_message() {
    assert_eq!(get_greeting_message(), "Hello, world!");
}
