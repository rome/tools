/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {consumeUnknown, Consumer} from '@romejs/consume';
import {
  LSPResponseMessage,
  LSPDiagnostic,
  LSPPosition,
  LSPTextEdit,
  LSPDiagnosticRelatedInformation,
  LSPRange,
} from './types';
import Master, {MasterClient} from '../Master';
import {
  createAbsoluteFilePath,
  AbsoluteFilePath,
  AbsoluteFilePathMap,
  AbsoluteFilePathSet,
} from '@romejs/path';
import {
  Diagnostics,
  DiagnosticsProcessor,
  DiagnosticLocation,
} from '@romejs/diagnostics';
import {Position} from '@romejs/parser-core';
import {coerce1to0, number0, Number0, inc} from '@romejs/ob1';
import {stripMarkupTags} from '@romejs/string-markup';
import {
  PartialMasterQueryRequest,
  MasterQueryResponse,
} from '@romejs/core/common/bridges/MasterBridge';
import Linter from '../linter/Linter';
import MasterRequest from '../MasterRequest';
import {DEFAULT_CLIENT_REQUEST_FLAGS} from '@romejs/core/common/types/client';
import stringDiff, {
  Diffs,
  constants as diffConstants,
} from '@romejs/string-diff';
import {JSONObject, JSONPropertyValue} from '@romejs/codec-json';
import {
  ReporterProgress,
  ReporterProgressBase,
  Reporter,
  ReporterProgressOptions,
} from '@romejs/cli-reporter';

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
      line: number0,
      character: number0,
    };
  } else {
    return {
      line: coerce1to0(pos.line),
      character: pos.column,
    };
  }
}

function convertDiagnosticLocationToLSPRange(
  location: DiagnosticLocation,
): LSPRange {
  return {
    start: convertPositionToLSP(location.start),
    end: convertPositionToLSP(location.end),
  };
}

function convertDiagnosticsToLSP(
  diagnostics: Diagnostics,
  master: Master,
): Array<LSPDiagnostic> {
  const lspDiagnostics: Array<LSPDiagnostic> = [];

  for (const {description, location} of diagnostics) {
    // Infer relatedInformation from log messages followed by frames
    let relatedInformation: Array<LSPDiagnosticRelatedInformation> = [];
    const {advice} = description;
    if (advice !== undefined) {
      for (let i = 0; i < advice.length; i++) {
        const item = advice[i];
        const nextItem = advice[i + 1];
        if (item.type === 'log' && nextItem !== undefined && nextItem.type ===
        'frame') {
          const abs = master.projectManager.getFilePathFromUidOrAbsolute(
            nextItem.location.filename,
          );
          if (abs !== undefined) {
            relatedInformation.push({
              message: stripMarkupTags(item.message),
              location: {
                uri: `file://${abs.join()}`,
                range: convertDiagnosticLocationToLSPRange(nextItem.location),
              },
            });
          }
        }
      }
    }

    lspDiagnostics.push({
      severity: 1,
      range: convertDiagnosticLocationToLSPRange(location),
      message: stripMarkupTags(description.message.value),
      code: description.category,
      source: 'rome',
      relatedInformation,
    });
  }

  return lspDiagnostics;
}

function getPathFromTextDocument(consumer: Consumer): AbsoluteFilePath {
  return createAbsoluteFilePath(consumer.get('uri').asString());
}

function diffTextEdits(original: string, desired: string): Array<LSPTextEdit> {
  const edits: Array<LSPTextEdit> = [];

  const diffs: Diffs = stringDiff(original, desired);

  let currLine: Number0 = number0;
  let currChar: Number0 = number0;

  function advance(str: string) {
    for (const char of str) {
      if (char === '\n') {
        currLine = inc(currLine);
        currChar = number0;
      } else {
        currChar = inc(currChar);
      }
    }
  }

  function getPosition(): LSPPosition {
    return {
      line: currLine,
      character: currChar,
    };
  }

  for (const [type, text] of diffs) {
    switch (type) {
      case diffConstants.ADD:
        const pos = getPosition();
        edits.push({
          range: {
            start: pos,
            end: pos,
          },
          newText: text,
        });
        break;

      case diffConstants.DELETE:
        const start: LSPPosition = getPosition();
        advance(text);
        const end: LSPPosition = getPosition();
        edits.push({
          range: {
            start,
            end,
          },
          newText: '',
        });
        break;

      case diffConstants.EQUAL:
        advance(text);
        break;
    }
  }

  return edits;
}

