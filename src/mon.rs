use crate::var::Variable;
use core::slice;
use sorted_vec::SortedVec;
use std::{
    fmt::{self, Debug, Display},
    ops::{Add, AddAssign, Mul, MulAssign},
    process::Output,
};

pub const MAX_MONOMIAL_DEGREE: usize = 10;

#[derive(Clone)]
pub enum Monomial<'a> {
    NonZero(SortedVec<&'a Variable>),
    Zero,
}

impl<'a> Monomial<'a> {
    pub fn one() -> Self {
        Monomial::NonZero(SortedVec::with_capacity(MAX_MONOMIAL_DEGREE))
    }
    pub fn new() -> Self {
        Monomial::one()
    }
    pub fn is_zero(&self) -> bool {
        if let Monomial::NonZero(_) = self {
            false
        } else {
            true
        }
    }

    pub fn is_one(&self) -> bool {
        if let Monomial::NonZero(vars) = self {
            vars.is_empty()
        } else {
            false
        }
    }
    pub fn degree(&self) -> usize {
        if let Monomial::NonZero(vars) = self {
            vars.len()
        } else {
            0
        }
    }
    pub fn iter(&self) -> Option<std::slice::Iter<'_, &Variable>> {
        if let Monomial::NonZero(vars) = self {
            Some(vars.iter())
        } else {
            None
        }
    }
}

impl<'a> Display for Monomial<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Monomial::NonZero(vars) = &self {
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

impl<'a> Debug for Monomial<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Monomial::Zero => write!(f, ""),
            Monomial::NonZero(vars) => {
                let vars = vars.clone();
                let vars = vars.to_vec();
                write!(f, "{:?}", vars)
            }
        }
    }
}

impl<'a> From<&'a Variable> for Monomial<'a> {
    fn from(v: &'a Variable) -> Monomial<'a> {
        let mut vars = SortedVec::with_capacity(MAX_MONOMIAL_DEGREE);
        vars.insert(v);
        Monomial::NonZero(vars)
    }
}

impl<'a> PartialEq for Monomial<'a> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Monomial::Zero, Monomial::Zero) => true,
            (Monomial::NonZero(vars_a), Monomial::NonZero(vars_b)) => vars_a == vars_b,
            _ => false,
        }
    }
}

impl<'a> Mul for Monomial<'a> {
    type Output = Monomial<'a>;
    fn mul(self, rhs: Self) -> Monomial<'a> {
        match (self.is_zero(), rhs.is_zero()) {
            (true, _) => Monomial::Zero,
            (_, true) => Monomial::Zero,
            (false, false) => {
                if let (Monomial::NonZero(mut vars_a), Monomial::NonZero(vars_b)) = (self, rhs) {
                    for v in vars_b.into_vec() {
                        vars_a.find_or_insert(&v);
                    }
                    Monomial::NonZero(vars_a)
                } else {
                    panic!("Invalid State in Mul")
                }
            }
        }
    }
}

impl<'a, 'b> MulAssign<&'b Monomial<'a>> for Monomial<'a> {
    fn mul_assign(&mut self, rhs: &'b Monomial<'a>) {
        match (self.is_zero(), rhs.is_zero()) {
            (true, _) => *self = Monomial::Zero,
            (_, true) => *self = Monomial::Zero,
            (false, false) => {
                if let (Monomial::NonZero(vars_a), Monomial::NonZero(vars_b)) = (self, rhs) {
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

impl<'a, 'b> Mul<&'b Monomial<'a>> for &Monomial<'a> {
    type Output = <Monomial<'a> as Mul<Monomial<'a>>>::Output;
    fn mul(self, rhs: &'b Monomial<'a>) -> Monomial<'a> {
        match (self.is_zero(), rhs.is_zero()) {
            (true, _) => Monomial::Zero,
            (_, true) => Monomial::Zero,
            (false, false) => {
                if let (Monomial::NonZero(vars_a), Monomial::NonZero(vars_b)) = (self, rhs) {
                    let mut vars_res = vars_a.clone();
                    for v in vars_b.iter() {
                        vars_res.find_or_insert(v);
                    }
                    Monomial::NonZero(vars_res)
                } else {
                    panic!("Invalid State in Mul")
                }
            }
        }
    }
}

impl<'a, 'b> Mul<&'a Variable> for Monomial<'a> {
    type Output = <Monomial<'a> as Mul<Monomial<'a>>>::Output;
    fn mul(self, rhs: &'a Variable) -> Monomial<'a> {
        if let Monomial::NonZero(vars_a) = self {
            let mut vars_res = vars_a.clone();
            vars_res.find_or_insert(rhs);
            Monomial::NonZero(vars_res)
        } else {
            Monomial::Zero
        }
    }
}

impl<'a, 'b> Mul<&'a Variable> for &Monomial<'a> {
    type Output = <Monomial<'a> as Mul<Monomial<'a>>>::Output;
    fn mul(self, rhs: &'a Variable) -> Monomial<'a> {
        if let Monomial::NonZero(vars_a) = self {
            let mut vars_res = vars_a.clone();
            vars_res.find_or_insert(rhs);
            Monomial::NonZero(vars_res)
        } else {
            Monomial::Zero
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn display() {
        assert_eq!(2 + 2, 4);
    }
}
