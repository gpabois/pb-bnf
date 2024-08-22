use std::fmt::Display;

use crate::{
    definition_set::{DefinitionSet, DefinitionSetRef},
    prelude::{self, IRule},
    symbol::{Symbol, SymbolRef},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rule {
    lhs: Symbol,
    rhs: DefinitionSet,
}

impl Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ::= {};", self.lhs, self.rhs)
    }
}

impl Rule {
    pub fn new(lhs: Symbol, rhs: DefinitionSet) -> Self {
        Self { lhs, rhs }
    }
}

impl prelude::IRule for Rule {
    type Lhs = Symbol;
    type Rhs = DefinitionSet;

    fn lhs(&self) -> &Self::Lhs {
        &self.lhs
    }

    fn rhs(&self) -> &Self::Rhs {
        &self.rhs
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuleRef<'a> {
    lhs: SymbolRef<'a>,
    rhs: DefinitionSetRef<'a>,
}

impl<'a> RuleRef<'a> {
    pub const fn new(lhs: SymbolRef<'a>, rhs: DefinitionSetRef<'a>) -> Self {
        Self { lhs, rhs }
    }
}

impl<'a> IRule for RuleRef<'a> {
    type Lhs = SymbolRef<'a>;
    type Rhs = DefinitionSetRef<'a>;

    fn lhs(&self) -> &Self::Lhs {
        &self.lhs
    }

    fn rhs(&self) -> &Self::Rhs {
        &self.rhs
    }
}

impl Display for RuleRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ::= {};", self.lhs, self.rhs)
    }
}
