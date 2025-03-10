# Template for Rust 2024 edition workspaces with lots of extra lints

This repo is intended to be a quick-startfor Rust projects. The template targets Rust 2024 edition,
creates a workspace, and includes stub binary and library crates. In addition, there are examples
of both unit tests and integration tests in the library crate.

## Lint additions and modifications

The major difference when using this template will be the extra lints that have been enabled. The
`pedantic`, `nursery`, and `cargo` lint groups have been set to warn-by-default, and many of the
lints in the `restriction` group have been enabled based on what I find to be reasonable defaults.

Note that the `arithmetic_side_effects` lint has been set to `allow`. If you're working on anything
that doesn't play well with random panics, like a `no-std` environment or realtime environments,
you will probably want to set this to `warn`.

Additionally, `arbitrary_source_item_ordering` has been set to `allow`. If you want a stricter code
formatting standard enforced, you could also change this default.

Finally, `pattern_type_mismatch` has been set to `allow`. I simply find that this lint raises too
many incomprehensible hits that may or may not even be false-positives, but the suggestions from
this lint never seem to improve readability or correctness from my point of view and for my general
use cases. See [the Clippy source](https://github.com/rust-lang/rust-clippy/blob/master/clippy_lints/src/pattern_type_mismatch.rs)
for this lint for more information if you want to consider using it.

There are hyperlinks next to each `restriction` group lint that I've added if you'd like to learn
more about any specific lint and what it's for. (This is super fun and informative, I recommed it!)

## Why does this exist?

I have found that I tend to create workspaces a lot, but cargo (probably rightly) does not support
workspace creation when creating a new project.

In addition, I like to run with lots of extra lints enabled. Not only do these lints often catch
things that could be future gotchas, I also find that I learn a lot when one of the lints prompts
me to re-write something in a new way. And who doesn't benefit from learning fun new things?

## Future work

While the workspace does target the 2024 edition of Rust, I created this initially during the 2021
era. I have not yet done the work of finding the new lints available in 2024 (especially in the
`restricted` group) and deciding which should be warn-by-default for this template.

## How to use the template

The process I usually use is something like this:

* Fork/clone or otherwise copy this repo to a new directory
* Delete the `.git` directory
* Run `git init` to get a fresh repo
* Search for all instances of "changeme", and make the name changes you'd like
    * Rename the directories for the bin and lib code
    * Update the names of the bin and lib in the project and workspace `Cargo.toml` files (this includes the `members` and `default-members` fields in the root `Cargo.toml`)
    * Update any references to the lib name to your newly chosen name
    * Update the `description`, 
* Run `cargo t` and ensure that both the unit test and the integration test still run
* Change this readme file with your own amusing and enlightening documentation