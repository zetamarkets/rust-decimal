use std::{fs, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=README.md");
    let readme = fs::read_to_string("README.md").unwrap();
    let output = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("README-lib.md");
    fs::write(output, prepare(&readme)).unwrap();
}

fn prepare(readme: &str) -> String {
    // This is a naive implementation to get things off the ground.
    // We just do a few things for this at the moment:
    // 1. Strip header stuff
    // 2. Replace the build document link
    // 3. Replace serde examples with ignore flags (to avoid feature flagging configuration in docs)
    let mut cleaned = String::new();
    let mut body = false;
    let mut feature_section = false;
    let mut feature = String::new();
    for line in readme.lines() {
        if !body {
            if line.starts_with("[docs]") {
                body = true;
            }
            continue;
        }

        // Add the line as is, unless it contains "(BUILD.md)"
        if line.contains("(BUILD.md)") {
            cleaned.push_str(&line.replace(
                "(BUILD.md)",
                "(https://github.com/paupino/rust-decimal/blob/master/BUILD.md)",
            ));
        } else if feature_section && line.starts_with("```rust") {
            // This is a bit naive, but it's to make the Serde examples cleaner. Should probably
            // be a bit more "defensive" here.
            cleaned.push_str("```rust\n");
            cleaned.push_str("# use rust_decimal::Decimal;\n");
            cleaned.push_str("# use serde::{Serialize, Deserialize};\n");
            cleaned.push_str(&format!("# #[cfg(features = \"{}\")]", feature));
        } else {
            if !feature_section && line.starts_with("## Features") {
                feature_section = true;
            } else if feature_section && line.starts_with("### ") {
                feature = line.replace("### ", "").replace('`', "");
            }
            cleaned.push_str(line);
        }
        cleaned.push('\n');
    }
    cleaned
}
