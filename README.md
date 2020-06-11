# `fortraith`
Compile-time compiler that compiles Forth to compile-time trait expressions.

## What?
Rust's trait system is Turing complete. This crate uses the principles from
[trait-eval](https://github.com/doctorn/trait-eval/) to implement necessary
traits for forth evalutaion and provides a `forth!` macro that transpiles
forth's syntax to trait expressions.

## Show me!
Here's a simple factorial implementation, the only non-standard word here is
`pred` which is a decrement operator, equivalent to `1 -`:
```rust
forth!(
    : factorial (n -- n) 1 swap fact0 ;
    : fact0 (n n -- n) dup 1 = if drop else dup rot * swap pred fact0 then ;
    5 factorial .
);
```
This prints `120`. As you can see not only you can define functions easily, but even conditional recursion is possible!
Now check out how it looks compiled to trait expressions (courtesy of `cargo expand`):
```rust
pub trait factorial {
   type Result;
}
impl<V, N> factorial for Node<V, N>
where
    Self: one,
    <Self as one>::Result: swap,
    <<Self as one>::Result as swap>::Result: fact0,
{
    type Result = <<<Self as one>::Result as swap>::Result as fact0>::Result;
}
pub trait fact0 {
    type Result;
}
impl <V ,N> fact0 for Node <V ,N>
where
    Self: dup,
    <Self as dup>::Result: one,
    <<Self as dup>::Result as one>::Result: eq,
    <<<Self as dup>::Result as one>::Result as eq>::Result: iff,
    <<<<Self as dup>::Result as one>::Result as eq>::Result as iff>::Result: drop,
    <<<<<Self as dup>::Result as one>::Result as eq>::Result as iff>::Result as drop>::Result: elsef,
    <<<<<<Self as dup>::Result as one>::Result as eq>::Result as iff>::Result as drop>::Result as elsef>::Result: dup,
    <<<<<<<Self as dup>::Result as one>::Result as eq>::Result as iff>::Result as drop>::Result as elsef>::Result as dup>::Result: rot,
    <<<<<<<<Self as dup>::Result as one>::Result as eq>::Result as iff>::Result as drop>::Result as elsef>::Result as dup>::Result as rot>::Result: mult,
    <<<<<<<<<Self as dup>::Result as one>::Result as eq>::Result as iff>::Result as drop>::Result as elsef>::Result as dup>::Result as rot>::Result as mult>::Result: swap,
    <<<<<<<<<<Self as dup>::Result as one>::Result as eq>::Result as iff>::Result as drop>::Result as elsef>::Result as dup>::Result as rot>::Result as mult>::Result as swap>::Result: pred,
    <<<<<<<<<<<Self as dup>::Result as one>::Result as eq>::Result as iff>::Result as drop>::Result as elsef>::Result as dup>::Result as rot>::Result as mult>::Result as swap>::Result as pred>::Result: fact0,
    <<<<<<<<<<<<Self as dup>::Result as one>::Result as eq>::Result as iff>::Result as drop>::Result as elsef>::Result as dup>::Result as rot>::Result as mult>::Result as swap>::Result as pred>::Result as fact0>::Result: then
{
    type Result = <<<<<<<<<<<<<Self as dup>::Result as one>::Result as eq>::Result as iff>::Result as drop>::Result as elsef>::Result as dup>::Result as rot>::Result as mult>::Result as swap>::Result as pred>::Result as fact0>::Result as then>::Result;
}
println!("{}", <<<Empty as five>::Result as factorial>::Result as top>::Result::eval());
```
Yeah, writing that manually would be no fun.

## What can I do with it?
Quite a bit is actually supported as you can see above. Every operation from
`trait-eval` is re-exported to work on the stack (except `if` which is done
differently), and a faw additional stack operations are provided. See docs
(TBD) for the details.

