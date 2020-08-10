// Copyright 2015, The inlinable_string crate Developers. See the COPYRIGHT file
// at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! A trait that exists to abstract string operations over any number of
//! concrete string type implementations.
//!
//! See the [crate level documentation](./../index.html) for more.

use std::borrow::{Borrow, BorrowMut, Cow};
use std::cmp::PartialEq;
use std::fmt::Display;
use std::ops::RangeBounds;
use std::str;
use std::string::{FromUtf16Error, FromUtf8Error};

/// A trait that exists to abstract string operations over any number of
/// concrete string type implementations.
///
/// See the [crate level documentation](./../index.html) for more.
pub trait StringExt
where
    for<'a> Self: Sized
        + Display
        + PartialEq<str>
        + PartialEq<String>
        + PartialEq<&'a str>
        + PartialEq<Cow<'a, str>>,
{
    /// Creates a new string buffer initialized with the empty string.
    ///
    /// # Examples
    ///
    /// ```
    /// use inlinable_string::{InlinableString, StringExt};
    ///
    /// let s = InlinableString::new();
    /// ```
    #[inline]
    fn new() -> Self {
        Self::with_capacity(0)
    }

    /// Creates a new string buffer with the given capacity. The string will be
    /// able to hold at least `capacity` bytes without reallocating. If
    /// `capacity` is less than or equal to `INLINE_STRING_CAPACITY`, the string
    /// will not heap allocate.
    ///
    /// # Examples
    ///
    /// ```
    /// use inlinable_string::{InlinableString, StringExt};
    ///
    /// let s = InlinableString::with_capacity(10);
    /// ```
    fn with_capacity(capacity: usize) -> Self;

    /// Returns the vector as a string buffer, if possible, taking care not to
    /// copy it.
    ///
    /// # Failure
    ///
    /// If the given vector is not valid UTF-8, then the original vector and the
    /// corresponding error is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use inlinable_string::{InlinableString, StringExt};
    ///
    /// let hello_vec = vec![104, 101, 108, 108, 111];
    /// let s = InlinableString::from_utf8(hello_vec).unwrap();
    /// assert_eq!(s, "hello");
    ///
    /// let invalid_vec = vec![240, 144, 128];
    /// let s = InlinableString::from_utf8(invalid_vec).err().unwrap();
    /// let err = s.utf8_error();
    /// assert_eq!(s.into_bytes(), [240, 144, 128]);
    /// ```
    #[inline]
    fn from_utf8(vec: Vec<u8>) -> Result<Self, FromUtf8Error> {
        String::from_utf8(vec).map(from_string)
    }

    /// Converts a vector of bytes to a new UTF-8 string.
    /// Any invalid UTF-8 sequences are replaced with U+FFFD REPLACEMENT CHARACTER.
    ///
    /// # Examples
    ///
    /// ```
    /// use inlinable_string::{InlinableString, StringExt};
    ///
    /// let input = b"Hello \xF0\x90\x80World";
    /// let output = InlinableString::from_utf8_lossy(input);
    /// assert_eq!(output, "Hello \u{FFFD}World");
    /// ```
    #[inline]
    fn from_utf8_lossy(v: &[u8]) -> Cow<str> {
        String::from_utf8_lossy(v)
    }

    /// Decode a UTF-16 encoded vector `v` into a `InlinableString`, returning `None`
    /// if `v` contains any invalid data.
    ///
    /// # Examples
    ///
    /// ```
    /// use inlinable_string::{InlinableString, StringExt};
    ///
    /// // 𝄞music
    /// let mut v = &mut [0xD834, 0xDD1E, 0x006d, 0x0075,
    ///                   0x0073, 0x0069, 0x0063];
    /// assert_eq!(InlinableString::from_utf16(v).unwrap(),
    ///            InlinableString::from("𝄞music"));
    ///
    /// // 𝄞mu<invalid>ic
    /// v[4] = 0xD800;
    /// assert!(InlinableString::from_utf16(v).is_err());
    /// ```
    #[inline]
    fn from_utf16(v: &[u16]) -> Result<Self, FromUtf16Error> {
        String::from_utf16(v).map(from_string)
    }

    /// Decode a UTF-16 encoded vector `v` into a string, replacing
    /// invalid data with the replacement character (U+FFFD).
    ///
    /// # Examples
    ///
    /// ```
    /// use inlinable_string::{InlinableString, StringExt};
    ///
    /// // 𝄞mus<invalid>ic<invalid>
    /// let v = &[0xD834, 0xDD1E, 0x006d, 0x0075,
    ///           0x0073, 0xDD1E, 0x0069, 0x0063,
    ///           0xD834];
    ///
    /// assert_eq!(InlinableString::from_utf16_lossy(v),
    ///            InlinableString::from("𝄞mus\u{FFFD}ic\u{FFFD}"));
    /// ```
    #[inline]
    fn from_utf16_lossy(v: &[u16]) -> Self {
        from_string(String::from_utf16_lossy(v))
    }

    /// Creates a new string from a length, capacity, and pointer.
    ///
    /// # Safety
    ///
    /// This function is just a shortened call to two other unsafe functions,
    /// therefore it inherits all unsafety of those:
    ///
    /// * First, [`Vec::from_raw_parts`] is called onto arguments;
    ///   see the method documentation for the invariants it expects.
    ///
    /// * Then [`StringExt::from_utf8_unchecked`] is called onto the given vector,
    ///   thus the vector must hold valid UTF-8 encoded string.
    ///
    /// [`Vec::from_raw_parts`]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.from_raw_parts
    /// [`StringExt::from_utf8_unchecked`]: #tymethod.from_utf8_unchecked
    #[inline]
    unsafe fn from_raw_parts(buf: *mut u8, length: usize, capacity: usize) -> Self {
        Self::from_utf8_unchecked(Vec::from_raw_parts(buf, length, capacity))
    }

    /// Converts a vector of bytes to a new `InlinableString` without checking
    /// if it contains valid UTF-8.
    ///
    /// # Safety
    ///
    /// This is unsafe because it assumes that the UTF-8-ness of the vector has
    /// already been validated.
    unsafe fn from_utf8_unchecked(bytes: Vec<u8>) -> Self;

    /// Returns the underlying byte buffer, encoded as UTF-8.
    ///
    /// # Examples
    ///
    /// ```
    /// use inlinable_string::{InlinableString, StringExt};
    ///
    /// let s = InlinableString::from("hello");
    /// let bytes = s.into_bytes();
    /// assert_eq!(bytes, [104, 101, 108, 108, 111]);
    /// ```
    #[inline]
    fn into_bytes(self) -> Vec<u8>
    where
        Self: Into<String>,
    {
        Into::into(self).into_bytes()
    }

    /// Pushes the given string onto this string buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use inlinable_string::{InlinableString, StringExt};
    ///
    /// let mut s = InlinableString::from("foo");
    /// s.push_str("bar");
    /// assert_eq!(s, "foobar");
    /// ```
    #[inline]
    fn push_str(&mut self, string: &str) {
        let len = self.len();
        self.insert_str(len, string);
    }

    /// Returns the number of bytes that this string buffer can hold without
    /// reallocating.
    ///
    /// # Examples
    ///
    /// ```
    /// use inlinable_string::{InlinableString, StringExt};
    ///
    /// let s = InlinableString::with_capacity(10);
    /// assert!(s.capacity() >= 10);
    /// ```
    fn capacity(&self) -> usize;

    /// Reserves capacity for at least `additional` more bytes to be inserted
    /// in the given `InlinableString`. The collection may reserve more space to avoid
    /// frequent reallocations.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity overflows `usize`.
    ///
    /// # Examples
    ///
    /// ```
    /// use inlinable_string::{InlinableString, StringExt};
    ///
    /// let mut s = InlinableString::new();
    /// s.reserve(10);
    /// assert!(s.capacity() >= 10);
    /// ```
    fn reserve(&mut self, additional: usize);

    /// Reserves the minimum capacity for exactly `additional` more bytes to be
    /// inserted in the given `InlinableString`. Does nothing if the capacity is already
    /// sufficient.
    ///
    /// Note that the allocator may give the collection more space than it
    /// requests. Therefore capacity can not be relied upon to be precisely
    /// minimal. Prefer `reserve` if future insertions are expected.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity overflows `usize`.
    ///
    /// # Examples
    ///
    /// ```
    /// use inlinable_string::{InlinableString, StringExt};
    ///
    /// let mut s = InlinableString::new();
    /// s.reserve_exact(10);
    /// assert!(s.capacity() >= 10);
    /// ```
    fn reserve_exact(&mut self, additional: usize);

    /// Shrinks the capacity of this string buffer to match its length. If the
    /// string's length is less than `INLINE_STRING_CAPACITY` and the string is
    /// heap-allocated, then it is demoted to inline storage.
    ///
    /// # Examples
    ///
    /// ```
    /// use inlinable_string::{InlinableString, StringExt};
    ///
    /// let mut s = InlinableString::from("foo");
    /// s.reserve(100);
    /// assert!(s.capacity() >= 100);
    /// s.shrink_to_fit();
    /// assert_eq!(s.capacity(), inlinable_string::INLINE_STRING_CAPACITY);
    /// ```
    fn shrink_to_fit(&mut self);

    /// Adds the given character to the end of the string.
    ///
    /// # Examples
    ///
    /// ```
    /// use inlinable_string::{InlinableString, StringExt};
    ///
    /// let mut s = InlinableString::from("abc");
    /// s.push('1');
    /// s.push('2');
    /// s.push('3');
    /// assert_eq!(s, "abc123");
    /// ```
    #[inline]
    fn push(&mut self, ch: char) {
        let len = self.len();
        self.insert(len, ch);
    }

    /// Works with the underlying buffer as a byte slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use inlinable_string::{InlinableString, StringExt};
    ///
    /// let s = InlinableString::from("hello");
    /// assert_eq!(s.as_bytes(), [104, 101, 108, 108, 111]);
    /// ```
    #[inline]
    fn as_bytes(&self) -> &[u8]
    where
        Self: Borrow<str>,
    {
        self.borrow().as_bytes()
    }

    /// Shortens a string to the specified length.
    ///
    /// # Panics
    ///
    /// Panics if `new_len` does not lie on a [`char`] boundary.
    ///
    /// For other possible panic conditions, read documentation of the given implementation.
    ///
    /// [`char`]: https://doc.rust-lang.org/std/primitive.char.html
    ///
    /// # Examples
    ///
    /// ```
    /// use inlinable_string::{InlinableString, StringExt};
    ///
    /// let mut s = InlinableString::from("hello");
    /// s.truncate(2);
    /// assert_eq!(s, "he");
    /// ```
    fn truncate(&mut self, new_len: usize);

    /// Removes the last character from the string buffer and returns it.
    /// Returns `None` if this string buffer is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use inlinable_string::{InlinableString, StringExt};
    ///
    /// let mut s = InlinableString::from("foo");
    /// assert_eq!(s.pop(), Some('o'));
    /// assert_eq!(s.pop(), Some('o'));
    /// assert_eq!(s.pop(), Some('f'));
    /// assert_eq!(s.pop(), None);
    /// ```
    fn pop(&mut self) -> Option<char>;

    /// Removes the character from the string buffer at byte position `idx` and
    /// returns it.
    ///
    /// # Warning
    ///
    /// This is an O(n) operation as it requires copying every element in the
    /// buffer.
    ///
    /// # Panics
    ///
    /// If `idx` does not lie on a character boundary, or if it is out of
    /// bounds, then this function will panic.
    ///
    /// # Examples
    ///
    /// ```
    /// use inlinable_string::{InlinableString, StringExt};
    ///
    /// let mut s = InlinableString::from("foo");
    /// assert_eq!(s.remove(0), 'f');
    /// assert_eq!(s.remove(1), 'o');
    /// assert_eq!(s.remove(0), 'o');
    /// ```
    fn remove(&mut self, idx: usize) -> char;

    /// Removes the specified range from the string buffer.
    ///
    /// # Panics
    ///
    /// Panics if the starting point or end point do not lie on a [`char`]
    /// boundary, or if they're out of bounds.
    ///
    /// [`char`]: https://doc.rust-lang.org/std/primitive.char.html
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use inlinable_string::{InlinableString, StringExt};
    ///
    /// let mut s = InlinableString::from("α is alpha, β is beta");
    /// let beta_offset = s.find('β').unwrap_or(s.len());
    ///
    /// // Remove the range up until the β from the string
    /// s.remove_range(..beta_offset);
    ///
    /// assert_eq!(s, "β is beta");
    ///
    /// // A full range clears the string
    /// s.remove_range(..);
    /// assert_eq!(s, "");
    /// ```
    #[inline]
    fn remove_range<R>(&mut self, range: R)
    where
        R: RangeBounds<usize>,
    {
        use ops::Bound::*;

        let len = self.len();
        let start = match range.start_bound() {
            Included(&n) => n,
            Excluded(&n) => n + 1,
            Unbounded => 0,
        };
        let end = match range.end_bound() {
            Included(&n) => n + 1,
            Excluded(&n) => n,
            Unbounded => len,
        };

        // Checking bounds.
        assert!(start <= end);

        let diff = end - start;

        let mut sum = 0;
        while sum < diff {
            sum += self.remove(start).len_utf8();
        }

        // Sanity check: number of deleted bytes must be equal
        // to the range length.
        assert_eq!(diff, sum);
    }

    /// Inserts a character into the string buffer at byte position `idx`.
    ///
    /// # Warning
    ///
    /// This is an O(n) operation as it requires copying every element in the
    /// buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use inlinable_string::{InlinableString, StringExt};
    ///
    /// let mut s = InlinableString::from("foo");
    /// s.insert(2, 'f');
    /// assert!(s == "fofo");
    /// ```
    ///
    /// # Panics
    ///
    /// If `idx` does not lie on a character boundary or is out of bounds, then
    /// this function will panic.
    #[inline]
    fn insert(&mut self, idx: usize, ch: char) {
        let mut bits = [0; 4];
        self.insert_str(idx, ch.encode_utf8(&mut bits));
    }

    /// Inserts a string into the string buffer at byte position `idx`.
    ///
    /// # Warning
    ///
    /// This is an O(n) operation as it requires copying every element in the
    /// buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use inlinable_string::{InlinableString, StringExt};
    ///
    /// let mut s = InlinableString::from("foo");
    /// s.insert_str(2, "bar");
    /// assert!(s == "fobaro");
    /// ```
    ///
    /// # Panics
    ///
    /// If `idx` does not lie on a character boundary or is out of bounds, then
    /// this function will panic.
    fn insert_str(&mut self, idx: usize, string: &str);
    /* It looks like `insert_str` is better manually implemented,
     * while provided `insert` is mostly okay.
    {
        let mut idx = idx;
        string.chars().for_each(|ch| {
            self.insert(idx, ch);
            idx += ch.len_utf8();
        });
    }
    */

    /// Views the string buffer as a mutable sequence of bytes.
    ///
    /// # Safety
    ///
    /// This is unsafe because it does not check to ensure that the resulting
    /// string will be valid UTF-8.
    ///
    /// # Examples
    ///
    /// ```
    /// use inlinable_string::{InlinableString, StringExt};
    ///
    /// let mut s = InlinableString::from("hello");
    /// unsafe {
    ///     let slice = s.as_mut_slice();
    ///     assert!(slice == &[104, 101, 108, 108, 111]);
    ///     slice.reverse();
    /// }
    /// assert_eq!(s, "olleh");
    /// ```
    #[inline]
    unsafe fn as_mut_slice(&mut self) -> &mut [u8]
    where
        Self: BorrowMut<str>,
    {
        self.borrow_mut().as_bytes_mut()
    }

    /// Returns the number of bytes in this string.
    ///
    /// # Examples
    ///
    /// ```
    /// use inlinable_string::{InlinableString, StringExt};
    ///
    /// let a = InlinableString::from("foo");
    /// assert_eq!(a.len(), 3);
    /// ```
    fn len(&self) -> usize;

    /// Returns true if the string contains no bytes
    ///
    /// # Examples
    ///
    /// ```
    /// use inlinable_string::{InlinableString, StringExt};
    ///
    /// let mut v = InlinableString::new();
    /// assert!(v.is_empty());
    /// v.push('a');
    /// assert!(!v.is_empty());
    /// ```
    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Truncates the string, returning it to 0 length.
    ///
    /// # Examples
    ///
    /// ```
    /// use inlinable_string::{InlinableString, StringExt};
    ///
    /// let mut s = InlinableString::from("foo");
    /// s.clear();
    /// assert!(s.is_empty());
    /// ```
    #[inline]
    fn clear(&mut self) {
        self.truncate(0);
    }

    /// Extracts a string slice containing the entire string buffer.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use inlinable_string::{InlinableString, StringExt};
    ///
    /// let s = InlinableString::from("foo");
    ///
    /// assert_eq!("foo", s.as_str());
    /// ```
    #[inline]
    fn as_str(&self) -> &str
    where
        Self: Borrow<str>,
    {
        self.borrow()
    }

    /// Converts this extandable string into a mutable string slice.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use inlinable_string::{InlinableString, StringExt};
    ///
    /// let mut s = InlinableString::from("foobar");
    /// let s_mut_str = s.as_mut_str();
    ///
    /// s_mut_str.make_ascii_uppercase();
    ///
    /// assert_eq!("FOOBAR", s_mut_str);
    /// ```
    #[inline]
    fn as_mut_str(&mut self) -> &mut str
    where
        Self: BorrowMut<str>,
    {
        self.borrow_mut()
    }

    /// Converts this `String` into a [`Box`]`<`[`str`]`>`.
    ///
    /// This will drop any excess capacity.
    ///
    /// [`Box`]: https://doc.rust-lang.org/std/boxed/struct.Box.html
    /// [`str`]: https://doc.rust-lang.org/std/primitive.str.html
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use inlinable_string::{InlinableString, StringExt};
    ///
    /// let s = InlinableString::from("hello");
    ///
    /// let b = s.into_boxed_str();
    /// ```
    #[inline]
    fn into_boxed_str(self) -> Box<str>
    where
        Self: Into<String>,
    {
        let s = self.into();
        <String>::into_boxed_str(s)
    }

    /// Splits the string into two at the given index.
    ///
    /// Returns a new buffer. `self` contains bytes `[0, at)`, and
    /// the returned buffer contains bytes `[at, len)`. `at` must be on the
    /// boundary of a UTF-8 code point.
    ///
    /// Note that the capacity of `self` does not change.
    ///
    /// # Panics
    ///
    /// Panics if `at` is not on a `UTF-8` code point boundary, or if it is beyond the last
    /// code point of the string.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() {
    /// use inlinable_string::{InlinableString, StringExt};
    ///
    /// let mut hello = InlinableString::from("Hello, World!");
    /// let world = hello.split_off(7);
    /// assert_eq!(hello, "Hello, ");
    /// assert_eq!(world, "World!");
    /// # }
    /// ```
    #[must_use = "use `.truncate()` if you don't need the other half"]
    fn split_off(&mut self, at: usize) -> Self;

/// Internal function to decrease the numbers of unsafe.
#[inline]
fn from_string<S: StringExt>(s: String) -> S {
    // SAFETY:
    // `s` is a well-formed string, turned into bytes.
    unsafe { S::from_utf8_unchecked(<String>::into_bytes(s)) }
}

impl StringExt for String {
    #[inline]
    fn new() -> Self {
        String::new()
    }

    #[inline]
    fn with_capacity(capacity: usize) -> Self {
        String::with_capacity(capacity)
    }

    #[inline]
    fn from_utf8(vec: Vec<u8>) -> Result<Self, FromUtf8Error> {
        String::from_utf8(vec)
    }

    #[inline]
    fn from_utf16(v: &[u16]) -> Result<Self, FromUtf16Error> {
        String::from_utf16(v)
    }

    #[inline]
    fn from_utf16_lossy(v: &[u16]) -> Self {
        String::from_utf16_lossy(v)
    }

    #[inline]
    unsafe fn from_raw_parts(buf: *mut u8, length: usize, capacity: usize) -> Self {
        String::from_raw_parts(buf, length, capacity)
    }

    #[inline]
    unsafe fn from_utf8_unchecked(bytes: Vec<u8>) -> Self {
        String::from_utf8_unchecked(bytes)
    }

    #[inline]
    fn into_bytes(self) -> Vec<u8> {
        String::into_bytes(self)
    }

    #[inline]
    fn push_str(&mut self, string: &str) {
        String::push_str(self, string)
    }

    #[inline]
    fn capacity(&self) -> usize {
        String::capacity(self)
    }

    #[inline]
    fn reserve(&mut self, additional: usize) {
        String::reserve(self, additional)
    }

    #[inline]
    fn reserve_exact(&mut self, additional: usize) {
        String::reserve_exact(self, additional)
    }

    #[inline]
    fn shrink_to_fit(&mut self) {
        String::shrink_to_fit(self)
    }

    #[inline]
    fn push(&mut self, ch: char) {
        String::push(self, ch)
    }

    #[inline]
    fn as_bytes(&self) -> &[u8] {
        String::as_bytes(self)
    }

    #[inline]
    fn truncate(&mut self, new_len: usize) {
        String::truncate(self, new_len)
    }

    #[inline]
    fn pop(&mut self) -> Option<char> {
        String::pop(self)
    }

    #[inline]
    fn remove(&mut self, idx: usize) -> char {
        String::remove(self, idx)
    }

    #[inline]
    fn remove_range<R>(&mut self, range: R)
    where
        R: RangeBounds<usize>,
    {
        String::drain(self, range);
    }

    #[inline]
    fn insert(&mut self, idx: usize, ch: char) {
        String::insert(self, idx, ch)
    }

    #[inline]
    fn insert_str(&mut self, idx: usize, string: &str) {
        String::insert_str(self, idx, string)
    }

    #[inline]
    unsafe fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut *(self.as_mut_str() as *mut str as *mut [u8])
    }

    #[inline]
    fn len(&self) -> usize {
        String::len(self)
    }

    #[inline]
    fn split_off(&mut self, at: usize) -> Self {
        <String>::split_off(self, at)
    }
}

