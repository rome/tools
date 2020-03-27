export interface ITask<T> {
    (): T;
}
export declare class Delayer<T> {
    defaultDelay: number;
    private timeout;
    private completionPromise;
    private onSuccess;
    private task;
    constructor(defaultDelay: number);
    trigger(task: ITask<T>, delay?: number): Promise<T>;
    forceDelivery(): T | undefined;
    isTriggered(): boolean;
    cancel(): void;
    private cancelTimeout;
}
