use std::cmp::Ordering;
use std::fmt::{self, Display};
#[derive(Debug, Clone, Copy)]
pub enum AssociatedVariableType {
    NoType,
    L(usize, usize, usize),
    K(usize),
    X(usize, usize),
    Y(usize, usize),
}

#[derive(Debug)]
pub struct Variable {
    name: String,
    order: usize,
    associated_type: AssociatedVariableType,
}

impl Variable {
    pub fn new<T: ToString>(name: T, order: usize) -> Self {
        Variable {
            name: name.to_string(),
            order,
            associated_type: AssociatedVariableType::NoType,
        }
    }
    pub fn set_name<T: ToString>(&mut self, name: T) {
        self.name = name.to_string();
    }
    pub fn set_associated_type(&mut self, associated_type: AssociatedVariableType) {
        self.associated_type = associated_type;
    }
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
    pub fn associated_type(&self) -> AssociatedVariableType {
        self.associated_type
    }
    pub fn order(&self) -> u32 {
        self.order as u32
    }
}
impl PartialOrd for Variable {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.order.partial_cmp(&other.order)
    }
}
impl Ord for Variable {
    fn cmp(&self, other: &Self) -> Ordering {
        self.order.cmp(&other.order)
    }
}

impl PartialEq for Variable {
    fn eq(&self, other: &Self) -> bool {
        self.order == other.order
    }
}

impl Eq for Variable {}

impl Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
