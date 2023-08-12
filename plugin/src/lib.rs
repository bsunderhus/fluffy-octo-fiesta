use tracing::debug;

use swc_core::{
    ecma::{
        ast::*,
        transforms::testing::test,
        visit::{as_folder, FoldWith, VisitMut, VisitMutWith},
    },
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};

pub struct TransformVisitor;

impl VisitMut for TransformVisitor {
    fn visit_mut_tagged_tpl(&mut self, node: &mut TaggedTpl) {
        debug!("TaggedTpl: {:?}", node);
        node.visit_mut_children_with(self);
    }
}

#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    program.fold_with(&mut as_folder(TransformVisitor))
}

test!(
    Default::default(),
    |_| as_folder(TransformVisitor),
    it_should_transform_bin_expr,
    // Input
    r#"deIndent`
        Hello world!,
        this is me.
    `"#,
    // Output
    r#"deIndent`
        Hello world!,
        this is me.
    `"#
);
