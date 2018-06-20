use {
    TomlDoc, SyntaxError, TextRange, TomlNode,
    ast::*,
    visitor
};

pub(crate) fn validate(doc: &TomlDoc) -> Vec<SyntaxError> {
    visitor::process(
        doc.parse_tree(),
        visitor::visitor(Vec::new())
            .visit::<KeyVal, _>(|errors, kv| {
                check_new_line(
                    errors,
                    kv.key(), kv.val(),
                    false,
                    "newlines are forbidden in entries",
                );
            })
            .visit::<Table, _>(|errors, table| {
                if let Some(entry) = table.entries().next() {
                    check_new_line(
                        errors,
                        table.header(), entry,
                        true,
                        "newline is mandatory after table header",
                    );
                }
            })
            .visit::<Table, _>(|errors, table| {
                if let Some(entry) = table.entries().next() {
                    check_new_line(
                        errors,
                        table.header(), entry,
                        true,
                        "newline is mandatory after table header",
                    );
                }
            })
    )
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
