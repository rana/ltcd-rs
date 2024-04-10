# ltcd

LeetCode solutions in Rust.


## Usage

Run individual test metrics at the terminal.
```sh
clear && cargo test --test <test-name>
```
Or, with the VSCode IDE.

* Compiler optimizations for tests are configured in `Cargo.toml`.
```toml
[profile.test]
opt-level = 3
```
  
# Notes

Use nightly build to support metrics from the [ben](https://github.com/rana/ben) library.

```sh
rustup default nightly
```