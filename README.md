# cargo-lichking

Automated **li**cense **ch**ec**king** for rust. `cargo lichking` is a [Cargo][]
subcommand that checks licensing information for dependencies.

**Liches are not lawyers**, the information output from this tool is provided as
a hint to where you may need to look for licensing issues but in no way
represents legal advice or guarantees correctness. The tool relies at a minimum
on package metadata containing correct licensing information, this is not
guaranteed so for real license checking it's necessary to verify all
dependencies manually.

To install simply run
`cargo install --git https://github.com/Nemo157/cargo-lichking --tag 0.2.1`
(coming to a crates.io near you soon). Unless you're using a homebrew installed
copy of openssl, then *"simply"* run:

```shall
OPENSSL_ROOT_DIR=`brew --prefix openssl` \
OPENSSL_LIB_DIR=`brew --prefix openssl`/lib \
OPENSSL_INCLUDE_DIR=`brew --prefix openssl`/include \
cargo install --git https://github.com/Nemo157/cargo-lichking --tag 0.2.1
```

To get a list of all your (transitive) dependencies licenses just run `cargo
lichking`. To check license compatibility based off this [License Slide][] by
David A. Wheeler run `cargo lichking --check`.

[Cargo]: https://github.com/rust-lang/cargo
[License Slide]: http://www.dwheeler.com/essays/floss-license-slide.html

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.

[release-badge]: https://img.shields.io/badge/crate-coming--soon-yellowgreen.svg?style=flat-square
[cargo]: https://crates.io/crates/git-appraise
[git-appraise]: https://github.com/google/git-appraise
[git2-rs]: https://github.com/alexcrichton/git2-rs

## Developing

If building on OS X with a `homebrew` installed copy of OpenSSL you'll need to
specify where this is to enable building `libssh2-sys`.  Use something like:

```sh
OPENSSL_ROOT_DIR=`brew --prefix openssl` \
OPENSSL_LIB_DIR=`brew --prefix openssl`/lib \
OPENSSL_INCLUDE_DIR=`brew --prefix openssl`/include \
cargo build
```

When running via `cargo run` you'll need to provide an initial `lichking`
argument to simulate running as a cargo subcommand, e.g. `cargo run -- lichking
--check`.
