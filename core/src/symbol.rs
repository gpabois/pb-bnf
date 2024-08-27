use std::ops::Deref;

use yalp_shared::{
    prelude::IntoSymbol,
    symbol::{Symbol, SymbolFragment},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BnfSymbol<'syntax>(Symbol<'syntax>);

impl<'syntax> IntoSymbol<'syntax> for BnfSymbol<'syntax> {
    fn into_symbol(self) -> Symbol<'syntax> {
        self.0
    }
}

impl From<String> for BnfSymbol<'_> {
    fn from(value: String) -> Self {
        Self(value.into())
    }
}

impl<'syntax> From<&'syntax str> for BnfSymbol<'syntax> {
    fn from(value: &'syntax str) -> Self {
        Self(value.into())
    }
}

impl std::fmt::Display for BnfSymbol<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}>", self.0.deref())
    }
}

impl BnfSymbol<'_> {
    pub fn is_parsable(input: &syn::parse::ParseStream) -> bool {
        use syn::Token;
        input.peek(Token![<])
    }
}

impl syn::parse::Parse for BnfSymbol<'_> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        use syn::Token;

        let mut parts = Vec::<String>::default();
        input.parse::<Token![<]>()?;

        while !input.peek(Token![>]) {
            let part = SymbolFragment::parse(input)?.into_string();
            parts.push(part);
        }

        input.parse::<Token![>]>()?;

        let whole = parts
            .into_iter()
            .reduce(|acc, part| {
                if acc.ends_with('-') {
                    format!("{}-{}", acc, part)
                } else {
                    format!("{} {}", acc, part)
                }
            })
            .map(Self::from)
            .unwrap();

        Ok(whole)
    }
}
