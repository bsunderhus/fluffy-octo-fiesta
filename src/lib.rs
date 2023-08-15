mod utils;
mod visitor;

use swc_core::{
    ecma::{ast::Program, visit::VisitMutWith},
    plugin::{
        plugin_transform,
        proxies::{PluginCommentsProxy, TransformPluginProgramMetadata},
    },
};

pub use utils::de_indent;
pub use visitor::DeIndentVisitor;

#[plugin_transform]
fn process_transform(mut program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    program.visit_mut_with(&mut DeIndentVisitor::new(PluginCommentsProxy));
    program
}
