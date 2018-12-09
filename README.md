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

To improve performance, Rust bundles 
[jemalloc](https://github.com/jemalloc/jemalloc), an allocator that often outperforms the
defeault system allocator, on some platforms. Bundling jemalloc does add around 200KB to the
resulting binary, however.

> Note that per https://github.com/rust-lang/rust/issues/36963, it will soon be the default on
stable to use the system allocator, at which point these steps will not be necessary.

Add this code to the top of `main.rs`:

```rust
use std::alloc::System;

#[global_allocator]
static A: System = System;
```

<!-- Badges -->
[travis-build-status]: https://travis-ci.org/johnthagen/min-sized-rust
[travis-build-status-svg]: https://travis-ci.org/johnthagen/min-sized-rust.svg?branch=master
