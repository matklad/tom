# TOM

[![Build Status](https://travis-ci.org/matklad/tom.svg?branch=master)](https://travis-ci.org/matklad/tom)
[![Crates.io](https://img.shields.io/crates/v/tom.svg)](https://crates.io/crates/tom)
[![API reference](https://docs.rs/tom/badge.svg)](https://docs.rs/tom/)


**Status** a rewrite to the [rowan](https://github.com/rust-analyzer/rowan) library is in progress.
Nothing works, but we have a language server in the lsp dir now!

Yet another TOML parser. Preserves whitespace for real this time!

Work in progress, take a look at
[Molten](https://github.com/LeopoldArkham/Molten) or
[toml-edit](https://github.com/ordian/toml_edit) for something
relatively more ready.

The best documentation at the moment is
[./crates/tom/examples/api-walkthrough.rs](./crates/tom/examples/api-walkthrough.rs).

There's a WASM demo of the parser here: [https://matklad.github.io/tom/](https://matklad.github.io/tom/).


# Contributing

Contributions are very much welcome! Keep in mind that the code is
very much in experimental state, and so good contributing guides are
missing, formatting is artisan, etc. Feel free to ask questions by
creating issues/PRs, or by pinging @matklad at the
[rust-analyzer zulip](https://rust-lang.zulipchat.com/#narrow/stream/185405-t-compiler.2Fwg-rls-2.2E0).

Checkout [E-easy](https://github.com/matklad/tom/issues?q=is%3Aopen+is%3Aissue+label%3AE-easy)
and [E-has-instructions](https://github.com/matklad/tom/issues?q=is%3Aopen+is%3Aissue+label%3AE-has-instructions) labels.


# Architecture

## Building the Code

Currently, beta version of Rust is required.

`cargo test`, as usual, runs the tests.

Code-generation is used heavily:

  * `cargo xtask gen-symbols` generates the `symbol` module,
  * `cargo xtask gen-ast` generates the `ast` module,
  * `cargo xtask gen-tests` generates tests from special comments.

The generated code is committed: this way, clients of the library don't
need to build the code-generator, which has a lot of dependencies.

See `.cargo/config` file and the `xtask` subdirectories to understand how
codegen works.

## Data Structures Walkthrough

The entry point of the library is the `TomlDoc` type.

The core data structure is `tree::Tree`, a generic mutable arena/index
based tree. The design is inspired by
[indextree](https://github.com/saschagrunert/indextree). Indices allow
to store parent links, and a flexible editing API: you can mutate the
tree without invalidating existing node indices and without running
into borrow-checker errors. The price for this flexibility is that
clients have to pass `&Tree` or `&mut Tree` to every method of a node,
because a node is just a 32-bit index and all the actual data are
stored in the `Tree`. This beauty/quirk of the API bleeds to all
higher-level layers.

On top of the `tree::Tree` a Concrete Syntax Tree data structure is
build (see the `cst` module). Each node in the CST has a `Symbol`,
which is a "type" of the node: is it a key-value pair, or a table, or
a whole document. There are about 30 different symbols in TOML, see
the generated `symbol` module for the whole list. Additionally, each
leaf node, including comments and whitespace, contains a `&str` with a
text (string interning is used to avoid allocating each token
separately). Thus, it is possible to reconstruct the text of each CST
node exactly by recursively walking its children and concatenating
texts of the leaves.

The CST is stored as a part of `TomlDoc`, which also contains the list
of syntax errors, and the cache of text ranges of nodes. Ranges are
recalculated by recursively walking the tree and summing lengths of
the leaves.

Two smaller "data structures" are the `intern::Intern` string interner
and the `chunked_text::ChunkedText` trait. The latter allows
processing the text of internal nodes without materializing it into a
single continuous `String`.

## Parser

Parsing is not too unusual: regular expressions based lexer +
hand-written recursive descent. The lexer is in `parser/lexer.rs`, the
parser is in `parser/grammar.rs`.

However, both parser and lexer do not abort on error and *always*
parse the document to the end. A special `ERROR` CST node is created
for those parts of inputs which can't be recognized as TOML.

Parsing and the actual tree construction are decoupled via the
`EventSink`. The parser notifies the sink when it starts/finishes
reading a particular node, and the sink takes care of actually
constructing the tree. `EventSink` also takes care of whitespace and
comment handing. The CST for `foo = 92 #comment` would include
`#comment` token as a child of `foo = 92` key-value, based on the
same-line heuristic (see `EventSink::trailing_ws`).

The grammar in `parser/grammar.rs` is interspersed with `// test`
comments. These comments help to map grammar's code to the TOML
syntax, and they are real regression tests as well: `cargo xtask gen-test`
collects all such comments and dumps them as test-cases to
`tests/data/inline`. Additional parser/lexer tests are found in
`tests/data/**`. Each tests is a pair of `.toml` file and a `.txt`
file with serialized CST representation.

Parser detects only strictly syntactical errors. Problems like "no
newlines are allowed in inline tables" are detected by an additional
validation pass over the CST. See `validator` for details.

## AST

AST is layered on top of the CST: each AST node is just a CST node
which remembers, at the type level, node's `Symbol`. As with `CST`,
you'll need to pass `&TomlDoc` as an argument to get anything useful.

AST lives in the `ast` module, which is generated by the `cargo
gen-ast` command.

## Editing

The underlying `tree::Tree` is mutable and document-editing API builds
on that. It is specified in the `edit.rs` file and is more-or less
just a wrapper of the corresponding `tree::Tree` API.

One interesting bit is that to create a completely new node, we just
parse it from text. That way, arbitrary comments and whitespace are
supported.

Because edits can create intermediate invalid documents, an edit
operation has to be explicitly delimited (`start/finish _edit`).
