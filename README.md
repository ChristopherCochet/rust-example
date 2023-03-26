# rust-example

- This repo example [here](https://www.rust-lang.org/learn/get-started)
- Hello world example [here](https://doc.rust-lang.org/book/ch01-02-hello-world.html)
- The rust standard library [here](https://doc.rust-lang.org/std/index.html)
- VSCode extension for rust [here](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Installing rust

```
$curl https://sh.rustup.rs -sSf | sh

info: downloading installer

Welcome to Rust!

This will download and install the official compiler for the Rust
programming language, and its package manager, Cargo.

Rustup metadata and toolchains will be installed into the Rustup
home directory, located at:

  /home/codespace/.rustup

This can be modified with the RUSTUP_HOME environment variable.

The Cargo home directory is located at:

  /home/codespace/.cargo

This can be modified with the CARGO_HOME environment variable.

The cargo, rustc, rustup and other commands will be added to
Cargo's bin directory, located at:

  /home/codespace/.cargo/bin

This path will then be added to your PATH environment variable by
modifying the profile files located at:

  /home/codespace/.profile
  /home/codespace/.bashrc
  /home/codespace/.zshenv

You can uninstall at any time with rustup self uninstall and
these changes will be reverted.

Current installation options:


   default host triple: x86_64-unknown-linux-gnu
     default toolchain: stable (default)
               profile: default
  modify PATH variable: yes

1) Proceed with installation (default)
2) Customize installation
3) Cancel installation
>1

info: profile set to 'default'
info: default host triple is x86_64-unknown-linux-gnu
info: syncing channel updates for 'stable-x86_64-unknown-linux-gnu'
info: latest update on 2023-03-23, rust version 1.68.1 (8460ca823 2023-03-20)
info: downloading component 'cargo'
info: downloading component 'clippy'
info: downloading component 'rust-docs'
info: downloading component 'rust-std'
info: downloading component 'rustc'
info: downloading component 'rustfmt'
info: installing component 'cargo'
info: installing component 'clippy'
info: installing component 'rust-docs'
 19.5 MiB /  19.5 MiB (100 %)   3.8 MiB/s in  5s ETA:  0s
info: installing component 'rust-std'
 29.8 MiB /  29.8 MiB (100 %)   8.4 MiB/s in  3s ETA:  0s
info: installing component 'rustc'
 68.3 MiB /  68.3 MiB (100 %)  10.4 MiB/s in  6s ETA:  0s
info: installing component 'rustfmt'
info: default toolchain set to 'stable-x86_64-unknown-linux-gnu'

  stable-x86_64-unknown-linux-gnu installed - rustc 1.68.1 (8460ca823 2023-03-20)


Rust is installed now. Great!

To get started you may need to restart your current shell.
This would reload your PATH environment variable to include
Cargo's bin directory ($HOME/.cargo/bin).

To configure your current shell, run:
source "$HOME/.cargo/env"
```

## Rust Tools

In the Rust development environment, all tools are installed to the ~/.cargo/bin directory, and this is where you will find the Rust toolchain, including rustc, cargo, and 
rustup.

``` 
$ ls $HOME/.cargo/bin/
cargo  cargo-clippy  cargo-fmt  cargo-miri  clippy-driver  rls  rust-gdb  rust-gdbgui  rust-lldb  rustc  rustdoc  rustfmt  rustup
```

```
# cargo --version
cargo 1.68.1 (115f34552 2023-02-26)
```

## Running Hello-World in a Rust Project

```
$ cargo new hello-rust
     Created binary (application) `hello-rust` package
```

```
$ cat hello-rust/src/main.rs 
fn main() {
    println!("Hello, world!");
}
```

```
$ cargo run
   Compiling hello-rust v0.1.0 (/workspaces/rust-example/hello-rust)
    Finished dev [unoptimized + debuginfo] target(s) in 0.82s
     Running `target/debug/hello-rust`
Hello, world!
```

## Another Rust Project with Dependencies: hello-fellow-rustaceans

```
$ cargo new hello-fellow-rustaceans
```

```
$ cargo run
    Updating crates.io index
  Downloaded smallvec v0.4.5
  Downloaded ferris-says v0.2.1
  Downloaded textwrap v0.13.4
  Downloaded smawk v0.3.1
  Downloaded unicode-width v0.1.10
  Downloaded 5 crates (99.9 KB) in 0.84s
   Compiling smawk v0.3.1
   Compiling unicode-width v0.1.10
   Compiling smallvec v0.4.5
   Compiling textwrap v0.13.4
   Compiling ferris-says v0.2.1
   Compiling hello-fellow-rustaceans v0.1.0 (/workspaces/rust-example/hello-fellow-rustaceans)
    Finished dev [unoptimized + debuginfo] target(s) in 1m 36s
     Running `target/debug/hello-fellow-rustaceans`
 __________________________
< Hello fellow Rustaceans! >
 --------------------------
        \
         \
            _~^~^~_
        \) /  o o  \ (/
          '_   -   _'
          / '-----' \
```

## Resources:

- https://www.rust-lang.org/learn
- https://doc.rust-lang.org/stable/rust-by-example/
- https://www.rust-lang.org/
- https://www.youtube.com/channel/UCaYhcUwRBNscFNUKTjgPFiA
