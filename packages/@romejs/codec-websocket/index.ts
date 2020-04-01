/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Frame, BuildFrameOpts, OPCODES, GUID} from './types';
import {parseFrame, buildFrame, unmaskPayload, isCompleteFrame} from './frame';
import {Event} from '@romejs/events';
import crypto = require('crypto');

import url = require('url');

import http = require('http');

import net = require('net');

import {Reporter} from '@romejs/cli-reporter';

export function createKey(key: string): string {
  return crypto.createHash('sha1').update(`${key}${GUID}`).digest('base64');
}

type WebSocketType = 'client' | 'server';

export class WebSocketInterface {
  constructor(type: WebSocketType, socket: net.Socket, reporter?: Reporter) {
    // When a frame is set here then any additional continuation frames payloads will be appended
    this.unfinishedFrame = undefined;

    // When a frame is set here, all additional chunks will be appended until we reach the correct payloadLength
    this.incompleteFrame = undefined;

    this.reporter = reporter;
    this.socket = socket;
    this.alive = true;
    this.type = type;

    this.completeFrameEvent = new Event({name: 'WebSocketInterface.message'});
    this.errorEvent = new Event({name: 'WebSocketInterface.error'});
    this.endEvent = new Event({name: 'WebSocketInterface.end', serial: true});

    socket.on('data', (buff) => {
      this.addBuffer(buff);
    });

    socket.on('error', (err: NodeJS.ErrnoException) => {
      if (err.code === 'ECONNRESET') {
        this.endEvent.send();
      } else {
        this.errorEvent.send(err);
      }
    });

    socket.on('close', () => {
      this.end();
    });
  }

  alive: boolean;
  type: WebSocketType;
  incompleteFrame: undefined | Frame;
  unfinishedFrame: undefined | Frame;
  socket: net.Socket;
  reporter: undefined | Reporter;

  completeFrameEvent: Event<Frame, void>;
  errorEvent: Event<Error, void>;
  endEvent: Event<void, void>;

  end() {
    if (!this.alive) {
      return;
    }

    this.alive = false;
    this.endEvent.send();
    this.socket.end();
  }

  send(buff: string | Buffer) {
    if (typeof buff === 'string') {
      this.sendFrame({
        opcode: OPCODES.TEXT,
        fin: true,
        data: Buffer.from(buff),
      });
    } else if (buff instanceof Buffer) {
      this.sendFrame({
        opcode: OPCODES.BINARY,
        fin: true,
        data: buff,
      });
    } else {
      throw new Error("Don't know how to send this");
    }
  }

  sendJSON(val: unknown) {
    this.send(String(JSON.stringify(val)));
  }

  sendFrame(frameOpts: BuildFrameOpts) {
    if (this.reporter !== undefined) {
      this.reporter.info('Sending frame', {
        fin: frameOpts.fin,
        opcode: frameOpts.opcode,
        msg: frameOpts.data,
      });
    }
    this.socket.write(buildFrame(frameOpts, this.type === 'client'));
  }

  completeFrame(frame: Frame) {
    // If we have an unfinished frame then only allow continuations
    const {unfinishedFrame} = this;
    if (unfinishedFrame !== undefined) {
      if (frame.opcode === OPCODES.CONTINUATION) {
        unfinishedFrame.payload = Buffer.concat([
          unfinishedFrame.payload,
          unmaskPayload(
            frame.payload,
            unfinishedFrame.mask,
            unfinishedFrame.payload.length,
          ),
        ]);

        if (frame.fin) {
          this.unfinishedFrame = undefined;
          this.completeFrame(unfinishedFrame);
        }
        return undefined;
      } else {
        // Silently ignore the previous frame...
        this.unfinishedFrame = undefined;
        /*throw new Error(
          `We're waiting for a frame to finish so only allow continuation frames. Received frame: ${JSON.stringify(
            frame,
          )} Unfinished frame: ${JSON.stringify(unfinishedFrame)}`,
        );*/
      }
    }

    if (frame.fin) {
      if (frame.opcode === OPCODES.PING) {
        this.sendFrame({
          opcode: OPCODES.PONG,
          fin: true,
          data: frame.payload,
        });
      } else {
        // Trim off any excess payload
        let excess;
        if (frame.payload.length > frame.payloadLength) {
          excess = frame.payload.slice(frame.payloadLength);
          frame.payload = frame.payload.slice(0, frame.payloadLength);
        }

        if (this.reporter !== undefined) {
          this.reporter.info('Received complete frame', {
            opcode: frame.opcode,
            length: frame.payloadLength,
            msg: frame.payload,
          });
        }

        this.completeFrameEvent.send(frame);

        if (excess !== undefined) {
          this.addBuffer(excess);
        }
      }
    } else {
      this.unfinishedFrame = frame;
    }
  }

  addBufferToIncompleteFrame(incompleteFrame: Frame, buff: Buffer) {
    incompleteFrame.payload = Buffer.concat([
      incompleteFrame.payload,
      unmaskPayload(buff, incompleteFrame.mask, incompleteFrame.payload.length),
    ]);

    if (isCompleteFrame(incompleteFrame)) {
      this.incompleteFrame = undefined;
      this.completeFrame(incompleteFrame);
    }
  }

  addBuffer(buff: Buffer) {
    // Check if we're still waiting for the rest of a payload
    const {incompleteFrame} = this;
    if (incompleteFrame !== undefined) {
      this.addBufferToIncompleteFrame(incompleteFrame, buff);
      return undefined;
    }

    const frame = parseFrame(buff);

    if (isCompleteFrame(frame)) {
      // Frame has been completed!
      this.completeFrame(frame);
    } else {
      this.incompleteFrame = frame;
    }
  }
}

export async function createClient(rawUrl: string): Promise<WebSocketInterface> {
  const parts = url.parse(rawUrl);

  return new Promise(
      (resolve, reject) => {
        const key = crypto.randomBytes(16).toString('base64');
        const digest = createKey(key);

        const req = http.request({
          hostname: parts.hostname,
          port: parts.port,
          path: parts.path,
          method: 'GET',
          headers: {
            Connection: 'Upgrade',
            Upgrade: 'websocket',
            'Sec-WebSocket-Key': key,
            'Sec-WebSocket-Version': '13',
          },
        });

        req.on('response', (res) => {
          if (res.statusCode && res.statusCode >= 400) {
            process.stderr.write(`Unexpected HTTP code: ${res.statusCode}\n`);
            res.pipe(process.stderr);
          } else {
            res.pipe(process.stderr);
          }
        });

        req.on(
          'upgrade',
          (res, socket, head) => {
            if (res.headers['sec-websocket-accept'] !== digest) {
              socket.end();
              reject(
                new Error(
                  `Digest mismatch ${digest} !== ${res.headers['sec-websocket-accept']}`,
                ),
              );
              return undefined;
            }

            const client = new WebSocketInterface('client', socket);
            //client.addBuffer(head);
            head;
            resolve(client);
          },
        );

        req.on('error', (err) => {
          reject(err);
        });

        req.end();
      },
    );
}