let progressTokenCounter = 0;

class LSPProgress extends ReporterProgressBase {
  constructor(
    server: LSPServer,
    reporter: Reporter,
    opts?: ReporterProgressOptions,
  ) {
    super(reporter, opts);
    this.server = server;
    this.token = progressTokenCounter++;
    this.lastRenderKey = '';

    server.write({
      type: '$/progress',
      params: {
        token: this.token,
        value: {
          kind: 'begin',
          cancellable: false,
          title: this.title,
          percentage: 0,
        },
      },
    });
  }

  lastRenderKey: string;
  token: number;
  server: LSPServer;

  render() {
    const total = this.total === undefined ? 0 : this.total;
    const percentage = Math.floor(100 / total * this.current);

    // Make sure we don't send pointless duplicate messages
    const renderKey = `percent:${percentage},text:${this.text}`;
    if (this.lastRenderKey === renderKey) {
      return;
    }

    this.lastRenderKey = renderKey;
    this.server.write({
      type: '$/progress',
      params: {
        token: this.token,
        value: {
          kind: 'report',
          cancellable: false,
          message: this.text,
          percentage,
        },
      },
    });
  }

  end() {
    this.server.write({
      type: '$/progress',
      params: {
        token: this.token,
        value: {
          kind: 'end',
        },
      },
    });
  }
}

export default class LSPServer {
  constructor(request: MasterRequest) {
    this.status = 'IDLE';
    this.buffer = '';
    this.nextHeaders = undefined;

    this.request = request;
    this.master = request.master;
    this.client = request.client;

    this.lintSessionsPending = new AbsoluteFilePathSet();
    this.lintSessions = new AbsoluteFilePathMap();

    request.endEvent.subscribe(() => {
      this.shutdown();
    });
  }

  request: MasterRequest;
  client: MasterClient;
  master: Master;
  nextHeaders: undefined | Headers;
  status: Status;
  buffer: string;
  lintSessionsPending: AbsoluteFilePathSet;
  lintSessions: AbsoluteFilePathMap<MasterRequest>;

  write(res: JSONObject) {
    const json = JSON.stringify(res);
    const out = `Content-Length: ${String(json.length)}${HEADERS_END}${json}`;
    this.client.bridge.lspFromServerBuffer.send(out);
  }

  createFakeMasterRequest(
    commandName: string,
    args: Array<string> = [],
  ): MasterRequest {
    return new MasterRequest({
      client: this.client,
      master: this.master,
      query: {
        requestFlags: DEFAULT_CLIENT_REQUEST_FLAGS,
        commandFlags: {},
        args,
        commandName,
        silent: true,
        noData: false,
        terminateWhenIdle: false,
      },
    });
  }

  unwatchProject(path: AbsoluteFilePath) {
    // TODO maybe unset all buffers?
    const req = this.lintSessions.get(path);
    if (req !== undefined) {
      req.teardown({
        type: 'SUCCESS',
        hasData: false,
        data: undefined,
        markers: [],
      });
      this.lintSessions.delete(path);
    }
  }

  createProgress(opts?: ReporterProgressOptions): ReporterProgress {
    return new LSPProgress(this, this.request.reporter, opts);
  }

  async watchProject(path: AbsoluteFilePath) {
    if (this.lintSessions.has(path) || this.lintSessionsPending.has(path)) {
      return;
    }

    this.lintSessionsPending.add(path);

    const project = await this.master.projectManager.findProject(path);

    if (project === undefined) {
      // Not a Rome project
      this.lintSessionsPending.delete(path);
      return;
    }

    const req = this.createFakeMasterRequest('lsp_project', [path.join()]);
    await req.init();

    const linter = new Linter(req, {});

    const subscription = await linter.watch({
      onRunStart: () => {},
      createProgress: () => {
        return this.createProgress();
      },
      onChanges: ({changes}) => {
        for (const {ref, diagnostics} of changes) {
          if (ref === undefined) {
            // Cannot display diagnostics without a reference
            continue;
          }

          // We want to filter pendingFixes because we'll autoformat the file on save if necessary and it's just noise
          const processor = new DiagnosticsProcessor();
          processor.addFilter({
            category: 'lint/pendingFixes',
          });
          processor.addDiagnostics(diagnostics);

          this.write({
            method: 'textDocument/publishDiagnostics',
            params: {
              uri: `file://${ref.real.join()}`,
              diagnostics: convertDiagnosticsToLSP(
                processor.getDiagnostics(),
                this.master,
              ),
            },
          });
        }
      },
    });

    req.endEvent.subscribe(() => {
      subscription.unsubscribe();
    });

    this.lintSessions.set(path, req);
    this.lintSessionsPending.delete(path);
  }

