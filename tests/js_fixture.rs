use std::path::PathBuf;

use swc_core::ecma::transforms::testing::test_fixture;
use swc_plugin_de_indent::DeIndentVisitor;

#[testing::fixture("tests/fixture/js/**/input.js")]
fn fixture_swc(input: PathBuf) {
    let output = input.parent().unwrap().join("output.js");

    test_fixture(
        Default::default(),
        &|tester| DeIndentVisitor::as_folder(tester.comments.clone()),
        &input,
        &output,
        Default::default(),
    );
}
