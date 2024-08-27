use std::{borrow::Cow, fmt::Display};

use itertools::Itertools;

use crate::rule::Rule;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct BnfSyntax<'syntax>(Cow<'syntax, [Rule<'syntax>]>);

impl Display for BnfSyntax<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_ref()
            .iter()
            .map(ToString::to_string)
            .enumerate()
            .map(|(rule_id, rule_str)| format!("{rule_id}. {rule_str}"))
            .join("\n")
            .fmt(f)
    }
}

impl<'syntax> FromIterator<Rule<'syntax>> for BnfSyntax<'syntax> {
    fn from_iter<T: IntoIterator<Item = Rule<'syntax>>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<'syntax> AsRef<[Rule<'syntax>]> for BnfSyntax<'syntax> {
    fn as_ref(&self) -> &[Rule<'syntax>] {
        &self.0
    }
}
