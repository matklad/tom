//! FIXME: write short doc here

use languageserver_types::{
    ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncOptions, TextDocumentSyncKind,
};

pub fn server_capabilities() -> ServerCapabilities {
    ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Options(
            TextDocumentSyncOptions {
                open_close: Some(true),
                change: Some(TextDocumentSyncKind::Full),
                will_save: None,
                will_save_wait_until: None,
                save: None,
            },
        )),
        ..Default::default()
    }
}
