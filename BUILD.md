# Building Artichoke

To build Artichoke, install the [prerequisites](#prerequisites) and run:

```console
$ git clone https://github.com/artichoke/artichoke.git
$ cd ./artichoke
$ cargo run --bin artichoke -- --version
$ cargo run --bin airb
```

## WebAssembly

Artichoke can be used in WebAssembly environments via the
`wasm32-unknown-emscripten` target. This target is not tested in CI and should
be considered unstable.

```sh
rustup target add wasm32-unknown-emscripten
cargo build --release --target wasm32-unknown-unknown
```

This on its own does not produce a usable artifact. To build a WebAssembly
bundle, depend on `artichoke` in a crate with a main. See the
[artichoke/playground] repository for an example.

[artichoke/playground]: https://github.com/artichoke/playground

## Prerequisites

### Rust Toolchain

Artichoke is a collection of Rust crates and requires a Rust compiler. The
specific version of Rust Artichoke requires is specified in the [toolchain
file].

Artichoke only guarantees support for the latest stable version of the Rust
compiler.

[toolchain file]: rust-toolchain.toml

#### Installation

The recommended way to install the Rust toolchain is with [rustup]. On macOS,
you can install rustup with [Homebrew]:

```sh
brew install rustup-init
rustup-init
```

On Windows, you can install rustup from the official site and follow the
prompts: <https://win.rustup.rs/>. This will automatically install Visual Studio
(the [Community Edition][vs-community]) and several C++ packages selected
through the VS component installer.

Once rustup is installed, ensure the correct toolchain is present by running:

```sh
rustup install
```

It is recommended to install `rustfmt` and `clippy` to help with static code
analysis and to do relevant checks prior to submitting PRs.

```sh
rustup component add rustfmt clippy
```

[rustup]: https://rustup.rs/
[homebrew]: https://docs.brew.sh/Installation
[vs-community]: https://visualstudio.microsoft.com/vs/community/

### Bindgen

Artichoke generates Rust declarations for C code at build time using
[`bindgen`]. `bindgen` is a build dependency of `artichoke-backend` and the
bindgen CLI is not required to be present on `$PATH`.

Bindgen requires a `libclang` shared object to be discoverable. On macOS, no
steps are necessary, but you may wish to install the lates LLVM, which you can
do with [Homebrew]:

```sh
brew install llvm
```

On Windows, install LLVM from `winget`:

```powershell
winget install --id=LLVM.LLVM -e
```

[`bindgen`]: https://github.com/rust-lang/rust-bindgen

### Rust Crates

Artichoke depends on several Rust libraries, or crates. Once you have the Rust
toolchain installed, you can install the crates specified in
[`Cargo.lock`](Cargo.lock) by running:

```sh
cargo build --workspace
```

### C Toolchain

Some artichoke dependencies, like the mruby [`sys`] FFI bindings and the
[`onig`] crate, build C static libraries and require a C compiler.

Artichoke specifically requires clang. WebAssembly targets require clang-8 or
newer.

On Windows, install the latest LLVM distribution from GitHub and add LLVM to
your PATH: <https://github.com/llvm/llvm-project/releases>.

[`sys`]: artichoke-backend/src/sys
[`onig`]: https://crates.io/crates/onig

#### `cc` Crate

Artichoke and some of its dependencies use the Rust [`cc` crate] to build. `cc`
uses a [platform-dependent C compiler] to compile C sources. On Unix, `cc` crate
uses the `cc` binary.

[`cc` crate]: https://crates.io/crates/cc
[platform-dependent c compiler]:
  https://github.com/alexcrichton/cc-rs#compile-time-requirements

### mruby Bindings

To build the Artichoke mruby backend, you will need a C compiler toolchain. By
default, mruby requires the following to compile:

- clang
- ar

You can override the requirement for clang by setting the `CC` and `LD`
environment variables.
