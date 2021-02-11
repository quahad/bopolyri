use crate::mon::Monomial;
use compare::Compare;
use std::cmp::Ordering;
pub trait MonomialOrdering<'a>: Compare<Monomial<'a>> + Clone + Copy {
    fn cmp(&self, a: &Monomial, b: &Monomial) -> Ordering;
}

#[derive(Clone, Copy)]
struct DegRevLex;

#[derive(Clone, Copy)]
pub struct DegLex;

#[derive(Clone, Copy)]
pub struct Lex;

impl<'a> Compare<Monomial<'a>> for DegRevLex {
    fn compare(&self, l: &Monomial<'a>, r: &Monomial<'a>) -> Ordering {
        self.cmp(l, r)
    }
}

impl<'a> MonomialOrdering<'a> for DegRevLex {
    fn cmp(&self, a: &Monomial, b: &Monomial) -> Ordering {
        match (a, b) {
            (Monomial::Zero, Monomial::Zero) => Ordering::Equal,
            (Monomial::Zero, _) => Ordering::Less,
            (_, Monomial::Zero) => Ordering::Greater,
            (Monomial::NonZero(vars_a), Monomial::NonZero(vars_b)) => {
                match vars_a.len().cmp(&vars_b.len()) {
                    Ordering::Equal => {
                        let mut z = vars_a.iter().rev().zip(vars_b.iter().rev());
                        while let Some((v_a, v_b)) = z.next() {
                            match v_b.cmp(v_a) {
                                Ordering::Equal => {}
                                order => return order,
                            }
                        }
                        Ordering::Equal
                    }
                    order => order,
                }
            }
        }
    }
}

impl<'a> Compare<Monomial<'a>> for DegLex {
    fn compare(&self, l: &Monomial<'a>, r: &Monomial<'a>) -> Ordering {
        self.cmp(l, r)
    }
}

impl<'a> MonomialOrdering<'a> for DegLex {
    fn cmp(&self, a: &Monomial, b: &Monomial) -> Ordering {
        match (a, b) {
            (Monomial::Zero, Monomial::Zero) => Ordering::Equal,
            (Monomial::Zero, _) => Ordering::Less,
            (_, Monomial::Zero) => Ordering::Greater,
            (Monomial::NonZero(vars_a), Monomial::NonZero(vars_b)) => {
                match vars_a.len().cmp(&vars_b.len()) {
                    Ordering::Equal => {
                        let mut z = vars_a.iter().zip(vars_b.iter());
                        while let Some((v_a, v_b)) = z.next() {
                            match v_b.cmp(v_a) {
                                Ordering::Equal => {}
                                order => return order,
                            }
                        }
                        Ordering::Equal
                    }
                    order => order,
                }
            }
        }
    }
}

impl<'a> Compare<Monomial<'a>> for Lex {
    fn compare(&self, l: &Monomial<'a>, r: &Monomial<'a>) -> Ordering {
        self.cmp(l, r)
    }
}

impl<'a> MonomialOrdering<'a> for Lex {
    fn cmp(&self, a: &Monomial, b: &Monomial) -> Ordering {
        match (a, b) {
            (Monomial::Zero, Monomial::Zero) => Ordering::Equal,
            (Monomial::Zero, _) => Ordering::Less,
            (_, Monomial::Zero) => Ordering::Greater,
            (Monomial::NonZero(vars_a), Monomial::NonZero(vars_b)) => {
                match (vars_a.len(), vars_b.len()) {
                    (0, 0) => Ordering::Equal,
                    (0, _) => Ordering::Less,
                    (_, 0) => Ordering::Greater,
                    _ => {
                        let mut z = vars_a.iter().zip(vars_b.iter());
                        while let Some((v_a, v_b)) = z.next() {
                            match v_b.cmp(v_a) {
                                Ordering::Equal => {}
                                order => return order,
                            }
                        }
                        vars_a.len().cmp(&vars_b.len())
                    }
                }
            }
        }
    }
}

mod tests {
    use super::{DegLex, DegRevLex, Lex, MonomialOrdering};
    use crate::poly::Polynomial;
    use crate::{mon::Monomial, ring::Ring};
    fn test_poly<'a, O: MonomialOrdering<'a>>(
        ring: &'a Ring<'a, O>,
        ordering: O,
    ) -> Polynomial<'a, O> {
        let x: Vec<_> = (0..4)
            .map(|i| Polynomial::from_variable(ring.var(i), ordering))
            .collect();
        let x = x.iter().collect::<Vec<_>>();
        let mut p = x[0] * x[1] * x[2]
            + x[0] * x[2]
            + x[0]
            + x[1] * x[2] * x[3]
            + x[1] * x[3]
            + x[2]
            + x[3];
        p.justify();
        p
    }

    #[test]
    fn lex_order() {
        let ordering = Lex;
        let ring = Ring::new(4, ordering);
        let p = test_poly(&ring, ordering);
        assert_eq!(
            "x_0*x_1*x_2 + x_0*x_2 + x_0 + x_1*x_2*x_3 + x_1*x_3 + x_2 + x_3",
            p.to_string()
        );
    }
    #[test]
    fn degrevlex_order() {
        let ordering = DegRevLex;
        let ring = Ring::new(4, ordering);
        let p = test_poly(&ring, ordering);
        assert_eq!(
            "x_2*x_1*x_0 + x_3*x_2*x_1 + x_2*x_0 + x_3*x_1 + x_0 + x_2 + x_3",
            p.to_string()
        );
    }

    #[test]
    fn deglex_order() {
        let ordering = DegLex;
        let ring = Ring::new(4, ordering);
        let p = test_poly(&ring, ordering);
        assert_eq!(
            "x_0*x_1*x_2 + x_1*x_2*x_3 + x_0*x_2 + x_1*x_3 + x_0 + x_2 + x_3",
            p.to_string()
        );
    }
}
