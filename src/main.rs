#![allow(non_camel_case_types)]
use std::marker::PhantomData;
use trait_eval::*;

pub trait Stack {
    type Size: Nat;
}

pub struct Empty {}

impl Stack for Empty {
    type Size = Zero;
}

pub struct Node<V, N> {
    _val: PhantomData<V>,
    _next: PhantomData<N>,
}

impl<V, N> Stack for Node<V, N>
where
    N: Stack,
{
    type Size = Succ<N::Size>;
}

pub trait top {
    type Result;
}

impl<V, N> top for Node<V, N> {
    type Result = V;
}

pub trait push<T> {
    type Result;
}

impl<T> push<T> for Empty {
    type Result = Node<T, Self>;
}

impl<T, V, N> push<T> for Node<V, N> {
    type Result = Node<T, Self>;
}

macro_rules! stack_op {
    ($name:ident, $op:ident, $type:ident) => {
        pub trait $name {
            type Result: Stack;
        }
        impl<V, N> $name for Node<V, N>
        where
            N: drop + top,
            V: $type,
            <N as top>::Result: $type + $op<V>,
        {
            type Result = Node<<<N as top>::Result as $op<V>>::Result, <N as drop>::Result>;
        }
    };
}

stack_op!(plus, Plus, Nat);
stack_op!(minus, Minus, Nat);
stack_op!(modulo, Mod, Nat);
stack_op!(mult, Times, Nat);
stack_op!(eq, Equals, Nat);
stack_op!(less, LessThan, Nat);
stack_op!(and, AndAlso, Bool);
stack_op!(or, OrElse, Bool);

pub trait drop {
    type Result: Stack;
}

impl<V, N> drop for Node<V, N>
where
    N: Stack,
{
    type Result = N;
}

pub trait dup {
    type Result: Stack;
}

impl<V, N> dup for Node<V, N>
where
    N: Stack,
{
    type Result = Node<V, Self>;
}