#[cfg(test)]
mod provided_methods_tests {

    use super::StringExt;
    use std::{
        borrow::{Borrow, BorrowMut, Cow},
        cmp::PartialEq,
        fmt,
        ops::{Deref, DerefMut},
    };

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct ReqImpl(String);

    impl From<ReqImpl> for String {
        fn from(s: ReqImpl) -> Self {
            s.0
        }
    }

    impl From<&str> for ReqImpl {
        fn from(s: &str) -> Self {
            Self(String::from(s))
        }
    }
    impl Deref for ReqImpl {
        type Target = str;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl DerefMut for ReqImpl {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl fmt::Display for ReqImpl {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.0.fmt(f)
        }
    }
    impl PartialEq<str> for ReqImpl {
        fn eq(&self, other: &str) -> bool {
            self.0.eq(other)
        }
    }
    impl PartialEq<String> for ReqImpl {
        fn eq(&self, other: &String) -> bool {
            self.0.eq(other)
        }
    }
    impl PartialEq<&str> for ReqImpl {
        fn eq(&self, other: &&str) -> bool {
            self.0.eq(other)
        }
    }
    impl PartialEq<Cow<'_, str>> for ReqImpl {
        fn eq(&self, other: &Cow<str>) -> bool {
            self.0.eq(other)
        }
    }
    impl Borrow<str> for ReqImpl {
        fn borrow(&self) -> &str {
            &self.0
        }
    }
    impl BorrowMut<str> for ReqImpl {
        fn borrow_mut(&mut self) -> &mut str {
            &mut self.0
        }
    }

