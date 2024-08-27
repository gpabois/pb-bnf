use std::fmt::Display;

use crate::{definition_set::BnfDefinitionSet, symbol::BnfSymbol};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rule<'syntax> {
    pub lhs: BnfSymbol<'syntax>,
    pub rhs: BnfDefinitionSet<'syntax>,
}

impl Display for Rule<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ::= {};", self.lhs, self.rhs)
    }
}

impl<'syntax> Rule<'syntax> {
    pub fn new(lhs: BnfSymbol<'syntax>, rhs: BnfDefinitionSet<'syntax>) -> Self {
        Self { lhs, rhs }
    }
}
