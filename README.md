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

As mentioned earlier, the Rust compiler, `rustc`, defaults its optimization level to `O2`,
which optimizes the binary for speed. To instruct `rustc` to optimize for minimal binary
size, use the `z` optimization level in 
[`Cargo.toml`](https://doc.rust-lang.org/cargo/reference/manifest.html):

```toml
[profile.release]
opt-level = 'z'  # Optimize for size.
```

# Enable Link Time Optimization (LTO)

![Minimum Rust: 1.0](https://img.shields.io/badge/Minimum%20Rust%20Version-1.0-brightgreen.svg)

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

# Optimize `std` with Xargo

![Nightly-2018-10-31](https://img.shields.io/badge/Rust%20Version-nightly_2018/10/31-orange.svg)

> **Note**: [Xargo is currently in maintenance status](https://github.com/japaric/xargo/issues/193),
  but eventually the features used below should make their way into Cargo.

> **Note**: Sometime in early November, 2018 `nightly` broke the way Xargo is used in this example,
so the nightly has been pinned.

> Example project is located in the [`xargo`](xargo) folder.

Rust ships pre-built copies of the standard library (`std`) with its toolchains. This means
that developers don't need to build `std` every time they build their applications. `std`
is statically linked into the binary instead.

While this is very convenient there are several drawbacks if a developer is trying to
aggressively optimize for size.

1. The prebuilt `std` is optimized for speed, not size.

2. It's not possible to remove portions of `std` that are not used in a particular application 
   (LTO).

This is where [Xargo](https://github.com/japaric/xargo) comes in. Xargo is able to compile
`std` with your application from the source. It does this with the `rust-src` component that
`rustup` conveniently provides.

Modify `main.rs`:

```rust
// Xargo must use a different way to disable jemalloc.
//use std::alloc::System;
//
//#[global_allocator]
//static A: System = System;

fn main() {
    println!("Hello, world!");
}
```

Add a `Xargo.toml` file to the root of your project 
(this doesn't replace `Cargo.toml`, just is in addition):

```toml
# Xargo.toml
[dependencies.std]
features = ["force_alloc_system"] # Disable jemalloc the Xargo way.
```

Install the appropriate toolchain and Xargo:

```bash
$ rustup toolchain install nightly-2018-10-31
$ rustup default nightly-2018-10-31
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

# Removing `libstd` with `![no_std]`

> Example project is located in the [`no_std`](no_std) folder.

Up until this point, our application with using the Rust standard library, `libstd`. `libstd`
provides many convenient, well tested cross platform APIs and data types. But if a user wants
to reduce binary size to an equivalent C program size, it is possible to depend only on `libc`.

It's important to understand that there are many drawbacks to this approach. For one, you'll
likely need to write a lot of `unsafe` and lose access to a majority of Rust crates
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

# TODO

- [`panic_immediate_abort`](https://github.com/rust-lang/rust/pull/55011)

- `codegen-units`


# References

- [Why is a Rust executable large? - 2016][why-rust-binary-large]

[why-rust-binary-large]: https://lifthrasiir.github.io/rustlog/why-is-a-rust-executable-large.html

- [Freestanding Rust Binary - 2018](https://os.phil-opp.com/freestanding-rust-binary/)

<!-- Badges -->
[travis-build-status]: https://travis-ci.org/johnthagen/min-sized-rust
[travis-build-status-svg]: https://travis-ci.org/johnthagen/min-sized-rust.svg?branch=master