    impl StringExt for ReqImpl {
        fn with_capacity(capacity: usize) -> Self {
            Self(String::with_capacity(capacity))
        }
        unsafe fn from_utf8_unchecked(bytes: Vec<u8>) -> Self {
            Self(String::from_utf8_unchecked(bytes))
        }
        fn capacity(&self) -> usize {
            self.0.capacity()
        }
        fn reserve(&mut self, additional: usize) {
            self.0.reserve(additional)
        }
        fn reserve_exact(&mut self, additional: usize) {
            self.0.reserve_exact(additional)
        }
        fn shrink_to_fit(&mut self) {
            self.0.shrink_to_fit()
        }
        fn truncate(&mut self, new_len: usize) {
            self.0.truncate(new_len)
        }
        fn pop(&mut self) -> Option<char> {
            self.0.pop()
        }
        fn remove(&mut self, idx: usize) -> char {
            self.0.remove(idx)
        }
        fn insert(&mut self, idx: usize, ch: char) {
            self.0.insert(idx, ch)
        }
        fn len(&self) -> usize {
            self.0.len()
        }
        fn split_off(&mut self, at: usize) -> Self {
            Self(self.0.split_off(at))
        }
    }

    #[test]
    fn test_as_bytes() {
        let s = ReqImpl::from("hello");
        assert_eq!(s.as_bytes(), [104, 101, 108, 108, 111]);
    }

