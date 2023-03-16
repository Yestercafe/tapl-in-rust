#[derive(Debug, Clone)]
pub enum T {
    True,
    False,
    If(Box<T>, Box<T>, Box<T>),
    Zero,
    Succ(Box<T>),
    Pred(Box<T>),
    Iszero(Box<T>),
}

#[derive(Debug, Clone)]
pub enum V {
    True,
    False,
    Nat(u64),
}
