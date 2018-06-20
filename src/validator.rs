use {
    TomlDoc, SyntaxError, TextRange, TomlNode,
    ast,
    visitor
};

pub(crate) fn validate(doc: &TomlDoc) -> Vec<SyntaxError> {
    visitor::process(
        doc.parse_tree(),
        visitor::visitor(Vec::new())
            .visit::<ast::KeyVal, _>(|errors, kv| {
                check_new_line(
                    errors,
                    kv.key(), kv.val(),
                    false,
                    "newlines are forbidden in entries",
                );
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
    table: impl ast::KeyValueOwner<'f> + ast::TableHeaderOwner<'f>
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
    left: impl Into<TomlNode<'f>>, right: impl Into<TomlNode<'f>>,
    new_line_required: bool,
    msg: &str
) {
    let left = left.into();
    let right = right.into();
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
