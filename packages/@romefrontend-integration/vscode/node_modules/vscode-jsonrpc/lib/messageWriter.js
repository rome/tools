/* --------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See License.txt in the project root for license information.
 * ------------------------------------------------------------------------------------------ */
'use strict';
Object.defineProperty(exports, "__esModule", { value: true });
const events_1 = require("./events");
const Is = require("./is");
let ContentLength = 'Content-Length: ';
let CRLF = '\r\n';
var MessageWriter;
(function (MessageWriter) {
    function is(value) {
        let candidate = value;
        return candidate && Is.func(candidate.dispose) && Is.func(candidate.onClose) &&
            Is.func(candidate.onError) && Is.func(candidate.write);
    }
    MessageWriter.is = is;
})(MessageWriter = exports.MessageWriter || (exports.MessageWriter = {}));
class AbstractMessageWriter {
    constructor() {
        this.errorEmitter = new events_1.Emitter();
        this.closeEmitter = new events_1.Emitter();
    }
    dispose() {
        this.errorEmitter.dispose();
        this.closeEmitter.dispose();
    }
    get onError() {
        return this.errorEmitter.event;
    }
    fireError(error, message, count) {
        this.errorEmitter.fire([this.asError(error), message, count]);
    }
    get onClose() {
        return this.closeEmitter.event;
    }
    fireClose() {
        this.closeEmitter.fire(undefined);
    }
    asError(error) {
        if (error instanceof Error) {
            return error;
        }
        else {
            return new Error(`Writer recevied error. Reason: ${Is.string(error.message) ? error.message : 'unknown'}`);
        }
    }
}
exports.AbstractMessageWriter = AbstractMessageWriter;
class StreamMessageWriter extends AbstractMessageWriter {
    constructor(writable, encoding = 'utf8') {
        super();
        this.writable = writable;
        this.encoding = encoding;
        this.errorCount = 0;
        this.writable.on('error', (error) => this.fireError(error));
        this.writable.on('close', () => this.fireClose());
    }
    write(msg) {
        let json = JSON.stringify(msg);
        let contentLength = Buffer.byteLength(json, this.encoding);
        let headers = [
            ContentLength, contentLength.toString(), CRLF,
            CRLF
        ];
        try {
            // Header must be written in ASCII encoding
            this.writable.write(headers.join(''), 'ascii');
            // Now write the content. This can be written in any encoding
            this.writable.write(json, this.encoding);
            this.errorCount = 0;
        }
        catch (error) {
            this.errorCount++;
            this.fireError(error, msg, this.errorCount);
        }
    }
}
exports.StreamMessageWriter = StreamMessageWriter;
class IPCMessageWriter extends AbstractMessageWriter {
    constructor(process) {
        super();
        this.process = process;
        this.errorCount = 0;
        this.queue = [];
        this.sending = false;
        let eventEmitter = this.process;
        eventEmitter.on('error', (error) => this.fireError(error));
        eventEmitter.on('close', () => this.fireClose);
    }
    write(msg) {
        if (!this.sending && this.queue.length === 0) {
            // See https://github.com/nodejs/node/issues/7657
            this.doWriteMessage(msg);
        }
        else {
            this.queue.push(msg);
        }
    }
    doWriteMessage(msg) {
        try {
            if (this.process.send) {
                this.sending = true;
                this.process.send(msg, undefined, undefined, (error) => {
                    this.sending = false;
                    if (error) {
                        this.errorCount++;
                        this.fireError(error, msg, this.errorCount);
                    }
                    else {
                        this.errorCount = 0;
                    }
                    if (this.queue.length > 0) {
                        this.doWriteMessage(this.queue.shift());
                    }
                });
            }
        }
        catch (error) {
            this.errorCount++;
            this.fireError(error, msg, this.errorCount);
        }
    }
}
exports.IPCMessageWriter = IPCMessageWriter;
class SocketMessageWriter extends AbstractMessageWriter {
    constructor(socket, encoding = 'utf8') {
        super();
        this.socket = socket;
        this.queue = [];
        this.sending = false;
        this.encoding = encoding;
        this.errorCount = 0;
        this.socket.on('error', (error) => this.fireError(error));
        this.socket.on('close', () => this.fireClose());
    }
    write(msg) {
        if (!this.sending && this.queue.length === 0) {
            // See https://github.com/nodejs/node/issues/7657
            this.doWriteMessage(msg);
        }
        else {
            this.queue.push(msg);
        }
    }
    doWriteMessage(msg) {
        let json = JSON.stringify(msg);
        let contentLength = Buffer.byteLength(json, this.encoding);
        let headers = [
            ContentLength, contentLength.toString(), CRLF,
            CRLF
        ];
        try {
            // Header must be written in ASCII encoding
            this.sending = true;
            this.socket.write(headers.join(''), 'ascii', (error) => {
                if (error) {
                    this.handleError(error, msg);
                }
                try {
                    // Now write the content. This can be written in any encoding
                    this.socket.write(json, this.encoding, (error) => {
                        this.sending = false;
                        if (error) {
                            this.handleError(error, msg);
                        }
                        else {
                            this.errorCount = 0;
                        }
                        if (this.queue.length > 0) {
                            this.doWriteMessage(this.queue.shift());
                        }
                    });
                }
                catch (error) {
                    this.handleError(error, msg);
                }
            });
        }
        catch (error) {
            this.handleError(error, msg);
        }
    }
    handleError(error, msg) {
        this.errorCount++;
        this.fireError(error, msg, this.errorCount);
    }
}
exports.SocketMessageWriter = SocketMessageWriter;
