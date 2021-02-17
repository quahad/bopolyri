use crate::{mon::Monomial, order::MonomialOrdering, ring::Ring, var::Variable};
use binary_heap_plus::{BinaryHeap, MaxComparator};
use compare::Compare;
use std::{
    collections::BTreeSet,
    fmt::Display,
    ops::{Add, AddAssign, Mul},
};

pub struct Polynomial<'a, T: MonomialOrdering> {
    mons: BTreeSet<Monomial<'a, T>>,
    ring: &'a Ring<T>,
}

impl<'a, T: MonomialOrdering> Clone for Polynomial<'a, T> {
    fn clone(&self) -> Self {
        Polynomial {
            mons: self.mons.clone(),
            ring: self.ring,
        }
    }
}

// impl<'a, T: MonomialOrdering> Default for Polynomial<'a, T> {
//     fn default() -> Self {
//         Self::new()
//     }
// }

impl<'a, T: MonomialOrdering> Polynomial<'a, T> {
    pub fn new(ring: &'a Ring<T>) -> Self {
        Polynomial::zero(ring)
    }
    pub fn zero(ring: &'a Ring<T>) -> Self {
        Polynomial {
            mons: BTreeSet::new(),
            ring,
        }
    }
    pub fn one(ring: &'a Ring<T>) -> Self {
        let mut mons = BTreeSet::new();
        mons.insert(Monomial::one(ring));
        Polynomial { mons, ring }
    }

    pub fn ring(&self) -> &'a Ring<T> {
        self.ring
    }

    pub fn lm(&self) -> &Monomial<T> {
        let lt = self.mons.last();
        lt.map_or(&Monomial::Zero, |m| m)
    }

    pub fn justify_lm(&mut self) {
        // while let Some(lm) = self.mons.pop() {
        //     if let Some(next_lm) = self.mons.peek() {
        //         if lm == *next_lm {
        //             self.mons.pop();
        //         } else {
        //             self.mons.insert(lm);
        //             break;
        //         }
        //     } else {
        //         self.mons.insert(lm);
        //         break;
        //     }
        // }
    }
    pub fn justify(&mut self) {
        // let mut just_mons = BTreeSet::new();
        // while let Some(lm) = self.mons.pop() {
        //     if let Some(next_lm) = self.mons.peek() {
        //         if lm != *next_lm {
        //             //println!("LM: {:?} NEXT_LM {:?}", lm, next_lm);
        //             just_mons.insert(lm);
        //         } else {
        //             self.mons.pop();
        //         }
        //     } else {
        //         just_mons.insert(lm);
        //     }
        // }
        // self.mons = just_mons;
    }
    pub fn from_monomial(ring: &'a Ring<T>, m: Monomial<'a, T>) -> Self {
        let mut pol = Self::new(ring);
        pol.mons.insert(m);
        pol
    }

    pub fn from_variable(ring: &'a Ring<T>, v: &'a Variable) -> Self {
        let mut pol = Self::new(ring);
        pol.mons.insert(Monomial::from_variable(ring, v));
        pol
    }

    pub fn is_zero(&self) -> bool {
        //self.justify();
        self.mons.is_empty() || (self.mons.len() == 1 && self.mons.last().unwrap().is_zero())
    }

    pub fn is_zero_mut(&mut self) -> bool {
        self.justify();
        self.mons.is_empty() || (self.mons.len() == 1 && self.mons.last().unwrap().is_zero())
    }

    // pub fn ordering(&self) -> T {
    //     self.ordering
    // }
}

impl<'a, T: MonomialOrdering> Display for Polynomial<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.mons.is_empty() {
            write!(f, "0")
        } else {
            let mut pol_str = String::new();
            let mons = self.mons.iter();
            for (i, mon) in mons.enumerate().rev() {
                pol_str += mon.to_string().as_str();
                if i != 0 {
                    pol_str += " + ";
                }
            }
            write!(f, "{}", pol_str)
        }
    }
}

impl<'a, T: MonomialOrdering> Add for Polynomial<'a, T> {
    type Output = Polynomial<'a, T>;
    fn add(self, rhs: Self) -> Self {
        let mut new_pol = self;
        new_pol.add_assign(&rhs);
        new_pol.justify_lm();
        new_pol
    }
}

