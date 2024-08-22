use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

use itertools::Itertools;

use crate::{
    prelude,
    term::{Term, TermRef},
};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Definition(Vec<Term>);

impl syn::parse::Parse for Definition {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut def = Self::default();
        while Term::is_parsable(&input) {
            def.push(input.parse()?);
        }
        Ok(def)
    }
}

impl Display for Definition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.iter().map(ToString::to_string).join(" ").fmt(f)
    }
}

impl FromIterator<Term> for Definition {
    fn from_iter<T: IntoIterator<Item = Term>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl prelude::IDefinition for Definition {
    type Term = Term;
}

impl AsRef<[Term]> for Definition {
    fn as_ref(&self) -> &[Term] {
        self.0.deref()
    }
}

impl Deref for Definition {
    type Target = Vec<Term>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Definition {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DefinitionRef<'a>(&'a [TermRef<'a>]);

impl Display for DefinitionRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_ref()
            .iter()
            .map(|term| term.to_string())
            .join(" ")
            .fmt(f)
    }
}

impl<'a> AsRef<[TermRef<'a>]> for DefinitionRef<'a> {
    fn as_ref(&self) -> &[TermRef<'a>] {
        self.0
    }
}

impl<'a> prelude::IDefinition for DefinitionRef<'a> {
    type Term = TermRef<'a>;
}
