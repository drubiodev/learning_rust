*Examples are from [The Rust Programming Language](https://doc.rust-lang.org/book/) book by Steve Klabnik and Carol Nichols*

# Todo
- [ ] Understand of Ownership
- [ ] Understand lifetimes

# Updating and Unistalling

- rustup update
- rustup self unistall

# Documantion

- rustup doc
- rustup docs --book

# Build

- rustc <filename>

# Cargo

- cargo new <project_name>
- cargo build
- cargo run
- cargo watch -x "run" (cargo install cargo-watch)
- cargo check (checks if code compiles)
- cargo build --release (target/release)
- cargo init (creates project in same folder)
- cargo clean (clean everything and just leave source code)
- cargo doc (generates documentation for project)
- cargo install cargo-edit (installs cargo add)

# Crates

- crates.io
- add reference to crates in .toml file under dependencies
- cargo build
- cargo update
- cargo doc --open (build documentation locally)

## Tools

### rustfmt

To install:

```sh
rustup component add rustfmt
```

To run on a cargo project in the current working directory:

```sh
cargo fmt
```

### rust-clippy

#### Step 1: Install rustup

You can install [rustup](https://rustup.rs/) on supported platforms. This will help
us install Clippy and its dependencies.

If you already have rustup installed, update to ensure you have the latest
rustup and compiler:

```terminal
rustup update
```

#### Step 2: Install Clippy

Once you have rustup and the latest stable release (at least Rust 1.29) installed, run the following command:

```terminal
rustup component add clippy
```
If it says that it can't find the `clippy` component, please run `rustup self update`.

#### Step 3: Run Clippy

Now you can run Clippy by invoking the following command:

```terminal
cargo clippy
```

#### Automatically applying Clippy suggestions

Clippy can automatically apply some lint suggestions.
Note that this is still experimental and only supported on the nightly channel:

```terminal
cargo clippy --fix -Z unstable-options
```

### Running Clippy from the command line without installing it

To have cargo compile your crate with Clippy without Clippy installation
in your code, you can use:

```terminal
cargo run --bin cargo-clippy --manifest-path=path_to_clippys_Cargo.toml
```

*Note:* Be sure that Clippy was compiled with the same version of rustc that cargo invokes here!