    #[test]
    fn test_as_mut_slice() {
        let mut s = ReqImpl::from("hello");
        unsafe {
            let slice = s.as_mut_slice();
            assert!(slice == &[104, 101, 108, 108, 111]);
            slice.reverse();
        }
        assert_eq!(s, "olleh");
    }

    #[test]
    fn test_as_mut_str() {
        let mut s = ReqImpl::from("foobar");
        let s_mut_str = s.as_mut_str();

        s_mut_str.make_ascii_uppercase();

        assert_eq!("FOOBAR", s_mut_str);
    }

    #[test]
    fn test_as_str() {
        let s = ReqImpl::from("foo");

        assert_eq!("foo", s.as_str());
    }

    #[test]
    fn test_clear() {
        let mut s = ReqImpl::from("foo");
        s.clear();
        assert!(s.is_empty());
    }

    #[test]
    fn test_from_raw_parts() {
        use std::mem;

        unsafe {
            let s = ReqImpl::from("hello");

            let mut s = mem::ManuallyDrop::new(s);

            let ptr = s.0.as_mut_ptr();
            let len = s.len();
            let capacity = s.capacity();

            let s = ReqImpl::from_raw_parts(ptr, len, capacity);

            assert_eq!(s, "hello");
        }
    }

    #[test]
    fn test_from_utf16() {
        // 𝄞music
        let v = &mut [0xD834, 0xDD1E, 0x006d, 0x0075, 0x0073, 0x0069, 0x0063];
        assert_eq!(ReqImpl::from_utf16(v).unwrap(), ReqImpl::from("𝄞music"));

        // 𝄞mu<invalid>ic
        v[4] = 0xD800;
        assert!(ReqImpl::from_utf16(v).is_err());
    }

