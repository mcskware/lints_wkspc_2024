//! changeme integration tests

#![cfg(test)]

use changeme_lib::get_greeting_message;

#[test]
fn integration_get_greeting_message() {
    assert_eq!(get_greeting_message(), "Hello, world!");
}
