/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {MasterRequest, Master, WebBridge} from '@romejs/core';
import Bundler from '../bundler/Bundler';
import {WebSocketInterface} from '@romejs/codec-websocket';
import prettyFormat from '@romejs/pretty-format';
import http = require('http');

import {escapeMarkup} from '@romejs/string-markup';
import {Reporter, ReporterStream} from '@romejs/cli-reporter';
import {
  MasterQueryRequest,
  MasterQueryResponse,
} from '../../common/bridges/MasterBridge';
import {MasterMarker} from '../Master';
import {ClientFlagsJSON} from '../../common/types/client';
import WebRequest, {stripBundleSuffix} from './WebRequest';
import {BundlerConfig} from '../../common/types/bundler';
import {AbsoluteFilePath} from '@romejs/path';
import {PLATFORMS} from '../../common/types/platform';
import {HmrClientLogMessage, HmrServerMessage} from './hmr';
import {ConsumableUrl} from '@romejs/codec-url';

export type WebMasterTime = {
  startTime: number;
  endTime: undefined | number;
};

export type WebMasterClient = WebMasterTime & {
  id: number;
  flags: ClientFlagsJSON;
  stdoutAnsi: string;
  stdoutHTML: string;
};

export type WebMasterRequest = WebMasterTime & {
  id: number;
  client: number;
  query: MasterQueryRequest;
  markers: Array<MasterMarker>;
  response: undefined | MasterQueryResponse;
};

export class WebServer {
  constructor(req: MasterRequest) {
    const {master} = req;

    this.masterRequest = req;
    this.reporter = req.reporter;
    this.master = master;

    this.bundlerCache = new Map();

    this.savingRequests = false;
    this.clientRequestHistory = new Map();
    this.clientHistory = new Map();

    this.deviceWebsockets = new Set();
    this.frontendWebsocketBridges = new Set();

    this.server = http.createServer((req, res) => {
      const webRequest = new WebRequest(this, req, res);
      webRequest.dispatch();
    });

    master.clientStartEvent.subscribe((client) => {
      if (!this.savingRequests) {
        return;
      }

      const data: WebMasterClient = {
        id: client.id,
        flags: {
          ...client.flags,
          cwd: client.flags.cwd.join(),
        },
        startTime: Date.now(),
        endTime: undefined,
        stdoutAnsi: '',
        stdoutHTML: '',
      };
      this.clientHistory.set(client.id, data);
      this.refreshRequests();

      const ansiReporterStream: ReporterStream = {
        type: 'all',
        format: 'ansi',
        columns: 100,
        unicode: true,
        write(chunk) {
          data.stdoutAnsi += chunk;
        },
      };

      const htmlReporterStream: ReporterStream = {
        type: 'all',
        format: 'html',
        columns: 100,
        unicode: true,
        write(chunk) {
          data.stdoutAnsi += chunk;
        },
      };

      client.reporter.addStream(ansiReporterStream);
      master.connectedReporters.addStream(ansiReporterStream);

      client.reporter.addStream(htmlReporterStream);
      master.connectedReporters.addStream(htmlReporterStream);

      client.bridge.endEvent.subscribe(() => {
        master.connectedReporters.removeStream(ansiReporterStream);
        master.connectedReporters.removeStream(htmlReporterStream);

        data.endTime = Date.now();
        this.refreshRequests();
      });
    });

    master.requestStartEvent.subscribe((request) => {
      if (!this.savingRequests) {
        return;
      }

      const data: WebMasterRequest = {
        id: request.id,
        client: request.client.id,
        query: request.query,
        markers: [],
        response: undefined,
        startTime: Date.now(),
        endTime: undefined,
      };
      this.clientRequestHistory.set(request.id, data);
      this.refreshRequests();

      request.markerEvent.subscribe((marker) => {
        data.markers.push(marker);
        this.refreshRequests();
      });

      request.endEvent.subscribe((response) => {
        // Update completion fields
        data.response = response;
        data.endTime = Date.now();
        this.refreshRequests();
      });
    });
  }