    #[test]
    fn test_from_utf16_lossy() {
        // 𝄞mus<invalid>ic<invalid>
        let v = &[
            0xD834, 0xDD1E, 0x006d, 0x0075, 0x0073, 0xDD1E, 0x0069, 0x0063, 0xD834,
        ];

        assert_eq!(
            ReqImpl::from_utf16_lossy(v),
            ReqImpl::from("𝄞mus\u{FFFD}ic\u{FFFD}")
        );
    }

    #[test]
    fn test_from_utf8() {
        let hello_vec = vec![104, 101, 108, 108, 111];
        let s = ReqImpl::from_utf8(hello_vec).unwrap();
        assert_eq!(s, "hello");

        let invalid_vec = vec![240, 144, 128];
        let s = ReqImpl::from_utf8(invalid_vec).err().unwrap();
        let _err = s.utf8_error();
        assert_eq!(s.into_bytes(), [240, 144, 128]);
    }

    #[test]
    fn test_from_utf8_lossy() {
        let input = b"Hello \xF0\x90\x80World";
        let output = ReqImpl::from_utf8_lossy(input);
        assert_eq!(output, "Hello \u{FFFD}World");
    }

    #[test]
    fn test_insert_str() {
        let mut s = ReqImpl::from("foo");
        s.insert_str(2, "bar");
        assert!(s == "fobaro");
    }

