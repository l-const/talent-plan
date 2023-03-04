- **Exercise: Serialize and deserialize 1000 data structures with `serde` (BSON)**.

  This one is slightly different. Where the previous exercises serialized and
  deserialized a single value to a buffer, in this one serialize 1000 different
  `Move` values to a single file, back-to-back, then deserialize them again.
  This time use the [BSON] format.

  Things to discover here are whether serde automatically maintains the correct
  file offsets (the "cursor") to deserialize multiple values in sequence, or if
  you need to parse your own "frames" around each value to define their size,
  and how to detect that there are no more values to parse at the end of the
  file.

  After you've succeeded at serializing and deserializing multiple values to a
  file, try it again with a `Vec<u8>`. Serializing and deserializing generally
  requires the destination implement the [`Write`] and [`Read`] traits. Does
  `Vec<u8>` implement either or both? What is the behavior of those
  implementations? You may need to wrap your buffer in wrapper types that
  implement these traits in order to get the correct behavior &mdash; the
  API docs for the traits list all their implementors in the standard library,
  and whatever you need will be in there somewhere.
