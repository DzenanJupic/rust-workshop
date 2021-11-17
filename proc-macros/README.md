# Rust Latam: procedural macros workshop

*This repo is taken from <https://github.com/dtolnay/proc-macro-workshop>, and slightly modified*

<br>

## Contents

- [**Projects**](#projects) — Introduction to each of the projects
  - [**Derive macro:** `derive(Builder)`](#derive-macro-derivebuilder)
  - [**Function-like macro:** `seq!`](#function-like-macro-seq)
  - [**Attribute macro:** `#[sorted]`](#attribute-macro-sorted)
- [**Test harness**](#test-harness) — Explanation of how testing is set up
- [**Workflow**](#workflow) — Recommended way to work through the workshop
- [**Debugging tips**](#debugging-tips)

<br>

## Projects

Here is an introduction to each of the projects. Note that each of these projects goes into more depth than what is
described in the introduction here.

### Derive macro: `derive(Builder)`

This macro generates the boilerplate code involved in implementing the [builder pattern] in Rust. Builders are a
mechanism for instantiating structs, especially structs with many fields, and especially if many of those fields are
optional or the set of fields may need to grow backward compatibly over time.

[builder pattern]: https://en.wikipedia.org/wiki/Builder_pattern

There are a few different possibilities for expressing builders in Rust. Unless you have a strong pre-existing
preference, to keep things simple for this project I would recommend following the example of the standard library's
[`std::process::Command`] builder in which the setter methods each receive and return `&mut self` to allow chained
method calls.

[`std::process::Command`]: https://doc.rust-lang.org/std/process/struct.Command.html

Callers will invoke the macro as follows.

```rust
use derive_builder::Builder;

#[derive(Builder)]
pub struct Command {
    executable: String,
    #[builder(each = "arg")]
    args: Vec<String>,
    current_dir: Option<String>,
}

fn main() {
    let command = Command::builder()
        .executable("cargo".to_owned())
        .arg("build".to_owned())
        .arg("--release".to_owned())
        .build()
        .unwrap();

    assert_eq!(command.executable, "cargo");
}
```

This project covers:

- traversing syntax trees;
- constructing output source code;
- processing helper attributes to customize the generated code.

*Project skeleton is located under the <kbd>builder</kbd> directory.*

### Attribute macro: `#[sorted]`

A macro for when your coworkers (or you yourself) cannot seem to keep enum variants in sorted order when adding variants
or refactoring. The macro will detect unsorted variants at compile time and emit an error pointing out which variants
are out of order.

```rust
#[sorted]
#[derive(Debug)]
pub enum Error {
    BlockSignal(signal::Error),
    CreateCrasClient(libcras::Error),
    CreateEventFd(sys_util::Error),
    CreateSignalFd(sys_util::SignalFdError),
    CreateSocket(io::Error),
    DetectImageType(qcow::Error),
    DeviceJail(io_jail::Error),
    NetDeviceNew(virtio::NetError),
    SpawnVcpu(io::Error),
}
```

This project covers:

- compile-time error reporting;
- application of visitor pattern to traverse a syntax tree;
- limitations of the currently stable macro API and some ways to work around them.

*Project skeleton is located under the <kbd>sorted</kbd> directory.*

### Function-like macro: `seq!`

This macro provides a syntax for stamping out sequentially indexed copies of an arbitrary chunk of code.

For example our application may require an enum with sequentially numbered variants like `Cpu0` `Cpu1` `Cpu2`
... `Cpu511`. But note that the same `seq!`
macro should work for any sort of compile-time loop; there is nothing specific to emitting enum variants. A different
caller might use it for generating an expression like `tuple.0 + tuple.1 + ... + tuple.511`.

```rust
use seq::seq;

seq!(N in 0..512 {
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub enum Processor {
        #(
            Cpu#N,
        )*
    }
});

fn main() {
    let cpu = Processor::Cpu8;

    assert_eq!(cpu as u8, 8);
    assert_eq!(cpu, Processor::Cpu8);
}
```

This project covers:

- parsing custom syntax;
- low-level representation of token streams;
- constructing output source code.

*Project skeleton is located under the <kbd>seq</kbd> directory.*

## Test harness

Testing macros thoroughly tends to be tricky. Rust and Cargo have a built-in testing framework via `cargo test` which
can work for testing the success cases, but we also really care that our macros produce good error message when they
detect a problem at compile time; Cargo isn't able to say that failing to compile is considered a success, and isn't
able to compare that the error message produced by the compiler is exactly what we expect.

The project skeletons in this repository use an alternative test harness called
[trybuild].

[trybuild]: https://github.com/dtolnay/trybuild

<p align="center">
<a href="#test-harness">
<img src="https://user-images.githubusercontent.com/1940490/55197640-eb390080-5191-11e9-8c1f-1183935c0c26.png" width="600">
</a>
</p>

The test harness is geared toward iterating on the implementation of a procedural macro, observing the errors emitted by
failed executions of the macro, and testing that those errors are as expected.

<br>

## Workflow

Every project has a test suite already written under its <kbd>tests</kbd>
directory. (But feel free to add more tests, remove tests for functionality you don't want to implement, or modify tests
as you see fit to align with your implementation.)

Run `cargo test` inside any of the 5 top-level project directories to run the test suite for that project.

Initially every projects starts with all of its tests disabled. Open up the project's *tests/progress.rs* file and
enable tests one at a time as you work through the implementation. **The test files (for example *tests/01-parse.rs*)
each contain a comment explaining what functionality is tested and giving some tips for how to implement it.** I
recommend working through tests in numbered order, each time enabling one more test and getting it passing before moving
on.

Tests come in two flavors: tests that should compile+run successfully, and tests that should fail to compile with a
specific error message.

If a test should compile and run successfully, but fails, the test runner will surface the compiler error or runtime
error output.

<p align="center">
<a href="#workflow">
<img src="https://user-images.githubusercontent.com/1940490/55197637-eb390080-5191-11e9-9197-5832071639ea.png" width="600">
</a>
</p>

For tests that should fail to compile, we compare the compilation output against a file of expected errors for that
test. If those errors match, the test is considered to pass. If they do not match, the test runner will surface the
expected and actual output.

Expected output goes in a file with the same name as the test except with an extension of _*.stderr_ instead of _*.rs_.

<p align="center">
<a href="#workflow">
<img src="https://user-images.githubusercontent.com/1940490/55197639-eb390080-5191-11e9-9c8f-a47cab89652d.png" width="600">
</a>
</p>

If there is no _*.stderr_ file for a test that is supposed to fail to compile, the test runner will save the compiler's
output into a directory called
<kbd>wip</kbd> adjacent to the <kbd>tests</kbd> directory. So the way to update the "expected" output is to delete the
existing _*.stderr_ file, run the tests again so that the output is written to *wip*, and then move the new output from
*wip* to *tests*.

<p align="center">
<a href="#workflow">
<img src="https://user-images.githubusercontent.com/1940490/55197642-ebd19700-5191-11e9-8f00-2d7c5f4be1a9.png" width="600">
</a>
</p>

<br>

## Debugging tips

To look at what code a macro is expanding into, install the [cargo expand] Cargo subcommand and then run `cargo expand`
in the repository root (outside of any of the project directories) to expand the main.rs file in that directory. You can
copy any of the test cases into this main.rs and tweak it as you iterate on the macro.

[cargo expand]: https://github.com/dtolnay/cargo-expand

If a macro is emitting syntactically invalid code (not just code that fails type-checking) then cargo expand will not be
able to show it. Instead have the macro print its generated TokenStream to stderr before returning the tokens.

```rust
eprintln!("TOKENS: {}", tokens);
```

Then a `cargo check` in the repository root (if you are iterating using main.rs)
or `cargo test` in the corresponding project directory will display this output during macro expansion.

Stderr is also a helpful way to see the structure of the syntax tree that gets parsed from the input of the macro.

```rust
eprintln!("INPUT: {:#?}", syntax_tree);
```

Note that in order for Syn's syntax tree types to provide Debug impls, you will need to
set `features = ["extra-traits"]` on the dependency on Syn. This is because adding hundreds of Debug impls adds an
appreciable amount of compile time to Syn, and we really only need this enabled while doing development on a macro
rather than when the finished macro is published to users.

<br>

### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this codebase by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
</sub>