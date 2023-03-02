- **Exercise: Serialize and deserialize a data structure with `serde` (JSON)**.

  This exercise and the next two will introduce basic serialization and
  deserialization with [`serde`]. `serde` serializes data quickly and is easy to
  use, while also being extensible and expressive.

  For your serializable data structure, imagine a flat game-playing surface
  covered in a grid of squares, like a chess board. Imagine you have a game
  character that every turn may move any number of squares in a single
  direction. Define a type, `Move` that represents a single move of that
  character.

  Derive the [`Debug`] trait so `Move` is easily printable with the `{:?}`
  format specifier.

  Write a `main` function that defines a variable, `a`, of type `Move`,
  serializes it with [`serde`] to a [`File`], then deserializes it back again to a
  variable, `b`, also of type `Move`.

  Use [JSON] as the serialization format.

  Print `a` and `b` with `println!` and the `{:?}` format specifier to verify
  successful deserialization.

  Note that the `serde` book has many [examples] to work off of.