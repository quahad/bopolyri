use std::{marker::PhantomData};

use crate::var::Variable;
use crate::{order::MonomialOrdering, var::AssociatedVariableType};

pub type BoxedRing<T> = Box<Ring<T>>;
pub struct Ring<T: MonomialOrdering> {
    vars: Vec<Variable>,
    order: PhantomData<T>,
}

impl<T: MonomialOrdering> Ring<T> {
    pub fn new(n: usize) -> Self {
        let mut vars = Vec::with_capacity(n);
        for i in 0..n {
            vars.push(Variable::new(format!("x_{}", i), i));
        }
        Ring {
            vars,
            order: PhantomData,
        }
    }
    pub fn set_variable_name<S: ToString>(&mut self, name: S, order: usize) {
        self.vars[order].set_name(name);
    }

    pub fn set_variable_type(&mut self, associated_type: AssociatedVariableType, order: usize) {
        self.vars[order].set_associated_type(associated_type);
    }

    pub fn var(&self, n: usize) -> &Variable {
        &self.vars[n]
    }

    pub fn gens(&self) -> usize {
        self.vars.len()
    }
}
