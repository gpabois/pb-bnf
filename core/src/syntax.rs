use crate::definition_set::BnfDefinitionSet;
use yalp_shared::{
    prelude::*,
    syntax::{RuleKernel, Syntax, SyntaxKernel},
};

pub type BnfSyntax<'syntax> = Syntax<'syntax, BnfDefinitionSet<'syntax>>;

#[derive(Default)]
pub struct KernelizeBnfSyntax<'syntax> {
    rules: Vec<RuleKernel<'syntax>>,
}

impl<'syntax> TransformSyntax<'syntax, KernelizeBnfSyntax<'syntax>> for BnfSyntax<'syntax> {
    type Transformed = SyntaxKernel<'syntax>;

    fn transform_syntax(self, ctx: &mut KernelizeBnfSyntax<'syntax>) -> Self::Transformed {
        self.into_iter().for_each(|rule| {
            let lhs = rule.lhs;
            ctx.rules.extend(rule.rhs.into_iter().map(|def| {
                RuleKernel {
                    lhs: lhs.clone(),
                    rhs: def
                        .into_iter()
                        .map(|term| term.into_symbol_identifier())
                        .collect(),
                }
            }));
        });

        std::mem::take(&mut ctx.rules).into_iter().collect()
    }
}
