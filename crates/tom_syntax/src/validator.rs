//! FIXME: write short doc here

use crate::{SyntaxNodeRef, SyntaxError, TextRange, TomlDoc, ChunkedText, ast};

pub(crate) fn validate(doc: &TomlDoc) -> Vec<SyntaxError> {
    let mut errors = Vec::new();
    for node in doc.cst().descendants() {
        if let Some(entry) = ast::Entry::cast(node) {
            if let Some(first_key) = entry.keys().next() {
                check_new_line(
                    &mut errors,
                    first_key,
                    entry.value(),
                    Forbid,
                    "newlines are forbidden in entries",
                );
            }
        } else if let Some(d) = ast::Dict::cast(node) {
            check_new_line(
                &mut errors,
                d.syntax().children().next().unwrap(),
                d.syntax().children().last().unwrap(),
                Forbid,
                "newlines are forbidden in inline tables",
            );
        } else if let Some(table) = ast::Table::cast(node) {
            check_table(&mut errors, table)
        } else if let Some(table) = ast::ArrayTable::cast(node) {
            check_table(&mut errors, table)
        }
    }
    errors
}

fn check_table<'a>(
    errors: &mut Vec<SyntaxError>,
    table: impl ast::EntryOwner<'a> + ast::TableHeaderOwner<'a>,
) {
    let header = table.header();
    let first = header.syntax().children().next();
    let last = header.syntax().children().last();

    if let (Some(first), Some(last)) = (first, last) {
        check_new_line(
            errors,
            first,
            last,
            Forbid,
            "table header must fit into a single line",
        );
    }
    if let Some(entry) = table.entries().next() {
        check_new_line(
            errors,
            header,
            entry,
            Require,
            "newline is mandatory after table header",
        );
    }
}

#[derive(PartialOrd, PartialEq)]
enum Requirement {
    Forbid,
    Require,
}
use self::Requirement::*;

fn check_new_line<'a>(
    errors: &mut Vec<SyntaxError>,
    left: impl Into<SyntaxNodeRef<'a>>,
    right: impl Into<SyntaxNodeRef<'a>>,
    r: Requirement,
    msg: &str,
) {
    let left = left.into();
    let right = right.into();
    assert_eq!(left.parent(), right.parent());
    let parent = left.parent().unwrap();
    // FIXME: more precise
    let start = left.range().start();
    let end = right.range().start();
    let range = TextRange::from_to(start, end);
    let has_newline = parent.chunked_substring(range).contains_char('\n');
    if has_newline != (r == Require) {
        errors.push(SyntaxError {
            range,
            message: msg.into(),
        });
    }
}
