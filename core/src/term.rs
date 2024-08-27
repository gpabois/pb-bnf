use yalp_shared::{prelude::IntoSymbol, symbol::Symbol};

use crate::{literal::BnfLiteral, symbol::BnfSymbol};

pub enum BnfTermKind {
    Symbol,
    Literal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BnfTerm<'syntax> {
    Symbol(BnfSymbol<'syntax>),
    Literal(BnfLiteral<'syntax>),
}

impl<'syntax> IntoSymbol<'syntax> for BnfTerm<'syntax> {
    fn into_symbol(self) -> Symbol<'syntax> {
        match self {
            BnfTerm::Symbol(sym) => sym.into_symbol(),
            BnfTerm::Literal(lit) => lit.into_symbol(),
        }
    }
}

impl BnfTerm<'_> {
    pub fn is_parsable(input: &syn::parse::ParseStream) -> bool {
        Symbol::is_parsable(input) || BnfLiteral::is_parsable(input)
    }
}

impl syn::parse::Parse for BnfTerm<'_> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if BnfSymbol::is_parsable(&input) {
            BnfSymbol::parse(input).map(Self::Symbol)
        } else {
            BnfLiteral::parse(input).map(Self::Literal)
        }
    }
}

impl std::fmt::Display for BnfTerm<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BnfTerm::Symbol(sym) => sym.fmt(f),
            BnfTerm::Literal(lit) => lit.fmt(f),
        }
    }
}

impl<'syntax> From<BnfSymbol<'syntax>> for BnfTerm<'syntax> {
    fn from(value: BnfSymbol<'syntax>) -> Self {
        Self::Symbol(value)
    }
}

impl<'syntax> From<BnfLiteral<'syntax>> for BnfTerm<'syntax> {
    fn from(value: BnfLiteral<'syntax>) -> Self {
        Self::Literal(value)
    }
}