macro_rules! forth {
    ({ $EX:ty }) => { };
    ({ $EX:ty } return) => {
        $EX
    };
    ({ $EX:ty } . $($token:tt)*) => {
        println!("{}", <$EX as top>::Result::eval());
        forth!({ <$EX as drop>::Result } $($token)*)
    };
    ({ $EX:ty } + $($token:tt)*) => {
        forth!({ <$EX as plus>::Result } $($token)*)
    };
    ({ $EX:ty } * $($token:tt)*) => {
        forth!({ <$EX as mult>::Result } $($token)*)
    };
    ({ $EX:ty } % $($token:tt)*) => {
        forth!({ <$EX as modulo>::Result } $($token)*)
    };
    ({ $EX:ty } - $($token:tt)*) => {
        forth!({ <$EX as minus>::Result } $($token)*)
    };
    ({ $EX:ty } : $name:ident $($token:tt)*) => {
        forth!(@compile $name ( ) { $EX } $($token)*)
    };
    ({ $EX:ty } ($($comment:tt)*) $($token:tt)*) => {
        forth!({ $EX } $($token)*)
    };
    ({ $EX:ty } true $($token:tt)*) => {
        forth!({ <$EX as push<True>>::Result } $($token)*)
    };
    ({ $EX:ty } false $($token:tt)*) => {
        forth!({ <$EX as push<False>>::Result } $($token)*)
    };
    ({ $EX:ty } 0 $($token:tt)*) => {
        forth!({ <$EX as push<Zero>>::Result } $($token)*)
    };
    ({ $EX:ty } 1 $($token:tt)*) => {
        forth!({ <$EX as push<One>>::Result } $($token)*)
    };
    ({ $EX:ty } 2 $($token:tt)*) => {
        forth!({ <$EX as push<Two>>::Result } $($token)*)
    };
    ({ $EX:ty } 3 $($token:tt)*) => {
        forth!({ <$EX as push<Three>>::Result } $($token)*)
    };
    ({ $EX:ty } 4 $($token:tt)*) => {
        forth!({ <$EX as push<Four>>::Result } $($token)*)
    };
    ({ $EX:ty } 5 $($token:tt)*) => {
        forth!({ <$EX as push<Five>>::Result } $($token)*)
    };
    ({ $EX:ty } 6 $($token:tt)*) => {
        forth!({ <$EX as push<Six>>::Result } $($token)*)
    };
    ({ $EX:ty } 7 $($token:tt)*) => {
        forth!({ <$EX as push<Seven>>::Result } $($token)*)
    };
    ({ $EX:ty } 8 $($token:tt)*) => {
        forth!({ <$EX as push<Eight>>::Result } $($token)*)
    };
    ({ $EX:ty } 9 $($token:tt)*) => {
        forth!({ <$EX as push<Nine>>::Result } $($token)*)
    };
    ({ $EX:ty } 10 $($token:tt)*) => {
        forth!({ <$EX as push<Ten>>::Result } $($token)*)
    };
    ({ $EX:ty } $tok:tt $($token:tt)*) => {
        forth!({ <$EX as $tok>::Result } $($token)*)
    };
    (@compile $name:ident ($($cmd:tt)*) { $EX:ty } ; $($token:tt)*) => {
        pub trait $name {
            type Result;
        }
        forth!(@bounds $name; $($cmd)*);
        forth!({ $EX } $($token)*);
    };
    (@compile $name:ident ($($cmd:tt)*) { $EX:ty } ($($comment:tt)*) $($token:tt)*) => {
        forth!(@compile $name ( $($cmd)* ) { $EX } $($token)*)
    };
    (@compile $name:ident ($($cmd:tt)*) { $EX:ty } * $($token:tt)*) => {
        forth!(@compile $name ( $($cmd)* mult ) { $EX } $($token)*)
    };
    (@compile $name:ident ($($cmd:tt)*) { $EX:ty } + $($token:tt)*) => {
        forth!(@compile $name ( $($cmd)* add ) { $EX } $($token)*)
    };
    (@compile $name:ident ($($cmd:tt)*) { $EX:ty } - $($token:tt)*) => {
        forth!(@compile $name ( $($cmd)* minus ) { $EX } $($token)*)
    };
    (@compile $name:ident ($($cmd:tt)*) { $EX:ty } % $($token:tt)*) => {
        forth!(@compile $name ( $($cmd)* modulo ) { $EX } $($token)*)
    };
    (@compile $name:ident ($($cmd:tt)*) { $EX:ty } $tok:tt $($token:tt)*) => {
        forth!(@compile $name ( $($cmd)* $tok ) { $EX } $($token)*)
    };
    (@bounds $proc:ident; $cmd1:tt) => {
        impl<V, N> $proc for Node<V, N>
        where
            forth!({ Self } return): $cmd1,
        {
            type Result = forth!({ Self } $cmd1 return);
        }
    };
    (@bounds $proc:ident; $cmd1:tt $cmd2:tt) => {
        impl<V, N> $proc for Node<V, N>
        where
            forth!({ Self } return): $cmd1,
            forth!({ Self } $cmd1 return): $cmd2,
        {
            type Result = forth!({Self} $cmd1 $cmd2 return);
        }
    };
    (@bounds $proc:ident; $cmd1:tt $cmd2:tt $cmd3:tt) => {
        impl<V, N> $proc for Node<V, N>
        where
            forth!({ Self } return): $cmd1,
            forth!({ Self } $cmd1 return): $cmd2,
            forth!({ Self } $cmd1 $cmd2 return): $cmd3
        {
            type Result = forth!({ Self } $cmd1 $cmd2 $cmd3 return);
        }
    };
    ($($token:tt)*) => {
        forth!({ Empty } $($token)*)
    };
}
fn main() {
    forth!(
        : square ( n -- n*n ) dup * ;
        6 square . true false and .
        1 5 less .
    );
}
