/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {dumpToBuffer, BunserBuf} from './bser';
import {Reporter} from '@romejs/cli-reporter';
import {Event} from '@romejs/events';
import child_process = require('child_process');

import util = require('util');

import net = require('net');

import {Consumer, consumeUnknown} from '@romejs/consume';
import {Dict} from '@romejs/typescript-helpers';

const exec = util.promisify(child_process.exec);

export type WatchmanSubscriptionValue = {
  'state-enter': undefined | string;
  'state-leave': undefined | string;
  unilateral: boolean;
  subscription: string;
  root: string;
  files: Array<{
    exists: boolean;
    type: 'f' | 'd';
    size: number;
    name: string;
    mtime: number;
  }>;
  is_fresh_instance: boolean;
  version: string;
  since: string;
  clock: string;
};

function normalizeWatchmanSubscription(res: Consumer): WatchmanSubscriptionValue {
  return {
    'state-enter': res.get('state-enter').asStringOrVoid(),
    'state-leave': res.get('state-leave').asStringOrVoid(),
    unilateral: res.get('unilateral').asBoolean(),
    subscription: res.get('subscription').asString(),
    root: res.get('root').asString(),

    // This can be a massive array... We should still probably efficiently validate it though somehow
    files: res.get('files').asAny(),

    is_fresh_instance: res.get('is_fresh_instance').asBoolean(),
    version: res.get('version').asString(),
    since: res.get('since').asString(),
    clock: res.get('clock').asString(),
  };
}

export class WatchmanClient {
  constructor(socket: net.Socket, reporter: Reporter) {
    this.reporter = reporter;
    this.socket = socket;

    this.subscriptionCounter = 0;
    this.subscriptions = new Map();

    this.logEvent = new Event({name: 'WatchmanClient.log'});

    this.callbacks = [];
    this.listen();
  }

  callbacks: Array<{
    resolve: (data: Consumer) => void;
    reject: (err: Error) => void;
  }>;

  subscriptions: Map<string, Event<WatchmanSubscriptionValue, void>>;
  subscriptionCounter: 0;

  reporter: Reporter;
  socket: net.Socket;
  logEvent: Event<void, void>;

  listen() {
    const {socket} = this;

    socket.on('error', function() {
      //
    });

    const bunser = new BunserBuf();

    bunser.valueEvent.subscribe((obj) => {
      this.processResponse(consumeUnknown(obj, 'parse/json'));
    });

    socket.on('data', (chunk) => {
      bunser.append(chunk);
    });

    socket.on('end', () => {
      this.end();
    });
  }

  processResponse(res: Consumer) {
    if (res.has('warn')) {
      this.reporter.warn(res.get('warn').asString());
    }

    if (res.has('subscription')) {
      const name = res.get('subscription').asString();
      const event = this.subscriptions.get(name);
      if (event === undefined) {
        this.reporter.warn(
          "Received a watchman subscription event for %s that we aren't listening for",
          name,
        );
      } else {
        event.send(normalizeWatchmanSubscription(res));
      }
      return;
    }

    if (res.has('log')) {
      // TODO
      return;
    }

    if (res.get('unilateral').asBooleanOrVoid() === true) {
      this.reporter.warn(
        "Received a watchman unilateral event that we don't support",
        res.asUnknown(),
      );
      return;
    }

    const callback = this.callbacks.shift();
    if (callback === undefined) {
      throw new Error('Received message but no callback');
    }

    if (res.has('error')) {
      callback.reject(new Error(res.get('error').asString()));
    } else {
      callback.resolve(res);
    }
  }

  async createSubscription(
    dir: string,
    opts: Dict<unknown>,
  ): Promise<Event<WatchmanSubscriptionValue, void>> {
    const name = `rome-${process.pid}.${String(this.subscriptionCounter++)}`;
    const event: Event<WatchmanSubscriptionValue, void> = new Event({
      name,
    });
    this.subscriptions.set(name, event);

    const consumer = await this.command(['watch-project', dir]);

    // Refine any filter to specifically files from the requested directory if we weren't the watchman root
    if (consumer.has('relative_path')) {
      opts = {
        ...opts,
        relative_root: consumer.get('relative_path').asString(),
      };
    }

    const root = consumer.get('watch').asString();
    await this.command(['subscribe', root, name, opts]);
    return event;
  }

  async command(args: Array<unknown>): Promise<Consumer> {
    return new Promise((resolve, reject) => {
      this.callbacks.push({resolve, reject});
      this.socket.write(dumpToBuffer(args));
    });
  }

  end() {
    for (const {reject} of this.callbacks) {
      reject(new Error('The watchman connection was closed'));
    }
    this.socket.end();
  }
}

export async function getWatchmanSocketLocation(): Promise<string> {
  // Environment variable may be set by invoking tools
  if (typeof process.env.WATCHMAN_SOCK === 'string') {
    return process.env.WATCHMAN_SOCK;
  }

  try {
    const {stdout, stderr} = await exec('watchman --no-pretty get-sockname');

    // Parse response
    try {
      const data = JSON.parse(stdout);

      // Validate JSON result
      if (typeof data !== 'object' || data == null || typeof data.sockname !==
          'string') {
        throw new Error(
            `Watchman returned JSON payload that wasnt an object with a sockname property`,
          );
      }

      return data.sockname;
    } catch (err) {
      // Better error message for syntatically invalid JSON
      if (err instanceof SyntaxError) {
        err = new Error(`Watchman returned malformed JSON payload`);
      }

      // Always add the stdout and stderr to messages for better readability
      err.message += ` ${JSON.stringify({stdout, stderr})}`;
      throw err;
    }
  } catch (err) {
    if (err.code === 127) {
      // Exit code for command not found
      throw new Error('No watchman binary command found');
    } else {
      throw err;
    }
  }
}

export async function createWatchmanClient(
  reporter: Reporter,
): Promise<WatchmanClient> {
  const sockname = await getWatchmanSocketLocation();
  const socket = net.createConnection(sockname);

  return new Promise((resolve, reject) => {
    socket.on('error', (err) => {
      reject(err);
    });

    socket.on('connect', () => {
      resolve(new WatchmanClient(socket, reporter));
    });
  });
}
