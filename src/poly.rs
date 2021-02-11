use crate::{mon::Monomial, order::MonomialOrdering, var::Variable};
use binary_heap_plus::{BinaryHeap, MaxComparator};
use compare::Compare;
use std::{
    fmt::Display,
    ops::{Add, AddAssign, Mul},
};

pub struct Polynomial<'a, T: MonomialOrdering<'a>> {
    mons: BinaryHeap<Monomial<'a>, T>,
    ordering: T,
}

impl<'a, T: MonomialOrdering<'a>> Clone for Polynomial<'a, T> {
    fn clone(&self) -> Self {
        Polynomial {
            mons: self.mons.clone(),
            ordering: self.ordering,
        }
    }
}

impl<'a, T: MonomialOrdering<'a>> Polynomial<'a, T> {
    pub fn new(ordering: T) -> Self {
        Polynomial::zero(ordering)
    }
    pub fn zero(ordering: T) -> Self {
        Polynomial {
            mons: BinaryHeap::from_vec_cmp(Vec::new(), ordering),
            ordering,
        }
    }
    pub fn one(ordering: T) -> Self {
        let mut mons = BinaryHeap::from_vec_cmp(Vec::new(), ordering);
        mons.push(Monomial::one());
        Polynomial {
            mons: mons,
            ordering,
        }
    }

    pub fn lm(&self) -> &Monomial {
        let lt = self.mons.peek();
        lt.map_or(&Monomial::Zero, |m| m)
    }

    pub fn justify_lm(&mut self) {
        while let Some(lm) = self.mons.pop() {
            if let Some(next_lm) = self.mons.peek() {
                if lm == *next_lm {
                    self.mons.pop();
                } else {
                    self.mons.push(lm);
                    break;
                }
            } else {
                self.mons.push(lm);
                break;
            }
        }
    }
    pub fn justify(&mut self) {
        let mut just_mons = BinaryHeap::from_vec_cmp(Vec::new(), self.ordering);
        while let Some(lm) = self.mons.pop() {
            if let Some(next_lm) = self.mons.peek() {
                if lm != *next_lm {
                    //println!("LM: {:?} NEXT_LM {:?}", lm, next_lm);
                    just_mons.push(lm);
                } else {
                    self.mons.pop();
                }
            } else {
                just_mons.push(lm);
            }
        }
        self.mons = just_mons;
    }
    pub fn from_monomial(m: Monomial<'a>, ordering: T) -> Self {
        let mut pol = Self::new(ordering);
        pol.mons.push(m);
        pol
    }

    pub fn from_variable(v: &'a Variable, ordering: T) -> Self {
        let mut pol = Self::new(ordering);
        pol.mons.push(v.into());
        pol
    }

    pub fn is_zero(&self) -> bool {
        //self.justify();
        if self.mons.is_empty() || (self.mons.len() == 1 && self.mons.peek().unwrap().is_zero()) {
            true
        } else {
            false
        }
    }

    pub fn is_zero_mut(&mut self) -> bool {
        self.justify();
        if self.mons.is_empty() || (self.mons.len() == 1 && self.mons.peek().unwrap().is_zero()) {
            true
        } else {
            false
        }
    }

    pub fn ordering(&self) -> T {
        self.ordering
    }
}

impl<'a, T: MonomialOrdering<'a>> Display for Polynomial<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.mons.is_empty() {
            write!(f, "0")
        } else {
            let mut pol_str = String::new();
            let mons = self.mons.clone().into_sorted_vec();
            for (i, mon) in mons.iter().enumerate().rev() {
                pol_str += mon.to_string().as_str();
                if i != 0 {
                    pol_str += " + ";
                }
            }
            write!(f, "{}", pol_str)
        }
    }
}

impl<'a, T: MonomialOrdering<'a>> Add for Polynomial<'a, T> {
    type Output = Polynomial<'a, T>;
    fn add(self, rhs: Self) -> Self {
        let mut res_pol = self.clone();
        for m in rhs.mons.iter() {
            res_pol.mons.push(m.clone());
        }
        res_pol.justify_lm();
        res_pol
    }
}

