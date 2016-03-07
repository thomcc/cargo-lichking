# cargo-lichking (automated LIcense CHecKING for rust)

`cargo lichking` is a [Cargo][] subcommand that checks licensing
information for dependencies

It will eventually have compatibility checking based off this [License
Slide][] by David A. Wheeler.

[Cargo]: https://github.com/rust-lang/cargo
[License Slide]: http://www.dwheeler.com/essays/floss-license-slide.html

## Developing

If building on OS X with a `homebrew` installed copy of OpenSSL you'll need to
specify where this is to enable building `libssh2-sys`.  Use something like:

```sh
OPENSSL_ROOT_DIR=`brew --prefix openssl` \
OPENSSL_LIB_DIR=`brew --prefix openssl`/lib \
OPENSSL_INCLUDE_DIR=`brew --prefix openssl`/include \
cargo build
```