    #[test]
    fn test_into_bytes() {
        let s = ReqImpl::from("hello");
        let bytes = s.into_bytes();
        assert_eq!(bytes, [104, 101, 108, 108, 111]);
    }

    #[test]
    fn test_is_empty() {
        let mut v = ReqImpl::new();
        assert!(v.is_empty());
        v.push('a');
        assert!(!v.is_empty());
    }

    #[test]
    fn test_new() {
        let s = ReqImpl::new();
        assert_eq!(ReqImpl::with_capacity(0), s);
    }

    #[test]
    fn test_push() {
        let mut s = ReqImpl::from("abc");
        s.push('1');
        s.push('2');
        s.push('3');
        assert_eq!(s, "abc123");
    }

    #[test]
    fn test_push_str() {
        let mut s = ReqImpl::from("foo");
        s.push_str("bar");
        assert_eq!(s, "foobar");
    }

    #[test]
    fn test_remove_range() {
        let mut s = ReqImpl::from("α is alpha, β is beta");
        let beta_offset = s.find('β').unwrap_or(s.len());

        // Remove the range up until the β from the string
        s.remove_range(..beta_offset);

        assert_eq!(s, "β is beta");

        // A full range clears the string
        s.remove_range(..);
        assert_eq!(s, "");
    }
}

