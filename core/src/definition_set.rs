use crate::term::BnfTerm;
use yalp_shared::syntax::DefinitionSet;

pub type BnfDefinitionSet<'syntax> = DefinitionSet<'syntax, BnfTerm<'syntax>>;
