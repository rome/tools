/**
 * A language server message
 */
export interface Message {
    jsonrpc: string;
}
/**
 * Request message
 */
export interface RequestMessage extends Message {
    /**
     * The request id.
     */
    id: number | string;
    /**
     * The method to be invoked.
     */
    method: string;
    /**
     * The method's params.
     */
    params?: any;
}
/**
 * Predefined error codes.
 */
export declare namespace ErrorCodes {
    const ParseError: number;
    const InvalidRequest: number;
    const MethodNotFound: number;
    const InvalidParams: number;
    const InternalError: number;
    const serverErrorStart: number;
    const serverErrorEnd: number;
    const ServerNotInitialized: number;
    const UnknownErrorCode: number;
    const RequestCancelled: number;
    const MessageWriteError: number;
    const MessageReadError: number;
}
export interface ResponseErrorLiteral<D> {
    /**
     * A number indicating the error type that occured.
     */
    code: number;
    /**
     * A string providing a short decription of the error.
     */
    message: string;
    /**
     * A Primitive or Structured value that contains additional
     * information about the error. Can be omitted.
     */
    data?: D;
}
/**
 * An error object return in a response in case a request
 * has failed.
 */
export declare class ResponseError<D> extends Error {
    readonly code: number;
    readonly data: D | undefined;
    constructor(code: number, message: string, data?: D);
    toJson(): ResponseErrorLiteral<D>;
}
/**
 * A response message.
 */
export interface ResponseMessage extends Message {
    /**
     * The request id.
     */
    id: number | string | null;
    /**
     * The result of a request. This can be omitted in
     * the case of an error.
     */
    result?: any;
    /**
     * The error object in case a request fails.
     */
    error?: ResponseErrorLiteral<any>;
}
/**
 * A LSP Log Entry.
 */
export declare type LSPMessageType = 'send-request' | 'receive-request' | 'send-response' | 'receive-response' | 'send-notification' | 'receive-notification';
export interface LSPLogMessage {
    type: LSPMessageType;
    message: RequestMessage | ResponseMessage | NotificationMessage;
    timestamp: number;
}
/**
 * An interface to type messages.
 */
export interface MessageType {
    readonly method: string;
    readonly numberOfParams: number;
}
/**
 * An abstract implementation of a MessageType.
 */
export declare abstract class AbstractMessageType implements MessageType {
    private _method;
    private _numberOfParams;
    constructor(_method: string, _numberOfParams: number);
    readonly method: string;
    readonly numberOfParams: number;
}
/**
 * End marker interface for request and notification types.
 */
export interface _EM {
    _$endMarker$_: number;
}
/**
 * Classes to type request response pairs
 */
export declare class RequestType0<R, E, RO> extends AbstractMessageType {
    readonly _?: [R, E, RO, _EM];
    constructor(method: string);
}
export declare class RequestType<P, R, E, RO> extends AbstractMessageType {
    readonly _?: [P, R, E, RO, _EM];
    constructor(method: string);
}
export declare class RequestType1<P1, R, E, RO> extends AbstractMessageType {
    readonly _?: [P1, R, E, RO, _EM];
    constructor(method: string);
}
export declare class RequestType2<P1, P2, R, E, RO> extends AbstractMessageType {
    readonly _?: [P1, P2, R, E, RO, _EM];
    constructor(method: string);
}
export declare class RequestType3<P1, P2, P3, R, E, RO> extends AbstractMessageType {
    readonly _?: [P1, P2, P3, R, E, RO, _EM];
    constructor(method: string);
}
export declare class RequestType4<P1, P2, P3, P4, R, E, RO> extends AbstractMessageType {
    readonly _?: [P1, P2, P3, P4, R, E, RO, _EM];
    constructor(method: string);
}
export declare class RequestType5<P1, P2, P3, P4, P5, R, E, RO> extends AbstractMessageType {
    readonly _?: [P1, P2, P3, P4, P5, R, E, RO, _EM];
    constructor(method: string);
}
export declare class RequestType6<P1, P2, P3, P4, P5, P6, R, E, RO> extends AbstractMessageType {
    readonly _?: [P1, P2, P3, P4, P5, P6, R, E, RO, _EM];
    constructor(method: string);
}
export declare class RequestType7<P1, P2, P3, P4, P5, P6, P7, R, E, RO> extends AbstractMessageType {
    readonly _?: [P1, P2, P3, P4, P5, P6, P7, R, E, RO, _EM];
    constructor(method: string);
}
export declare class RequestType8<P1, P2, P3, P4, P5, P6, P7, P8, R, E, RO> extends AbstractMessageType {
    readonly _?: [P1, P2, P3, P4, P5, P6, P7, P8, R, E, RO, _EM];
    constructor(method: string);
}
export declare class RequestType9<P1, P2, P3, P4, P5, P6, P7, P8, P9, R, E, RO> extends AbstractMessageType {
    readonly _?: [P1, P2, P3, P4, P5, P6, P7, P8, P9, R, E, RO, _EM];
    constructor(method: string);
}
/**
 * Notification Message
 */