  bundlerCache: Map<string, Bundler>;

  savingRequests: boolean;
  clientRequestHistory: Map<number, WebMasterRequest>;
  clientHistory: Map<number, WebMasterClient>;

  deviceWebsockets: Set<WebSocketInterface>;
  frontendWebsocketBridges: Set<WebBridge>;

  reporter: Reporter;
  masterRequest: MasterRequest;
  master: Master;
  server: http.Server;

  sendRequests(bridge: WebBridge) {
    bridge.requests.send({
      requests: Array.from(this.clientRequestHistory.values()),
      clients: Array.from(this.clientHistory.values()),
    });
  }

  refreshRequests() {
    for (const bridge of this.frontendWebsocketBridges) {
      this.sendRequests(bridge);
    }
  }

  close() {
    this.server.close();
  }

  listen(port: number) {
    this.server.listen(port);

    //this.reporter.clear();
    const url = `http://localhost:${String(port)}`;
    this.reporter.success(`Listening on <hyperlink emphasis>${url}</hyperlink>`);
    this.reporter.info(
      `Web console available at <hyperlink emphasis>${url}/__rome__</hyperlink>`,
    );
  }

  printConsoleLog(msg: HmrClientLogMessage) {
    const {reporter} = this.masterRequest;

    let buf = msg.data.map((arg) => {
      if (typeof arg === 'string') {
        return escapeMarkup(arg);
      } else {
        return prettyFormat(arg, {markup: true});
      }
    }).join(' ');

    switch (msg.level) {
      case 'info': {
        reporter.info(buf);
        break;
      }

      case 'warn': {
        reporter.warn(buf);
        break;
      }

      case 'log':
      case 'trace': {
        reporter.verboseForce(buf);
        break;
      }

      case 'group':
      case 'groupCollapsed':
      case 'groupEnd':
        reporter.logAll('TODO');
    }
  }

  async pathnameToAbsolutePath(
    pathname: string,
  ): Promise<undefined | AbsoluteFilePath> {
    const project = await this.masterRequest.assertClientCwdProject();
    const possibleStaticPath = project.folder.append(pathname);

    // This check makes sure that files outside of the project directory cannot be served
    if (possibleStaticPath.isRelativeTo(project.folder)) {
      return possibleStaticPath;
    } else {
      return undefined;
    }
  }

  sendToAllDeviceWebsockets(msg: HmrServerMessage) {
    const text = JSON.stringify(msg);
    for (const socket of this.deviceWebsockets) {
      socket.send(text);
    }
  }

  async getBundler(url: ConsumableUrl): Promise<{
    bundler: Bundler;
    path: AbsoluteFilePath;
  }> {
    const pathname = stripBundleSuffix(String(url.path.asString()));

    const absolute = await this.pathnameToAbsolutePath(pathname);
    if (absolute === undefined) {
      throw new Error('Pathname is attempting to escalate out of cwd');
    }

    const pathPointer = url.path.getDiagnosticLocation();
    const path = await this.master.resolver.resolveEntryAssertPath({
      origin: this.masterRequest.client.flags.cwd,
      source: absolute,
    }, pathPointer === undefined ? undefined : {location: pathPointer});

    const platform = url.query.get('platform').asStringSetOrVoid(PLATFORMS);
    const cacheKey = JSON.stringify({
      platform,
    });

    const cached = this.bundlerCache.get(cacheKey);
    if (cached !== undefined) {
      return {bundler: cached, path};
    }

    const bundlerConfig: BundlerConfig = this.masterRequest.getBundlerConfigFromFlags(
      {
        platform,
      },
    );

    const bundler = new Bundler(this.masterRequest, bundlerConfig);
    this.bundlerCache.set(cacheKey, bundler);
    return {bundler, path};
  }
}
