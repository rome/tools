import { MessageReader } from './messageReader';
import { MessageWriter } from './messageWriter';
export declare function generateRandomPipeName(): string;
export interface PipeTransport {
    onConnected(): Thenable<[MessageReader, MessageWriter]>;
}
export declare function createClientPipeTransport(pipeName: string, encoding?: string): Thenable<PipeTransport>;
export declare function createServerPipeTransport(pipeName: string, encoding?: string): [MessageReader, MessageWriter];
