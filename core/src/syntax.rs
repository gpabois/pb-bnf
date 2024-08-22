use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

use itertools::Itertools;

use crate::{
    prelude,
    rule::{Rule, RuleRef},
};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Syntax(Vec<Rule>);

impl Display for Syntax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.iter()
            .map(ToString::to_string)
            .enumerate()
            .map(|(rule_id, rule_str)| format!("{rule_id}. {rule_str}"))
            .join("\n")
            .fmt(f)
    }
}

impl FromIterator<Rule> for Syntax {
    fn from_iter<T: IntoIterator<Item = Rule>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl prelude::ISyntax for Syntax {
    type Rule = Rule;
}

impl AsRef<[Rule]> for Syntax {
    fn as_ref(&self) -> &[Rule] {
        &self.0
    }
}

impl Deref for Syntax {
    type Target = Vec<Rule>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Syntax {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct SyntaxRef<'a>(&'a [RuleRef<'a>]);

impl<'a> SyntaxRef<'a> {
    pub const fn new(rules: &'a [RuleRef<'a>]) -> Self {
        Self(rules)
    }
}

impl<'a> AsRef<[RuleRef<'a>]> for SyntaxRef<'a> {
    fn as_ref(&self) -> &[RuleRef<'a>] {
        self.0
    }
}

impl<'a> prelude::ISyntax for SyntaxRef<'a> {
    type Rule = RuleRef<'a>;
}

impl Display for SyntaxRef<'_> {
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
