/// <reference types="node" />
import { Socket } from 'net';
import { ChildProcess } from 'child_process';
import { Message } from './messages';
import { Event } from './events';
export interface DataCallback {
    (data: Message): void;
}
export interface PartialMessageInfo {
    readonly messageToken: number;
    readonly waitingTime: number;
}
export interface MessageReader {
    readonly onError: Event<Error>;
    readonly onClose: Event<void>;
    readonly onPartialMessage: Event<PartialMessageInfo>;
    listen(callback: DataCallback): void;
    dispose(): void;
}
export declare namespace MessageReader {
    function is(value: any): value is MessageReader;
}
export declare abstract class AbstractMessageReader {
    private errorEmitter;
    private closeEmitter;
    private partialMessageEmitter;
    constructor();
    dispose(): void;
    readonly onError: Event<Error>;
    protected fireError(error: any): void;
    readonly onClose: Event<void>;
    protected fireClose(): void;
    readonly onPartialMessage: Event<PartialMessageInfo>;
    protected firePartialMessage(info: PartialMessageInfo): void;
    private asError;
}
export declare class StreamMessageReader extends AbstractMessageReader implements MessageReader {
    private readable;
    private callback;
    private buffer;
    private nextMessageLength;
    private messageToken;
    private partialMessageTimer;
    private _partialMessageTimeout;
    constructor(readable: NodeJS.ReadableStream, encoding?: string);
    partialMessageTimeout: number;
    listen(callback: DataCallback): void;
    private onData;
    private clearPartialMessageTimer;
    private setPartialMessageTimer;
}
export declare class IPCMessageReader extends AbstractMessageReader implements MessageReader {
    private process;
    constructor(process: NodeJS.Process | ChildProcess);
    listen(callback: DataCallback): void;
}
export declare class SocketMessageReader extends StreamMessageReader {
    constructor(socket: Socket, encoding?: string);
}
