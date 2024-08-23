use std::ops::Deref;

use syn::{LitChar, LitStr};

use crate::{
    prelude::{self, ISymbol},
    symbol::{Symbol, SymbolRef},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Literal(Symbol);

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.0.deref())
    }
}

impl Literal {
    pub fn is_parsable(input: &syn::parse::ParseStream) -> bool {
        input.peek(LitChar) || input.peek(LitStr)
    }
}

impl syn::parse::Parse for Literal {
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

impl From<Symbol> for Literal {
    fn from(value: Symbol) -> Self {
        Self(value)
    }
}

impl From<&str> for Literal {
    fn from(value: &str) -> Self {
        Self(Symbol::from(value))
    }
}

impl From<String> for Literal {
    fn from(value: String) -> Self {
        Self(Symbol::from(value))
    }
}

impl Deref for Literal {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<Symbol> for Literal {
    fn as_ref(&self) -> &Symbol {
        &self.0
    }
}

impl prelude::ILiteral for Literal {
    type Symbol = Symbol;

    fn borrow(&self) -> self::LiteralRef<'_> {
        LiteralRef(ISymbol::borrow(&self.0))
    }

    fn to_owned(&self) -> self::Literal {
        self.clone()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LiteralRef<'a>(SymbolRef<'a>);

impl<'a> LiteralRef<'a> {
    pub const fn new(value: &'a str) -> Self {
        Self(SymbolRef::new(value))
    }
}

impl<'a> From<&'a str> for LiteralRef<'a> {
    fn from(value: &'a str) -> Self {
        Self(SymbolRef::from(value))
    }
}

impl Deref for LiteralRef<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl<'a> AsRef<SymbolRef<'a>> for LiteralRef<'a> {
    fn as_ref(&self) -> &SymbolRef<'a> {
        &self.0
    }
}

impl std::fmt::Display for LiteralRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.0.deref())
    }
}

impl<'a> prelude::ILiteral for LiteralRef<'a> {
    type Symbol = SymbolRef<'a>;

    fn borrow(&self) -> self::LiteralRef<'_> {
        *self
    }

    fn to_owned(&self) -> self::Literal {
        Literal(ISymbol::to_owned(&self.0))
    }
}
