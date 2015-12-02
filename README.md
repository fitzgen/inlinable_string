# `inlinable_string`

[![](http://meritbadge.herokuapp.com/inlinable_string)![](https://img.shields.io/crates/d/inlinable_string.png)](https://crates.io/crates/inlinable_string)

[![Build Status](https://travis-ci.org/fitzgen/inlinable_string.png?branch=master)](https://travis-ci.org/fitzgen/inlinable_string)

[![Coverage Status](https://coveralls.io/repos/fitzgen/inlinable_string/badge.svg?branch=master&service=github)](https://coveralls.io/github/fitzgen/inlinable_string?branch=master)

The `inlinable_string` crate provides the `InlinableString` type &mdash; an
owned, grow-able UTF-8 string that stores small strings inline and avoids
heap-allocation &mdash; and the `StringExt` trait which abstracts string
operations over both `std::string::String` and `InlinableString` (or even your
own custom string type).

`StringExt`'s API is mostly identical to `std::string::String`; unstable and
deprecated methods are not included. A `StringExt` implementation is provided
for both `std::string::String` and `InlinableString`. This enables
`InlinableString` to generally work as a drop-in replacement for
`std::string::String` and `&StringExt` to work with references to either type.

## But is it actually faster than using `std::string::String`?

Here are some current (micro)benchmark results. I encourage you to verify them
yourself by running `cargo bench --feature nightly` with a nightly Rust! I am
also very open to adding more realistic and representative benchmarks! Share
some ideas with me!

Constructing from a large `&str`:

```
test benches::bench_inlinable_string_from_large     ... bench:          23 ns/iter (+/- 3)
test benches::bench_std_string_from_large           ... bench:          22 ns/iter (+/- 1)
```

Constructing from a small `&str`:

```
test benches::bench_inlinable_string_from_small     ... bench:           1 ns/iter (+/- 0)
test benches::bench_std_string_from_small           ... bench:          20 ns/iter (+/- 1)
```

Pushing a large `&str` onto an empty string:

```
test benches::bench_inlinable_string_push_str_large ... bench:          33 ns/iter (+/- 3)
test benches::bench_std_string_push_str_large       ... bench:          24 ns/iter (+/- 1)
```

Pushing a small `&str` onto an empty string:

```
test benches::bench_inlinable_string_push_str_small ... bench:          11 ns/iter (+/- 1)
test benches::bench_std_string_push_str_small       ... bench:          22 ns/iter (+/- 2)
```

## Install

Either

    $ cargo add inlinable_string

or add this to your `Cargo.toml`:

    [dependencies]
    inlinable_string = "0.1.0"

## Documentation

[Documentation](http://fitzgen.github.io/inlinable_string/inlinable_string/index.html)
