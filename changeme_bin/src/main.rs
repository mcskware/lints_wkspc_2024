//! Binary crate.

mod logging;

use logging::init_tracing;
use tracing::info;

/// Main function.
fn main() {
    init_tracing();

    let greeting = changeme_lib::get_greeting_message();
    info!("{greeting}");
}
