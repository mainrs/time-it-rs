# time-it-rs

>  A simple timing proc-macro.

## Usage

```rust
use time_it::time_it;

#[time_it]
fn test(a: i16, b: String) -> Vec<u64> {
    todo!()
}
```

This will log the following message to your terminal:

```console
$ cargo run
test(i16, String) -> Vec<u64>: <your execution time>
```

#### License

<sup>
Licensed under either of <a href="license-apache">Apache License, Version
2.0</a> or <a href="license-mit">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>
