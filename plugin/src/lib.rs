// use tracing::debug;
use swc_core::{
    ecma::{
        ast::{Expr, Program, TaggedTpl, Tpl},
        transforms::testing::test,
        visit::{Fold, FoldWith},
    },
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};

pub struct FoldDeIndentTaggedTemplate;

impl Fold for FoldDeIndentTaggedTemplate {
    fn fold_expr(&mut self, node: Expr) -> Expr {
        let expr = node.fold_children_with(self);
        if let Expr::TaggedTpl(mut tagged_tpl) = expr {
            if let Some(ident) = tagged_tpl.tag.as_ident() {
                if ident.sym == String::from("deIndent") {
                    tagged_tpl.tpl.quasis.iter_mut().for_each(|tpl_element| {
                        tpl_element.raw = de_indent(&tpl_element.raw).into();
                    });
                    return Expr::Tpl(*tagged_tpl.tpl);
                }
            }
            return Expr::TaggedTpl(tagged_tpl);
        }
        expr
    }
}

fn de_indent(input: &str) -> String {
    let mut result = String::new();
    for line in input.lines() {
        let trimmed_line = line.trim_start();
        result.push_str(trimmed_line);
        if !trimmed_line.is_empty() {
            result.push('\n');
        }
    }
    if result.ends_with('\n') {
        result.pop();
    }
    result
}

#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    program.fold_with(&mut FoldDeIndentTaggedTemplate)
}

test!(
    Default::default(),
    |_| FoldDeIndentTaggedTemplate,
    it_should_transform_tagged_template,
    // Input
    r#"deIndent`
        Hello world!,
        this is ${name}.
        how are you?
    `"#,
    // Output
    r#"`Hello world!,
this is ${name}.
how are you?`"#
);
