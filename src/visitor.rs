use swc_core::{
    common::comments::Comments,
    ecma::{
        ast::Tpl,
        visit::{as_folder, Folder, VisitMut, VisitMutWith},
    },
};
use tracing::debug;

pub struct DeIndentVisitor<C>
where
    C: Comments,
{
    comments: C,
}

impl<C> DeIndentVisitor<C>
where
    C: Comments,
{
    pub fn new(comments: C) -> Self {
        Self { comments }
    }
    pub fn as_folder(comments: C) -> Folder<Self> {
        as_folder(Self::new(comments))
    }
}

impl<C> VisitMut for DeIndentVisitor<C>
where
    C: Comments,
{
    fn visit_mut_tpl(&mut self, tpl: &mut Tpl) {
        tpl.visit_mut_children_with(self);
        if self.comments.has_leading(tpl.span.lo()) {
            debug!(
                "leading comments: {:#?}",
                self.comments.get_leading(tpl.span.lo())
            );
        } else {
            debug!("no leading comments");
        }
    }
}