impl<'a, T: MonomialOrdering<'a>> Add<&Polynomial<'a, T>> for Polynomial<'a, T> {
    type Output = <Polynomial<'a, T> as Add<Polynomial<'a, T>>>::Output;
    fn add(self, rhs: &'_ Self) -> Polynomial<'a, T> {
        let mut res_pol = self.clone();
        for m in rhs.mons.iter() {
            res_pol.mons.push(m.clone());
        }
        res_pol.justify_lm();
        res_pol
    }
}

impl<'a, T: MonomialOrdering<'a>> Add<&Polynomial<'a, T>> for &Polynomial<'a, T> {
    type Output = <Polynomial<'a, T> as Add<Polynomial<'a, T>>>::Output;
    fn add(self, rhs: &'_ Polynomial<'a, T>) -> Polynomial<'a, T> {
        let mut res_pol = self.clone();
        for m in rhs.mons.iter() {
            res_pol.mons.push(m.clone());
        }
        res_pol.justify_lm();
        res_pol
    }
}

impl<'a, T: MonomialOrdering<'a>> Add<Polynomial<'a, T>> for &Polynomial<'a, T> {
    type Output = <Polynomial<'a, T> as Add<Polynomial<'a, T>>>::Output;
    fn add(self, rhs: Polynomial<'a, T>) -> Polynomial<'a, T> {
        let mut res_pol = self.clone();
        for m in rhs.mons.iter() {
            res_pol.mons.push(m.clone());
        }
        res_pol.justify_lm();
        res_pol
    }
}

impl<'a, T: MonomialOrdering<'a>> Add<Monomial<'a>> for Polynomial<'a, T> {
    type Output = <Polynomial<'a, T> as Add<Polynomial<'a, T>>>::Output;
    fn add(self, rhs: Monomial<'a>) -> Polynomial<'a, T> {
        if !rhs.is_zero() {
            let mut res_pol = self.clone();
            res_pol.mons.push(rhs);
            res_pol.justify_lm();
            res_pol
        } else {
            self
        }
    }
}
impl<'a, T: MonomialOrdering<'a>> Add<&Monomial<'a>> for Polynomial<'a, T> {
    type Output = <Polynomial<'a, T> as Add<Polynomial<'a, T>>>::Output;
    fn add(self, rhs: &'_ Monomial<'a>) -> Polynomial<'a, T> {
        if !rhs.is_zero() {
            let mut res_pol = self.clone();
            res_pol.mons.push(rhs.clone());
            res_pol.justify_lm();
            res_pol
        } else {
            self
        }
    }
}

impl<'a, T: MonomialOrdering<'a>> Add<Monomial<'a>> for &Polynomial<'a, T> {
    type Output = <Polynomial<'a, T> as Add<Polynomial<'a, T>>>::Output;
    fn add(self, rhs: Monomial<'a>) -> Polynomial<'a, T> {
        if !rhs.is_zero() {
            let mut res_pol = self.clone();
            res_pol.mons.push(rhs);
            res_pol.justify_lm();
            res_pol
        } else {
            self.clone()
        }
    }
}

impl<'a, T: MonomialOrdering<'a>> Add<&Monomial<'a>> for &Polynomial<'a, T> {
    type Output = <Polynomial<'a, T> as Add<Polynomial<'a, T>>>::Output;
    fn add(self, rhs: &'_ Monomial<'a>) -> Polynomial<'a, T> {
        if !rhs.is_zero() {
            let mut res_pol = self.clone();
            res_pol.mons.push(rhs.clone());
            res_pol.justify_lm();
            res_pol
        } else {
            self.clone()
        }
    }
}

impl<'a, T: MonomialOrdering<'a>> AddAssign<&Monomial<'a>> for Polynomial<'a, T> {
    fn add_assign(&mut self, m: &'_ Monomial<'a>) {
        self.mons.push(m.clone());
        self.justify_lm();
    }
}

impl<'a, T: MonomialOrdering<'a>> AddAssign<&Polynomial<'a, T>> for Polynomial<'a, T> {
    fn add_assign(&mut self, rhs: &'_ Self) {
        for m in rhs.mons.iter() {
            self.mons.push(m.clone());
        }
        self.justify_lm();
    }
}

impl<'a, T: MonomialOrdering<'a>> AddAssign<u64> for Polynomial<'a, T> {
    fn add_assign(&mut self, rhs: u64) {
        if rhs % 2 == 1 {
            self.mons.push(Monomial::one());
        }
        self.justify_lm();
    }
}

