use yalp_shared::{
    prelude::IntoSymbolIdentifier,
    symbol::{SymbolFragment, SymbolId},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BnfSymbolId<'syntax>(SymbolId<'syntax>);

impl<'syntax> IntoSymbolIdentifier<'syntax> for BnfSymbolId<'syntax> {
    fn into_symbol_identifier(self) -> SymbolId<'syntax> {
        self.0
    }
}

impl From<String> for BnfSymbolId<'_> {
    fn from(value: String) -> Self {
        Self(value.into())
    }
}

impl<'syntax> From<&'syntax str> for BnfSymbolId<'syntax> {
    fn from(value: &'syntax str) -> Self {
        Self(value.into())
    }
}

impl BnfSymbolId<'_> {
    pub fn is_parsable(input: &syn::parse::ParseStream) -> bool {
        use syn::Token;
        input.peek(Token![<])
    }
}

impl syn::parse::Parse for BnfSymbolId<'_> {
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
