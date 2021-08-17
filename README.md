# Minimizing Rust Binary Size

[![GitHub Actions][github-actions-badge]](https://github.com/johnthagen/min-sized-rust/actions)

[github-actions-badge]: https://github.com/johnthagen/min-sized-rust/workflows/build/badge.svg

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

> Note: Looking for a tool to help automated `strip`ing? Check out 
[`cargo-strip`](https://github.com/guedou/cargo-strip) or follow 
[Cargo #3483](https://github.com/rust-lang/cargo/issues/3483).

> See also: [`sstrip`](https://github.com/BR903/ELFkickers) a small utility that removes a few
bytes from an executable that strip leaves behind. `sstrip` should be run after `strip`.

By default on Linux and macOS, symbol information is included in the compiled `.elf` file. This
information is not needed to properly execute the binary.
To remove this, run [`strip`](https://linux.die.net/man/1/strip) on the `.elf` file:

```bash
$ strip target/release/min-sized-rust
```

Available starting `1.45.0-nightly (2020-05-28)`,
[Cargo has `strip` functionality built in](https://doc.rust-lang.org/cargo/reference/unstable.html#profile-strip-option):

![Minimum Rust: Nightly](https://img.shields.io/badge/Minimum%20Rust%20Version-nightly%201.45.0-orange.svg)

Modify `Cargo.toml` in this way:

```toml
cargo-features = ["strip"]

[profile.release]
strip = true  # Automatically strip symbols from the binary.
```

# Optimize For Size

![Minimum Rust: 1.28](https://img.shields.io/badge/Minimum%20Rust%20Version-1.28-brightgreen.svg)

[Cargo defaults its optimization level to `3` for release builds][cargo-profile],
which optimizes the binary for **speed**. To instruct Cargo to optimize for minimal binary
**size**, use the `z` optimization level in 
[`Cargo.toml`](https://doc.rust-lang.org/cargo/reference/manifest.html):

[cargo-profile]: https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles

```toml
[profile.release]
opt-level = "z"  # Optimize for size.
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
![Maximum Rust: 1.31](https://img.shields.io/badge/Maximum%20Rust%20Version-1.31-brightgreen.svg)

As of Rust 1.32, 
[`jemalloc` is removed by default](https://blog.rust-lang.org/2019/01/17/Rust-1.32.0.html). 
**If using Rust 1.32 or newer, no action is needed to reduce binary size regarding this 
feature**.

**Prior to Rust 1.32**, to improve performance on some platforms Rust bundled
[jemalloc](https://github.com/jemalloc/jemalloc), an allocator that often 
outperforms the default system allocator. Bundling jemalloc added around 200KB 
to the resulting binary, however.

To remove `jemalloc` on Rust 1.28 - Rust 1.31, add this code to the top of `main.rs`:

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
panic = "abort"
```

# Optimize `libstd` with `build-std`

![Minimum Rust: Nightly](https://img.shields.io/badge/Minimum%20Rust%20Version-nightly-orange.svg)

> **Note**: See also [Xargo](https://github.com/japaric/xargo), the predecessor to `build-std`.
  [Xargo is currently in maintenance status](https://github.com/japaric/xargo/issues/193).

> Example project is located in the [`build_std`](build_std) folder.

Rust ships pre-built copies of the standard library (`libstd`) with its toolchains. This means
that developers don't need to build `libstd` every time they build their applications. `libstd`
is statically linked into the binary instead.

While this is very convenient there are several drawbacks if a developer is trying to
aggressively optimize for size.

1. The prebuilt `libstd` is optimized for speed, not size.

2. It's not possible to remove portions of `libstd` that are not used in a particular application 
   (e.g. LTO and panic behaviour).

This is where [`build-std`](https://doc.rust-lang.org/cargo/reference/unstable.html#build-std) 
comes in. The `build-std` feature is able to compile `libstd` with your application from the
source. It does this with the `rust-src` component that `rustup` conveniently provides.

Install the appropriate toolchain and the `rust-src` component:

```bash
$ rustup toolchain install nightly
$ rustup component add rust-src --toolchain nightly
```

Build using `build-std`:

```bash
# Find your host's target triple. 
$ rustc -vV
...
host: x86_64-apple-darwin

# Use that target triple when building with build-std.
# Add the =std,panic_abort to the option to make panic = "abort" Cargo.toml option work.
# See: https://github.com/rust-lang/wg-cargo-std-aware/issues/56
$ cargo +nightly build -Z build-std=std,panic_abort --target x86_64-apple-darwin --release
```

Remember to `strip` the resulting executable. On macOS, the final binary size is reduced to 51KB.

# Remove `panic` String Formatting with `panic_immediate_abort`

![Minimum Rust: Nightly](https://img.shields.io/badge/Minimum%20Rust%20Version-nightly-orange.svg)

Even if `panic = "abort"` is specified in `Cargo.toml`, `rustc` will still include panic strings
and formatting code in final binary by default. 
[An unstable `panic_immediate_abort` feature](https://github.com/rust-lang/rust/pull/55011)
has been merged into the `nightly` `rustc` compiler to address this.

To use this, repeat the instructions above to use `build-std`, but also pass the following
[`-Z build-std-features=panic_immediate_abort`](https://doc.rust-lang.org/cargo/reference/unstable.html#build-std-features)
option.

```bash
$ cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort \
    --target x86_64-apple-darwin --release
```

Remember to `strip` the resulting executable. On macOS, the final binary size is reduced to 30KB.

# Remove `core::fmt` with `#![no_main]` and Careful Usage of `libstd`

![Minimum Rust: Nightly](https://img.shields.io/badge/Minimum%20Rust%20Version-nightly-orange.svg)

> Example project is located in the [`no_main`](no_main) folder.

> This section was contributed in part by [@vi](https://github.com/vi)
 
Up until this point, we haven't restricted what utilities we used from `libstd`. In this section
we will restrict our usage of `libstd` in order to reduce binary size further.

If you want an executable smaller than 20 kilobytes, Rust's string formatting code, 
[`core::fmt`](https://doc.rust-lang.org/core/fmt/index.html) must 
be removed. `panic_immediate_abort` only removes some usages of this code. There is a lot of other 
code that uses formatting in some cases. That includes Rust's "pre-main" code in `libstd`.

By using a C entry point (by adding the `#![no_main]` attribute) , managing stdio manually, and 
carefully analyzing which chunks of code you or your dependencies include, you can sometimes 
make use of `libstd` while avoiding bloated `core::fmt`.

Expect the code to be hacky and unportable, with more `unsafe{}`s than usual. It feels like 
`no_std`, but with `libstd`.

Start with an empty executable, ensure 
[`xargo bloat --release --target=...`](https://github.com/RazrFalcon/cargo-bloat) contains no 
`core::fmt` or something about padding. Add (uncomment) a little bit. See that `xargo bloat` now 
reports drastically more. Review source code that you've just added. Probably some external crate or 
a new `libstd` function is used. Recurse into that with your review process
(it requires `[replace]` Cargo dependencies and maybe digging in `libstd`), find out why it 
weighs more than it should. Choose alternative way or patch dependencies to avoid unnecessary
features. Uncomment a bit more of your code, debug exploded size with `xargo bloat` and so on.

On macOS, the final stripped binary is reduced to 8KB.

# Removing `libstd` with `#![no_std]`

![Minimum Rust: 1.30](https://img.shields.io/badge/Minimum%20Rust%20Version-1.30-brightgreen.svg)

> Example project is located in the [`no_std`](no_std) folder.

Up until this point, our application was using the Rust standard library, `libstd`. `libstd`
provides many convenient, well tested cross-platform APIs and data types. But if a user wants
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

# Compress the binary

Up until this point, all size-reducing techniques were Rust-specific. This section describes
a language-agnostic binary packing tool that is an option to reduce binary size further.

[UPX](https://github.com/upx/upx) is a powerful tool for creating a self contained, compressed
binary with no addition runtime requirements.  It claims to typically reduce binary size by 50-70%,
but the actual result depends on your executable.

```bash
$ upx --best --lzma target/release/min-sized-rust
```

It should be noted that there have been times that UPX-packed binaries have flagged
heuristic-based anti-virus software because malware often uses UPX.

# Tools

- [`cargo-bloat`](https://github.com/RazrFalcon/cargo-bloat) - Find out what takes most of the 
  space in your executable.

# Containers

Sometimes it's advantageous to deploy Rust into containers 
(e.g. [Docker](https://www.docker.com/)). There are several great existing resources to help 
create minimum sized Docker containers that run Rust binaries.

- [mini-docker-rust](https://github.com/kpcyrd/mini-docker-rust)
- [rust-musl-builder](https://github.com/emk/rust-musl-builder)
- [muslrust](https://github.com/clux/muslrust)

# References

- [Why is a Rust executable large? - 2016][why-rust-binary-large]
- [Freestanding Rust Binary - 2018](https://os.phil-opp.com/freestanding-rust-binary/)
- [Tiny Rocket - 2018](https://jamesmunns.com/blog/tinyrocket/)
- [Formatting is Unreasonably Expensive for Embedded Rust - 2019][fmt-unreasonably-expensive]
- [Tiny Windows executable in Rust - 2019][tiny-windows-exe]
- [Making a really tiny WebAssembly graphics demos - 2019][tiny-webassembly-graphics]
- [Reducing the size of the Rust GStreamer plugin - 2020][gstreamer-plugin]
- [`min-sized-rust-windows`][min-sized-rust-windows] - Windows-specific tricks to reduce binary size

[why-rust-binary-large]: https://lifthrasiir.github.io/rustlog/why-is-a-rust-executable-large.html
[fmt-unreasonably-expensive]: https://jamesmunns.com/blog/fmt-unreasonably-expensive/
[tiny-windows-exe]: https://www.codeslow.com/2019/12/tiny-windows-executable-in-rust.html
[tiny-webassembly-graphics]: https://cliffle.com/blog/bare-metal-wasm/
[gstreamer-plugin]: https://www.collabora.com/news-and-blog/blog/2020/04/28/reducing-size-rust-gstreamer-plugin/
[min-sized-rust-windows]: https://github.com/mcountryman/min-sized-rust-windows

<!-- Badges -->
[travis-build-status]: https://travis-ci.com/johnthagen/min-sized-rust
[travis-build-status-svg]: https://travis-ci.com/johnthagen/min-sized-rust.svg?branch=master
