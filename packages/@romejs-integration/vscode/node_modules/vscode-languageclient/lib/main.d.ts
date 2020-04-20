/// <reference types="node" />
import * as cp from 'child_process';
import ChildProcess = cp.ChildProcess;
import { BaseLanguageClient, LanguageClientOptions, MessageTransports, StaticFeature, DynamicFeature } from './client';
import { Disposable } from 'vscode';
export * from './client';
export interface ExecutableOptions {
    cwd?: string;
    stdio?: string | string[];
    env?: any;
    detached?: boolean;
    shell?: boolean;
}
export interface Executable {
    command: string;
    args?: string[];
    options?: ExecutableOptions;
}
export interface ForkOptions {
    cwd?: string;
    env?: any;
    encoding?: string;
    execArgv?: string[];
}
export declare enum TransportKind {
    stdio = 0,
    ipc = 1,
    pipe = 2,
    socket = 3
}
export interface SocketTransport {
    kind: TransportKind.socket;
    port: number;
}
/**
 * To avoid any timing, pipe name or port number issues the pipe (TransportKind.pipe)
 * and the sockets (TransportKind.socket and SocketTransport) are owned by the
 * VS Code processes. The server process simply connects to the pipe / socket.
 * In node term the VS Code process calls `createServer`, then starts the server
 * process, waits until the server process has connected to the pipe / socket
 * and then signals that the connection has been established and messages can
 * be send back and forth. If the language server is implemented in a different
 * programm language the server simply needs to create a connection to the
 * passed pipe name or port number.
 */
export declare type Transport = TransportKind | SocketTransport;
export interface NodeModule {
    module: string;
    transport?: Transport;
    args?: string[];
    runtime?: string;
    options?: ForkOptions;
}
export interface StreamInfo {
    writer: NodeJS.WritableStream;
    reader: NodeJS.ReadableStream;
    detached?: boolean;
}
export interface ChildProcessInfo {
    process: ChildProcess;
    detached: boolean;
}
export declare type ServerOptions = Executable | {
    run: Executable;
    debug: Executable;
} | {
    run: NodeModule;
    debug: NodeModule;
} | NodeModule | (() => Thenable<ChildProcess | StreamInfo | MessageTransports | ChildProcessInfo>);
export declare class LanguageClient extends BaseLanguageClient {
    private _serverOptions;
    private _forceDebug;
    private _serverProcess;
    private _isDetached;
    constructor(name: string, serverOptions: ServerOptions, clientOptions: LanguageClientOptions, forceDebug?: boolean);
    constructor(id: string, name: string, serverOptions: ServerOptions, clientOptions: LanguageClientOptions, forceDebug?: boolean);
    private checkVersion;
    stop(): Thenable<void>;
    private checkProcessDied;
    protected handleConnectionClosed(): void;
    protected createMessageTransports(encoding: string): Thenable<MessageTransports>;
    registerProposedFeatures(): void;
    protected registerBuiltinFeatures(): void;
    private _mainGetRootPath;
    private _getServerWorkingDir;
}
export declare class SettingMonitor {
    private _client;
    private _setting;
    private _listeners;
    constructor(_client: LanguageClient, _setting: string);
    start(): Disposable;
    private onDidChangeConfiguration;
}
export declare namespace ProposedFeatures {
    function createAll(_client: BaseLanguageClient): (StaticFeature | DynamicFeature<any>)[];
}
