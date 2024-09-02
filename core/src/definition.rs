use crate::term::BnfTerm;
use yalp_shared::syntax::Definition;

pub type BnfDefinition<'syntax> = Definition<'syntax, BnfTerm<'syntax>>;
