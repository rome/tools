/* --------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See License.txt in the project root for license information.
 * ------------------------------------------------------------------------------------------ */
'use strict';
Object.defineProperty(exports, "__esModule", { value: true });
const UUID = require("./utils/uuid");
const vscode_1 = require("vscode");
const vscode_languageserver_protocol_1 = require("vscode-languageserver-protocol");
function access(target, key) {
    if (target === void 0) {
        return undefined;
    }
    return target[key];
}
class WorkspaceFoldersFeature {
    constructor(_client) {
        this._client = _client;
        this._listeners = new Map();
    }
    get messages() {
        return vscode_languageserver_protocol_1.DidChangeWorkspaceFoldersNotification.type;
    }
    fillInitializeParams(params) {
        let folders = vscode_1.workspace.workspaceFolders;
        if (folders === void 0) {
            params.workspaceFolders = null;
        }
        else {
            params.workspaceFolders = folders.map(folder => this.asProtocol(folder));
        }
    }
    fillClientCapabilities(capabilities) {
        capabilities.workspace = capabilities.workspace || {};
        capabilities.workspace.workspaceFolders = true;
    }
    initialize(capabilities) {
        let client = this._client;
        client.onRequest(vscode_languageserver_protocol_1.WorkspaceFoldersRequest.type, (token) => {
            let workspaceFolders = () => {
                let folders = vscode_1.workspace.workspaceFolders;
                if (folders === void 0) {
                    return null;
                }
                let result = folders.map((folder) => {
                    return this.asProtocol(folder);
                });
                return result;
            };
            let middleware = client.clientOptions.middleware.workspace;
            return middleware && middleware.workspaceFolders
                ? middleware.workspaceFolders(token, workspaceFolders)
                : workspaceFolders(token);
        });
        let value = access(access(access(capabilities, 'workspace'), 'workspaceFolders'), 'changeNotifications');
        let id;
        if (typeof value === 'string') {
            id = value;
        }
        else if (value === true) {
            id = UUID.generateUuid();
        }
        if (id) {
            this.register(this.messages, {
                id: id,
                registerOptions: undefined
            });
        }
    }
    register(_message, data) {
        let id = data.id;
        let client = this._client;
        let disposable = vscode_1.workspace.onDidChangeWorkspaceFolders((event) => {
            let didChangeWorkspaceFolders = (event) => {
                let params = {
                    event: {
                        added: event.added.map(folder => this.asProtocol(folder)),
                        removed: event.removed.map(folder => this.asProtocol(folder))
                    }
                };
                this._client.sendNotification(vscode_languageserver_protocol_1.DidChangeWorkspaceFoldersNotification.type, params);
            };
            let middleware = client.clientOptions.middleware.workspace;
            middleware && middleware.didChangeWorkspaceFolders
                ? middleware.didChangeWorkspaceFolders(event, didChangeWorkspaceFolders)
                : didChangeWorkspaceFolders(event);
        });
        this._listeners.set(id, disposable);
    }
    unregister(id) {
        let disposable = this._listeners.get(id);
        if (disposable === void 0) {
            return;
        }
        this._listeners.delete(id);
        disposable.dispose();
    }
    dispose() {
        for (let disposable of this._listeners.values()) {
            disposable.dispose();
        }
        this._listeners.clear();
    }
    asProtocol(workspaceFolder) {
        if (workspaceFolder === void 0) {
            return null;
        }
        return { uri: this._client.code2ProtocolConverter.asUri(workspaceFolder.uri), name: workspaceFolder.name };
    }
}
exports.WorkspaceFoldersFeature = WorkspaceFoldersFeature;
