# Rust Serde JSON Types Test

Quick test to demo difference between Serde JSON handling of Struct types vs. Value types.

TL;DR - _Struct type prints values as a Rust string, Value types includes JSON double quotes when printing_

## Usage

```
rust-json-test $> cargo build
rust-json-test $> cargo run
```

Expected output:

```
rust-json-test (master ✔) ᐅ cargo build
   Compiling proc-macro2 v1.0.13
   Compiling syn v1.0.22
   Compiling serde v1.0.110
   Compiling ryu v1.0.4
   Compiling quote v1.0.6
   Compiling serde_derive v1.0.110
   Compiling serde_json v1.0.53
   Compiling rust-test v0.0.1 (../rust-json-test)
    Finished dev [unoptimized + debuginfo] target(s) in 36.36s
rust-json-test (master ✔) ᐅ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/rust-test`
Struct example of name John Doe and number +44 1234567
Value example of name "John Doe" and number "+44 1234567"
```
