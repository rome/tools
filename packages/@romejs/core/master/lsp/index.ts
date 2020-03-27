/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {consumeUnknown} from '@romejs/consume';
import {
  LSPNotificationMessage,
  LSPRequestMessage,
  LSPResponseMessage,
  LSPDiagnostic,
  LSPPosition,
} from './types';
import Master, {MasterClient} from '../Master';
import {createAbsoluteFilePath} from '@romejs/path';
import {Diagnostics} from '@romejs/diagnostics';
import {Position} from '@romejs/parser-core';
import {coerce1to0, get0} from '@romejs/ob1';
import {stripMarkupTags} from '@romejs/string-markup';

type Status = 'IDLE' | 'WAITING_FOR_HEADERS_END' | 'WAITING_FOR_RESPONSE_END';

type Headers = {
  length: number;
  extra: Map<string, string>;
};

const HEADERS_END = '\r\n\r\n';

function parseHeaders(buffer: string): Headers {
  const headers: Map<string, string> = new Map();

  for (const line of buffer.split('\n')) {
    const clean = line.trim();
    const match = clean.match(/^(.*?): (.*?)$/);
    if (match == null) {
      throw new Error(`Invalid header: ${clean}`);
    }

    const [, key, value] = match;
    headers.set(key.toLowerCase(), value);
  }

  const length = headers.get('content-length');
  if (length === undefined) {
    throw new Error('Expected Content-Length');
  }
  headers.delete('content-length');

  return {
    length: Number(length),
    extra: headers,
  };
}

function convertPositionToLSP(pos: undefined | Position): LSPPosition {
  if (pos === undefined) {
    return {
      line: 0,
      character: 0,
    };
  } else {
    return {
      line: get0(coerce1to0(pos.line)),
      character: get0(pos.column),
    };
  }
}

function convertDiagnosticsToLSP(diagnostics: Diagnostics): Array<LSPDiagnostic> {
  const lspDiagnostics: Array<LSPDiagnostic> = [];

  for (const {description, location} of diagnostics) {
    lspDiagnostics.push({
      severity: 1,
      range: {
        start: convertPositionToLSP(location.start),
        end: convertPositionToLSP(location.end),
      },
      message: stripMarkupTags(description.message.value),
      code: description.category,
      source: 'rome',
    });
  }

  return lspDiagnostics;
}

type Writer = (chunk: string) => void;

export class LSPConnection {
  constructor(master: Master, client: MasterClient, write: Writer) {
    this.status = 'IDLE';
    this.buffer = '';
    this.nextHeaders = undefined;
    this._write = write;
    this.master = master;
    this.client = client;
  }

  client: MasterClient;
  master: Master;
  _write: Writer;
  nextHeaders: undefined | Headers;
  status: Status;
  buffer: string;

  write(res: unknown) {
    const json = JSON.stringify(res);
    const out = `Content-Length: ${String(json.length)}${HEADERS_END}${json}`;
    this._write(out);
  }

  getNextHeaders(): Headers {
    const {nextHeaders} = this;
    if (nextHeaders === undefined) {
      throw new Error('Expected headers due to our status');
    }
    return nextHeaders;
  }

  async handleRequest(req: LSPRequestMessage): Promise<unknown> {
    const params = consumeUnknown(req.params, 'lsp/parse');

    switch (req.method) {
      case 'initialize':
        await this.master.projectManager.assertProject(createAbsoluteFilePath(
          params.get('rootUri').asString(),
        ));
        return {
          capabilities: {
            textDocumentSync: 1,
            documentFormattingProvider: true,
          },
          serverInfo: {
            name: 'rome',
          },
        };

      case 'textDocument/formatting':
        const uri = params.get('textDocument').get('uri').asString();
        const res = await this.master.handleRequest(this.client, {
          command: 'format',
          args: [uri],
          silent: true,
        });
        res;
    }

    return {};
  }

  async handleNotification(notif: LSPNotificationMessage): Promise<void> {
    const params = consumeUnknown(notif.params, 'lsp/parse');

    switch (notif.method) {
      case 'textDocument/didOpen':
        const uri = params.get('textDocument').get('uri').asString();
        const res = await this.master.handleRequest(this.client, {
          command: 'lint',
          args: [uri],
          silent: true,
        });
        if (res.type === 'DIAGNOSTICS') {
          this.write({
            method: 'textDocument/publishDiagnostics',
            params: {
              uri,
              diagnostics: convertDiagnosticsToLSP(res.diagnostics),
            },
          });
        }
    }
  }

  async onMessage(headers: Headers, content: string) {
    const data = JSON.parse(content);
    const consumer = consumeUnknown(data, 'lsp/parse');

    const notif: LSPNotificationMessage = {
      method: consumer.get('method').asString(),
      params: consumer.get('params').asUnknown(),
    };

    if (consumer.has('id')) {
      const req: LSPRequestMessage = {
        // id can also be a string?
        id: consumer.get('id').asNumber(),
        ...notif,
      };

      try {
        const res: LSPResponseMessage = {
          id: req.id,
          result: await this.handleRequest(req),
        };
        this.write(res);
      } catch (err) {
        const res: LSPResponseMessage = {
          id: req.id,
          error: {
            code: -32_603,
            message: err.message,
          },
        };
        this.write(res);
      }
    } else {
      await this.handleNotification(notif);
    }
  }

  process() {
    switch (this.status) {
      case 'IDLE':
        if (this.buffer.length > 0) {
          this.status = 'WAITING_FOR_HEADERS_END';
          this.process();
        }
        break;

      case 'WAITING_FOR_HEADERS_END':
        const endIndex = this.buffer.indexOf(HEADERS_END);
        if (endIndex !== -1) {
          // Parse headers
          const rawHeaders = this.buffer.slice(0, endIndex);
          this.nextHeaders = parseHeaders(rawHeaders);

          // Process rest of the buffer
          this.status = 'WAITING_FOR_RESPONSE_END';
          this.buffer = this.buffer.slice(endIndex + HEADERS_END.length);
          this.process();
        }
        break;

      case 'WAITING_FOR_RESPONSE_END':
        const headers = this.getNextHeaders();
        if (this.buffer.length >= headers.length) {
          const content = this.buffer.slice(0, headers.length);
          this.onMessage(headers, content);

          // Reset headers and trim content
          this.nextHeaders = undefined;
          this.buffer = this.buffer.slice(headers.length);

          // Process rest of the buffer
          this.status = 'IDLE';
          this.process();
        }
        break;
    }
  }

  append(data: string) {
    this.buffer += data;
    this.process();
  }
}
