# Minimizing Rust Binary Size

| Build Status |                                                                                |
|--------------|--------------------------------------------------------------------------------|
| Travis       | [![Travis Build Status][travis-build-status-svg]][travis-build-status]         |

This repository demonstrates how to minimize the size of a Rust binary.

By default, Rust optimizes for execution speed rather than binary size, since for the vast
majority of applications this is ideal. But for situations where a developer wants to optimize
for binary size instead, Rust provides mechanisms to accomplish this.

# Build in Release Mode

By default, `cargo build` builds the Rust binary in debug mode. Debug mode disables many
optimizations, which helps debuggers (and IDEs that run them) provide a better debugging
experience. Debug binaries can be 30% or more larger than release binaries.

To minimize binary size, build in release mode:

```bash
$ cargo build --release
```

<!-- Badges -->
[travis-build-status]: https://travis-ci.org/johnthagen/min-sized-rust
[travis-build-status-svg]: https://travis-ci.org/johnthagen/min-sized-rust.svg?branch=master
