use yalp_shared::{prelude::IntoSymbolIdentifier, symbol::SymbolId};

use crate::{literal::BnfLiteral, symbol::BnfSymbolId};

pub enum BnfTermKind {
    Symbol,
    Literal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BnfTerm<'syntax> {
    Symbol(BnfSymbolId<'syntax>),
    Literal(BnfLiteral<'syntax>),
}

impl<'syntax> IntoSymbolIdentifier<'syntax> for BnfTerm<'syntax> {
    fn into_symbol_identifier(self) -> SymbolId<'syntax> {
        match self {
            BnfTerm::Symbol(sym) => sym.into_symbol_identifier(),
            BnfTerm::Literal(lit) => lit.into_symbol_identifier(),
        }
    }
}

impl BnfTerm<'_> {
    pub fn is_parsable(input: &syn::parse::ParseStream) -> bool {
        BnfSymbolId::is_parsable(input) || BnfLiteral::is_parsable(input)
    }
}

impl syn::parse::Parse for BnfTerm<'_> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if BnfSymbolId::is_parsable(&input) {
            BnfSymbolId::parse(input).map(Self::Symbol)
        } else {
            BnfLiteral::parse(input).map(Self::Literal)
        }
    }
}

impl<'syntax> From<BnfSymbolId<'syntax>> for BnfTerm<'syntax> {
    fn from(value: BnfSymbolId<'syntax>) -> Self {
        Self::Symbol(value)
    }
}

impl<'syntax> From<BnfLiteral<'syntax>> for BnfTerm<'syntax> {
    fn from(value: BnfLiteral<'syntax>) -> Self {
        Self::Literal(value)
    }
}
