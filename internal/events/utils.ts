/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import { DIAGNOSTIC_CATEGORIES, isDiagnosticErrorOfCategory } from "@internal/diagnostics";
import { AsyncVoidCallback } from "@internal/typescript-helpers";
import Event from "./Event";
import {EventSubscription, PartialEventSubscription} from "./types";

// Allow event subscriptions to be easily merged
export function createEventSubscription(...seed: (PartialEventSubscription | EventSubscription | AsyncVoidCallback)[]): EventSubscription {
	let subscriptions: Set<PartialEventSubscription> = new Set();
	const onUnsubscribeEvent: EventSubscription["onUnsubscribeEvent"] = new Event("onUnsubscribe");
	
	let wrapper: EventSubscription = {
		addDependency(sub: EventSubscription) {
			sub.onUnsubscribeEvent.subscribe(() => {
				subscriptions.delete(sub);
			});

			subscriptions.add(sub);
		},
		onUnsubscribeEvent,
		async unsubscribe() {
			// Return true if at least one subscription was unsubscribed
			let unsubscribed = false;

			let currSubscriptions = subscriptions;
			subscriptions = new Set();
			await Promise.all(Array.from(currSubscriptions, async (sub) => {
				if (await sub.unsubscribe()) {
					unsubscribed = true;
				}
			}));

			if (unsubscribed) {
				await onUnsubscribeEvent.callOptional();
			}
			
			return unsubscribed;
		},
	};

	for (const elem of seed) {
		if (typeof elem === "function") {
			subscriptions.add(createSubscriptionFromCallback(elem));
		} else {
			if ("onUnsubscribeEvent" in elem) {
				wrapper.addDependency(elem);
			} else {
				subscriptions.add(elem);
			}
		}
	}

	return wrapper;
}

function createSubscriptionFromCallback(callback: AsyncVoidCallback): PartialEventSubscription {
	let subscribed = true;
	return {
		async unsubscribe() {
			if (subscribed) {
				subscribed = false;
				if (callback !== undefined) {
					await callback();
				}
				return true;
			} else {
				return false;
			}
		},
	};
}

export function isBridgeClosedDiagnosticError(err: Error): boolean {
	return isDiagnosticErrorOfCategory(err, DIAGNOSTIC_CATEGORIES["bridge/closed"]);
}