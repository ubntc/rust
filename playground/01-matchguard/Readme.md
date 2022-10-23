# Easy Match Guard

While learning Rust, I was looking for the Rust equivalent of Go's plain `switch`.

```go
switch {
case a && b: ...
case c || d: ...
default: ...
}
```

The Go non-argument `switch` can be handy when dealing with many logic options
based on multiple variables, without the need to refactor the code. It can make
long if-else chains more readable.

The Rust equivalent is the 0-tuple match with guards.

```rust
match () {
    () if a && b => ...
    () if a && b => ...
    () => ...
}
```

But this is considered a code smell, since it abuses the `match` statement somehow.

The whole search for nice Rust syntax led to a
[Stackoverflow question](https://stackoverflow.com/questions/74163130/alternatives-to-if-else-chains-with-complex-conditions-in-rust)
that taught me some Rust and made me summarize my (biased) best practice for
how I would structure complex Go `switch` logic using Rust syntax.

The [present lesson code](main.rs) captures these learnings outside of Stackoverflow.
