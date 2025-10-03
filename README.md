# Zero2Prod

## Before starting

- I already had rust installed, but I did need to run 

```
rustup update stable
rustup default stable
```

## Chapter 1
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


## sqlx
- `sqlx migrate add create_subscriptions_table` to create a new migration
- `sqlx migrate run` to migrate
- `cargo sqlx prepare --workspace -- --all-targets`