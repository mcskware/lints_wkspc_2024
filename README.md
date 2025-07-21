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

## Things you might encounter

### `#[allow]` vs `#[expect]`

When running with a lot of lints, especially from the `restriction` group, you might find that
you want to just use `#[allow]` here and there, especially when something is a false positive
or when you're just writing test code.

Note that the `allow_attributes` lint requries you to use the `#[expect]` attribute instead of `#[allow]` -
this attribute will trigger a warning when the lint expectation is not fulfilled. I find this
incredibly useful, as after a big refactor, you might no longer need a few of those pesky
`#[allow]` attributes anymore.

In addition, the `allow_attributes_without_reason` lint requires that you supply a reason for
each `#[expect]` attribute, like this:

```rust
impl Display for Bcs {
    #[expect(clippy::min_ident_chars, reason = "From the Rust stdlib")]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "BCS[{:02X?}] {}", self.opcode, self.mode)
    }
}
```

In this example, the `min_ident_chars` lint triggered on the `f: &mut Formatter<'_>` argument.
This lint is present since it's generally not a great idea to have single character variable
names. However, in this case, if you were to change the argument name to something more
informative, like `fmt`, you would trigger the `renamed-function-params`, which exists to
inform you when you have used a different argument name in a trait implementation (which would
be bad because it breaks the expectations of readers familiar with this trait).

So in this case, I have opted to use an `#[expect]` attribute to bypass the `min_ident_chars`
lint, but I gave the explicit reason that we're following the standard from the Rust stdlib.

### Readability concerns

There are quite a few lints from the `restriction` group that hover around idiomatic Rust
patterns. However, there are quite a few of these that can often sacrifice readability while
not providing any real performance or correctness benefits. Don't be afraid to dismiss the
changes these lints suggest if you believe the alternative to be better for your needs.

For example:

```rust
fn create_instruction(opcode: u8, cpu: &mut Cpu) -> Option<Box<Self>> {
    #[expect(clippy::if_then_some_else_none, reason = "Readability")]
    if opcode == 0x90 {
        Some(Box::from(Self {
            opcode,
            mode: AddressingMode::Relative(u8_to_i8(cpu.read_next_byte())),
        }))
    } else {
        None
    }
}
```

In this example, I have opted out of the `if_then_some_else_none` lint, as I prefer the
simple readability of the code you see above. If we were to follow the lint suggestion, we
would get something like the following:

```rust
fn create_instruction(opcode: u8, cpu: &mut Cpu) -> Option<Box<Self>> {
    (opcode == 0x90).then(|| Box::from(Self {
        opcode,
        mode: AddressingMode::Relative(u8_to_i8(cpu.read_next_byte())),
    }))
}
```

To my eyes, this just doesn't hit the same. It takes me a few cycles to recognize that there's
a branching step in this logic, and it's probably not obvious to new Rust programmers what's
even going on here. Yes, this may be more idiomatic to some teams - the point is rather that
you should be willing to consider when these lints may not work for your use case, and break
out the `#[expect]` attribute with a note about readability.

### Warnings when using `.unwrap()` and `.expect()`

I happen to code a lot in envinroments where panics aren't helpful, so I learned the hard way
that I need to avoid `.unwrap()`s and `.expect()`s in almost all cases. Your use case may be
quite different, but I still recommend changing your return signature to use a `Result` instead
of the potential panic behavior from `.unwrap()` and `.expect()`.

That said, you will definitely run into cases where you simple know more than the compiler
can infer. A super common example is when you have already vetted that a slice has at least
one element, and then you call `.first()`. Using `.unwrap()` here is generally fine (async dragons be damned), so this is a
case where using the `#[expect]` attribute with a reasonable `reason` field makes sense.

### Cargo.lock

I have included a `Cargo.lock` file in this repo. This is intentional, as I didn't want to
exclude it in `.gitignore`, since that's a judgment I don't want to make for all projects.
You are free to add it to your `.gitignore` as you see fit (if you're building a library
only crate, for example).

### Comments

There are several lints enabled that tackle comments. Specifically, you'll probably run into
warnings when you don't include module-level documentation for each module, as well as
comments for structs and struct fields.

It is very tempting to address these by just tacking on a short comment or letting your IDE
fill them in with a suggestion. I just want to warn you here: be strong! Write good comments
early, and be descriptive about your struct fields (especially `pub` fields). I personally
have a huge bias to think to myself, "oh this is super understandable, a short comment should
be fine!" But even one short hour later, I run into that field, and I have already forgotten
if there were any nuances, invariants to consider, or other gotchas that might be present.

This is super common advice, of course, I just thought I'd mention it here (mostly to remind
myself when I use this template again).