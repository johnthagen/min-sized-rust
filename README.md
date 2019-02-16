# Minimizing Rust Binary Size

| Build Status |                                                                                |
|--------------|--------------------------------------------------------------------------------|
| Travis       | [![Travis Build Status][travis-build-status-svg]][travis-build-status]         |

This repository demonstrates how to minimize the size of a Rust binary.

By default, Rust optimizes for execution speed rather than binary size, since for the vast
majority of applications this is ideal. But for situations where a developer wants to optimize
for binary size instead, Rust provides mechanisms to accomplish this.

# Build in Release Mode

![Minimum Rust: 1.0](https://img.shields.io/badge/Minimum%20Rust%20Version-1.0-brightgreen.svg)

By default, `cargo build` builds the Rust binary in debug mode. Debug mode disables many
optimizations, which helps debuggers (and IDEs that run them) provide a better debugging
experience. Debug binaries can be 30% or more larger than release binaries.

To minimize binary size, build in release mode:

```bash
$ cargo build --release
```

# `strip` Symbols from Binary

![OS: *nix](https://img.shields.io/badge/OS-*nix-brightgreen.svg)

By default on Linux and macOS, symbol information is included in the compiled `.elf` file. This
information is not needed to properly execute the binary.
To remove this, run [`strip`](https://linux.die.net/man/1/strip) on the `.elf` file:

```bash
$ strip target/release/min-sized-rust
```

# Optimize For Size

![Minimum Rust: 1.28](https://img.shields.io/badge/Minimum%20Rust%20Version-1.28-brightgreen.svg)

[Cargo defaults its optimization level to `3` for release builds][cargo-profile],
which optimizes the binary for speed. To instruct Cargo to optimize for minimal binary
size, use the `z` optimization level in 
[`Cargo.toml`](https://doc.rust-lang.org/cargo/reference/manifest.html):

[cargo-profile]: https://doc.rust-lang.org/cargo/reference/manifest.html#the-profile-sections

```toml
[profile.release]
opt-level = 'z'  # Optimize for size.
```

# Enable Link Time Optimization (LTO)

![Minimum Rust: 1.0](https://img.shields.io/badge/Minimum%20Rust%20Version-1.0-brightgreen.svg)

By default, 
[Cargo instructs compilation units to be compiled and optimized in isolation][cargo-profile]. 
[LTO](https://llvm.org/docs/LinkTimeOptimization.html) instructs the linker to optimize at the
link stage. This can, for example, remove dead code and often times reduces binary size.

Enable LTO in `Cargo.toml`:

```toml
[profile.release]
lto = true
```

# Remove Jemalloc

![Minimum Rust: 1.28](https://img.shields.io/badge/Minimum%20Rust%20Version-1.28-brightgreen.svg)
![Maximum Rust: 1.32](https://img.shields.io/badge/Maximum%20Rust%20Version-1.32-brightgreen.svg)

As of Rust 1.32, 
[`jemalloc` is removed by default](https://blog.rust-lang.org/2019/01/17/Rust-1.32.0.html). If
using Rust 1.32 or newer, no action is needed to reduce binary size regarding this feature.

**Prior to Rust 1.32**, to improve performance Rust bundled
[jemalloc](https://github.com/jemalloc/jemalloc), an allocator that often outperforms the
defeault system allocator, on some platforms. Bundling jemalloc added around 200KB to the
resulting binary, however.

To remove `jemalloc` on Rust 1.28 - Rust 1.32, add this code to the top of `main.rs`:

```rust
use std::alloc::System;

#[global_allocator]
static A: System = System;
```

# Reduce Parallel Code Generation Units to Increase Optimization

[By default][cargo-profile], Cargo specifies 16 parallel codegen units for release builds.
This improves compile times, but prevents some optimizations. 

Set this to `1` in `Cargo.toml` to allow for maximum size reduction optimizations:

```toml
[profile.release]
codegen-units = 1
```

# Abort on Panic

![Minimum Rust: 1.10](https://img.shields.io/badge/Minimum%20Rust%20Version-1.10-brightgreen.svg)

> **Note**: Up to this point, the features discussed to reduce binary size did not have an
impact on the behaviour of the program (only its execution speed). This feature does
have an impact on behavior.

[By default][cargo-profile], when Rust code encounters a situation when it must call `panic!()`, 
it unwinds the stack and produces a helpful backtrace. The unwinding code, however, does require 
extra binary size. `rustc` can be instructed to abort immediately rather than unwind, which 
removes the need for this extra unwinding code.

Enable this in `Cargo.toml`:

```toml
[profile.release]
panic = 'abort'
```

# Optimize `libstd` with Xargo

![Minimum Rust: Nightly](https://img.shields.io/badge/Minimum%20Rust%20Version-nightly-orange.svg)

> **Note**: [Xargo is currently in maintenance status](https://github.com/japaric/xargo/issues/193),
  but eventually the features used below should make their way into Cargo.

> Example project is located in the [`xargo`](xargo) folder.

Rust ships pre-built copies of the standard library (`libstd`) with its toolchains. This means
that developers don't need to build `libstd` every time they build their applications. `libstd`
is statically linked into the binary instead.

While this is very convenient there are several drawbacks if a developer is trying to
aggressively optimize for size.

1. The prebuilt `libstd` is optimized for speed, not size.

2. It's not possible to remove portions of `libstd` that are not used in a particular application 
   (e.g. LTO and panic behaviour).

This is where [Xargo](https://github.com/japaric/xargo) comes in. Xargo is able to compile
`libstd` with your application from the source. It does this with the `rust-src` component that
`rustup` conveniently provides.

Modify `main.rs`:

```rust
fn main() {
    println!("Hello, world!");
}
```

Add a `Xargo.toml` file to the root of your project 
(this doesn't replace `Cargo.toml`, just is in addition):

```toml
[dependencies]
std = {default-features=false}
```

Install the appropriate toolchain and Xargo:

```bash
$ rustup toolchain install nightly
$ rustup default nightly
$ rustup component add rust-src
$ cargo install xargo
```

Build using Xargo:

```bash
# Find your host's target triple. 
$ rustc -vV
...
host: x86_64-apple-darwin

# Use that target triple when building with Xargo.
$ xargo build --target x86_64-apple-darwin --release
```

Remember to `strip` the resulting executable. On macOS, the final binary size is reduced to 51KB.

# Remove `panic` String Formatting with `panic_immediate_abort`

![Minimum Rust: Nightly](https://img.shields.io/badge/Minimum%20Rust%20Version-nightly-orange.svg)

> Example project is located in the [`panic_immediate_abort`](panic_immediate_abort) folder.

Even if `panic = abort` is specified in `Cargo.toml`, `rustc` will still include panic strings
and formatting code in final binary by default. 
[An unstable `panic_immediate_abort` feature](https://github.com/rust-lang/rust/pull/55011)
has been merged into the `nightly` `rustc` compiler to address this.

To use this, repeat the instructions above to use Xargo, but instead use the following
`Xargo.toml`:

```toml
[dependencies]
std = {default-features=false, features=["panic_immediate_abort"]}
```

Remember to `strip` the resulting executable. On macOS, the final binary size is reduced to 30KB.

# Removing `libstd` with `![no_std]`

![Minimum Rust: 1.30](https://img.shields.io/badge/Minimum%20Rust%20Version-1.30-brightgreen.svg)

> Example project is located in the [`no_std`](no_std) folder.

Up until this point, our application was using the Rust standard library, `libstd`. `libstd`
provides many convenient, well tested cross platform APIs and data types. But if a user wants
to reduce binary size to an equivalent C program size, it is possible to depend only on `libc`.

It's important to understand that there are many drawbacks to this approach. For one, you'll
likely need to write a lot of `unsafe` code and lose access to a majority of Rust crates
that depend on `libstd`. Nevertheless, it is one (albeit extreme) option to reducing binary size.

A `strip`ed binary built this way is around 8KB.

```rust
#![no_std]
#![no_main]

extern crate libc;

#[no_mangle]
pub extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    // Since we are passing a C string the final null character is mandatory.
    const HELLO: &'static str = "Hello, world!\n\0";
    unsafe {
        libc::printf(HELLO.as_ptr() as *const _);
    }
    0
}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
```

# References

- [Why is a Rust executable large? - 2016][why-rust-binary-large]

[why-rust-binary-large]: https://lifthrasiir.github.io/rustlog/why-is-a-rust-executable-large.html

- [Freestanding Rust Binary - 2018](https://os.phil-opp.com/freestanding-rust-binary/)

<!-- Badges -->
[travis-build-status]: https://travis-ci.org/johnthagen/min-sized-rust
[travis-build-status-svg]: https://travis-ci.org/johnthagen/min-sized-rust.svg?branch=master
