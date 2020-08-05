/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {EventSubscription, EventSubscriptions} from "./types";

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

export function createEmptySubscription(): EventSubscription {
	return {
		async unsubscribe() {},
	};
}

export interface SubscriptionWrapperHelpers extends EventSubscription {
	add: (sub: EventSubscription) => void;
}

type SubscriptionWrapperCallback<Ret, Args extends Array<unknown>> = (
	helper: SubscriptionWrapperHelpers,
	...args: Args
) => Promise<Ret>;

export function createSubscriptionHelper(): SubscriptionWrapperHelpers {
	const subscriptions: EventSubscriptions = [];

	return {
		add(sub: EventSubscription) {
			subscriptions.push(sub);
		},
		async unsubscribe() {
			await mergeEventSubscriptions(subscriptions).unsubscribe();
		},
	};
}

// A safe way to wrap subscriptions and ensure they're properly closed on errors
export function wrapSubscriptionConsumer<Ret, Args extends Array<unknown>>(
	callback: SubscriptionWrapperCallback<Ret, Args>,
): (...args: Args) => Promise<Ret> {
	return async function(...args: Args): Promise<Ret> {
		const helper = createSubscriptionHelper();

		try {
			return await callback(helper, ...args);
		} finally {
			await helper.unsubscribe();
		}
	};
}
