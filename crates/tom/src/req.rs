//! FIXME: write short doc here

use serde::{Serialize, Deserialize};
use languageserver_types::{
    Url, Range, TextDocumentIdentifier,
    request::Request,
    notification::{Notification},
};

pub enum SyntaxTree {}

impl Request for SyntaxTree {
    type Params = SyntaxTreeParams;
    type Result = String;
    const METHOD: &'static str = "m/syntaxTree";
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SyntaxTreeParams {
    pub text_document: TextDocumentIdentifier,
}

pub enum ExtendSelection {}

impl Request for ExtendSelection {
    type Params = ExtendSelectionParams;
    type Result = ExtendSelectionResult;
    const METHOD: &'static str = "m/extendSelection";
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExtendSelectionParams {
    pub text_document: TextDocumentIdentifier,
    pub selections: Vec<Range>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExtendSelectionResult {
    pub selections: Vec<Range>,
}

pub enum DecorationsRequest {}

impl Request for DecorationsRequest {
    type Params = TextDocumentIdentifier;
    type Result = Vec<Decoration>;
    const METHOD: &'static str = "m/decorationsRequest";
}

pub enum PublishDecorations {}

impl Notification for PublishDecorations {
    type Params = PublishDecorationsParams;
    const METHOD: &'static str = "m/publishDecorations";
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PublishDecorationsParams {
    #[serde(with = "url_serde")]
    pub uri: Url,
    pub decorations: Vec<Decoration>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Decoration {
    pub range: Range,
    pub tag: &'static str,
}
