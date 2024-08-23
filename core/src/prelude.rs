use std::ops::Deref;

use crate::{
    literal, symbol,
    term::{self, TermKind},
};

pub trait ISyntax: AsRef<[Self::Rule]> + Clone {
    type Rule: IRule;
}

pub trait IRule: Clone {
    type Lhs: ISymbol;
    type Rhs: IDefinitionSet;

    fn lhs(&self) -> &Self::Lhs;
    fn rhs(&self) -> &Self::Rhs;
}

pub trait IDefinitionSet: AsRef<[Self::Definition]> + Clone {
    type Definition: IDefinition;
}

pub trait IDefinition: AsRef<[Self::Term]> + Clone {
    type Term: ITerm;
}

pub trait ITerm: Clone {
    type Symbol: ISymbol;
    type Literal: ILiteral;

    fn kind(&self) -> TermKind;

    fn borrow(&self) -> term::TermRef<'_>;
    fn to_owned(&self) -> term::Term;
}

pub trait IterSymbols<'a> {
    type Symbol: ISymbol + 'a;
    type Iter: Iterator<Item = &'a Self::Symbol>;

    fn iter_symbols(&'a self) -> Self::Iter;
}

pub trait ISymbol: Deref<Target = str> + Clone {
    fn borrow(&self) -> symbol::SymbolRef<'_>;
    fn to_owned(&self) -> symbol::Symbol;
}

pub trait ILiteral: Deref<Target = str> + AsRef<Self::Symbol> + Clone {
    type Symbol: ISymbol;

    fn borrow(&self) -> literal::LiteralRef<'_>;
    fn to_owned(&self) -> literal::Literal;
}