impl<'a, T: MonomialOrdering<'a>> AddAssign for Polynomial<'a, T> {
    fn add_assign(&mut self, rhs: Self) {
        self.add_assign(&rhs);
        self.justify_lm();
    }
}

impl<'a, T: MonomialOrdering<'a>> Mul for Polynomial<'a, T> {
    type Output = Polynomial<'a, T>;
    fn mul(self, rhs: Self) -> Self {
        self.mul(&rhs)
    }
}

impl<'a, T: MonomialOrdering<'a>> Mul<&Polynomial<'a, T>> for Polynomial<'a, T> {
    type Output = <Polynomial<'a, T> as Mul<Polynomial<'a, T>>>::Output;
    fn mul(self, rhs: &'_ Self) -> Self {
        (&self).mul(rhs)
    }
}

impl<'a, T: MonomialOrdering<'a>> Mul<&Polynomial<'a, T>> for &Polynomial<'a, T> {
    type Output = <Polynomial<'a, T> as Mul<Polynomial<'a, T>>>::Output;
    fn mul(self, rhs: &Polynomial<'a, T>) -> Polynomial<'a, T> {
        let mut res_pol = Polynomial::zero(self.ordering);
        if self.is_zero() || rhs.is_zero() {
            return Polynomial::zero(self.ordering);
        }
        for m in rhs.mons.iter() {
            res_pol += self * m;
        }
        res_pol.justify();
        res_pol
    }
}
impl<'a, T: MonomialOrdering<'a>> Mul<&Monomial<'a>> for Polynomial<'a, T> {
    type Output = <Polynomial<'a, T> as Mul<Polynomial<'a, T>>>::Output;
    fn mul(self, rhs: &'_ Monomial<'a>) -> Polynomial<'a, T> {
        if rhs.is_zero() {
            Polynomial::zero(self.ordering)
        } else if rhs.is_one() {
            self
        } else {
            let mut new_pol = Polynomial::zero(self.ordering);
            for m in self.mons.iter() {
                new_pol.mons.push(m * rhs);
            }
            new_pol.justify();
            new_pol
        }
    }
}

impl<'a, T: MonomialOrdering<'a>> Mul<&Monomial<'a>> for &Polynomial<'a, T> {
    type Output = <Polynomial<'a, T> as Mul<Polynomial<'a, T>>>::Output;
    fn mul(self, rhs: &'_ Monomial<'a>) -> Polynomial<'a, T> {
        if rhs.is_zero() {
            Polynomial::zero(self.ordering)
        } else if rhs.is_one() {
            self.clone()
        } else {
            let mut new_pol = Polynomial::zero(self.ordering);
            for m in self.mons.iter() {
                new_pol.mons.push(m * rhs);
            }
            new_pol.justify();
            new_pol
        }
    }
}

impl<'a, T: MonomialOrdering<'a>> Mul<Monomial<'a>> for Polynomial<'a, T> {
    type Output = <Polynomial<'a, T> as Mul<Polynomial<'a, T>>>::Output;
    fn mul(self, rhs: Monomial<'a>) -> Polynomial<'a, T> {
        self.mul(&rhs)
    }
}

impl<'a, T: MonomialOrdering<'a>> Mul<Monomial<'a>> for &Polynomial<'a, T> {
    type Output = <Polynomial<'a, T> as Mul<Polynomial<'a, T>>>::Output;
    fn mul(self, rhs: Monomial<'a>) -> Polynomial<'a, T> {
        self.mul(&rhs)
    }
}

mod tests {
    use crate::{mon::Monomial, order::Lex, ring::Ring};

    use super::Polynomial;
    #[test]
    fn mul() {
        let ring = Ring::new(8, Lex);
        let p1 = Polynomial::from_variable(ring.var(0), Lex);
        let p2 = Polynomial::from_variable(ring.var(1), Lex);
        assert_eq!("x_0*x_1", (p1 * p2).to_string());
    }

    #[test]
    fn mul_mon() {
        let ordering = Lex;
        let ring = Ring::new(2, ordering);
        let mut p1 = Polynomial::from_variable(ring.var(0), ordering);
        p1 += Polynomial::from_monomial(Monomial::one(), ordering);
        let m: Monomial = ring.var(1).into();
        assert_eq!("x_0*x_1 + x_1", (p1 * m).to_string());
    }
}
