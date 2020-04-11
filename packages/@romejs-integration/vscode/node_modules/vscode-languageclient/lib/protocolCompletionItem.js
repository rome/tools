/* --------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See License.txt in the project root for license information.
 * ------------------------------------------------------------------------------------------ */
'use strict';
Object.defineProperty(exports, "__esModule", { value: true });
const code = require("vscode");
class ProtocolCompletionItem extends code.CompletionItem {
    constructor(label) {
        super(label);
    }
}
exports.default = ProtocolCompletionItem;
