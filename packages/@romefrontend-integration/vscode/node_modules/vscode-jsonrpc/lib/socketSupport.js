/* --------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See License.txt in the project root for license information.
 * ------------------------------------------------------------------------------------------ */
'use strict';
Object.defineProperty(exports, "__esModule", { value: true });
const net_1 = require("net");
const messageReader_1 = require("./messageReader");
const messageWriter_1 = require("./messageWriter");
function createClientSocketTransport(port, encoding = 'utf-8') {
    let connectResolve;
    let connected = new Promise((resolve, _reject) => {
        connectResolve = resolve;
    });
    return new Promise((resolve, reject) => {
        let server = net_1.createServer((socket) => {
            server.close();
            connectResolve([
                new messageReader_1.SocketMessageReader(socket, encoding),
                new messageWriter_1.SocketMessageWriter(socket, encoding)
            ]);
        });
        server.on('error', reject);
        server.listen(port, '127.0.0.1', () => {
            server.removeListener('error', reject);
            resolve({
                onConnected: () => { return connected; }
            });
        });
    });
}
exports.createClientSocketTransport = createClientSocketTransport;
function createServerSocketTransport(port, encoding = 'utf-8') {
    const socket = net_1.createConnection(port, '127.0.0.1');
    return [
        new messageReader_1.SocketMessageReader(socket, encoding),
        new messageWriter_1.SocketMessageWriter(socket, encoding)
    ];
}
exports.createServerSocketTransport = createServerSocketTransport;
