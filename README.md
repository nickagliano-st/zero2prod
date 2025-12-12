# Zero2Prod

This is basically my notes or reminders to myself of helpful commands as I work through the Zero2Prod book.

## Before starting

- I already had rust installed, but I did need to run:

```
rustup update stable
rustup default stable
```

- In chapter 5 we added `cargo-chef`, so to run the `chef_prepare_recipe.sh` script you'll need to install the cargo-chef cli binary, like:
  - `cargo install cargo-chef --locked`

## Chapter 1 notes
- I didn't do the LLVM, faster linker setup (that would go in `.cargo/config`)--might come back to that.
- `rustup component add llvm-tools-preview && cargo install cargo-llvm-cov` for code coverage in CI
  - Run manually with `cargo llvm-cov`
- Linting
  - `cargo clippy`
  - `cargo clippy -- -D warnings`
- Formatting
  - `cargo fmt`
  - `cargo fmt -- --check`
- Security
  - Install like: `cargo install cargo-audit`
  - Run with `cargo audit`


## sqlx commands
- `sqlx migrate add create_subscriptions_table` to create a new migration
- `sqlx migrate run` to migrate
- `cargo sqlx prepare --workspace -- --all-targets`


## Docker commands
- `docker build --tag zero2prod .`