export default function createDeferredPromise<T = void>(): {
	resolve: (res: T) => void;
	promise: Promise<T>;
	reject: (err: Error) => void;
} {
	let resolve: undefined | ((res: T) => void);
	let reject: undefined | ((err: Error) => void);
	const promise: Promise<T> = new Promise((_resolve, _reject) => {
		resolve = _resolve;
		reject = _reject;
	});

	if (resolve === undefined || reject === undefined) {
		throw new Error("Promise factory should have been executed...");
	}

	return {resolve, reject, promise};
}
