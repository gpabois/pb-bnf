use std::ops::Deref;

use syn::{LitChar, LitStr};
use yalp_shared::{prelude::IntoSymbolIdentifier, symbol::SymbolId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BnfLiteral<'syntax>(SymbolId<'syntax>);

impl<'syntax> IntoSymbolIdentifier<'syntax> for BnfLiteral<'syntax> {
    fn into_symbol_identifier(self) -> SymbolId<'syntax> {
        self.0
    }
}

impl std::fmt::Display for BnfLiteral<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.0.deref())
    }
}

impl BnfLiteral<'_> {
    pub fn is_parsable(input: &syn::parse::ParseStream) -> bool {
        input.peek(LitChar) || input.peek(LitStr)
    }
}

impl syn::parse::Parse for BnfLiteral<'_> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(LitChar) {
            input
                .parse::<LitChar>()
                .map(|ch| ch.value().to_string())
                .map(Self::from)
        } else {
            input
                .parse::<LitStr>()
                .map(|s| s.value().to_string())
                .map(Self::from)
        }
    }
}

impl<'syntax> From<SymbolId<'syntax>> for BnfLiteral<'syntax> {
    fn from(value: SymbolId<'syntax>) -> Self {
        Self(value)
    }
}

impl<'syntax> From<&'syntax str> for BnfLiteral<'syntax> {
    fn from(value: &'syntax str) -> Self {
        Self(SymbolId::from(value))
    }
}

impl From<String> for BnfLiteral<'_> {
    fn from(value: String) -> Self {
        Self(SymbolId::from(value))
    }
}

impl<'syntax> Deref for BnfLiteral<'syntax> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'syntax> AsRef<SymbolId<'syntax>> for BnfLiteral<'syntax> {
    fn as_ref(&self) -> &SymbolId<'syntax> {
        &self.0
    }
}
