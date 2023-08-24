use std::{fs, path::PathBuf};

use swc_plugin_de_indent::DeIndent;

#[testing::fixture("tests/fixture/raw/**/input.txt")]
fn fixture_raw(input: PathBuf) {
    let input_str = fs::read_to_string(&input).expect("Error in reading the input file");
    let output_str = fs::read_to_string(&input.parent().unwrap().join("output.txt"))
        .expect("Error in reading the output file");
    assert_eq!(input_str.de_indent(), output_str);
}
