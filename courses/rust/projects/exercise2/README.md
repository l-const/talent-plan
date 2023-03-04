- **Exercise: Serialize and deserialize a data structure to a buffer with
    `serde` (RON)**.

  Do the same as above, except this time, instead of serializing to a `File`,
  serialize to a `Vec<u8>` buffer, and after that try using [RON] instead of
  JSON as the format. Are there any differences in serialization to a `Vec`
  instead of a `File`? What about in using the RON crate vs the JSON crate?

  Convert the `Vec<u8>` to `String` with [`str::from_utf8`], unwrapping the
  result, then print that serialized string representation to see what `Move`
  looks like serialized to RON.