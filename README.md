# retro-c-compiler

## What is this?
A C compiler, targetting the Transputer for the Parachute project, and SIBO (the
OS of the Psion Series 3 PDAs). Built in Rust with TDD, and following the book
["Writing a C Compiler" by Nora Sandler]
(https://nostarch.com/writing-c-compiler).

## Project Status
Started May 2024, there's currently nothing to see here.
On Chapter 1, writing the compiler driver.

# Overview
The intention is to build a C compiler using the progressive approach from
Nora's book. Following the book closely for the SIBO target; adapting it as
needed for the Transputer, which is quite different to the 8086.

* Language: (eventually) C89, some C99 features; Objective-C extensions.
* Runs on: Linux, HaikuOS, Windows, the BSD family, macOS.

This project will provide a compiler driver, and the C compiler itself. It will
not provide a preprocessor (we'll depend on gcc's preprocessor), assembler, or
linker.

## Transputer requirements
It should be able to generate optimised assembly for the Parachute
TMASM assembler, for the T425ish that is currently emulated.
* Target: T425

## SIBO requirements
It should generate optimised assembly for an as-yet-undecided assembler.
* Target: 8086; NEC V20 extensions
* Replicate the output of JPI/Clarion TopSpeed C 3.10 as closely as possible
  (pure small memory model)


# Development

## Technology
All code is in Rust, and is developed as far as is practical using Test Driven
Development.

## Building
You will need Rust, at least version 1.75.0. Only the stable version is used.
To run tests, and build the binaries:

* cargo test
* cargo build --release


# Documentation
When there is some, it'll be in the 'docs' directory.


# Acknowledgements
Alex Brown for planting the seed of the project.

Nora Sandler for her blog posts, that eventually turned into the 'Writing a C
Compiler' book.

Brian Kernighan & the late Dennis Ritchie, of course!


# License, Copyright & Contact info
This code is released under the Apache 2.0 License: http://www.apache.org/licenses/LICENSE-2.0.html.

(C) 2024 Matt J. Gumbley and hopefully others!

matt.gumbley@devzendo.org

Mastodon: @M0CUV@mastodon.radio

http://devzendo.github.io/parachute


