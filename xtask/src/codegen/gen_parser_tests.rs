//! This module greps parser's code for specially formatted comments and turnes
//! them into tests.

use crate::{codegen, not_bash::fs2, project_root_dir};
use anyhow::Result;
use std::{iter, collections::HashMap};
use itertools::Itertools;

pub fn gen_parser_tests(mode: codegen::Mode) -> Result<()> {
    let tests_source = fs2::read_to_string(project_root_dir().join(codegen::GRAMMAR_DIR_PATH))?;
    let tests = collect_tests(&tests_source);
    let out_dir = project_root_dir().join(codegen::INLINE_TESTS_OUT_DIR_PATH);

    for (name, file_contents) in &tests {
        let path = out_dir.join(format!("test-{}.toml", name));
        codegen::verify_or_overwrite(mode, &path, file_contents)?;
    }
    // Check for strays.
    for file in fs2::read_dir(&out_dir)? {
        let path = file?.path();
        let stem = path.file_stem().unwrap();
        let name = stem.to_str().unwrap();
        if name.starts_with("test-") {
            let name = &name["test-".len()..];
            if !tests.contains_key(name) {
                panic!(
                    "File `{}` exists, but no inline test found.",
                    path.display()
                );
            }
        }
    }
    Ok(())
}

fn collect_tests(rust_source_code: &str) -> HashMap<String, String> {
    let mut res = HashMap::new();
    let comment_blocks = rust_source_code
        .lines()
        .map(str::trim_start)
        .group_by(|line| line.starts_with("//"));

    let comment_blocks = comment_blocks
        .into_iter()
        .filter_map(|(is_comment, block)| Some(block).filter(|_| is_comment));

    for block in comment_blocks {
        let mut block = block.map(|line| {
            let prefix = if line.starts_with("// ") { "// " } else { "//" };
            &line[prefix.len()..]
        });

        let name = match block.next() {
            Some(line) if line.starts_with("test-") => line[5..].to_owned(),
            _ => continue,
        };
        let test_body = block.chain(iter::once("")).join("\n");
        assert!(!test_body.trim().is_empty() && test_body.ends_with('\n'));
        if let Some(name) = res.insert(name, test_body) {
            panic!("Test name `{}` already used.", name);
        }
    }
    res
}
