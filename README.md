# TOM

[![Build Status](https://travis-ci.org/matklad/tom.svg?branch=master)](https://travis-ci.org/matklad/tom)
[![Crates.io](https://img.shields.io/crates/v/tom.svg)](https://crates.io/crates/tom)
[![API reference](https://docs.rs/tom/badge.svg)](https://docs.rs/tom/)

Yet another TOML parser. Preserves whitespace for real this time!

Work in progress, take a look at
[Molten](https://github.com/LeopoldArkham/Molten) or
[toml-edit](https://github.com/ordian/toml_edit) for something
relatively more ready.

The best documentation at the moment is
[./examples/api-walkthrough.rs](./examples/api-walkthrough.rs).

There's a WASM demo of the parser here: [https://matklad.github.io/tom/](https://matklad.github.io/tom/).


# Contributing

Contributions are very much welcome! Keep in mind that the code is
very much in experimental state, and so good contributing guides are
missing, formatting is artisan, etc.  Feel free to ask questions by
creating issues/PRs, or by pinging @matklad at the
[Rust discord](https://discordapp.com/channels/442252698964721669/).

Checkout [E-easy](https://github.com/matklad/tom/issues?q=is%3Aopen+is%3Aissue+label%3AE-easy)
and [E-mentor](https://github.com/matklad/tom/issues?q=is%3Aopen+is%3Aissue+label%3AE-mentor) labels.


# Architecture

Currently, beta version of Rust is required to build the code.

`cargo test`, as usual, runs the tests.

Code-generation is used heavily:

  * `cargo gen-symbols` generates the `symbol` module,
  * `cargo gen-ast` generates the `ast` module,
  * `cargo gen-tests` generates tests from special comments.

The generated code is committed: this way, clients of the library don't
need to build the code-generator, which has a lot of dependencies.
