use std::{borrow::Cow, fmt::Display};

use itertools::Itertools;

use crate::definition::BnfDefinition;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct BnfDefinitionSet<'syntax>(Cow<'syntax, [BnfDefinition<'syntax>]>);

impl<'syntax> BnfDefinitionSet<'syntax> {
    pub fn push(&mut self, definition: BnfDefinition<'syntax>) {
        self.0.to_mut().push(definition)
    }
}

impl syn::parse::Parse for BnfDefinitionSet<'_> {
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

impl Display for BnfDefinitionSet<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_ref()
            .iter()
            .map(ToString::to_string)
            .join(" | ")
            .fmt(f)
    }
}

impl<'syntax> FromIterator<BnfDefinition<'syntax>> for BnfDefinitionSet<'syntax> {
    fn from_iter<T: IntoIterator<Item = BnfDefinition<'syntax>>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<'syntax> AsRef<[BnfDefinition<'syntax>]> for BnfDefinitionSet<'syntax> {
    fn as_ref(&self) -> &[BnfDefinition<'syntax>] {
        self.0.as_ref()
    }
}
