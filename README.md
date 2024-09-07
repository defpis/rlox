# RLox

### Use nightly

```bash
rustup install nightly
rustup override set nightly
```

### Run REPL

```bash
cargo run --package rlox --bin rlox
```

### Run hello.lox

```bash
cargo run --package rlox --bin rlox hello.lox
```

### Run all tests

```bash
cargo test
```

### Test all [lox scripts](/tests/scripts/)

```bash
cargo test --package rlox --test lox_test -- lox_test --show-output
```
