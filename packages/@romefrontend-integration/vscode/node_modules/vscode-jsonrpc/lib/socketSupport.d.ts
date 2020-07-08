import { MessageReader } from './messageReader';
import { MessageWriter } from './messageWriter';
export interface SocketTransport {
    onConnected(): Thenable<[MessageReader, MessageWriter]>;
}
export declare function createClientSocketTransport(port: number, encoding?: string): Thenable<SocketTransport>;
export declare function createServerSocketTransport(port: number, encoding?: string): [MessageReader, MessageWriter];
