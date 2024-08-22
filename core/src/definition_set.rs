use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

use itertools::Itertools;

use crate::{
    definition::{Definition, DefinitionRef},
    prelude::IDefinitionSet,
};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct DefinitionSet(Vec<Definition>);

impl syn::parse::Parse for DefinitionSet {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        use syn::Token;

        let mut set = Self::default();
        let mut state: u8 = 0; // == 0 wait for Definition, == 1 wait for |

        loop {
            if state == 0 {
                set.push(input.parse()?);
                state = 1;
            } else if state == 0 && input.peek(Token![|]) {
                input.parse::<Token![|]>()?;
                state = 0;
            } else {
                break;
            }
        }

        Ok(set)
    }
}

impl Display for DefinitionSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.iter().map(ToString::to_string).join(" | ").fmt(f)
    }
}

impl FromIterator<Definition> for DefinitionSet {
    fn from_iter<T: IntoIterator<Item = Definition>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl AsRef<[Definition]> for DefinitionSet {
    fn as_ref(&self) -> &[Definition] {
        self.0.as_slice()
    }
}

impl Deref for DefinitionSet {
    type Target = Vec<Definition>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DefinitionSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IDefinitionSet for DefinitionSet {
    type Definition = Definition;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DefinitionSetRef<'a>(&'a [DefinitionRef<'a>]);

impl<'a> AsRef<[DefinitionRef<'a>]> for DefinitionSetRef<'a> {
    fn as_ref(&self) -> &[DefinitionRef<'a>] {
        self.0
    }
}

impl<'a> DefinitionSetRef<'a> {
    pub const fn new(defs: &'a [DefinitionRef<'a>]) -> Self {
        Self(defs)
    }
}

impl<'a> IDefinitionSet for DefinitionSetRef<'a> {
    type Definition = DefinitionRef<'a>;
}

impl Display for DefinitionSetRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_ref()
            .iter()
            .map(ToString::to_string)
            .join(" | ")
            .fmt(f)
    }
}
