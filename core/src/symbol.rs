use std::ops::Deref;

use syn::{parse::Parse, LitBool, LitInt};

use crate::prelude;

pub struct SymbolFragment(String);

impl SymbolFragment {
    pub fn into_string(self) -> String {
        self.0
    }
    pub fn is_parsable(input: &syn::parse::ParseStream) -> bool {
        use syn::Token;

        input.peek(Token![as])
            || input.peek(Token![break])
            || input.peek(Token![const])
            || input.peek(Token![continue])
            || input.peek(Token![crate])
            || input.peek(Token![else])
            || input.peek(Token![enum])
            || input.peek(Token![extern])
            || input.peek(LitBool)
            || input.peek(Token![fn])
            || input.peek(Token![for])
            || input.peek(Token![if])
            || input.peek(Token![impl])
            || input.peek(Token![in])
            || input.peek(Token![let])
            || input.peek(Token![loop])
            || input.peek(Token![match])
            || input.peek(Token![mod])
            || input.peek(Token![move])
            || input.peek(Token![mut])
            || input.peek(Token![pub])
            || input.peek(Token![ref])
            || input.peek(Token![return])
            || input.peek(Token![self])
            || input.peek(Token![Self])
            || input.peek(Token![static])
            || input.peek(Token![struct])
            || input.peek(Token![super])
            || input.peek(Token![trait])
            || input.peek(Token![type])
            || input.peek(Token![unsafe])
            || input.peek(Token![use])
            || input.peek(Token![where])
            || input.peek(Token![while])
            || input.peek(Token![-])
            || input.peek(syn::Ident)
            || input.peek(syn::LitInt)
    }
}

impl Parse for SymbolFragment {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        use syn::Token;