export interface NotificationMessage extends Message {
    /**
     * The method to be invoked.
     */
    method: string;
    /**
     * The notification's params.
     */
    params?: any;
}
export declare class NotificationType<P, RO> extends AbstractMessageType {
    readonly _?: [P, RO, _EM];
    constructor(method: string);
}
export declare class NotificationType0<RO> extends AbstractMessageType {
    readonly _?: [RO, _EM];
    constructor(method: string);
}
export declare class NotificationType1<P1, RO> extends AbstractMessageType {
    readonly _?: [P1, RO, _EM];
    constructor(method: string);
}
export declare class NotificationType2<P1, P2, RO> extends AbstractMessageType {
    readonly _?: [P1, P2, RO, _EM];
    constructor(method: string);
}
export declare class NotificationType3<P1, P2, P3, RO> extends AbstractMessageType {
    readonly _?: [P1, P2, P3, RO, _EM];
    constructor(method: string);
}
export declare class NotificationType4<P1, P2, P3, P4, RO> extends AbstractMessageType {
    readonly _?: [P1, P2, P3, P4, RO, _EM];
    constructor(method: string);
}
export declare class NotificationType5<P1, P2, P3, P4, P5, RO> extends AbstractMessageType {
    readonly _?: [P1, P2, P3, P4, P5, RO, _EM];
    constructor(method: string);
}
export declare class NotificationType6<P1, P2, P3, P4, P5, P6, RO> extends AbstractMessageType {
    readonly _?: [P1, P2, P3, P4, P5, P6, RO, _EM];
    constructor(method: string);
}
export declare class NotificationType7<P1, P2, P3, P4, P5, P6, P7, RO> extends AbstractMessageType {
    readonly _?: [P1, P2, P3, P4, P5, P6, P7, RO, _EM];
    constructor(method: string);
}
export declare class NotificationType8<P1, P2, P3, P4, P5, P6, P7, P8, RO> extends AbstractMessageType {
    readonly _?: [P1, P2, P3, P4, P5, P6, P7, P8, RO, _EM];
    constructor(method: string);
}
export declare class NotificationType9<P1, P2, P3, P4, P5, P6, P7, P8, P9, RO> extends AbstractMessageType {
    readonly _?: [P1, P2, P3, P4, P5, P6, P7, P8, P9, RO, _EM];
    constructor(method: string);
}
/**
 * Tests if the given message is a request message
 */
export declare function isRequestMessage(message: Message | undefined): message is RequestMessage;
/**
 * Tests if the given message is a notification message
 */
export declare function isNotificationMessage(message: Message | undefined): message is NotificationMessage;
/**
 * Tests if the given message is a response message
 */
export declare function isResponseMessage(message: Message | undefined): message is ResponseMessage;
