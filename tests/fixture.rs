use std::path::PathBuf;

use swc_core::ecma::transforms::testing::test_fixture;
use swc_plugin_de_indent::DeIndentTplFolder;

#[testing::fixture("tests/fixture/**/input.js")]
fn fixture(input: PathBuf) {
    let output = input.parent().unwrap().join("output.js");

    test_fixture(
        Default::default(),
        &|tester| DeIndentTplFolder::new(tester.comments.clone()),
        &input,
        &output,
        Default::default(),
    );
}
