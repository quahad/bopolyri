use crate::{order::MonomialOrdering, var::Variable};
use core::slice;
use sorted_vec::SortedVec;
use std::{
    cmp::Ordering,
    fmt::{self, Debug, Display},
    marker::PhantomData,
    ops::{Add, AddAssign, Mul, MulAssign},
    process::Output,
};

pub const MAX_MONOMIAL_DEGREE: usize = 10;

#[derive(Clone)]
pub enum Monomial<'a, T: MonomialOrdering> {
    NonZero {
        vars: SortedVec<&'a Variable>,
        order: PhantomData<T>,
    },
    Zero,
}

impl<'a, T: MonomialOrdering> Default for Monomial<'a, T> {
    fn default() -> Self {
        Self::one()
    }
}
impl<'a, T: MonomialOrdering> Monomial<'a, T> {
    pub fn one() -> Self {
        Monomial::NonZero {
            vars: SortedVec::with_capacity(MAX_MONOMIAL_DEGREE),
            order: PhantomData,
        }
    }
    pub fn new() -> Self {
        Monomial::one()
    }
    pub fn is_zero(&self) -> bool {
        !matches!(self, Monomial::NonZero { .. })
    }

    pub fn is_one(&self) -> bool {
        if let Monomial::NonZero { vars, .. } = self {
            vars.is_empty()
        } else {
            false
        }
    }
    pub fn degree(&self) -> usize {
        if let Monomial::NonZero { vars, .. } = self {
            vars.len()
        } else {
            0
        }
    }
    pub fn iter(&self) -> Option<std::slice::Iter<'_, &Variable>> {
        if let Monomial::NonZero { vars, .. } = self {
            Some(vars.iter())
        } else {
            None
        }
    }
}

impl<'a, T: MonomialOrdering> Display for Monomial<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Monomial::NonZero { vars, .. } = &self {
            if vars.is_empty() {
                write!(f, "1")
            } else {
                let mut mon_str = String::new();
                for (i, v) in vars.iter().enumerate() {
                    mon_str += v.to_string().as_str();
                    if i != vars.len() - 1 {
                        mon_str += "*";
                    }
                }
                write!(f, "{}", mon_str)
            }
        } else {
            write!(f, "0")
        }
    }
}

impl<'a, T: MonomialOrdering> Debug for Monomial<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Monomial::Zero => write!(f, ""),
            Monomial::NonZero { vars, .. } => {
                let vars = vars.clone();
                let vars = vars.to_vec();
                write!(f, "{:?}", vars)
            }
        }
    }
}

impl<'a, T: MonomialOrdering> From<&'a Variable> for Monomial<'a, T> {
    fn from(v: &'a Variable) -> Monomial<'a, T> {
        let mut vars = SortedVec::with_capacity(MAX_MONOMIAL_DEGREE);
        vars.insert(v);
        Monomial::NonZero {
            vars,
            order: PhantomData,
        }
    }
}

impl<'a, T: MonomialOrdering> PartialEq for Monomial<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Monomial::Zero, Monomial::Zero) => true,
            (Monomial::NonZero { vars: vars_a, .. }, Monomial::NonZero { vars: vars_b, .. }) => {
                vars_a == vars_b
            }
            _ => false,
        }
    }
}

impl<'a, T: MonomialOrdering> Mul for Monomial<'a, T> {
    type Output = Monomial<'a, T>;
    fn mul(self, rhs: Self) -> Monomial<'a, T> {
        match (self.is_zero(), rhs.is_zero()) {
            (true, _) => Monomial::Zero,
            (_, true) => Monomial::Zero,
            (false, false) => {
                if let (
                    Monomial::NonZero {
                        vars: mut vars_a, ..
                    },
                    Monomial::NonZero { vars: vars_b, .. },
                ) = (self, rhs)
                {
                    for v in vars_b.into_vec() {
                        vars_a.find_or_insert(&v);
                    }
                    Monomial::NonZero {
                        vars: vars_a,
                        order: PhantomData,
                    }
                } else {
                    panic!("Invalid State in Mul")
                }
            }
        }
    }
}

