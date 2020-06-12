#![allow(non_camel_case_types)]
#![recursion_limit = "256"]
use std::marker::PhantomData;
use trait_eval::*;
pub use trait_eval::Eval;

#[doc(hidden)]
pub struct Empty {}

#[doc(hidden)]
pub struct Node<V, N> {
    _val: PhantomData<V>,
    _next: PhantomData<N>,
}

#[doc(hidden)]
pub struct Stop<N> {
    _next: PhantomData<N>,
}

macro_rules! pub_trait {
    ($($(#[$meta:meta])* $name:ident),*) => {
        $(
            $(#[$meta])*
            pub trait $name {
                type Result;
            }
        )*
    }
}

pub_trait!(
    /// Remove the top element
    /// # Examples
    /// Drop 2 from the stack:
    /// ```
    /// # #[macro_use] extern crate fortraith;
    /// # use fortraith::*;
    /// forth!(
    ///     1 2 drop
    ///     return type Out as top
    /// );
    /// assert_eq!(Out::eval(), 1);
    /// ```
    drop,

    /// Duplicate the top element
    /// # Examples
    /// Duplicate 2 and add it to itself:
    /// ```
    /// # #[macro_use] extern crate fortraith;
    /// # use fortraith::*;
    /// forth!(
    ///     2 dup +
    ///     return type Out as top
    /// );
    /// assert_eq!(Out::eval(), 4);
    /// ```
    dup,

    /// Swap two top elements
    /// # Examples
    /// Swap 1 and 2 and subtract
    /// ```
    /// # #[macro_use] extern crate fortraith;
    /// # use fortraith::*;
    /// forth!(
    ///     1 2 swap -
    ///     return type Out as top
    /// );
    /// assert_eq!(Out::eval(), 1);
    /// ```
    swap,

    /// Rotate three top elements
    /// # Examples
    /// Rotate 1 2 3 -> 2 3 1 -> 3 1 2
    /// ```
    /// # #[macro_use] extern crate fortraith;
    /// # use fortraith::*;
    /// forth!(
    ///     1 2 3 rot rot
    ///     return type Out as top
    /// );
    /// assert_eq!(Out::eval(), 2);
    /// ```
    rot,

    /// Get the top element
    /// # Examples
    /// WARNING! This effectively discards the stack, so it can only be used with the `return`
    /// statement
    /// ```
    /// # #[macro_use] extern crate fortraith;
    /// # use fortraith::*;
    /// forth!(
    ///     10
    ///     return type Out1 as top
    ///     1 + top
    ///     return type Out2
    /// );
    /// type Out3 = forth!(9 top return);
    /// assert_eq!(Out1::eval(), 10);
    /// assert_eq!(Out2::eval(), 11);
    /// assert_eq!(Out3::eval(), 9);
    /// ```
    top,

    /// ( if / else / then ) conditional expression
    ///
    /// ifs can be nested and can be used both inside and outside of a function. `else` clause is
    /// optional
    /// # Examples
    /// Return 9 if 10 is less than 1, else return 9
    /// ```
    /// # #[macro_use] extern crate fortraith;
    /// # use fortraith::*;
    /// forth!(
    ///     10 1 < if 5 else 9 then
    ///     return type Out as top
    /// );
    /// assert_eq!(Out::eval(), 9);
    /// ```
    /// FizzBuzz (of course), there are no strings or chars in fortraith so Fizz = true, Buzz =
    /// false and FizzBuzz = 0
    /// ```
    /// # #![recursion_limit="256"]
    /// # #[macro_use] extern crate fortraith;
    /// # use fortraith::*;
    /// forth!(
    ///    : FizzBuzz
    ///    dup 3 % 0 = if
    ///        5 % 0 = if
    ///            0 (FizzBuzz)
    ///        else
    ///            true (Fizz)
    ///        then
    ///    else
    ///        dup 5 % 0 = if
    ///            drop false (Buzz)
    ///        then
    ///    then ;
    ///    1 FizzBuzz return type Out1 as top
    ///    2 FizzBuzz return type Out2 as top
    ///    3 FizzBuzz return type Out3 as top
    ///    4 FizzBuzz return type Out4 as top
    ///    5 FizzBuzz return type Out5 as top
    ///    10 5 + FizzBuzz return type Out15 as top
    /// );
    /// assert_eq!(Out1::eval(), 1);
    /// assert_eq!(Out2::eval(), 2);
    /// assert_eq!(Out3::eval(), true);
    /// assert_eq!(Out4::eval(), 4);
    /// assert_eq!(Out5::eval(), false);
    /// assert_eq!(Out15::eval(), 0);
    /// ```
    iff,

    #[doc(hidden)]
    elsef,

    #[doc(hidden)]
    then,

    /// Apply logical not to the top element
    not,
    /// Decrement the top element
    pred, 
    /// Index the fibonacci sequence with the top element
    fib,
    /// Calculate the factorial of the top element
    fact, 
    /// ( + ) Add two top elements togeth
    plus, 
    /// ( - ) Subtract the top element from the second top eleme
    minus,
    /// ( % ) Calculate the rest from dividing the second top element by the top o
    modulo,
    /// ( * ) Multiply two top elemen
    mult, 
    /// ( = ) Chech if two top elements are equ
    eq,
    /// ( < ) Check if the second top element is less than the top eleme
    less,
    /// Logical and two top elements
    and,
    /// Logical or two top elements
    or,
    /// ( 0 ) Constant number
    zero,
    /// ( 1 ) Constant number
    one,
    /// ( 2 ) Constant number
    two,
    /// ( 3 ) Constant number
    three,
    /// ( 4 ) Constant number
    four,
    /// ( 5 ) Constant number
    five,
    /// ( 6 ) Constant number
    six,
    /// ( 7 ) Constant number
    seven,
    /// ( 8 ) Constant number
    eight,
    /// ( 9 ) Constant number
    nine,
    /// ( 10 ) Constant number 
    ten,
    /// ( true ) Constant boolean
    truef,
    /// ( false ) Constant boolean
    falsef
);

macro_rules! stack_op {
    (1, $name:ident, $op:ident, $type:ident) => {
        impl<V, N> $name for Node<V, N>
        where
            V: $op + $type,
        {
            type Result = Node<V::Result, N>;
        }
    };
    (2, $name:ident, $op:ident, $type:ident) => {
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

stack_op!(1, not, Not, Bool);
stack_op!(1, pred, Pred, Nat);
stack_op!(1, fib, Fib, Nat);
stack_op!(1, fact, Fact, Nat);

stack_op!(2, plus, Plus, Nat);
stack_op!(2, minus, Minus, Nat);
stack_op!(2, modulo, Mod, Nat);
stack_op!(2, mult, Times, Nat);
stack_op!(2, eq, Equals, Nat);
stack_op!(2, less, LessThan, Nat);
stack_op!(2, and, AndAlso, Bool);
stack_op!(2, or, OrElse, Bool);

macro_rules! constant {
    ($name:ident, $con:ty) => {
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

impl<V, N> drop for Node<V, N> {
    type Result = N;
}

impl<V, N> dup for Node<V, N> {
    type Result = Node<V, Self>;
}

impl<V, N> swap for Node<V, N>
where
    N: top + drop,
{
    type Result = Node<<N as top>::Result, Node<V, <N as drop>::Result>>;
}

impl<V, N> rot for Node<V, N>
where
    N: top + drop,
    <N as drop>::Result: top + drop,
{
    type Result = Node<
        <<N as drop>::Result as top>::Result,
        Node<V, Node<<N as top>::Result, <<N as drop>::Result as drop>::Result>>,
    >;
}

impl<V, N> top for Node<V, N> {
    type Result = V;
}

impl<N> iff for Node<True, N> {
    type Result = N;
}
impl<N> iff for Node<False, N> {
    type Result = Stop<N>;
}
impl<N> iff for Stop<N> {
    type Result = Stop<Self>;
}

impl<V, N> elsef for Node<V, N> {
    type Result = Stop<Self>;
}
impl<N> elsef for Stop<Stop<N>> {
    type Result = Self;
}
impl<V, N> elsef for Stop<Node<V, N>> {
    type Result = Node<V, N>;
}
impl elsef for Stop<Empty> {
    type Result = Empty;
}

impl<V, N> then for Node<V, N> {
    type Result = Self;
}
impl then for Empty {
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
    top, drop, dup, plus, minus, modulo, mult, eq, less, and, or, zero, one, two, three, four,
    five, six, seven, eight, nine, ten, truef, falsef, swap, rot, not, pred, fact, fib
);

/// Compile forth to trait expressions
#[macro_export]
macro_rules! forth {
    ({ $EX:ty }) => { };
    ({ $EX:ty } return) => {
        $EX
    };
    ({ $EX:ty } return type $name:ident as $tok:tt $($token:tt)*) => {
        type $name = <$EX as $tok>::Result;
        forth!({ $EX } $($token)*)
    };
    ({ $EX:ty } return type $name:ident $($token:tt)*) => {
        type $name = $EX;
        forth!({ $EX } $($token)*)
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
        impl<N> $name for Stop<N> {
            type Result = Self;
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        forth!(
            : factorial (n -- n) 1 swap fact0 ;
            : fact0 (n n -- n) dup 1 = if drop else dup rot * swap pred fact0 then ;
            5 factorial
            top return type Out
        );
        assert_eq!(Out::eval(), 120);
    }
}
