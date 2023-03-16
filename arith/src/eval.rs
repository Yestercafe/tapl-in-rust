use crate::syntax::{T, V};

pub fn eval(expr: T) -> Result<V, &'static str> {
    match expr {
        T::True => Ok(V::True),
        T::False => Ok(V::False),
        T::Zero => Ok(V::Nat(0)),

        T::If(t1, t2, t3) => match eval(*t1).unwrap() {
            V::True => eval(*t2),
            V::False => eval(*t3),
            _ => Err("Semantic error: (Num) in condition of if")
        },

        T::Succ(t1) => match eval(*t1).unwrap() {
            V::Nat(n) => Ok(V::Nat(n + 1)),
            _ => Err("Sematic error: Succ (NotANum)"),
        }

        T::Pred(t1) => match eval(*t1).unwrap() {
            V::Nat(0) => Ok(V::Nat(0)),
            V::Nat(n) => Ok(V::Nat(n - 1)),
            _ => Err("Semantic error: Pred (NotANum)"),
        }

        T::Iszero(t1) => match eval(*t1).unwrap() {
            V::Nat(0) => Ok(V::True),
            V::Nat(_) => Ok(V::False),
            _ => Err("Semantic error: Num (NotANum)")
        },
    }
}
