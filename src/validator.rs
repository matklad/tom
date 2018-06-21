use {
    TomlDoc, SyntaxError, TextRange, CstNode,
    ast,
    visitor
};

pub(crate) fn validate(doc: &TomlDoc) -> Vec<SyntaxError> {
    visitor::process(
        doc.cst(),
        visitor::visitor(Vec::new())
            .visit::<ast::Entry, _>(|errors, entry| {
                if let Some(first_key) = entry.keys().next() {
                    check_new_line(
                        errors,
                        first_key, entry.val(),
                        Forbid,
                        "newlines are forbidden in entries",
                    );
                }
            })
            .visit::<ast::Dict, _>(|errors, d| {
                check_new_line(
                    errors,
                    d.node().children().next().unwrap(),
                    d.node().children().last().unwrap(),
                    Forbid,
                    "newlines are forbidden in inline tables"
                )
            })
            .visit::<ast::Table, _>(check_table)
            .visit::<ast::ArrayTable, _>(check_table)
    )
}

fn check_table<'f>(
    errors: &mut Vec<SyntaxError>,
    table: impl ast::EntryOwner<'f> + ast::TableHeaderOwner<'f>
) {
    let header = table.header();
    match (header.node().children().next(), header.node().children().last()) {
        (Some(first), Some(last)) => {
            check_new_line(
                errors,
                first, last,
                Forbid,
                "table header must fit into a single line",
            )
        },
        _ => (),
    }
    if let Some(entry) = table.entries().next() {
        check_new_line(
            errors,
            header, entry,
            Require,
            "newline is mandatory after table header",
        );
    }
}

#[derive(PartialOrd, PartialEq)]
enum Requirement {
    Forbid, Require,
}
use self::Requirement::*;


fn check_new_line<'f>(
    errors: &mut Vec<SyntaxError>,
    left: impl Into<CstNode<'f>>, right: impl Into<CstNode<'f>>,
    r: Requirement,
    msg: &str
) {
    let left = left.into();
    let right = right.into();
    // TODO: more precise
    let start = left.range().start();
    let end = right.range().start();
    let range = TextRange::from_to(start, end);
    let text = &left.doc().text()[range];
    if text.contains("\n")  != (r == Require) {
        errors.push(SyntaxError {
            range,
            message: msg.into()
        });
    }
}
