use std::path::PathBuf;

use swc_core::ecma::transforms::testing::test_fixture;
use swc_ecma_parser::{Syntax, TsConfig};
use swc_plugin_de_indent::DeIndentVisitor;

#[testing::fixture("tests/fixture/ts/**/input.ts")]
fn fixture_swc(input: PathBuf) {
    let output = input.parent().unwrap().join("output.ts");

    test_fixture(
        Syntax::Typescript(TsConfig {
            tsx: true,
            decorators: false,
            dts: false,
            no_early_errors: true,
            disallow_ambiguous_jsx_like: true,
        }),
        &|tester| {
            DeIndentVisitor::as_folder(
                tester.comments.clone(),
                serde_json::from_str("{}").unwrap_or_default(),
            )
        },
        &input,
        &output,
        Default::default(),
    );
}
