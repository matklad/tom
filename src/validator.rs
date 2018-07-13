use {
    CstNode, SyntaxError, TextRange, TomlDoc, ChunkedText,
    ast,
    visitor,
};

pub(crate) fn validate(doc: &TomlDoc) -> Vec<SyntaxError> {
    visitor::process(
        doc.cst(),
        doc,
        visitor::visitor(Vec::new())
            .visit::<ast::Entry, _>(|errors, entry| {
                if let Some(first_key) = entry.keys(doc).next() {
                    check_new_line(
                        doc,
                        errors,
                        first_key,
                        entry.value(doc),
                        Forbid,
                        "newlines are forbidden in entries",
                    );
                }
            })
            .visit::<ast::Dict, _>(|errors, d| {
                check_new_line(
                    doc,
                    errors,
                    d.cst().children(doc).first().unwrap(),
                    d.cst().children(doc).last().unwrap(),
                    Forbid,
                    "newlines are forbidden in inline tables",
                )
            })
            .visit::<ast::Table, _>(|errors, table| check_table(doc, errors, table))
            .visit::<ast::ArrayTable, _>(|errors, table| check_table(doc, errors, table)),
    )
}

fn check_table<'f>(
    doc: &TomlDoc,
    errors: &mut Vec<SyntaxError>,
    table: impl ast::EntryOwner + ast::TableHeaderOwner,
) {
    let header = table.header(doc);
    match (
        header.cst().children(doc).first(),
        header.cst().children(doc).last(),
    ) {
        (Some(first), Some(last)) => check_new_line(
            doc,
            errors,
            first,
            last,
            Forbid,
            "table header must fit into a single line",
        ),
        _ => (),
    }
    if let Some(entry) = table.entries(doc).next() {
        check_new_line(
            doc,
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

fn check_new_line<'f>(
    doc: &TomlDoc,
    errors: &mut Vec<SyntaxError>,
    left: impl Into<CstNode>,
    right: impl Into<CstNode>,
    r: Requirement,
    msg: &str,
) {
    let left = left.into();
    let right = right.into();
    assert_eq!(left.parent(doc), right.parent(doc));
    let parent = left.parent(doc).unwrap();
    // TODO: more precise
    let start = left.range(doc).start();
    let end = right.range(doc).start();
    let range = TextRange::from_to(start, end);
    let has_newline = parent.chunked_substring(doc, range).contains_char('\n');
    if has_newline != (r == Require) {
        errors.push(SyntaxError {
            range,
            message: msg.into(),
        });
    }
}
