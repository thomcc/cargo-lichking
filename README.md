# cargo-lichking <small>automated LIcense CHecKING for rust</small>

`cargo lichking` is a [Cargo][] subcommand that checks licensing information for
dependencies. **Liches are not lawyers**, the information output from this tool
is provided as a hint to where you may need to look for licensing issues but in
no way represents legal advice or guarantees correctness.

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
