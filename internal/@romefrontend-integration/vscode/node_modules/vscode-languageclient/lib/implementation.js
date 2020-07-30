/* --------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See License.txt in the project root for license information.
 * ------------------------------------------------------------------------------------------ */
'use strict';
Object.defineProperty(exports, "__esModule", { value: true });
const UUID = require("./utils/uuid");
const Is = require("./utils/is");
const vscode_1 = require("vscode");
const vscode_languageserver_protocol_1 = require("vscode-languageserver-protocol");
const client_1 = require("./client");
function ensure(target, key) {
    if (target[key] === void 0) {
        target[key] = {};
    }
    return target[key];
}
class ImplementationFeature extends client_1.TextDocumentFeature {
    constructor(client) {
        super(client, vscode_languageserver_protocol_1.ImplementationRequest.type);
    }
    fillClientCapabilities(capabilites) {
        let implementationSupport = ensure(ensure(capabilites, 'textDocument'), 'implementation');
        implementationSupport.dynamicRegistration = true;
        implementationSupport.linkSupport = true;
    }
    initialize(capabilities, documentSelector) {
        if (!capabilities.implementationProvider) {
            return;
        }
        if (capabilities.implementationProvider === true) {
            if (!documentSelector) {
                return;
            }
            this.register(this.messages, {
                id: UUID.generateUuid(),
                registerOptions: Object.assign({}, { documentSelector: documentSelector })
            });
        }
        else {
            const implCapabilities = capabilities.implementationProvider;
            const id = Is.string(implCapabilities.id) && implCapabilities.id.length > 0 ? implCapabilities.id : UUID.generateUuid();
            const selector = implCapabilities.documentSelector || documentSelector;
            if (selector) {
                this.register(this.messages, {
                    id,
                    registerOptions: Object.assign({}, { documentSelector: selector })
                });
            }
        }
    }
    registerLanguageProvider(options) {
        let client = this._client;
        let provideImplementation = (document, position, token) => {
            return client.sendRequest(vscode_languageserver_protocol_1.ImplementationRequest.type, client.code2ProtocolConverter.asTextDocumentPositionParams(document, position), token).then(client.protocol2CodeConverter.asDefinitionResult, (error) => {
                client.logFailedRequest(vscode_languageserver_protocol_1.ImplementationRequest.type, error);
                return Promise.resolve(null);
            });
        };
        let middleware = client.clientOptions.middleware;
        return vscode_1.languages.registerImplementationProvider(options.documentSelector, {
            provideImplementation: (document, position, token) => {
                return middleware.provideImplementation
                    ? middleware.provideImplementation(document, position, token, provideImplementation)
                    : provideImplementation(document, position, token);
            }
        });
    }
}
exports.ImplementationFeature = ImplementationFeature;
