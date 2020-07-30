/* --------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See License.txt in the project root for license information.
 * ------------------------------------------------------------------------------------------ */
'use strict';
Object.defineProperty(exports, "__esModule", { value: true });
const events_1 = require("./events");
const Is = require("./is");
let DefaultSize = 8192;
let CR = Buffer.from('\r', 'ascii')[0];
let LF = Buffer.from('\n', 'ascii')[0];
let CRLF = '\r\n';
class MessageBuffer {
    constructor(encoding = 'utf8') {
        this.encoding = encoding;
        this.index = 0;
        this.buffer = Buffer.allocUnsafe(DefaultSize);
    }
    append(chunk) {
        var toAppend = chunk;
        if (typeof (chunk) === 'string') {
            var str = chunk;
            var bufferLen = Buffer.byteLength(str, this.encoding);
            toAppend = Buffer.allocUnsafe(bufferLen);
            toAppend.write(str, 0, bufferLen, this.encoding);
        }
        if (this.buffer.length - this.index >= toAppend.length) {
            toAppend.copy(this.buffer, this.index, 0, toAppend.length);
        }
        else {
            var newSize = (Math.ceil((this.index + toAppend.length) / DefaultSize) + 1) * DefaultSize;
            if (this.index === 0) {
                this.buffer = Buffer.allocUnsafe(newSize);
                toAppend.copy(this.buffer, 0, 0, toAppend.length);
            }
            else {
                this.buffer = Buffer.concat([this.buffer.slice(0, this.index), toAppend], newSize);
            }
        }
        this.index += toAppend.length;
    }
    tryReadHeaders() {
        let result = undefined;
        let current = 0;
        while (current + 3 < this.index && (this.buffer[current] !== CR || this.buffer[current + 1] !== LF || this.buffer[current + 2] !== CR || this.buffer[current + 3] !== LF)) {
            current++;
        }
        // No header / body separator found (e.g CRLFCRLF)
        if (current + 3 >= this.index) {
            return result;
        }
        result = Object.create(null);
        let headers = this.buffer.toString('ascii', 0, current).split(CRLF);
        headers.forEach((header) => {
            let index = header.indexOf(':');
            if (index === -1) {
                throw new Error('Message header must separate key and value using :');
            }
            let key = header.substr(0, index);
            let value = header.substr(index + 1).trim();
            result[key] = value;
        });
        let nextStart = current + 4;
        this.buffer = this.buffer.slice(nextStart);
        this.index = this.index - nextStart;
        return result;
    }
    tryReadContent(length) {
        if (this.index < length) {
            return null;
        }
        let result = this.buffer.toString(this.encoding, 0, length);
        let nextStart = length;
        this.buffer.copy(this.buffer, 0, nextStart);
        this.index = this.index - nextStart;
        return result;
    }
    get numberOfBytes() {
        return this.index;
    }
}
var MessageReader;
(function (MessageReader) {
    function is(value) {
        let candidate = value;
        return candidate && Is.func(candidate.listen) && Is.func(candidate.dispose) &&
            Is.func(candidate.onError) && Is.func(candidate.onClose) && Is.func(candidate.onPartialMessage);
    }
    MessageReader.is = is;
})(MessageReader = exports.MessageReader || (exports.MessageReader = {}));
class AbstractMessageReader {
    constructor() {
        this.errorEmitter = new events_1.Emitter();
        this.closeEmitter = new events_1.Emitter();
        this.partialMessageEmitter = new events_1.Emitter();
    }
    dispose() {
        this.errorEmitter.dispose();
        this.closeEmitter.dispose();
    }
    get onError() {
        return this.errorEmitter.event;
    }
    fireError(error) {
        this.errorEmitter.fire(this.asError(error));
    }
    get onClose() {
        return this.closeEmitter.event;
    }
    fireClose() {
        this.closeEmitter.fire(undefined);
    }
    get onPartialMessage() {
        return this.partialMessageEmitter.event;
    }
    firePartialMessage(info) {
        this.partialMessageEmitter.fire(info);
    }
    asError(error) {
        if (error instanceof Error) {
            return error;
        }
        else {
            return new Error(`Reader recevied error. Reason: ${Is.string(error.message) ? error.message : 'unknown'}`);
        }
    }
}
exports.AbstractMessageReader = AbstractMessageReader;
class StreamMessageReader extends AbstractMessageReader {
    constructor(readable, encoding = 'utf8') {
        super();
        this.readable = readable;
        this.buffer = new MessageBuffer(encoding);
        this._partialMessageTimeout = 10000;
    }
    set partialMessageTimeout(timeout) {
        this._partialMessageTimeout = timeout;
    }
    get partialMessageTimeout() {
        return this._partialMessageTimeout;
    }
    listen(callback) {
        this.nextMessageLength = -1;
        this.messageToken = 0;
        this.partialMessageTimer = undefined;
        this.callback = callback;
        this.readable.on('data', (data) => {
            this.onData(data);
        });
        this.readable.on('error', (error) => this.fireError(error));
        this.readable.on('close', () => this.fireClose());
    }
    onData(data) {
        this.buffer.append(data);
        while (true) {
            if (this.nextMessageLength === -1) {
                let headers = this.buffer.tryReadHeaders();
                if (!headers) {
                    return;
                }
                let contentLength = headers['Content-Length'];
                if (!contentLength) {
                    throw new Error('Header must provide a Content-Length property.');
                }
                let length = parseInt(contentLength);
                if (isNaN(length)) {
                    throw new Error('Content-Length value must be a number.');
                }
                this.nextMessageLength = length;
                // Take the encoding form the header. For compatibility
                // treat both utf-8 and utf8 as node utf8
            }
            var msg = this.buffer.tryReadContent(this.nextMessageLength);
            if (msg === null) {
                /** We haven't recevied the full message yet. */
                this.setPartialMessageTimer();
                return;
            }
            this.clearPartialMessageTimer();
            this.nextMessageLength = -1;
            this.messageToken++;
            var json = JSON.parse(msg);
            this.callback(json);
        }
    }
    clearPartialMessageTimer() {
        if (this.partialMessageTimer) {
            clearTimeout(this.partialMessageTimer);
            this.partialMessageTimer = undefined;
        }
    }
    setPartialMessageTimer() {
        this.clearPartialMessageTimer();
        if (this._partialMessageTimeout <= 0) {
            return;
        }
        this.partialMessageTimer = setTimeout((token, timeout) => {
            this.partialMessageTimer = undefined;
            if (token === this.messageToken) {
                this.firePartialMessage({ messageToken: token, waitingTime: timeout });
                this.setPartialMessageTimer();
            }
        }, this._partialMessageTimeout, this.messageToken, this._partialMessageTimeout);
    }
}
exports.StreamMessageReader = StreamMessageReader;
class IPCMessageReader extends AbstractMessageReader {
    constructor(process) {
        super();
        this.process = process;
        let eventEmitter = this.process;
        eventEmitter.on('error', (error) => this.fireError(error));
        eventEmitter.on('close', () => this.fireClose());
    }
    listen(callback) {
        this.process.on('message', callback);
    }
}
exports.IPCMessageReader = IPCMessageReader;
class SocketMessageReader extends StreamMessageReader {
    constructor(socket, encoding = 'utf-8') {
        super(socket, encoding);
    }
}
exports.SocketMessageReader = SocketMessageReader;