  shutdown() {
    for (const path of this.lintSessions.keys()) {
      this.unwatchProject(path);
    }
    this.lintSessions.clear();
  }

  async sendClientRequest(
    req: PartialMasterQueryRequest,
  ): Promise<MasterQueryResponse> {
    return this.master.handleRequest(this.client, {
      silent: true,
      ...req,
    });
  }

  async handleRequest(
    method: string,
    params: Consumer,
  ): Promise<JSONPropertyValue> {
    switch (method) {
      case 'initialize':
        const rootUri = params.get('rootUri');
        if (rootUri.exists()) {
          this.watchProject(createAbsoluteFilePath(rootUri.asString()));
        }

        const workspaceFolders = params.get('workspaceFolders');
        if (workspaceFolders.exists()) {
          for (const elem of workspaceFolders.asArray()) {
            this.watchProject(getPathFromTextDocument(elem));
          }
        }

        return {
          capabilities: {
            textDocumentSync: {
              openClose: true,
              // This sends over the full text on change. We should make this incremental later
              change: 1,
            },
            documentFormattingProvider: true,
            workspaceFolders: {
              supported: true,
              changeNotifications: true,
            },
          },
          serverInfo: {
            name: 'rome',
          },
        };

      case 'textDocument/formatting':
        const path = getPathFromTextDocument(params.get('textDocument'));

        const project = this.master.projectManager.findProjectExisting(path);
        if (project === undefined) {
          // Not in a Rome project
          return null;
        }

        const res = await this.request.requestWorkerFormat(path);
        if (res === undefined) {
          // Not a file we support formatting
          return null;
        }

        return diffTextEdits(res.original, res.formatted);

      case 'shutdown':
        this.shutdown();
        break;
    }

    return null;
  }

  async handleNotification(method: string, params: Consumer): Promise<void> {
    switch (method) {
      case 'workspace/didChangeWorkspaceFolders':
        for (const elem of params.get('added').asArray()) {
          this.watchProject(getPathFromTextDocument(elem));
        }
        for (const elem of params.get('removed').asArray()) {
          this.unwatchProject(getPathFromTextDocument(elem));
        }
        break;

      case 'textDocument/didChange':
        const path = getPathFromTextDocument(params.get('textDocument'));
        const content =
          params.get('contentChanges').asArray()[0].get('text').asString();
        await this.request.requestWorkerUpdateBuffer(path, content);
        break;
    }
  }

  normalizeMessage(content: string): undefined | Consumer {
    try {
      const data = JSON.parse(content);
      const consumer = consumeUnknown(data, 'lsp/parse');
      return consumer;
    } catch (err) {
      if (err instanceof SyntaxError) {
        console.error('JSON parse error', content);
        return undefined;
      } else {
        throw err;
      }
    }
  }

  async onMessage(headers: Headers, content: string) {
    const consumer = this.normalizeMessage(content);
    if (consumer === undefined) {
      return;
    }

    if (!consumer.has('method')) {
      console.error('NO METHOD', content);
      return;
    }

    const method: string = consumer.get('method').asString();
    const params = consumer.get('params');

    if (consumer.has('id')) {
      const id = consumer.get('id').asNumber();

      try {
        const res: LSPResponseMessage = {
          id,
          result: await this.handleRequest(method, params),
        };
        this.write(res);
      } catch (err) {
        const res: LSPResponseMessage = {
          id,
          error: {
            code: -32_603,
            message: err.message,
          },
        };
        this.write(res);
      }
    } else {
      await this.handleNotification(method, params);
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
        const headers = this.nextHeaders;
        if (headers === undefined) {
          throw new Error('Expected headers due to our status');
        }
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