        if input.peek(Token![as]) {
            input.parse::<Token![as]>()?;
            Ok(Self("as".to_owned()))
        } else if input.peek(Token![break]) {
            input.parse::<Token![break]>()?;
            Ok(Self("break".to_owned()))
        } else if input.peek(Token![const]) {
            input.parse::<Token![const]>()?;
            Ok(Self("const".to_owned()))
        } else if input.peek(Token![continue]) {
            input.parse::<Token![continue]>()?;
            Ok(Self("continue".to_owned()))
        } else if input.peek(Token![crate]) {
            input.parse::<Token![crate]>()?;
            Ok(Self("crate".to_owned()))
        } else if input.peek(Token![else]) {
            input.parse::<Token![else]>()?;
            Ok(Self("else".to_owned()))
        } else if input.peek(Token![enum]) {
            input.parse::<Token![enum]>()?;
            Ok(Self("enum".to_owned()))
        } else if input.peek(Token![extern]) {
            input.parse::<Token![extern]>()?;
            Ok(Self("extern".to_owned()))
        } else if input.peek(LitBool) {
            let lit = input.parse::<LitBool>()?;
            let val = lit
                .value()
                .then(|| String::from("true"))
                .unwrap_or(String::from("false"));
            Ok(Self(val))
        } else if input.peek(LitInt) {
            let lit = input.parse::<LitInt>()?;
            let val = lit.to_string();
            Ok(Self(val))
        } else if input.peek(Token![fn]) {
            input.parse::<Token![fn]>()?;
            Ok(Self("fn".to_owned()))
        } else if input.peek(Token![for]) {
            input.parse::<Token![for]>()?;
            Ok(Self("for".to_owned()))
        } else if input.peek(Token![if]) {
            input.parse::<Token![if]>()?;
            Ok(Self("if".to_owned()))
        } else if input.peek(Token![impl]) {
            input.parse::<Token![impl]>()?;
            Ok(Self("impl".to_owned()))
        } else if input.peek(Token![in]) {
            input.parse::<Token![in]>()?;
            Ok(Self("in".to_owned()))
        } else if input.peek(Token![let]) {
            input.parse::<Token![let]>()?;
            Ok(Self("let".to_owned()))
        } else if input.peek(Token![loop]) {
            input.parse::<Token![loop]>()?;
            Ok(Self("loop".to_owned()))
        } else if input.peek(Token![match]) {
            input.parse::<Token![match]>()?;
            Ok(Self("match".to_owned()))
        } else if input.peek(Token![mod]) {
            input.parse::<Token![mod]>()?;
            Ok(Self("mod".to_owned()))
        } else if input.peek(Token![move]) {
            input.parse::<Token![move]>()?;
            Ok(Self("move".to_owned()))
        } else if input.peek(Token![mut]) {
            input.parse::<Token![mut]>()?;
            Ok(Self("mut".to_owned()))
        } else if input.peek(Token![pub]) {
            input.parse::<Token![pub]>()?;
            Ok(Self("pub".to_owned()))
        } else if input.peek(Token![ref]) {
            input.parse::<Token![ref]>()?;
            Ok(Self("ref".to_owned()))
        } else if input.peek(Token![return]) {
            input.parse::<Token![return]>()?;
            Ok(Self("return".to_owned()))
        } else if input.peek(Token![self]) {
            input.parse::<Token![self]>()?;
            Ok(Self("self".to_owned()))
        } else if input.peek(Token![Self]) {
            input.parse::<Token![Self]>()?;
            Ok(Self("Self".to_owned()))
        } else if input.peek(Token![static]) {
            input.parse::<Token![static]>()?;
            Ok(Self("static".to_owned()))
        } else if input.peek(Token![struct]) {
            input.parse::<Token![struct]>()?;
            Ok(Self("struct".to_owned()))
        } else if input.peek(Token![super]) {
            input.parse::<Token![super]>()?;
            Ok(Self("super".to_owned()))
        } else if input.peek(Token![trait]) {
            input.parse::<Token![trait]>()?;
            Ok(Self("trait".to_owned()))
        } else if input.peek(Token![type]) {
            input.parse::<Token![type]>()?;
            Ok(Self("type".to_owned()))
        } else if input.peek(Token![unsafe]) {
            input.parse::<Token![unsafe]>()?;
            Ok(Self("const".to_owned()))
        } else if input.peek(Token![use]) {
            input.parse::<Token![use]>()?;
            Ok(Self("use".to_owned()))
        } else if input.peek(Token![where]) {
            input.parse::<Token![where]>()?;
            Ok(Self("where".to_owned()))
        } else if input.peek(Token![while]) {
            input.parse::<Token![while]>()?;
            Ok(Self("while".to_owned()))
        } else if input.peek(Token![-]) {
            input.parse::<Token![-]>()?;
            Ok(Self("-".to_owned()))
        } else {
            let id = input.parse::<syn::Ident>()?;
            Ok(Self(id.to_string()))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Symbol(String);

impl Symbol {
    pub fn is_parsable(input: &syn::parse::ParseStream) -> bool {
        use syn::Token;
        input.peek(Token![<])
    }
}

impl Parse for Symbol {
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
            .map(Self)
            .unwrap();

        Ok(whole)
    }
}

impl From<&str> for Symbol {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for Symbol {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}>", self.0)
    }
}

impl Deref for Symbol {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl prelude::ISymbol for Symbol {
    fn borrow(&self) -> self::SymbolRef<'_> {
        SymbolRef::from(self.0.as_str())
    }

    fn to_owned(&self) -> self::Symbol {
        self.clone()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SymbolRef<'a>(&'a str);

impl<'a> SymbolRef<'a> {
    pub const fn new(value: &'a str) -> Self {
        Self(value)
    }
}

impl Deref for SymbolRef<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl prelude::ISymbol for SymbolRef<'_> {
    fn borrow(&self) -> self::SymbolRef<'_> {
        *self
    }

    fn to_owned(&self) -> self::Symbol {
        Symbol::from(self.0)
    }
}

impl<'a> From<&'a str> for SymbolRef<'a> {
    fn from(value: &'a str) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for SymbolRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}>", self.0)
    }
}