impl<'a, T: MonomialOrdering> Add<&Polynomial<'a, T>> for Polynomial<'a, T> {
    type Output = <Polynomial<'a, T> as Add<Polynomial<'a, T>>>::Output;
    fn add(self, rhs: &'_ Self) -> Polynomial<'a, T> {
        let mut new_pol = self;
        new_pol.add_assign(rhs);
        new_pol.justify_lm();
        new_pol
    }
}

impl<'a, T: MonomialOrdering> Add<&Polynomial<'a, T>> for &Polynomial<'a, T> {
    type Output = <Polynomial<'a, T> as Add<Polynomial<'a, T>>>::Output;
    fn add(self, rhs: &'_ Polynomial<'a, T>) -> Polynomial<'a, T> {
        let mut res_pol = self.clone();
        res_pol.add_assign(rhs);
        res_pol.justify_lm();
        res_pol
    }
}

impl<'a, T: MonomialOrdering> Add<Polynomial<'a, T>> for &Polynomial<'a, T> {
    type Output = <Polynomial<'a, T> as Add<Polynomial<'a, T>>>::Output;
    fn add(self, rhs: Polynomial<'a, T>) -> Polynomial<'a, T> {
        let mut res_pol = self.clone();
        res_pol.add_assign(&rhs);
        res_pol.justify_lm();
        res_pol
    }
}

impl<'a, T: MonomialOrdering> Add<Monomial<'a, T>> for Polynomial<'a, T> {
    type Output = <Polynomial<'a, T> as Add<Polynomial<'a, T>>>::Output;
    fn add(self, rhs: Monomial<'a, T>) -> Polynomial<'a, T> {
        if !rhs.is_zero() {
            let mut new_pol = self;
            new_pol.add_assign(&rhs);
            new_pol
        } else {
            self
        }
    }
}
impl<'a, T: MonomialOrdering> Add<&Monomial<'a, T>> for Polynomial<'a, T> {
    type Output = <Polynomial<'a, T> as Add<Polynomial<'a, T>>>::Output;
    fn add(self, rhs: &'_ Monomial<'a, T>) -> Polynomial<'a, T> {
        if !rhs.is_zero() {
            let mut new_pol = self;
            new_pol.add_assign(rhs);
            new_pol
        } else {
            self
        }
    }
}

impl<'a, T: MonomialOrdering> Add<Monomial<'a, T>> for &Polynomial<'a, T> {
    type Output = <Polynomial<'a, T> as Add<Polynomial<'a, T>>>::Output;
    fn add(self, rhs: Monomial<'a, T>) -> Polynomial<'a, T> {
        if !rhs.is_zero() {
            let mut res_pol = self.clone();
            if !res_pol.mons.insert(rhs.clone()) {
                res_pol.mons.remove(&rhs);
            }
            res_pol.justify_lm();
            res_pol
        } else {
            self.clone()
        }
    }
}

impl<'a, T: MonomialOrdering> Add<&Monomial<'a, T>> for &Polynomial<'a, T> {
    type Output = <Polynomial<'a, T> as Add<Polynomial<'a, T>>>::Output;
    fn add(self, rhs: &'_ Monomial<'a, T>) -> Polynomial<'a, T> {
        if !rhs.is_zero() {
            let mut res_pol = self.clone();
            if !res_pol.mons.insert(rhs.clone()) {
                res_pol.mons.remove(rhs);
            }
            res_pol.justify_lm();
            res_pol
        } else {
            self.clone()
        }
    }
}

impl<'a, T: MonomialOrdering> AddAssign<&Monomial<'a, T>> for Polynomial<'a, T> {
    fn add_assign(&mut self, m: &'_ Monomial<'a, T>) {
        if !self.mons.insert(m.clone()) {
            self.mons.remove(m);
        }
        self.justify_lm();
    }
}

impl<'a, T: MonomialOrdering> AddAssign<&Polynomial<'a, T>> for Polynomial<'a, T> {
    fn add_assign(&mut self, rhs: &'_ Self) {
        // for m in rhs.mons.iter() {
        //     self.mons.insert(m.clone());
        // }
        let mut mons = BTreeSet::new();
        for m in self.mons.symmetric_difference(&rhs.mons).into_iter() {
            mons.insert(m.clone());
        }
        self.mons = mons;

        self.justify_lm();
    }
}

impl<'a, T: MonomialOrdering> AddAssign<u64> for Polynomial<'a, T> {
    fn add_assign(&mut self, rhs: u64) {
        if rhs % 2 == 1 {
            self.mons.insert(Monomial::one(self.ring));
        }
        self.justify_lm();
    }
}

