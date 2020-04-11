/* --------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See License.txt in the project root for license information.
 * ------------------------------------------------------------------------------------------ */
'use strict';
Object.defineProperty(exports, "__esModule", { value: true });
const is = require("./is");
/**
 * Predefined error codes.
 */
var ErrorCodes;
(function (ErrorCodes) {
    // Defined by JSON RPC
    ErrorCodes.ParseError = -32700;
    ErrorCodes.InvalidRequest = -32600;
    ErrorCodes.MethodNotFound = -32601;
    ErrorCodes.InvalidParams = -32602;
    ErrorCodes.InternalError = -32603;
    ErrorCodes.serverErrorStart = -32099;
    ErrorCodes.serverErrorEnd = -32000;
    ErrorCodes.ServerNotInitialized = -32002;
    ErrorCodes.UnknownErrorCode = -32001;
    // Defined by the protocol.
    ErrorCodes.RequestCancelled = -32800;
    // Defined by VSCode library.
    ErrorCodes.MessageWriteError = 1;
    ErrorCodes.MessageReadError = 2;
})(ErrorCodes = exports.ErrorCodes || (exports.ErrorCodes = {}));
/**
 * An error object return in a response in case a request
 * has failed.
 */
class ResponseError extends Error {
    constructor(code, message, data) {
        super(message);
        this.code = is.number(code) ? code : ErrorCodes.UnknownErrorCode;
        this.data = data;
        Object.setPrototypeOf(this, ResponseError.prototype);
    }
    toJson() {
        return {
            code: this.code,
            message: this.message,
            data: this.data,
        };
    }
}
exports.ResponseError = ResponseError;
/**
 * An abstract implementation of a MessageType.
 */
class AbstractMessageType {
    constructor(_method, _numberOfParams) {
        this._method = _method;
        this._numberOfParams = _numberOfParams;
    }
    get method() {
        return this._method;
    }
    get numberOfParams() {
        return this._numberOfParams;
    }
}
exports.AbstractMessageType = AbstractMessageType;
/**
 * Classes to type request response pairs
 */
class RequestType0 extends AbstractMessageType {
    constructor(method) {
        super(method, 0);
        this._ = undefined;
    }
}
exports.RequestType0 = RequestType0;
class RequestType extends AbstractMessageType {
    constructor(method) {
        super(method, 1);
        this._ = undefined;
    }
}
exports.RequestType = RequestType;
class RequestType1 extends AbstractMessageType {
    constructor(method) {
        super(method, 1);
        this._ = undefined;
    }
}
exports.RequestType1 = RequestType1;
class RequestType2 extends AbstractMessageType {
    constructor(method) {
        super(method, 2);
        this._ = undefined;
    }
}
exports.RequestType2 = RequestType2;
class RequestType3 extends AbstractMessageType {
    constructor(method) {
        super(method, 3);
        this._ = undefined;
    }
}
exports.RequestType3 = RequestType3;
class RequestType4 extends AbstractMessageType {
    constructor(method) {
        super(method, 4);
        this._ = undefined;
    }
}
exports.RequestType4 = RequestType4;
class RequestType5 extends AbstractMessageType {
    constructor(method) {
        super(method, 5);
        this._ = undefined;
    }
}
exports.RequestType5 = RequestType5;
class RequestType6 extends AbstractMessageType {
    constructor(method) {
        super(method, 6);
        this._ = undefined;
    }
}
exports.RequestType6 = RequestType6;
class RequestType7 extends AbstractMessageType {
    constructor(method) {
        super(method, 7);
        this._ = undefined;
    }
}
exports.RequestType7 = RequestType7;
class RequestType8 extends AbstractMessageType {
    constructor(method) {
        super(method, 8);
        this._ = undefined;
    }
}
exports.RequestType8 = RequestType8;
class RequestType9 extends AbstractMessageType {
    constructor(method) {
        super(method, 9);
        this._ = undefined;
    }
}
exports.RequestType9 = RequestType9;
class NotificationType extends AbstractMessageType {
    constructor(method) {
        super(method, 1);
        this._ = undefined;
    }
}
exports.NotificationType = NotificationType;
class NotificationType0 extends AbstractMessageType {
    constructor(method) {
        super(method, 0);
        this._ = undefined;
    }
}
exports.NotificationType0 = NotificationType0;
class NotificationType1 extends AbstractMessageType {
    constructor(method) {
        super(method, 1);
        this._ = undefined;
    }
}
exports.NotificationType1 = NotificationType1;
class NotificationType2 extends AbstractMessageType {
    constructor(method) {
        super(method, 2);
        this._ = undefined;
    }
}
exports.NotificationType2 = NotificationType2;
class NotificationType3 extends AbstractMessageType {
    constructor(method) {
        super(method, 3);
        this._ = undefined;
    }
}
exports.NotificationType3 = NotificationType3;
class NotificationType4 extends AbstractMessageType {
    constructor(method) {
        super(method, 4);
        this._ = undefined;
    }
}
exports.NotificationType4 = NotificationType4;
class NotificationType5 extends AbstractMessageType {
    constructor(method) {
        super(method, 5);
        this._ = undefined;
    }
}
exports.NotificationType5 = NotificationType5;
class NotificationType6 extends AbstractMessageType {
    constructor(method) {
        super(method, 6);
        this._ = undefined;
    }
}
exports.NotificationType6 = NotificationType6;
class NotificationType7 extends AbstractMessageType {
    constructor(method) {
        super(method, 7);
        this._ = undefined;
    }
}
exports.NotificationType7 = NotificationType7;
class NotificationType8 extends AbstractMessageType {
    constructor(method) {
        super(method, 8);
        this._ = undefined;
    }
}
exports.NotificationType8 = NotificationType8;
class NotificationType9 extends AbstractMessageType {
    constructor(method) {
        super(method, 9);
        this._ = undefined;
    }
}
exports.NotificationType9 = NotificationType9;
/**
 * Tests if the given message is a request message
 */
function isRequestMessage(message) {
    let candidate = message;
    return candidate && is.string(candidate.method) && (is.string(candidate.id) || is.number(candidate.id));
}
exports.isRequestMessage = isRequestMessage;
/**
 * Tests if the given message is a notification message
 */
function isNotificationMessage(message) {
    let candidate = message;
    return candidate && is.string(candidate.method) && message.id === void 0;
}
exports.isNotificationMessage = isNotificationMessage;
/**
 * Tests if the given message is a response message
 */
function isResponseMessage(message) {
    let candidate = message;
    return candidate && (candidate.result !== void 0 || !!candidate.error) && (is.string(candidate.id) || is.number(candidate.id) || candidate.id === null);
}
exports.isResponseMessage = isResponseMessage;
