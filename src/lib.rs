mod utils;

use swc_core::{
    common::comments::Comments,
    ecma::{
        ast::{Expr, Program, Tpl},
        visit::{Fold, FoldWith},
    },
    plugin::{
        plugin_transform,
        proxies::{PluginCommentsProxy, TransformPluginProgramMetadata},
    },
};
use tracing::debug;

pub struct DeIndentTplFolder<C>
where
    C: Comments,
{
    comments: C,
}

impl<C> DeIndentTplFolder<C>
where
    C: Comments,
{
    pub fn new(comments: C) -> Self {
        Self { comments }
    }
}

impl<C> Fold for DeIndentTplFolder<C>
where
    C: Comments,
{
    fn fold_tpl(&mut self, node: Tpl) -> Tpl {
        let tpl = node.fold_children_with(self);
        if self.comments.has_leading(tpl.span.lo()) {
            debug!(
                "leading comments: {:#?}",
                self.comments.get_leading(tpl.span.lo())
            );
        } else {
            debug!("no leading comments");
        }
        tpl
    }
    fn fold_expr(&mut self, node: Expr) -> Expr {
        let expr = node.fold_children_with(self);
        if let Expr::TaggedTpl(mut tagged_tpl) = expr {
            if let Some(ident) = tagged_tpl.tag.as_ident() {
                if ident.sym == String::from("deIndent") {
                    tagged_tpl.tpl.quasis.iter_mut().for_each(|tpl_element| {
                        tpl_element.raw = utils::de_indent(&tpl_element.raw).into();
                    });
                    return Expr::Tpl(*tagged_tpl.tpl);
                }
            }
            return Expr::TaggedTpl(tagged_tpl);
        }
        expr
    }
}

#[plugin_transform]
fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    program.fold_with(&mut DeIndentTplFolder::new(PluginCommentsProxy))
}