#[cfg(test)]
mod std_string_stringext_sanity_tests {
    // Sanity tests for std::string::String's StringExt implementation.

    use super::StringExt;

    #[test]
    fn test_new() {
        let s = <String as StringExt>::new();
        assert!(StringExt::is_empty(&s));
    }

    #[test]
    fn test_with_capacity() {
        let s = <String as StringExt>::with_capacity(10);
        assert!(StringExt::capacity(&s) >= 10);
    }

    #[test]
    fn test_from_utf8() {
        let s = <String as StringExt>::from_utf8(vec![104, 101, 108, 108, 111]);
        assert_eq!(s.unwrap(), "hello");
    }

    #[test]
    fn test_from_utf16() {
        let v = &mut [0xD834, 0xDD1E, 0x006d, 0x0075, 0x0073, 0x0069, 0x0063];
        let s = <String as StringExt>::from_utf16(v);
        assert_eq!(s.unwrap(), "𝄞music");
    }

    #[test]
    fn test_from_utf16_lossy() {
        let input = b"Hello \xF0\x90\x80World";
        let output = <String as StringExt>::from_utf8_lossy(input);
        assert_eq!(output, "Hello \u{FFFD}World");
    }

    #[test]
    fn test_into_bytes() {
        let s = String::from("hello");
        let bytes = StringExt::into_bytes(s);
        assert_eq!(bytes, [104, 101, 108, 108, 111]);
    }

