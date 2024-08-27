use std::{borrow::Cow, fmt::Display};

use itertools::Itertools;

use crate::term::BnfTerm;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct BnfDefinition<'syntax>(Cow<'syntax, [BnfTerm<'syntax>]>);

impl<'syntax> BnfDefinition<'syntax> {
    pub fn push(&mut self, term: BnfTerm<'syntax>) {
        self.0.to_mut().push(term);
    }
}

impl syn::parse::Parse for BnfDefinition<'_> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut def = Self::default();
        while BnfTerm::is_parsable(&input) {
            def.push(input.parse()?);
        }
        Ok(def)
    }
}

impl Display for BnfDefinition<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_ref()
            .iter()
            .map(ToString::to_string)
            .join(" ")
            .fmt(f)
    }
}

impl<'syntax> FromIterator<BnfTerm<'syntax>> for BnfDefinition<'syntax> {
    fn from_iter<T: IntoIterator<Item = BnfTerm<'syntax>>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<'syntax> AsRef<[BnfTerm<'syntax>]> for BnfDefinition<'syntax> {
    fn as_ref(&self) -> &[BnfTerm<'syntax>] {
        self.0.as_ref()
    }
}
