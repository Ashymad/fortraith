use std::marker::PhantomData;

pub trait Nat {}

pub struct Zero {}

impl Nat for Zero {}

impl<T: Nat> Nat for Succ<T> {}

pub type One = Succ<Zero>;
pub type Two = Succ<One>;
pub type Three = Succ<Two>;
pub type Four = Succ<Three>;
pub type Five = Succ<Four>;
pub type Six = Succ<Five>;
pub type Seven = Succ<Six>;
pub type Eight = Succ<Seven>;
pub type Nine = Succ<Eight>;
pub type Ten = Succ<Nine>;

pub struct Succ<T> where T: Nat {
    _marker: PhantomData<T>,
}

pub trait Stack {
    type Size: Nat;
}

pub struct Empty {}

impl Stack for Empty {
    type Size = Zero;
}

impl<V, N> Stack for Node<V, N> where N: Stack {
    type Size = Succ<N::Size>;
}

pub struct Node<V, N> {
    _val: PhantomData<V>,
    _next: PhantomData<N>
}

pub trait Top {
    type Result;
}

impl<V, N> Top for Node<V, N> {
    type Result = V;
}

pub trait Push<T> {
    type Result;
}

impl<T> Push<T> for Empty {
    type Result = Node<T, Self>;
}

impl<T, V, N> Push<T> for Node<V, N> {
    type Result = Node<T, Self>;
}

pub trait Eval {
    type Output;

    fn eval() -> Self::Output;
}

impl Eval for Zero {
    type Output = usize;

    #[inline]
    fn eval() -> Self::Output {
        0
    }
}

impl<T: Nat> Eval for Succ<T> where T: Eval<Output = usize> {
    type Output = usize;

    #[inline]
    fn eval() -> Self::Output {
        1 + T::eval()
    }
}

pub trait Add<T: Nat>: Nat {
    type Result: Nat;
}

impl<T: Nat> Add<T> for Zero {
    type Result = T;
}

impl<T: Nat, U: Nat> Add<T> for Succ<U>
where
    U: Add<T>,
{
    type Result = Succ<U::Result>;
}

pub trait Plus {
    type Result: Stack;
}

impl<V, N> Plus for Node<V, N>
where
    N: Drop + Top,
    V: Nat + Add<<N as Top>::Result>,
    <N as Top>::Result: Nat
{
    type Result = Node<V::Result, <N as Drop>::Result>;
}

pub trait Drop {
    type Result: Stack;
}

impl<V, N> Drop for Node<V, N> where N: Stack {
    type Result = N;
}

macro_rules! forth {
    ({ $EX:ty }) => {
        $EX
    };
    ({ $EX:ty } + $($token:tt)*) => {
        forth!({ <$EX as Plus>::Result } $($token)*)
    };
    ({ $EX:ty } . as $out:ident) => {
        type $out = <$EX as Top>::Result;
    };
    ({ $EX:ty } . as $out:ident $($token:tt)+) => {
        type $out = <$EX as Top>::Result;
        forth!({ $EX } $($token)*)
    };
    ({ $EX:ty } : $name:ident $($token:tt)*) => {
        forth!(@compile $name ( ) { $EX } $($token)*)
    };
    ({ $EX:ty } ($($comment:tt)*) $($token:tt)*) => {
        forth!({ $EX } $($token)*)
    };
    ({ $EX:ty } 0 $($token:tt)*) => {
        forth!({ <$EX as Push<Zero>>::Result } $($token)*)
    };
    ({ $EX:ty } 1 $($token:tt)*) => {
        forth!({ <$EX as Push<One>>::Result } $($token)*)
    };
    ({ $EX:ty } 2 $($token:tt)*) => {
        forth!({ <$EX as Push<Two>>::Result } $($token)*)
    };
    ({ $EX:ty } 3 $($token:tt)*) => {
        forth!({ <$EX as Push<Three>>::Result } $($token)*)
    };
    ({ $EX:ty } $tok:tt $($token:tt)*) => {
        forth!({ <$EX as $tok>::Result } $($token)*)
    };
    (@compile $name:ident ($($cmd:tt)*) { $EX:ty } ($($comment:tt)*) $($token:tt)*) => {
        forth!(@compile $name ( $($cmd)* ) { $EX } $($token)*)
    };
    (@compile $name:ident ($($cmd:tt)*) { $EX:ty } ; $($token:tt)*) => {
        pub trait $name {
            type Result;
        }
        forth!(@bounds $name; $($cmd)*);
        forth!({ $EX } $($token)*);
    };
    (@compile $name:ident ($($cmd:tt)*) { $EX:ty } $tok:tt $($token:tt)*) => {
        forth!(@compile $name ( $($cmd)* $tok ) { $EX } $($token)*)
    };
    (@bounds $proc:ident; $cmd1:tt) => {
        impl<V, N> $proc for Node<V, N>
        where
            forth!({ Self }): $cmd1,
        {
            type Result = forth!({ Self } $cmd1);
        }
    };
    (@bounds $proc:ident; $cmd1:tt $cmd2:tt) => {
        impl<V, N> $proc for Node<V, N>
        where
            forth!({ Self }): $cmd1,
            forth!({ Self } $cmd1): $cmd2,
        {
            type Result = forth!({Self} $cmd1 $cmd2);
        }
    };
    (@bounds $proc:ident; $cmd1:tt $cmd2:tt $cmd3:tt) => {
        impl<V, N> $proc for Node<V, N>
        where
            forth!({ Self }): $cmd1,
            forth!({ Self } $cmd1): $cmd2,
            forth!({ Self } $cmd1 $cmd2): $cmd3
        {
            type Result = forth!({ Self } $cmd1 $cmd2 $cmd3);
        }
    };
    ($($token:tt)*) => {
        forth!({ Empty } $($token)*)
    };
}
fn main() {
    forth!(
        1 1 1
        : DoubleAdd Plus Plus ;
        . as Out1
        1 1 1
        : QuadrupleAdd DoubleAdd DoubleAdd ;
        QuadrupleAdd
        . as Out2
    );

    println!("{}", Out1::eval());
    println!("{}", Out2::eval());
}
