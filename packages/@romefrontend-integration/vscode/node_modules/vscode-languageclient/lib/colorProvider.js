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
class ColorProviderFeature extends client_1.TextDocumentFeature {
    constructor(client) {
        super(client, vscode_languageserver_protocol_1.DocumentColorRequest.type);
    }
    fillClientCapabilities(capabilites) {
        ensure(ensure(capabilites, 'textDocument'), 'colorProvider').dynamicRegistration = true;
    }
    initialize(capabilities, documentSelector) {
        if (!capabilities.colorProvider) {
            return;
        }
        const implCapabilities = capabilities.colorProvider;
        const id = Is.string(implCapabilities.id) && implCapabilities.id.length > 0 ? implCapabilities.id : UUID.generateUuid();
        const selector = implCapabilities.documentSelector || documentSelector;
        if (selector) {
            this.register(this.messages, {
                id,
                registerOptions: Object.assign({}, { documentSelector: selector })
            });
        }
    }
    registerLanguageProvider(options) {
        let client = this._client;
        let provideColorPresentations = (color, context, token) => {
            const requestParams = {
                color,
                textDocument: client.code2ProtocolConverter.asTextDocumentIdentifier(context.document),
                range: client.code2ProtocolConverter.asRange(context.range)
            };
            return client.sendRequest(vscode_languageserver_protocol_1.ColorPresentationRequest.type, requestParams, token).then(this.asColorPresentations.bind(this), (error) => {
                client.logFailedRequest(vscode_languageserver_protocol_1.ColorPresentationRequest.type, error);
                return Promise.resolve(null);
            });
        };
        let provideDocumentColors = (document, token) => {
            const requestParams = {
                textDocument: client.code2ProtocolConverter.asTextDocumentIdentifier(document)
            };
            return client.sendRequest(vscode_languageserver_protocol_1.DocumentColorRequest.type, requestParams, token).then(this.asColorInformations.bind(this), (error) => {
                client.logFailedRequest(vscode_languageserver_protocol_1.ColorPresentationRequest.type, error);
                return Promise.resolve(null);
            });
        };
        let middleware = client.clientOptions.middleware;
        return vscode_1.languages.registerColorProvider(options.documentSelector, {
            provideColorPresentations: (color, context, token) => {
                return middleware.provideColorPresentations
                    ? middleware.provideColorPresentations(color, context, token, provideColorPresentations)
                    : provideColorPresentations(color, context, token);
            },
            provideDocumentColors: (document, token) => {
                return middleware.provideDocumentColors
                    ? middleware.provideDocumentColors(document, token, provideDocumentColors)
                    : provideDocumentColors(document, token);
            }
        });
    }
    asColor(color) {
        return new vscode_1.Color(color.red, color.green, color.blue, color.alpha);
    }
    asColorInformations(colorInformation) {
        if (Array.isArray(colorInformation)) {
            return colorInformation.map(ci => {
                return new vscode_1.ColorInformation(this._client.protocol2CodeConverter.asRange(ci.range), this.asColor(ci.color));
            });
        }
        return [];
    }
    asColorPresentations(colorPresentations) {
        if (Array.isArray(colorPresentations)) {
            return colorPresentations.map(cp => {
                let presentation = new vscode_1.ColorPresentation(cp.label);
                presentation.additionalTextEdits = this._client.protocol2CodeConverter.asTextEdits(cp.additionalTextEdits);
                presentation.textEdit = this._client.protocol2CodeConverter.asTextEdit(cp.textEdit);
                return presentation;
            });
        }
        return [];
    }
}
exports.ColorProviderFeature = ColorProviderFeature;
