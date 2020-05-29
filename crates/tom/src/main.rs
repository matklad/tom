//! FIXME: write short doc here

mod line_index;
mod req;
mod caps;

use std::collections::HashMap;

use crossbeam_channel::{Sender, Receiver};
use languageserver_types::{
    InitializeParams, Url, Range, Position,
    notification::{self},
};
use gen_lsp_server::{
    run_server, stdio_transport, handle_shutdown, RawMessage, RawResponse, RawNotification,
};
use flexi_logger::{Logger, Duplicate};
use tom_syntax::{TomlDoc, TextRange, TextUnit, symbol::*};
use failure::format_err;

use line_index::{LineIndex, LineCol};

fn main() -> Result<(), failure::Error> {
    ::std::env::set_var("RUST_BACKTRACE", "short");
    Logger::with_env_or_str("error")
        .duplicate_to_stderr(Duplicate::All)
        .log_to_file()
        .directory("log")
        .start()?;
    let (receiver, sender, io_threads) = stdio_transport();
    run_server(caps::server_capabilities(), receiver, sender, main_loop)?;
    io_threads.join()?;
    Ok(())
}

fn main_loop(
    _params: InitializeParams,
    receiver: &Receiver<RawMessage>,
    sender: &Sender<RawMessage>,
) -> Result<(), failure::Error> {
    let mut state = State::default();

    for msg in receiver {
        match msg {
            RawMessage::Request(req) => {
                let req = match handle_shutdown(req, sender) {
                    None => return Ok(()),
                    Some(req) => req,
                };
                let req = match req.cast::<req::DecorationsRequest>() {
                    Ok((id, params)) => {
                        let decorations = state.decorations(&params.uri);
                        let resp = RawResponse::ok::<req::DecorationsRequest>(id, &decorations);
                        sender.send(RawMessage::Response(resp));
                        continue;
                    }
                    Err(req) => req,
                };
                let req = match req.cast::<req::SyntaxTree>() {
                    Ok((id, params)) => {
                        let tree = state.syntax_tree(&params.text_document.uri);
                        let resp = RawResponse::ok::<req::SyntaxTree>(id, &tree);
                        sender.send(RawMessage::Response(resp));
                        continue;
                    }
                    Err(req) => req,
                };
                let _req = match req.cast::<req::ExtendSelection>() {
                    Ok((id, params)) => {
                        let selections =
                            state.extend_selections(&params.text_document.uri, &params.selections);
                        let resp = RawResponse::ok::<req::ExtendSelection>(
                            id,
                            &req::ExtendSelectionResult { selections },
                        );
                        sender.send(RawMessage::Response(resp));
                        continue;
                    }
                    Err(req) => req,
                };
            }
            RawMessage::Response(_resp) => (),
            RawMessage::Notification(not) => {
                let not = match not.cast::<notification::DidOpenTextDocument>() {
                    Ok(params) => {
                        let uri = params.text_document.uri;
                        state.add_file(uri.clone(), &params.text_document.text);
                        state.publish_decorations(uri, sender);
                        continue;
                    }
                    Err(not) => not,
                };
                let not = match not.cast::<notification::DidChangeTextDocument>() {
                    Ok(mut params) => {
                        let uri = params.text_document.uri;
                        let text = params
                            .content_changes
                            .pop()
                            .ok_or_else(|| format_err!("empty changes"))?
                            .text;
                        state.add_file(uri.clone(), &text);
                        state.publish_decorations(uri, sender);
                        continue;
                    }
                    Err(not) => not,
                };
                let _not = match not.cast::<notification::DidCloseTextDocument>() {
                    Ok(params) => {
                        let uri = params.text_document.uri;
                        state.remove_file(&uri);
                        continue;
                    }
                    Err(not) => not,
                };
            }
        }
    }
    Ok(())
}

#[derive(Default)]
struct State {
    files: HashMap<Url, (TomlDoc, LineIndex)>,
}

impl State {
    fn add_file(&mut self, url: Url, text: &str) -> TomlDoc {
        let doc = TomlDoc::new(text);
        let line_index = LineIndex::new(text);
        self.files.insert(url, (doc.clone(), line_index));
        doc
    }
    fn remove_file(&mut self, url: &Url) {
        self.files.remove(url);
    }

    fn decorations(&self, url: &Url) -> Vec<req::Decoration> {
        let (doc, line_index) = match self.files.get(url) {
            Some((doc, line_index)) => (doc, line_index),
            None => return Vec::new(),
        };
        let mut decorations = Vec::new();
        for node in doc.cst().descendants() {
            let tag = match node.symbol() {
                TABLE_HEADER => "keyword",

                BASIC_STRING
                | MULTILINE_BASIC_STRING
                | LITERAL_STRING
                | MULTILINE_LITERAL_STRING => "string",
                COMMENT => "comment",
                _ => continue,
            };

            decorations.push(req::Decoration {
                range: to_vs_range(node.range(), line_index),
                tag,
            });
        }
        decorations
    }

    fn publish_decorations(&self, url: Url, sender: &Sender<RawMessage>) {
        let decorations = self.decorations(&url);
        let params = req::PublishDecorationsParams {
            uri: url,
            decorations,
        };
        let msg =
            RawMessage::Notification(RawNotification::new::<req::PublishDecorations>(&params));
        sender.send(msg);
    }

    fn extend_selections(&self, uri: &Url, selections: &[Range]) -> Vec<Range> {
        let (doc, line_index) = match self.files.get(uri) {
            Some((doc, line_index)) => (doc, line_index),
            None => return selections.to_vec(),
        };
        let mut res = Vec::new();
        for &sel in selections.iter() {
            let sel = from_vs_range(sel, line_index);
            let sel = extend(doc, sel);
            let sel = to_vs_range(sel, line_index);
            res.push(sel);
        }
        res
    }

    fn syntax_tree(&self, uri: &Url) -> String {
        let doc = match self.files.get(uri) {
            Some((doc, _line_index)) => doc,
            None => return "".to_string(),
        };
        doc.debug()
    }
}

pub(crate) fn extend(doc: &TomlDoc, range: TextRange) -> TextRange {
    let node = doc.cst().covering_node(range);

    match node.ancestors().find(|n| n.range() != range) {
        None => range,
        Some(parent) => parent.range(),
    }
}

fn to_vs_range(range: TextRange, line_index: &LineIndex) -> Range {
    Range::new(
        to_vs_position(range.start(), line_index),
        to_vs_position(range.end(), line_index),
    )
}

fn to_vs_position(offset: TextUnit, line_index: &LineIndex) -> Position {
    let line_col = line_index.line_col(offset);
    Position::new(u64::from(line_col.line), u64::from(u32::from(line_col.col)))
}

fn from_vs_range(range: Range, line_index: &LineIndex) -> TextRange {
    TextRange::from_to(
        from_vs_position(range.start, line_index),
        from_vs_position(range.end, line_index),
    )
}

fn from_vs_position(position: Position, line_index: &LineIndex) -> TextUnit {
    let line_col = LineCol {
        line: position.line as u32,
        col: (position.character as u32).into(),
    };
    line_index.offset(line_col)
}