impl<'a, 'b, T: MonomialOrdering> MulAssign<&'b Monomial<'a, T>> for Monomial<'a, T> {
    fn mul_assign(&mut self, rhs: &'b Monomial<'a, T>) {
        match (self.is_zero(), rhs.is_zero()) {
            (true, _) => *self = Monomial::Zero,
            (_, true) => *self = Monomial::Zero,
            (false, false) => {
                if let (
                    Monomial::NonZero { vars: vars_a, .. },
                    Monomial::NonZero { vars: vars_b, .. },
                ) = (self, rhs)
                {
                    for v in vars_b.iter() {
                        vars_a.find_or_insert(v);
                    }
                } else {
                    panic!("Invalid State in Mul")
                }
            }
        }
    }
}

impl<'a, 'b, T: MonomialOrdering> Mul<&'b Monomial<'a, T>> for &Monomial<'a, T> {
    type Output = <Monomial<'a, T> as Mul<Monomial<'a, T>>>::Output;
    fn mul(self, rhs: &'b Monomial<'a, T>) -> Monomial<'a, T> {
        match (self.is_zero(), rhs.is_zero()) {
            (true, _) => Monomial::Zero,
            (_, true) => Monomial::Zero,
            (false, false) => {
                if let (
                    Monomial::NonZero { vars: vars_a, .. },
                    Monomial::NonZero { vars: vars_b, .. },
                ) = (self, rhs)
                {
                    let mut vars_res = vars_a.clone();
                    for v in vars_b.iter() {
                        vars_res.find_or_insert(v);
                    }
                    Monomial::NonZero {
                        vars: vars_res,
                        order: PhantomData,
                    }
                } else {
                    panic!("Invalid State in Mul")
                }
            }
        }
    }
}

impl<'a, 'b, T: MonomialOrdering> Mul<&'a Variable> for Monomial<'a, T> {
    type Output = <Monomial<'a, T> as Mul<Monomial<'a, T>>>::Output;
    fn mul(self, rhs: &'a Variable) -> Monomial<'a, T> {
        if let Monomial::NonZero { vars: vars_a, .. } = self {
            let mut vars_res = vars_a.clone();
            vars_res.find_or_insert(rhs);
            Monomial::NonZero {
                vars: vars_res,
                order: PhantomData,
            }
        } else {
            Monomial::Zero
        }
    }
}

impl<'a, 'b, T: MonomialOrdering> Mul<&'a Variable> for &Monomial<'a, T> {
    type Output = <Monomial<'a, T> as Mul<Monomial<'a, T>>>::Output;
    fn mul(self, rhs: &'a Variable) -> Monomial<'a, T> {
        if let Monomial::NonZero { vars: vars_a, .. } = self {
            let mut vars_res = vars_a.clone();
            vars_res.find_or_insert(rhs);
            Monomial::NonZero {
                vars: vars_res,
                order: PhantomData,
            }
        } else {
            Monomial::Zero
        }
    }
}

impl<'a, T: MonomialOrdering> PartialOrd for Monomial<'a, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(T::cmp(self, other))
        // match (self, other) {
        //     (Monomial::Zero, Monomial::Zero) => Some(Ordering::Equal),
        //     (Monomial::Zero, _) => Some(Ordering::Less),
        //     (_, Monomial::Zero) => Some(Ordering::Greater),
        //     (Monomial::NonZero { vars: vars_a, .. }, Monomial::NonZero { vars: vars_b, .. }) => {
        //         T::cmp()
        //     }
        // }
    }
}
impl<'a, T: MonomialOrdering> Ord for Monomial<'a, T> {
    fn cmp(&self, other: &Self) -> Ordering {
        T::cmp(self, other)
    }
}

// impl<'a, T: MonomialOrdering> PartialEq for Monomial<'a, T> {
//     fn eq(&self, other: &Self) -> bool {
//         self.order == other.order
//     }
// }

impl<'a, T: MonomialOrdering> Eq for Monomial<'a, T> {}

#[cfg(test)]
mod tests {
    #[test]
    fn display() {
        assert_eq!(2 + 2, 4);
    }
}
