use crate::syntax::{T, V};

pub fn eval(expr: T) -> V {
    match expr {
        T::True => V::True,
        T::False => V::False,
        T::If(t1, t2, t3) => match eval(*t1) {
            V::True => eval(*t2),
            V::False => eval(*t3),
        },
    }
}
