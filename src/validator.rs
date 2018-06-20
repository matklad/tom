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
                        false,
                        "newlines are forbidden in entries",
                    );
                }
            })
            .visit::<ast::Dict, _>(|errors, d| {
                check_new_line(
                    errors,
                    d.node().children().next().unwrap(),
                    d.node().children().last().unwrap(),
                    false,
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
    if let Some(entry) = table.entries().next() {
        check_new_line(
            errors,
            table.header(), entry,
            true,
            "newline is mandatory after table header",
        );
    }
}


fn check_new_line<'f>(
    errors: &mut Vec<SyntaxError>,
    left: impl Into<CstNode<'f>>, right: impl Into<CstNode<'f>>,
    new_line_required: bool,
    msg: &str
) {
    let left = left.into();
    let right = right.into();
    // TODO: more precise
    let start = left.range().end();
    let end = right.range().start();
    let range = TextRange::from_to(start, end);
    let text = &left.doc().text()[range];
    if text.contains("\n") != new_line_required {
        errors.push(SyntaxError {
            range,
            message: msg.into()
        });
    }
}
