import { Event } from './events';
/**
 * Defines a CancellationToken. This interface is not
 * intended to be implemented. A CancellationToken must
 * be created via a CancellationTokenSource.
 */
export interface CancellationToken {
    /**
     * Is `true` when the token has been cancelled, `false` otherwise.
     */
    readonly isCancellationRequested: boolean;
    /**
     * An [event](#Event) which fires upon cancellation.
     */
    readonly onCancellationRequested: Event<any>;
}
export declare namespace CancellationToken {
    const None: CancellationToken;
    const Cancelled: CancellationToken;
    function is(value: any): value is CancellationToken;
}
export declare class CancellationTokenSource {
    private _token;
    readonly token: CancellationToken;
    cancel(): void;
    dispose(): void;
}