    #[test]
    fn test_push_str() {
        let mut s = String::from("hello");
        StringExt::push_str(&mut s, " world");
        assert_eq!(s, "hello world");
    }

    #[test]
    fn test_capacity() {
        let s = <String as StringExt>::with_capacity(100);
        assert!(String::capacity(&s) >= 100);
    }

    #[test]
    fn test_reserve() {
        let mut s = <String as StringExt>::new();
        StringExt::reserve(&mut s, 100);
        assert!(String::capacity(&s) >= 100);
    }

    #[test]
    fn test_reserve_exact() {
        let mut s = <String as StringExt>::new();
        StringExt::reserve_exact(&mut s, 100);
        assert!(String::capacity(&s) >= 100);
    }

    #[test]
    fn test_shrink_to_fit() {
        let mut s = <String as StringExt>::with_capacity(100);
        StringExt::push_str(&mut s, "foo");
        StringExt::shrink_to_fit(&mut s);
        assert_eq!(String::capacity(&s), 3);
    }

    #[test]
    fn test_push() {
        let mut s = String::new();
        StringExt::push(&mut s, 'a');
        assert_eq!(s, "a");
    }

    #[test]
    fn test_truncate() {
        let mut s = String::from("foo");
        StringExt::truncate(&mut s, 1);
        assert_eq!(s, "f");
    }

    #[test]
    fn test_pop() {
        let mut s = String::from("foo");
        assert_eq!(StringExt::pop(&mut s), Some('o'));
        assert_eq!(StringExt::pop(&mut s), Some('o'));
        assert_eq!(StringExt::pop(&mut s), Some('f'));
        assert_eq!(StringExt::pop(&mut s), None);
    }

    #[test]
    fn test_insert_str() {
        let mut s = String::from("foo");
        StringExt::insert_str(&mut s, 1, "bar");
        assert_eq!(s, "fbaroo");
    }

    #[test]
    fn test_remove_range() {
        let mut s = String::from("foobar");
        StringExt::remove_range(&mut s, 1..3);
        assert_eq!(s, "fbar");
    }

    #[test]
    fn test_split_off() {
        let mut s = String::from("foobar");
        let right_part = StringExt::split_off(&mut s, 3);
        assert_eq!(s, "foo");
        assert_eq!(right_part, "bar");
    }
}
