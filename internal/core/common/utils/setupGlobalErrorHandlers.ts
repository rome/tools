/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ErrorCallback, VoidCallback} from "@internal/typescript-helpers";

export default function setupGlobalErrorHandlers(
	callback: ErrorCallback,
): VoidCallback {
	const onUncaughtException: NodeJS.UncaughtExceptionListener = (err: Error) => {
		callback(err);
	};
	process.on("uncaughtException", onUncaughtException);

	const onUnhandledRejection: NodeJS.UnhandledRejectionListener = (
		reason: unknown,
		promise: Promise<unknown>,
	) => {
		promise.then(() => {
			throw new Error("Promise is rejected so should never hit this condition");
		}).catch((err) => {
			callback(err);
		});
	};
	process.on("unhandledRejection", onUnhandledRejection);

	return () => {
		process.removeListener("uncaughtException", onUncaughtException);
		process.removeListener("unhandledRejection", onUnhandledRejection);
	};
}
