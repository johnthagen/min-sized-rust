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

# `strip` Symbols from Binary

By default on Linux and macOS, symbol information is included in the compiled `.elf` file. This
information is not needed to properly execute the binary.
To remove this, run [`strip`](https://linux.die.net/man/1/strip) on the `.elf` file:

```bash
$ strip target/release/min-sized-rust
```

# Optimize For Size

![Minimum Rust: 1.28](https://img.shields.io/badge/Minimum%20Rust%20Version-1.28-brightgreen.svg)

As mentioned earlier, the Rust compiler, `rustc`, defaults its optimization level to `O2`,
which optimizes the binary for speed. To instruct `rustc` to optimize for minimal binary
size, use the `z` optimization level in 
[`Cargo.toml`](https://doc.rust-lang.org/cargo/reference/manifest.html):

```toml
[profile.release]
opt-level = 'z'  # Optimize for size.
```

# Enable Link Time Optimization (LTO)

Be default, compilation units are compiled and optimized in isolation. 
[LTO](https://llvm.org/docs/LinkTimeOptimization.html) instructs the linker to optimize at the
link stage. This can, for example, remove dead code and often times reduces binary size.

Enable LTO in `Cargo.toml`:

```toml
[profile.release]
lto = true
```

# Disable Jemalloc

![Minimum Rust: 1.28](https://img.shields.io/badge/Minimum%20Rust%20Version-1.28-brightgreen.svg)

To improve performance, Rust bundles 
[jemalloc](https://github.com/jemalloc/jemalloc), an allocator that often outperforms the
defeault system allocator, on some platforms. Bundling jemalloc does add around 200KB to the
resulting binary, however.

> Note that per https://github.com/rust-lang/rust/issues/36963#issuecomment-445553110, 
Rust 1.32 stable will soon default to use the system allocator, at which point these 
steps will no longer be necessary.

Add this code to the top of `main.rs`:

```rust
use std::alloc::System;

#[global_allocator]
static A: System = System;
```

# Abort on Panic

![Minimum Rust: 1.10](https://img.shields.io/badge/Minimum%20Rust%20Version-1.10-brightgreen.svg)

> **Note**: Up to this point, the features discussed to reduce binary size did not have an
impact on the behaviour of the program (only its execution speed). This feature does
have an impact on behavior.

By default, when Rust code encounters a situation when it must call `panic!()`, it unwinds the
stack and produces a helpful backtrace. The unwinding code, however, does require extra binary
size. `rustc` can be instructed to abort immediately rather than unwind, which removes the
need for this extra unwinding code.

Enable this in `Cargo.toml`:

```toml
[profile.release]
panic = 'abort'
```

<!-- Badges -->
[travis-build-status]: https://travis-ci.org/johnthagen/min-sized-rust
[travis-build-status-svg]: https://travis-ci.org/johnthagen/min-sized-rust.svg?branch=master
