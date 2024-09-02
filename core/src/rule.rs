use crate::{definition_set::BnfDefinitionSet, symbol::BnfSymbolId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rule<'syntax> {
    pub lhs: BnfSymbolId<'syntax>,
    pub rhs: BnfDefinitionSet<'syntax>,
}

impl<'syntax> Rule<'syntax> {
    pub fn new(lhs: BnfSymbolId<'syntax>, rhs: BnfDefinitionSet<'syntax>) -> Self {
        Self { lhs, rhs }
    }
}
