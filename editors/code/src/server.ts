import * as lc from 'vscode-languageclient';

import { Config } from './config';
import { Highlighter } from './highlighting';

export class Server {
    public static highlighter = new Highlighter();
    public static config = new Config();
    public static client: lc.LanguageClient;

    public static start(
        notificationHandlers: Iterable<[string, lc.GenericNotificationHandler]>
    ) {
        const run: lc.Executable = {
            command: process.env.__TOM_LSP_SERVER_DEBUG ?? 'tom_lsp_server',
            options: { cwd: '.' }
        };
        const serverOptions: lc.ServerOptions = {
            run,
            debug: run
        };
        const clientOptions: lc.LanguageClientOptions = {
            documentSelector: [{ scheme: 'file', language: 'toml' }]
        };

        Server.client = new lc.LanguageClient(
            'tom-lsp',
            'tom language server',
            serverOptions,
            clientOptions
        );
        Server.client.onReady().then(() => {
            for (const [type, handler] of notificationHandlers) {
                Server.client.onNotification(type, handler);
            }
        });
        Server.client.start();
    }
}
