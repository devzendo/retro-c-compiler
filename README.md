# retro-c-compiler

## What is this?
A C compiler, targetting the Transputer for the Parachute project, EPOC16 (the
OS of the Psion Series 3 PDAs), and x86_64 (for testing).

Built in Rust with TDD, and following the book ["Writing a C Compiler" by Nora Sandler]
(https://nostarch.com/writing-c-compiler).

It is part of the [Parachute Project](https://devzendo.github.io/parachute).

## Project Status
Actively in development.

Started May 2024, there's currently the compiler driver, and the start of the actual
compiler. 

On Chapter 1, writing the compiler's lexer.

# Overview
The intention is to build a C compiler using the progressive approach from
Nora's book. Following the book closely for the EPOC16 target; adapting it as
needed for the Transputer, which is quite different to the 8086.

* Language: (eventually) C89, some C99 features; Objective-C extensions.
* Runs on: Linux (Debian derivatives), HaikuOS, Windows, the BSD family, macOS.
  Maybe docker?
* Continuous test/build provided by CircleCI (Linux).

This project will provide a compiler driver, and the C compiler itself. It will
not provide a preprocessor (we'll depend on gcc's preprocessor), assembler, or
linker.

## Transputer requirements
It should be able to generate optimised assembly for the Parachute
TMASM assembler, for the T425ish that is currently emulated.
* Target: T425

## EPOC16 requirements
It should generate optimised assembly for an as-yet-undecided assembler.
* Target: 8086; NEC V20 extensions
* Replicate the output of JPI/Clarion TopSpeed C 3.10 as closely as possible
  (pure small memory model)


# Development

## Technology
All code is in Rust, and is developed as far as is practical using Test Driven
Development.

The intention with TDD is two-fold: 
* to provide a pressure against coupled designs (if it's hard to get into a test, it's too coupled)
* to provide an indicator of quality (if all the tests pass, we can be confident it is shippable)

## Project structure
The project uses Cargo workspaces, with several modules in subdirectories
under the 'crates' directory: See [https://matklad.github.io/2021/08/22/large-rust-workspaces.html]

* rcc is the main compiler driver, the program you use to do compilation. It
  calls the preprocessor, assembler etc.
* rcc1 is the C compiler proper. It translates preprocessed C into the
  appropriate assembler.
* common is code that's common to the other crates.
* test_common is code for use in tests, that's common to the other crates.

## Building
You will need Rust, at least version 1.74.0. Only the stable version is used.
To run tests, and build the binaries:

* cargo test
* cargo build --release

This will give you two executables: `target/release/rcc` and `target/release/rcc1`.

# Packaging
At some point, the executables will be packaged into the relevant package formats for the
various OSs: .deb, whatever HaikuOS uses, .msi, .pkg.. or perhaps just a .zip that you
extract somewhere and add to the PATH.

The executables will also be shipped as part of the overall Parachute project.

# Documentation
When there is some, it'll be in the 'docs' directory.


# Acknowledgements
Alex Brown for planting the seed of the project. See [https://hackaday.io/project/161291-the-last-psion]

Nora Sandler for her blog posts, that eventually turned into the 'Writing a C
Compiler' book.

Brian Kernighan & the late Dennis Ritchie, of course!


# License, Copyright & Contact info
This code is released under the Apache 2.0 License: http://www.apache.org/licenses/LICENSE-2.0.html.

(C) 2024 Matt J. Gumbley and hopefully others!

matt.gumbley@devzendo.org

Mastodon: @M0CUV@mastodon.radio

http://devzendo.github.io/parachute


