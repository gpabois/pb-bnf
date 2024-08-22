use crate::{
    literal::{Literal, LiteralRef},
    prelude::{self, ILiteral, ISymbol},
    symbol::{Symbol, SymbolRef},
};

pub enum TermKind {
    Symbol,
    Literal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    Symbol(Symbol),
    Literal(Literal),
}

impl Term {
    pub fn is_parsable(input: &syn::parse::ParseStream) -> bool {
        Symbol::is_parsable(input) || Literal::is_parsable(input)
    }
}

impl syn::parse::Parse for Term {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if Symbol::is_parsable(&input) {
            Symbol::parse(input).map(Self::Symbol)
        } else {
            Literal::parse(input).map(Self::Literal)
        }
    }
}

impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Term::Symbol(sym) => sym.fmt(f),
            Term::Literal(lit) => lit.fmt(f),
        }
    }
}

impl From<Symbol> for Term {
    fn from(value: Symbol) -> Self {
        Self::Symbol(value)
    }
}

impl From<Literal> for Term {
    fn from(value: Literal) -> Self {
        Self::Literal(value)
    }
}

impl prelude::ITerm for Term {
    type Symbol = Symbol;
    type Literal = Literal;

    fn kind(&self) -> TermKind {
        match self {
            Term::Symbol(_) => TermKind::Symbol,
            Term::Literal(_) => TermKind::Literal,
        }
    }

    fn borrow(&self) -> self::TermRef<'_> {
        match self {
            Term::Symbol(sym) => ISymbol::borrow(sym).into(),
            Term::Literal(lit) => ILiteral::borrow(lit).into(),
        }
    }

    fn to_owned(&self) -> self::Term {
        self.clone()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TermRef<'a> {
    Symbol(SymbolRef<'a>),
    Literal(LiteralRef<'a>),
}

impl<'a> From<SymbolRef<'a>> for TermRef<'a> {
    fn from(value: SymbolRef<'a>) -> Self {
        Self::Symbol(value)
    }
}

impl<'a> From<LiteralRef<'a>> for TermRef<'a> {
    fn from(value: LiteralRef<'a>) -> Self {
        Self::Literal(value)
    }
}

impl<'a> prelude::ITerm for TermRef<'a> {
    type Symbol = SymbolRef<'a>;
    type Literal = LiteralRef<'a>;

    fn kind(&self) -> TermKind {
        match self {
            TermRef::Symbol(_) => TermKind::Symbol,
            TermRef::Literal(_) => TermKind::Literal,
        }
    }

    fn borrow(&self) -> self::TermRef<'_> {
        *self
    }

    fn to_owned(&self) -> self::Term {
        match self {
            TermRef::Symbol(sym) => ISymbol::to_owned(sym).into(),
            TermRef::Literal(lit) => ILiteral::to_owned(lit).into(),
        }
    }
}

impl std::fmt::Display for TermRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TermRef::Symbol(sym) => sym.fmt(f),
            TermRef::Literal(lit) => lit.fmt(f),
        }
    }
}
