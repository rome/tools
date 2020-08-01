/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {EventSubscription, EventSubscriptions} from "./types";
import {AsyncVoidCallback} from "@internal/typescript-helpers";

export function mergeEventSubscriptions(
	subs: EventSubscriptions,
): EventSubscription {
	return {
		async unsubscribe() {
			for (const sub of subs) {
				await sub.unsubscribe();
			}
		},
	};
}

export type SubscriptionWrapperHelpers = {
	add: (sub: EventSubscription) => void;
	unsubscribe: AsyncVoidCallback;
};
type SubscriptionWrapperCallback<Ret, Args extends Array<unknown>> = (
	helper: SubscriptionWrapperHelpers,
	...args: Args
) => Promise<Ret>;

// A safe way to wrap subscriptions and ensure they're properly closed on errors
export function wrapSubscriptionConsumer<Ret, Args extends Array<unknown>>(
	callback: SubscriptionWrapperCallback<Ret, Args>,
): (...args: Args) => Promise<Ret> {
	return async function(...args: Args): Promise<Ret> {
		const subscriptions: EventSubscriptions = [];

		const helper: SubscriptionWrapperHelpers = {
			add(sub: EventSubscription) {
				subscriptions.push(sub);
			},
			async unsubscribe() {
				await mergeEventSubscriptions(subscriptions).unsubscribe();
			},
		};

		try {
			return await callback(helper, ...args);
		} finally {
			await helper.unsubscribe();
		}
	};
}
