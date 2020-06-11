#![allow(non_camel_case_types)]
use std::marker::PhantomData;
use trait_eval::*;

pub struct Empty {}

pub struct Node<V, N> {
    _val: PhantomData<V>,
    _next: PhantomData<N>,
}

pub struct Stop<N> {
    _next: PhantomData<N>,
}

macro_rules! stack_op {
    ($name:ident, $op:ident, $type:ident) => {
        pub trait $name {
            type Result;
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

macro_rules! constant {
    ($name:ident, $con:ty) => {
        pub trait $name {
            type Result;
        }
        impl<V, N> $name for Node<V, N> {
            type Result = Node<$con, Self>;
        }
        impl $name for Empty {
            type Result = Node<$con, Self>;
        }
    };
}

constant!(zero, Zero);
constant!(one, One);
constant!(two, Two);
constant!(three, Three);
constant!(four, Four);
constant!(five, Five);
constant!(six, Six);
constant!(seven, Seven);
constant!(eight, Eight);
constant!(nine, Nine);
constant!(ten, Ten);
constant!(truef, True);
constant!(falsef, False);

pub trait drop {
    type Result;
}
impl<V, N> drop for Node<V, N> {
    type Result = N;
}

pub trait dup {
    type Result;
}
impl<V, N> dup for Node<V, N> {
    type Result = Node<V, Self>;
}

pub trait top {
    type Result;
}
impl<V, N> top for Node<V, N> {
    type Result = V;
}

pub trait iff {
    type Result;
}
impl<N> iff for Node<True, N> {
    type Result = N;
}
impl<N> iff for Node<False, N> {
    type Result = Stop<N>;
}

pub trait elsef {
    type Result;
}
impl<V, N> elsef for Node<V, N> {
    type Result = Stop<Self>;
}
impl<N> elsef for Stop<N> {
    type Result = N;
}

pub trait then {
    type Result;
}
impl<V, N> then for Node<V, N> {
    type Result = Self;
}
impl<N> then for Stop<N> {
    type Result = N;
}

macro_rules! impl_for_stop {
    ($($trait:ident),*) => {
        $(
            impl<N> $trait for Stop<N> {
                type Result = Self;
            }
        )*
    };
}

impl_for_stop!(
    top, drop, dup, iff, plus, minus, modulo, eq, less, and, or, zero, one, two, three, four, five, six,
    seven, eight, nine, ten, truef, falsef
);

macro_rules! forth {
    ({ $EX:ty }) => { };
    ({ $EX:ty } return) => {
        $EX
    };
    ({ $EX:ty } . $($token:tt)*) => {
        println!("{}", <$EX as top>::Result::eval());
        forth!({ <$EX as drop>::Result } $($token)*)
    };
    ({ $EX:ty } : $name:ident $($token:tt)*) => {
        forth!(@compile $name ( ) { $EX } $($token)*)
    };
    ({ $EX:ty } $tok:tt $($token:tt)*) => {
        forth!({ <$EX as $tok>::Result } $($token)*)
    };
    (@compile $name:ident ($($cmd:tt)*) { $EX:ty } ; $($token:tt)*) => {
        pub trait $name {
            type Result;
        }
        impl_for_stop!($name);
        forth!(@compile_impl $name; $($cmd)*);
        forth!({ $EX } $($token)*);
    };
    (@compile $name:ident ($($cmd:tt)*) { $EX:ty } $tok:tt $($token:tt)*) => {
        forth!(@compile $name ( $($cmd)* $tok ) { $EX } $($token)*)
    };
    (@compile_impl $name:ident; {$(($($cmdl:tt)*))*} ($($cmdr:tt)*) $new:tt $($tbd:tt)*) => {
        forth!(@compile_impl $name; {$(($($cmdl)*))* ($($cmdr)*)} ($($cmdr)* $new) $($tbd)*)
    };
    (@compile_impl $name:ident; {$(($($cmdl:tt)*))*} ($($cmdr:tt)*)) => {
        impl<V, N> $name for Node<V, N>
        where $(
            forth!({Self} $($cmdl)* return): $cmdr
        ),*
        {
            type Result = forth!({ Self } $($cmdr)* return);
        }
    };
    (@compile_impl $name:ident; $cmd1:tt $($cmds:tt)*) => {
        forth!(@compile_impl $name; {()} ($cmd1) $($cmds)*)
    };
    (@subs ($($subst:tt)*) {$EX:ty}) => {
        forth!({$EX} $($subst)*)
    };
    (@subs ($($subst:tt)*) {$EX:ty} ($($comment:tt)*) $($token:tt)*) => {
        forth!(@subs ($($subst)*) {$EX} $($token)*)
    };
    (@subs ($($subst:tt)*) {$EX:ty} + $($token:tt)*) => {
        forth!(@subs ($($subst)* plus) {$EX} $($token)*)
    };
    (@subs ($($subst:tt)*) {$EX:ty} - $($token:tt)*) => {
        forth!(@subs ($($subst)* minus) {$EX} $($token)*)
    };
    (@subs ($($subst:tt)*) {$EX:ty} * $($token:tt)*) => {
        forth!(@subs ($($subst)* mult) {$EX} $($token)*)
    };
    (@subs ($($subst:tt)*) {$EX:ty} % $($token:tt)*) => {
        forth!(@subs ($($subst)* modulo) {$EX} $($token)*)
    };
    (@subs ($($subst:tt)*) {$EX:ty} = $($token:tt)*) => {
        forth!(@subs ($($subst)* eq) {$EX} $($token)*)
    };
    (@subs ($($subst:tt)*) {$EX:ty} < $($token:tt)*) => {
        forth!(@subs ($($subst)* less) {$EX} $($token)*)
    };
    (@subs ($($subst:tt)*) {$EX:ty} if $($token:tt)*) => {
        forth!(@subs ($($subst)* iff) {$EX} $($token)*)
    };
    (@subs ($($subst:tt)*) {$EX:ty} else $($token:tt)*) => {
        forth!(@subs ($($subst)* elsef) {$EX} $($token)*)
    };
    (@subs ($($subst:tt)*) {$EX:ty} 0 $($token:tt)*) => {
        forth!(@subs ($($subst)* zero) {$EX} $($token)*)
    };
    (@subs ($($subst:tt)*) {$EX:ty} 1 $($token:tt)*) => {
        forth!(@subs ($($subst)* one) {$EX} $($token)*)
    };
    (@subs ($($subst:tt)*) {$EX:ty} 2 $($token:tt)*) => {
        forth!(@subs ($($subst)* two) {$EX} $($token)*)
    };
    (@subs ($($subst:tt)*) {$EX:ty} 3 $($token:tt)*) => {
        forth!(@subs ($($subst)* three) {$EX} $($token)*)
    };
    (@subs ($($subst:tt)*) {$EX:ty} 4 $($token:tt)*) => {
        forth!(@subs ($($subst)* four) {$EX} $($token)*)
    };
    (@subs ($($subst:tt)*) {$EX:ty} 5 $($token:tt)*) => {
        forth!(@subs ($($subst)* five) {$EX} $($token)*)
    };
    (@subs ($($subst:tt)*) {$EX:ty} 6 $($token:tt)*) => {
        forth!(@subs ($($subst)* six) {$EX} $($token)*)
    };
    (@subs ($($subst:tt)*) {$EX:ty} 7 $($token:tt)*) => {
        forth!(@subs ($($subst)* seven) {$EX} $($token)*)
    };
    (@subs ($($subst:tt)*) {$EX:ty} 8 $($token:tt)*) => {
        forth!(@subs ($($subst)* eight) {$EX} $($token)*)
    };
    (@subs ($($subst:tt)*) {$EX:ty} 9 $($token:tt)*) => {
        forth!(@subs ($($subst)* nine) {$EX} $($token)*)
    };
    (@subs ($($subst:tt)*) {$EX:ty} 10 $($token:tt)*) => {
        forth!(@subs ($($subst)* ten) {$EX} $($token)*)
    };
    (@subs ($($subst:tt)*) {$EX:ty} true $($token:tt)*) => {
        forth!(@subs ($($subst)* truef) {$EX} $($token)*)
    };
    (@subs ($($subst:tt)*) {$EX:ty} false $($token:tt)*) => {
        forth!(@subs ($($subst)* falsef) {$EX} $($token)*)
    };
    (@subs ($($subst:tt)*) {$EX:ty} $tok:tt $($token:tt)*) => {
        forth!(@subs ($($subst)* $tok) {$EX} $($token)*)
    };
    ($($token:tt)*) => {
        forth!(@subs () { Empty } $($token)*)
    };
}

fn main() {
    forth!(
        : truetotwo if truetotwo else 1 then ;
        0 false true truetotwo .
    );
}
