# `inlinable_string`

[![](http://meritbadge.herokuapp.com/inlinable_string) ![](https://img.shields.io/crates/d/inlinable_string)](https://crates.io/crates/inlinable_string)

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

## Install

Either

    $ cargo add inlinable_string

or add this to your `Cargo.toml`:

    [dependencies]
    inlinable_string = "0.1.0"

## Documentation

[Documentation](http://fitzgen.github.io/inlinable_string/inlinable_string/index.html)