impl<'a, T: MonomialOrdering> AddAssign for Polynomial<'a, T> {
    fn add_assign(&mut self, rhs: Self) {
        self.add_assign(&rhs);
        self.justify_lm();
    }
}

impl<'a, T: MonomialOrdering> Mul for Polynomial<'a, T> {
    type Output = Polynomial<'a, T>;
    fn mul(self, rhs: Self) -> Self {
        self.mul(&rhs)
    }
}

impl<'a, T: MonomialOrdering> Mul<&Polynomial<'a, T>> for Polynomial<'a, T> {
    type Output = <Polynomial<'a, T> as Mul<Polynomial<'a, T>>>::Output;
    fn mul(self, rhs: &'_ Self) -> Self {
        (&self).mul(rhs)
    }
}

impl<'a, T: MonomialOrdering> Mul<&Polynomial<'a, T>> for &Polynomial<'a, T> {
    type Output = <Polynomial<'a, T> as Mul<Polynomial<'a, T>>>::Output;
    fn mul(self, rhs: &Polynomial<'a, T>) -> Polynomial<'a, T> {
        let mut res_pol = Polynomial::zero(self.ring);
        if self.is_zero() || rhs.is_zero() {
            return Polynomial::zero(self.ring);
        }
        for m in rhs.mons.iter() {
            res_pol += self * m;
        }
        res_pol.justify();
        res_pol
    }
}
impl<'a, T: MonomialOrdering> Mul<&Monomial<'a, T>> for Polynomial<'a, T> {
    type Output = <Polynomial<'a, T> as Mul<Polynomial<'a, T>>>::Output;
    fn mul(self, rhs: &'_ Monomial<'a, T>) -> Polynomial<'a, T> {
        if rhs.is_zero() {
            Polynomial::zero(self.ring)
        } else if rhs.is_one() {
            self
        } else {
            let mut new_pol = Polynomial::zero(self.ring);
            for m in self.mons.iter() {
                new_pol.mons.insert(m * rhs);
            }
            new_pol.justify();
            new_pol
        }
    }
}

impl<'a, T: MonomialOrdering> Mul<&Monomial<'a, T>> for &Polynomial<'a, T> {
    type Output = <Polynomial<'a, T> as Mul<Polynomial<'a, T>>>::Output;
    fn mul(self, rhs: &'_ Monomial<'a, T>) -> Polynomial<'a, T> {
        if rhs.is_zero() {
            Polynomial::zero(self.ring)
        } else if rhs.is_one() {
            self.clone()
        } else {
            let mut new_pol = Polynomial::zero(self.ring);
            for m in self.mons.iter() {
                new_pol.mons.insert(m * rhs);
            }
            new_pol.justify();
            new_pol
        }
    }
}

impl<'a, T: MonomialOrdering> Mul<Monomial<'a, T>> for Polynomial<'a, T> {
    type Output = <Polynomial<'a, T> as Mul<Polynomial<'a, T>>>::Output;
    fn mul(self, rhs: Monomial<'a, T>) -> Polynomial<'a, T> {
        self.mul(&rhs)
    }
}

impl<'a, T: MonomialOrdering> Mul<Monomial<'a, T>> for &Polynomial<'a, T> {
    type Output = <Polynomial<'a, T> as Mul<Polynomial<'a, T>>>::Output;
    fn mul(self, rhs: Monomial<'a, T>) -> Polynomial<'a, T> {
        self.mul(&rhs)
    }
}

mod tests {
    use crate::{mon::Monomial, order::Lex, ring::Ring};

    use super::Polynomial;
    #[test]
    fn mul() {
        let ring = Ring::<Lex>::new(8);
        let p1 = Polynomial::from_variable(&ring, ring.var(0));
        let p2 = Polynomial::from_variable(&ring, ring.var(1));
        assert_eq!("x_0*x_1", (p1 * p2).to_string());
    }

    #[test]
    fn mul_mon() {
        let ordering = Lex;
        let ring = Ring::<Lex>::new(2);
        let mut p1 = Polynomial::from_variable(&ring, ring.var(0));
        p1 += Polynomial::from_monomial(&ring, Monomial::one(&ring));
        let m: Monomial<_> = Monomial::from_variable(&ring, ring.var(1));
        assert_eq!("x_0*x_1 + x_1", (p1 * m).to_string());
    }
}
