use std::{fs, path::PathBuf};

use swc_plugin_de_indent::de_indent;

#[testing::fixture("tests/fixture/raw/**/input.txt")]
fn fixture_raw(input: PathBuf) {
    let input_str = fs::read_to_string(&input).expect("Error in reading the input file");
    let output_str = fs::read_to_string(&input.parent().unwrap().join("output.txt"))
        .expect("Error in reading the output file");
    assert_eq!(de_indent(&input_str), output_str);
}
