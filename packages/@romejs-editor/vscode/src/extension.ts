/* --------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See License.txt in the project root for license information.
 * ------------------------------------------------------------------------------------------ */

import * as path from 'path';
import {workspace, ExtensionContext} from 'vscode';

import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  TransportKind,
} from 'vscode-languageclient';

let client: LanguageClient;

export function activate(context: ExtensionContext) {
  let serverOptions: ServerOptions = {
    module: '/tmp/rome-dev/index.js',
    args: ['lsp'],
    transport: TransportKind.stdio,
    runtime: 'node',
  };

  let clientOptions: LanguageClientOptions = {
    documentSelector: [
      {scheme: 'file', language: 'javascript'},
      {scheme: 'file', language: 'javascriptreact'},
      {scheme: 'file', language: 'typescript'},
      {scheme: 'file', language: 'typescriptreact'},
      {scheme: 'file', language: 'json'},
    ],
  };

  client = new LanguageClient('rome', 'Rome', serverOptions, clientOptions);

  client.start();
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop();
}
